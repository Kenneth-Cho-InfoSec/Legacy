use std::f32::consts::TAU;
use std::sync::Arc;
use std::sync::atomic::{AtomicU8, AtomicU32, AtomicU64, Ordering};

use radiant_engine::{LiveChannelConfig, LiveGraphPublisher, LiveGraphRuntime, LiveGraphTopology};
use radiant_media::{AudioBuffer, sanitize};
use rtrb::{Consumer, Producer, RingBuffer};

use crate::AudioError;

const MIDI_VOICE_COUNT: usize = 32;
const REALTIME_BLOCK_CAPACITY: usize = 8192;

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
#[repr(u8)]
pub enum RealtimeState {
    #[default]
    Stopped = 0,
    Playing = 1,
    Paused = 2,
}

impl From<u8> for RealtimeState {
    fn from(value: u8) -> Self {
        match value {
            1 => Self::Playing,
            2 => Self::Paused,
            _ => Self::Stopped,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum AudioCommand {
    Play,
    Pause,
    Stop,
    Seek(u64),
    SetLoop(Option<(u64, u64)>),
    SetRecording(bool),
    SetRecordTap(bool),
    AllNotesOff,
    SetMixerChannel {
        index: usize,
        gain: f32,
        pan: f32,
        muted: bool,
        solo: bool,
        phase_inverted: bool,
        stereo_width: f32,
    },
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MidiMessageKind {
    NoteOn,
    NoteOff,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MidiMessage {
    pub audio_frame: u64,
    pub kind: MidiMessageKind,
    pub channel: u8,
    pub pitch: u8,
    pub velocity: u8,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RecordedMidiEvent {
    pub playhead_frame: u64,
    pub kind: MidiMessageKind,
    pub channel: u8,
    pub pitch: u8,
    pub velocity: u8,
}

#[derive(Debug, Default)]
pub(crate) struct RealtimeCounters {
    playhead_frame: AtomicU64,
    audio_clock_frame: AtomicU64,
    callback_clock_nanos: AtomicU64,
    callback_cycles: AtomicU64,
    peak_left: AtomicU32,
    peak_right: AtomicU32,
    rms_left: AtomicU32,
    rms_right: AtomicU32,
    clipped: AtomicU8,
    xruns: AtomicU64,
    deadline_misses: AtomicU64,
    command_overflows: AtomicU64,
    midi_overflows: AtomicU64,
    record_overflows: AtomicU64,
    reconnects: AtomicU64,
    quantum: AtomicU32,
    sample_rate: AtomicU32,
    state: AtomicU8,
    fault: AtomicU8,
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct RealtimeDiagnostics {
    pub playhead_frame: u64,
    pub audio_clock_frame: u64,
    pub callback_cycles: u64,
    pub peak_left: f32,
    pub peak_right: f32,
    pub rms_left: f32,
    pub rms_right: f32,
    pub clipped: bool,
    pub xruns: u64,
    pub deadline_misses: u64,
    pub command_overflows: u64,
    pub midi_overflows: u64,
    pub record_overflows: u64,
    pub reconnects: u64,
    pub quantum: u32,
    pub sample_rate: u32,
    pub state: RealtimeState,
    pub faulted: bool,
}

impl RealtimeCounters {
    fn snapshot(&self) -> RealtimeDiagnostics {
        RealtimeDiagnostics {
            playhead_frame: self.playhead_frame.load(Ordering::Relaxed),
            audio_clock_frame: self.audio_clock_frame.load(Ordering::Relaxed),
            callback_cycles: self.callback_cycles.load(Ordering::Relaxed),
            peak_left: f32::from_bits(self.peak_left.load(Ordering::Relaxed)),
            peak_right: f32::from_bits(self.peak_right.load(Ordering::Relaxed)),
            rms_left: f32::from_bits(self.rms_left.load(Ordering::Relaxed)),
            rms_right: f32::from_bits(self.rms_right.load(Ordering::Relaxed)),
            clipped: self.clipped.load(Ordering::Relaxed) != 0,
            xruns: self.xruns.load(Ordering::Relaxed),
            deadline_misses: self.deadline_misses.load(Ordering::Relaxed),
            command_overflows: self.command_overflows.load(Ordering::Relaxed),
            midi_overflows: self.midi_overflows.load(Ordering::Relaxed),
            record_overflows: self.record_overflows.load(Ordering::Relaxed),
            reconnects: self.reconnects.load(Ordering::Relaxed),
            quantum: self.quantum.load(Ordering::Relaxed),
            sample_rate: self.sample_rate.load(Ordering::Relaxed),
            state: self.state.load(Ordering::Relaxed).into(),
            faulted: self.fault.load(Ordering::Acquire) != 0,
        }
    }

    pub(crate) fn clock_snapshot(&self) -> (u64, u64) {
        (
            self.audio_clock_frame.load(Ordering::Acquire),
            self.callback_clock_nanos.load(Ordering::Acquire),
        )
    }

    pub(crate) fn report_stream_error(&self, xrun: bool, fault: bool) {
        if xrun {
            self.xruns.fetch_add(1, Ordering::Relaxed);
        }
        if fault {
            self.fault.store(1, Ordering::Release);
        }
    }
}

pub struct RealtimeController {
    commands: Producer<AudioCommand>,
    recorded: Consumer<RecordedMidiEvent>,
    audio_recorded: Consumer<f32>,
    graph_publisher: LiveGraphPublisher,
    counters: Arc<RealtimeCounters>,
}

impl RealtimeController {
    pub fn send(&mut self, command: AudioCommand) -> Result<(), AudioError> {
        self.commands.push(command).map_err(|_| {
            self.counters
                .command_overflows
                .fetch_add(1, Ordering::Relaxed);
            AudioError::QueueFull
        })
    }

    pub fn pop_recorded(&mut self) -> Option<RecordedMidiEvent> {
        self.recorded.pop().ok()
    }

    pub fn pop_recorded_audio(&mut self) -> Option<f32> {
        self.audio_recorded.pop().ok()
    }

    pub fn drain_audio_tap(&mut self, destination: &mut Vec<f32>, maximum_samples: usize) {
        for _ in 0..maximum_samples {
            let Some(sample) = self.pop_recorded_audio() else {
                break;
            };
            destination.push(sample);
        }
    }

    pub fn publish_live_topology(&self, topology: LiveGraphTopology) {
        self.graph_publisher.publish(topology);
    }

    #[must_use]
    pub fn diagnostics(&self) -> RealtimeDiagnostics {
        self.counters.snapshot()
    }

    pub(crate) fn counters(&self) -> Arc<RealtimeCounters> {
        self.counters.clone()
    }
}

pub struct MidiIngress {
    messages: Producer<MidiMessage>,
    counters: Arc<RealtimeCounters>,
}

impl MidiIngress {
    pub fn send(&mut self, message: MidiMessage) -> Result<(), AudioError> {
        self.messages.push(message).map_err(|_| {
            self.counters.midi_overflows.fetch_add(1, Ordering::Relaxed);
            AudioError::QueueFull
        })
    }
}

#[derive(Clone, Copy, Debug, Default)]
struct MidiVoice {
    active: bool,
    channel: u8,
    pitch: u8,
    velocity: f32,
    phase: f32,
}

pub struct RealtimeProcessor {
    audio: Arc<AudioBuffer>,
    commands: Consumer<AudioCommand>,
    midi: Consumer<MidiMessage>,
    recorded: Producer<RecordedMidiEvent>,
    counters: Arc<RealtimeCounters>,
    state: RealtimeState,
    playhead_frame: u64,
    audio_clock_frame: u64,
    loop_range: Option<(u64, u64)>,
    recording: bool,
    record_tap: bool,
    audio_recorded: Producer<f32>,
    live_graph: LiveGraphRuntime,
    graph_input: Vec<[f32; 2]>,
    graph_output: Vec<[f32; 2]>,
    voices: [MidiVoice; MIDI_VOICE_COUNT],
    frequencies: [f32; 128],
}

#[must_use]
pub fn create_realtime(
    audio: Arc<AudioBuffer>,
    command_capacity: usize,
    midi_capacity: usize,
    record_capacity: usize,
) -> (RealtimeController, MidiIngress, RealtimeProcessor) {
    let (command_tx, command_rx) = RingBuffer::new(command_capacity.max(1));
    let (midi_tx, midi_rx) = RingBuffer::new(midi_capacity.max(1));
    let (record_tx, record_rx) = RingBuffer::new(record_capacity.max(1));
    let (audio_record_tx, audio_record_rx) = RingBuffer::new(record_capacity.max(1) * 2);
    let topology = LiveGraphTopology::compile(&[LiveChannelConfig::default()], &[], 0)
        .expect("single master live topology");
    let live_graph = LiveGraphRuntime::new(topology, 32);
    let graph_publisher = live_graph.publisher();
    let counters = Arc::new(RealtimeCounters::default());
    counters
        .sample_rate
        .store(audio.sample_rate(), Ordering::Relaxed);
    let controller = RealtimeController {
        commands: command_tx,
        recorded: record_rx,
        audio_recorded: audio_record_rx,
        graph_publisher: graph_publisher.clone(),
        counters: counters.clone(),
    };
    let midi = MidiIngress {
        messages: midi_tx,
        counters: counters.clone(),
    };
    let processor = RealtimeProcessor::new(
        audio,
        command_rx,
        midi_rx,
        record_tx,
        audio_record_tx,
        live_graph,
        counters,
    );
    (controller, midi, processor)
}

impl RealtimeProcessor {
    fn new(
        audio: Arc<AudioBuffer>,
        commands: Consumer<AudioCommand>,
        midi: Consumer<MidiMessage>,
        recorded: Producer<RecordedMidiEvent>,
        audio_recorded: Producer<f32>,
        live_graph: LiveGraphRuntime,
        counters: Arc<RealtimeCounters>,
    ) -> Self {
        let mut frequencies = [0.0; 128];
        for (pitch, frequency) in frequencies.iter_mut().enumerate() {
            #[allow(clippy::cast_precision_loss)]
            let pitch = pitch as f32;
            *frequency = 440.0 * 2.0_f32.powf((pitch - 69.0) / 12.0);
        }
        Self {
            audio,
            commands,
            midi,
            recorded,
            audio_recorded,
            live_graph,
            graph_input: vec![[0.0; 2]; REALTIME_BLOCK_CAPACITY],
            graph_output: vec![[0.0; 2]; REALTIME_BLOCK_CAPACITY],
            counters,
            state: RealtimeState::Stopped,
            playhead_frame: 0,
            audio_clock_frame: 0,
            loop_range: None,
            recording: false,
            record_tap: false,
            voices: [MidiVoice::default(); MIDI_VOICE_COUNT],
            frequencies,
        }
    }

    pub fn process(&mut self, output: &mut [f32], channels: usize) {
        if channels == 0 {
            return;
        }
        self.apply_commands();
        let frame_count = output.len() / channels;
        let mut peak = [0.0_f32; 2];
        let mut square_sum = [0.0_f64; 2];
        let mut clipped = false;

        let mut processed_frames = 0;
        while processed_frames < frame_count {
            let block_frames = (frame_count - processed_frames).min(self.graph_input.len());
            for index in 0..block_frames {
                self.apply_due_midi();
                let mut sample = if self.state == RealtimeState::Playing {
                    [
                        self.audio.sample(0, self.playhead_index()),
                        self.audio.sample(
                            usize::from(self.audio.channel_count() > 1),
                            self.playhead_index(),
                        ),
                    ]
                } else {
                    [0.0, 0.0]
                };
                let monitored = self.render_midi_voices();
                sample[0] = sanitize(sample[0] + monitored);
                sample[1] = sanitize(sample[1] + monitored);
                self.graph_input[index] = sample;
                self.advance_playhead();
                self.audio_clock_frame = self.audio_clock_frame.saturating_add(1);
            }
            let input = [&self.graph_input[..block_frames]];
            self.live_graph
                .process_block(&input, &mut self.graph_output[..block_frames]);
            for (index, frame) in output
                .chunks_exact_mut(channels)
                .skip(processed_frames)
                .take(block_frames)
                .enumerate()
            {
                let sample = self.graph_output[index];
                if self.record_tap {
                    let _ = self.audio_recorded.push(sample[0]);
                    let _ = self.audio_recorded.push(sample[1]);
                }
                frame[0] = sample[0];
                if channels > 1 {
                    frame[1] = sample[1];
                }
                frame.iter_mut().skip(2).for_each(|value| *value = 0.0);
                for channel in 0..2 {
                    clipped |= sample[channel].abs() >= 1.0;
                    peak[channel] = peak[channel].max(sample[channel].abs());
                    square_sum[channel] += f64::from(sample[channel]) * f64::from(sample[channel]);
                }
            }
            processed_frames += block_frames;
        }

        #[allow(clippy::cast_precision_loss)]
        let divisor = frame_count.max(1) as f64;
        self.counters
            .peak_left
            .store(peak[0].to_bits(), Ordering::Relaxed);
        self.counters
            .peak_right
            .store(peak[1].to_bits(), Ordering::Relaxed);
        #[allow(clippy::cast_possible_truncation)]
        let rms_left = (square_sum[0] / divisor).sqrt() as f32;
        #[allow(clippy::cast_possible_truncation)]
        let rms_right = (square_sum[1] / divisor).sqrt() as f32;
        self.counters
            .rms_left
            .store(rms_left.to_bits(), Ordering::Relaxed);
        self.counters
            .rms_right
            .store(rms_right.to_bits(), Ordering::Relaxed);
        if clipped {
            self.counters.clipped.store(1, Ordering::Relaxed);
        }
        self.counters
            .playhead_frame
            .store(self.playhead_frame, Ordering::Release);
        self.counters
            .audio_clock_frame
            .store(self.audio_clock_frame, Ordering::Release);
        self.counters.quantum.store(
            u32::try_from(frame_count).unwrap_or(u32::MAX),
            Ordering::Relaxed,
        );
        self.counters
            .state
            .store(self.state as u8, Ordering::Release);
        self.counters
            .callback_cycles
            .fetch_add(1, Ordering::Relaxed);
    }

    fn apply_commands(&mut self) {
        while let Ok(command) = self.commands.pop() {
            match command {
                AudioCommand::Play => self.state = RealtimeState::Playing,
                AudioCommand::Pause => self.state = RealtimeState::Paused,
                AudioCommand::Stop => {
                    self.state = RealtimeState::Stopped;
                    self.playhead_frame = 0;
                    self.voices.fill(MidiVoice::default());
                }
                AudioCommand::Seek(frame) => self.playhead_frame = frame,
                AudioCommand::SetLoop(range) => {
                    self.loop_range = range.filter(|(start, end)| end > start);
                }
                AudioCommand::SetRecording(recording) => self.recording = recording,
                AudioCommand::SetRecordTap(record_tap) => self.record_tap = record_tap,
                AudioCommand::SetMixerChannel {
                    index,
                    gain,
                    pan,
                    muted,
                    solo,
                    phase_inverted,
                    stereo_width,
                } => {
                    let _ = self.live_graph.set_channel_config(
                        index,
                        LiveChannelConfig {
                            gain,
                            pan,
                            muted,
                            solo,
                            phase_inverted,
                            stereo_width,
                            compensation_delay: 0,
                        },
                    );
                }
                AudioCommand::AllNotesOff => self.voices.fill(MidiVoice::default()),
            }
        }
    }

    fn apply_due_midi(&mut self) {
        while self
            .midi
            .peek()
            .is_ok_and(|message| message.audio_frame <= self.audio_clock_frame)
        {
            let Ok(message) = self.midi.pop() else {
                break;
            };
            if self.recording {
                let recorded = RecordedMidiEvent {
                    playhead_frame: self.playhead_frame,
                    kind: message.kind,
                    channel: message.channel,
                    pitch: message.pitch,
                    velocity: message.velocity,
                };
                if self.recorded.push(recorded).is_err() {
                    self.counters
                        .record_overflows
                        .fetch_add(1, Ordering::Relaxed);
                }
            }
            match message.kind {
                MidiMessageKind::NoteOn if message.velocity > 0 => self.note_on(message),
                MidiMessageKind::NoteOn | MidiMessageKind::NoteOff => self.note_off(message),
            }
        }
    }

    fn note_on(&mut self, message: MidiMessage) {
        let index = self
            .voices
            .iter()
            .position(|voice| !voice.active)
            .unwrap_or(0);
        self.voices[index] = MidiVoice {
            active: true,
            channel: message.channel,
            pitch: message.pitch.min(127),
            velocity: f32::from(message.velocity) / 127.0,
            phase: 0.0,
        };
    }

    fn note_off(&mut self, message: MidiMessage) {
        for voice in &mut self.voices {
            if voice.active && voice.channel == message.channel && voice.pitch == message.pitch {
                voice.active = false;
            }
        }
    }

    #[allow(clippy::cast_precision_loss)]
    fn render_midi_voices(&mut self) -> f32 {
        let mut sample = 0.0;
        let sample_rate = self.audio.sample_rate() as f32;
        for voice in &mut self.voices {
            if !voice.active {
                continue;
            }
            sample += voice.phase.sin() * voice.velocity * 0.15;
            voice.phase += TAU * self.frequencies[usize::from(voice.pitch)] / sample_rate;
            if voice.phase >= TAU {
                voice.phase -= TAU;
            }
        }
        sanitize(sample)
    }

    fn playhead_index(&self) -> usize {
        usize::try_from(self.playhead_frame).unwrap_or(usize::MAX)
    }

    fn advance_playhead(&mut self) {
        if self.state != RealtimeState::Playing {
            return;
        }
        self.playhead_frame = self.playhead_frame.saturating_add(1);
        if let Some((start, end)) = self.loop_range {
            if self.playhead_frame >= end {
                self.playhead_frame = start + (self.playhead_frame - start) % (end - start);
            }
        } else if self.playhead_index() >= self.audio.frames() {
            self.state = RealtimeState::Stopped;
            self.playhead_frame = 0;
        }
    }

    pub(crate) fn set_callback_clock_nanos(&self, nanos: u64) {
        self.counters
            .callback_clock_nanos
            .store(nanos, Ordering::Release);
    }

    pub(crate) fn report_deadline_miss(&self) {
        self.counters
            .deadline_misses
            .fetch_add(1, Ordering::Relaxed);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_audio(frames: usize) -> Arc<AudioBuffer> {
        let left: Vec<_> = (0..frames)
            .map(|frame| f32::from(u16::try_from(frame % 100).unwrap_or(0)) / 100.0)
            .collect();
        let right: Vec<_> = left
            .iter()
            .map(|sample| {
                if sample.abs() < f32::EPSILON {
                    0.0
                } else {
                    -*sample
                }
            })
            .collect();
        Arc::new(AudioBuffer::new(48_000, vec![left, right]).expect("audio"))
    }

    #[test]
    fn realtime_blocks_match_offline_snapshot_exactly() {
        let audio = test_audio(1_024);
        let (mut controller, _midi, mut processor) = create_realtime(audio.clone(), 8, 8, 8);
        controller.send(AudioCommand::Play).expect("play");
        let mut rendered = vec![0.0; 1_024 * 2];
        for block in rendered.chunks_mut(74) {
            processor.process(block, 2);
        }
        for frame in 0..1_024 {
            assert_eq!(
                rendered[frame * 2].to_bits(),
                audio.sample(0, frame).to_bits()
            );
            assert_eq!(
                rendered[frame * 2 + 1].to_bits(),
                audio.sample(1, frame).to_bits()
            );
        }
    }

    #[test]
    fn transport_seek_pause_stop_and_loop_are_sample_exact() {
        let audio = test_audio(100);
        let (mut controller, _midi, mut processor) = create_realtime(audio, 16, 8, 8);
        controller.send(AudioCommand::Seek(10)).expect("seek");
        controller
            .send(AudioCommand::SetLoop(Some((10, 14))))
            .expect("loop");
        controller.send(AudioCommand::Play).expect("play");
        let mut output = [0.0; 12];
        processor.process(&mut output, 2);
        assert_eq!(controller.diagnostics().playhead_frame, 12);
        assert_eq!(
            &output[0..8],
            &[0.1, -0.1, 0.11, -0.11, 0.12, -0.12, 0.13, -0.13]
        );
        controller.send(AudioCommand::Pause).expect("pause");
        processor.process(&mut output[..4], 2);
        assert_eq!(controller.diagnostics().playhead_frame, 12);
        controller.send(AudioCommand::Stop).expect("stop");
        processor.process(&mut output[..2], 2);
        assert_eq!(controller.diagnostics().state, RealtimeState::Stopped);
        assert_eq!(controller.diagnostics().playhead_frame, 0);
    }

    #[test]
    fn midi_monitoring_and_recording_use_audio_frame_timestamps() {
        let audio = Arc::new(AudioBuffer::silence(48_000, 2, 1_000));
        let (mut controller, mut midi, mut processor) = create_realtime(audio, 16, 16, 16);
        controller
            .send(AudioCommand::SetRecording(true))
            .expect("record arm");
        controller.send(AudioCommand::Play).expect("play");
        midi.send(MidiMessage {
            audio_frame: 4,
            kind: MidiMessageKind::NoteOn,
            channel: 2,
            pitch: 69,
            velocity: 127,
        })
        .expect("MIDI note");
        let mut output = [0.0; 24];
        processor.process(&mut output, 2);
        assert!(output[..8].iter().all(|sample| sample.abs() < f32::EPSILON));
        assert!(output[10..].iter().any(|sample| sample.abs() > 0.01));
        let recorded = controller.pop_recorded().expect("recorded event");
        assert_eq!(recorded.playhead_frame, 4);
        assert_eq!(recorded.channel, 2);
        assert_eq!(recorded.pitch, 69);
    }

    #[test]
    fn queue_overflow_is_visible_in_diagnostics() {
        let audio = test_audio(8);
        let (mut controller, mut midi, _processor) = create_realtime(audio, 1, 1, 1);
        controller.send(AudioCommand::Play).expect("first command");
        assert!(matches!(
            controller.send(AudioCommand::Pause),
            Err(AudioError::QueueFull)
        ));
        midi.send(MidiMessage {
            audio_frame: 0,
            kind: MidiMessageKind::NoteOn,
            channel: 0,
            pitch: 60,
            velocity: 100,
        })
        .expect("first MIDI event");
        assert!(
            midi.send(MidiMessage {
                audio_frame: 1,
                kind: MidiMessageKind::NoteOff,
                channel: 0,
                pitch: 60,
                velocity: 0,
            })
            .is_err()
        );
        let diagnostics = controller.diagnostics();
        assert_eq!(diagnostics.command_overflows, 1);
        assert_eq!(diagnostics.midi_overflows, 1);
    }

    #[test]
    fn sustained_callback_processing_allocates_nothing() {
        let audio = test_audio(8_192);
        let (mut controller, _midi, mut processor) = create_realtime(audio, 16, 16, 16);
        controller.send(AudioCommand::Play).expect("play");
        let mut output = [0.0_f32; 256];
        processor.process(&mut output, 2);
        let allocations = allocation_counter::measure(|| {
            for _ in 0..1_000 {
                processor.process(&mut output, 2);
            }
        });
        assert_eq!(allocations.count_total, 0);
        assert_eq!(allocations.bytes_total, 0);
        assert_eq!(controller.diagnostics().callback_cycles, 1_001);
    }

    #[test]
    fn post_mixer_record_tap_is_drained_off_callback() {
        let audio = test_audio(64);
        let (mut controller, _midi, mut processor) = create_realtime(audio, 16, 16, 16);
        controller
            .send(AudioCommand::SetRecordTap(true))
            .expect("tap");
        controller.send(AudioCommand::Play).expect("play");
        let mut output = [0.0_f32; 8];
        processor.process(&mut output, 2);
        let mut tapped = Vec::new();
        controller.drain_audio_tap(&mut tapped, 8);
        assert_eq!(tapped.len(), 8);
        assert_eq!(tapped, output.to_vec());
    }

    #[test]
    fn control_thread_can_publish_a_new_route_topology() {
        let audio = test_audio(64);
        let (mut controller, _midi, mut processor) = create_realtime(audio, 16, 16, 16);
        let configs = [LiveChannelConfig::default(); 2];
        let routes = [radiant_engine::LiveRoute {
            source: 0,
            destination: 1,
            gain: 0.5,
            pre_fader: false,
            sidechain: false,
        }];
        let topology = LiveGraphTopology::compile(&configs, &routes, 1).expect("topology");
        controller.publish_live_topology(topology);
        controller.send(AudioCommand::Play).expect("play");
        let mut output = [0.0_f32; 8];
        processor.process(&mut output, 2);
        assert!((output[2] - 0.005).abs() < 0.000_001);
    }

    #[test]
    fn faults_xruns_deadlines_and_record_overflow_are_observable() {
        let audio = Arc::new(AudioBuffer::silence(48_000, 2, 64));
        let (mut controller, mut midi, mut processor) = create_realtime(audio, 8, 8, 1);
        controller
            .send(AudioCommand::SetRecording(true))
            .expect("recording");
        for pitch in [60, 62] {
            midi.send(MidiMessage {
                audio_frame: 0,
                kind: MidiMessageKind::NoteOn,
                channel: 0,
                pitch,
                velocity: 100,
            })
            .expect("MIDI input");
        }
        let mut output = [0.0; 4];
        processor.process(&mut output, 2);
        processor.report_deadline_miss();
        controller.counters.report_stream_error(true, true);
        let diagnostics = controller.diagnostics();
        assert_eq!(diagnostics.record_overflows, 1);
        assert_eq!(diagnostics.deadline_misses, 1);
        assert_eq!(diagnostics.xruns, 1);
        assert!(diagnostics.faulted);
    }
}
