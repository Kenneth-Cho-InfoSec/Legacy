#![allow(clippy::cast_precision_loss)]

use serde::{Deserialize, Serialize};
use std::f64::consts::TAU;
use std::path::Path;

use radiant_domain::{AutomationTarget, ChannelId, Tick};
use radiant_media::{AudioBuffer, sanitize};

use crate::ChannelStrip;
use crate::EngineError;
use crate::automation::{SamplePoint, evaluate as evaluate_automation};
use crate::plan::{AutomationEvent, AutomationPoint, RenderPlan, Voice};

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum DitherMode {
    #[default]
    None,
    Triangular,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct RenderOptions {
    pub output_frames: Option<u64>,
    pub loop_range: Option<(Tick, Tick)>,
    pub metronome: bool,
    pub normalize_peak: Option<f32>,
    pub dither: DitherMode,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct ExportMetadata {
    pub title: String,
    pub artist: String,
    pub album: String,
    pub comments: String,
}

pub fn render_offline(
    plan: &RenderPlan,
    block_size: usize,
    options: RenderOptions,
) -> Result<AudioBuffer, EngineError> {
    if block_size == 0 {
        return Err(EngineError::InvalidBlockSize);
    }
    let total_frames = options.output_frames.unwrap_or(plan.length_frames);
    let total = usize::try_from(total_frames).map_err(|_| EngineError::RenderTooLong)?;
    let mut left = vec![0.0_f32; total];
    let mut right = vec![0.0_f32; total];
    let mut strips: Vec<_> = plan
        .inserts
        .iter()
        .map(|insert| {
            let mut strip = ChannelStrip::new(1.0, insert.pan, 64);
            strip.set_delay(usize::try_from(insert.compensation_delay).unwrap_or(usize::MAX));
            strip
        })
        .collect();
    let loop_frames = options.loop_range.and_then(|(start, end)| {
        let start = plan.timeline.tick_to_frame(start);
        let end = plan.timeline.tick_to_frame(end);
        (end > start).then_some((start, end))
    });

    for output_start in (0..total).step_by(block_size) {
        let output_end = output_start.saturating_add(block_size).min(total);
        render_block(
            plan,
            output_start,
            output_end,
            loop_frames,
            options.metronome,
            &mut left,
            &mut right,
            &mut strips,
            if loop_frames.is_some() { 0 } else { 64 },
            None,
        );
    }
    if let Some(target) = options
        .normalize_peak
        .filter(|value| value.is_finite() && *value > 0.0)
    {
        let peak = left
            .iter()
            .chain(&right)
            .map(|sample| sample.abs())
            .fold(0.0_f32, f32::max);
        if peak > 0.0 {
            let gain = target.min(1.0) / peak;
            for sample in left.iter_mut().chain(&mut right) {
                *sample *= gain;
            }
        }
    }
    if options.dither == DitherMode::Triangular {
        for (index, sample) in left.iter_mut().chain(&mut right).enumerate() {
            let a = ((index as u64)
                .wrapping_mul(1_103_515_245)
                .wrapping_add(12_345)
                & 0xffff) as f32
                / 65_535.0;
            let b = ((index as u64)
                .wrapping_mul(224_682_719)
                .wrapping_add(54_321)
                & 0xffff) as f32
                / 65_535.0;
            *sample += (a - b) / 8_388_608.0;
        }
    }
    for sample in left.iter_mut().chain(&mut right) {
        *sample = sanitize(*sample);
    }
    AudioBuffer::new(plan.sample_rate, vec![left, right]).map_err(EngineError::from)
}

pub fn export_wav(
    plan: &RenderPlan,
    path: impl AsRef<Path>,
    block_size: usize,
    options: RenderOptions,
) -> Result<(), EngineError> {
    let buffer = render_offline(plan, block_size, options)?;
    radiant_media::write_wav(path, &buffer).map_err(EngineError::from)
}

/// Exports audio and a portable JSON metadata sidecar. The WAV remains valid
/// for players that ignore metadata, while the sidecar preserves full fields.
pub fn export_wav_with_metadata(
    plan: &RenderPlan,
    path: impl AsRef<Path>,
    block_size: usize,
    options: RenderOptions,
    metadata: &ExportMetadata,
) -> Result<(), EngineError> {
    let path = path.as_ref();
    export_wav(plan, path, block_size, options)?;
    let sidecar = path.with_extension("wav.json");
    std::fs::write(sidecar, serde_json::to_vec_pretty(metadata)?)?;
    Ok(())
}

/// Renders one stem per mixer insert while preserving the compiled routing graph.
pub fn render_stems(
    plan: &RenderPlan,
    block_size: usize,
    options: RenderOptions,
) -> Result<Vec<AudioBuffer>, EngineError> {
    (0..plan.inserts.len())
        .map(|stem| render_offline_stem(plan, block_size, options, stem))
        .collect()
}

/// Bounces the immutable render snapshot without mutating the session.
pub fn bounce(
    plan: &RenderPlan,
    block_size: usize,
    options: RenderOptions,
) -> Result<AudioBuffer, EngineError> {
    render_offline(plan, block_size, options)
}

/// Renders a freeze cache from the same snapshot used for playback and export.
pub fn freeze(
    plan: &RenderPlan,
    block_size: usize,
    options: RenderOptions,
) -> Result<AudioBuffer, EngineError> {
    render_offline(plan, block_size, options)
}

fn render_offline_stem(
    plan: &RenderPlan,
    block_size: usize,
    options: RenderOptions,
    stem: usize,
) -> Result<AudioBuffer, EngineError> {
    if block_size == 0 {
        return Err(EngineError::InvalidBlockSize);
    }
    let total_frames = options.output_frames.unwrap_or(plan.length_frames);
    let total = usize::try_from(total_frames).map_err(|_| EngineError::RenderTooLong)?;
    let mut left = vec![0.0_f32; total];
    let mut right = vec![0.0_f32; total];
    let mut strips: Vec<_> = plan
        .inserts
        .iter()
        .map(|insert| {
            let mut strip = ChannelStrip::new(1.0, insert.pan, 64);
            strip.set_delay(usize::try_from(insert.compensation_delay).unwrap_or(usize::MAX));
            strip
        })
        .collect();
    let loop_frames = options.loop_range.and_then(|(start, end)| {
        let start = plan.timeline.tick_to_frame(start);
        let end = plan.timeline.tick_to_frame(end);
        (end > start).then_some((start, end))
    });
    for output_start in (0..total).step_by(block_size) {
        let output_end = output_start.saturating_add(block_size).min(total);
        render_block(
            plan,
            output_start,
            output_end,
            loop_frames,
            options.metronome,
            &mut left,
            &mut right,
            &mut strips,
            if loop_frames.is_some() { 0 } else { 64 },
            Some(stem),
        );
    }
    for sample in left.iter_mut().chain(&mut right) {
        *sample = sanitize(*sample);
    }
    AudioBuffer::new(plan.sample_rate, vec![left, right]).map_err(EngineError::from)
}

#[allow(clippy::too_many_arguments)]
fn render_block(
    plan: &RenderPlan,
    output_start: usize,
    output_end: usize,
    loop_frames: Option<(u64, u64)>,
    metronome: bool,
    output_left: &mut [f32],
    output_right: &mut [f32],
    strips: &mut [ChannelStrip],
    smoothing_frames: u32,
    stem: Option<usize>,
) {
    let insert_count = plan.inserts.len();
    let mut raw = vec![[0.0_f32; 2]; insert_count];
    let mut sidechain = vec![0.0_f32; insert_count];
    for output_frame in output_start..output_end {
        let absolute_frame = u64::try_from(output_frame).unwrap_or(u64::MAX);
        let source_frame = loop_source_frame(absolute_frame, loop_frames);
        raw.fill([0.0, 0.0]);
        sidechain.fill(0.0);

        for voice in &plan.voices {
            if stem.is_some_and(|selected| voice.mixer != selected) {
                continue;
            }
            if let Some(sample) = voice_sample(voice, source_frame, plan.sample_rate) {
                let volume = f64_to_f32(channel_automation(
                    &plan.automation,
                    source_frame,
                    voice.channel_id,
                    true,
                ));
                let automation_pan = f64_to_f32(channel_automation(
                    &plan.automation,
                    source_frame,
                    voice.channel_id,
                    false,
                ));
                let (left, right) = stereo_pan(
                    sample * volume,
                    (voice.pan + automation_pan).clamp(-1.0, 1.0),
                );
                raw[voice.mixer][0] += left;
                raw[voice.mixer][1] += right;
            }
        }
        for event in &plan.audio {
            if stem.is_some_and(|selected| event.mixer != selected) {
                continue;
            }
            if source_frame < event.start || source_frame >= event.end {
                continue;
            }
            let relative = usize::try_from(source_frame - event.start).unwrap_or(usize::MAX);
            #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
            let stretched = (relative as f64 * event.stretch_ratio).max(0.0) as usize;
            let source = event.source_start.saturating_add(stretched);
            if source < event.buffer.frames() {
                let mut gain = event.gain;
                let fade_in = event.fade_in.max(event.crossfade_in);
                let fade_out = event.fade_out.max(event.crossfade_out);
                if fade_in > 0 && source_frame.saturating_sub(event.start) < fade_in {
                    gain *= source_frame.saturating_sub(event.start) as f32 / fade_in as f32;
                }
                if fade_out > 0 && event.end.saturating_sub(source_frame) < fade_out {
                    gain *= event.end.saturating_sub(source_frame) as f32 / fade_out as f32;
                }
                raw[event.mixer][0] += event.buffer.sample(0, source) * gain;
                let right_channel = usize::from(event.buffer.channel_count() > 1);
                raw[event.mixer][1] += event.buffer.sample(right_channel, source) * gain;
            }
        }
        if metronome {
            add_metronome(plan, source_frame, &mut raw[plan.master_index]);
        }

        for &index in plan.graph.order() {
            let insert = &plan.inserts[index];
            let dry = raw[index];
            let mut processed = dry;
            let mixer_gain = f64_to_f32(automation_value(
                &plan.automation,
                source_frame,
                |target| matches!(target, AutomationTarget::MixerGain { mixer_insert_id } if *mixer_insert_id == insert.id),
                1.0,
            ));
            let gain = if insert.muted {
                0.0
            } else {
                insert.gain * mixer_gain
            };
            strips[index].set_targets(gain, insert.pan, smoothing_frames);
            processed = strips[index].process(processed);
            // Sidechain routes are control signals: they never become audible
            // program material, but duck the destination before its inserts.
            let duck = 1.0 / (1.0 + sidechain[index] * 2.0);
            processed[0] *= duck;
            processed[1] *= duck;
            raw[index] = processed;
            for route in &insert.routes {
                if route.sidechain {
                    sidechain[route.destination] +=
                        (processed[0].abs() + processed[1].abs()) * 0.5 * route.gain.abs();
                    continue;
                }
                let sent = if route.pre_fader { dry } else { processed };
                raw[route.destination][0] += sent[0] * route.gain;
                raw[route.destination][1] += sent[1] * route.gain;
            }
        }
        output_left[output_frame] = raw[plan.master_index][0];
        output_right[output_frame] = raw[plan.master_index][1];
    }
}

#[allow(clippy::cast_possible_truncation, clippy::cast_precision_loss)]
fn voice_sample(voice: &Voice, frame: u64, sample_rate: u32) -> Option<f32> {
    if frame < voice.start || frame >= voice.release_end {
        return None;
    }
    let phase_frames = frame - voice.start;
    #[allow(clippy::cast_precision_loss)]
    let phase = TAU * voice.frequency * phase_frames as f64 / f64::from(sample_rate);
    let attack = ((frame - voice.start) as f32 / 64.0).min(1.0);
    let release = if frame < voice.end {
        1.0
    } else {
        #[allow(clippy::cast_precision_loss)]
        let remaining = (voice.release_end - frame) as f32;
        #[allow(clippy::cast_precision_loss)]
        let length = (voice.release_end - voice.end) as f32;
        remaining / length.max(1.0)
    };
    let age = phase_frames as f64 / f64::from(sample_rate);
    let decay = (-age * 18.0).exp() as f32;
    let noise = || {
        let mut value = phase_frames
            .wrapping_mul(1_664_525)
            .wrapping_add(u64::from(voice.timbre) * 1_013_904_223);
        value ^= value >> 13;
        value ^= value << 17;
        ((value & 0xffff) as f32 / 32_767.5) - 1.0
    };
    let sample = match voice.timbre {
        1 => {
            let kick_phase = TAU * (voice.frequency * (1.0 + 7.0 * (-age * 28.0).exp())) * age;
            kick_phase.sin() as f32 * decay
        }
        2 => noise() * decay * 0.8 + (phase * 0.5).sin() as f32 * decay * 0.25,
        3 => noise() * decay * 0.38,
        4 => ((phase * 0.5).sin() as f32 * 0.65 + (phase * 1.99).sin() as f32 * 0.25) * release,
        5 => ((phase.sin() + (phase * 2.01).sin() * 0.35) as f32) * release,
        6 => phase.sin() as f32 * decay + (phase * 2.0).sin() as f32 * decay * 0.25,
        7 => ((phase.sin() + (phase * 1.01).sin() * 0.5) as f32) * release * 0.65,
        8 => noise() * (1.0 - release).max(0.05) * 0.25,
        9 => noise() * decay * 0.7 + phase.sin() as f32 * decay * 0.3,
        _ => phase.sin() as f32 * release,
    };
    Some(sample * voice.velocity * attack * release)
}

fn stereo_pan(sample: f32, base_pan: f32) -> (f32, f32) {
    let (left, right) = pan_gains(base_pan);
    (sample * left, sample * right)
}

fn pan_gains(pan: f32) -> (f32, f32) {
    let pan = pan.clamp(-1.0, 1.0);
    (((1.0 - pan) * 0.5).sqrt(), ((1.0 + pan) * 0.5).sqrt())
}

fn loop_source_frame(frame: u64, range: Option<(u64, u64)>) -> u64 {
    let Some((start, end)) = range else {
        return frame;
    };
    if frame < end {
        frame
    } else {
        start + (frame - start) % (end - start)
    }
}

fn automation_value(
    events: &[AutomationEvent],
    frame: u64,
    matches: impl Fn(&AutomationTarget) -> bool,
    default: f64,
) -> f64 {
    events
        .iter()
        .rev()
        .find(|event| frame >= event.start && frame < event.end && matches(&event.target))
        .map_or(default, |event| interpolate(&event.points, frame, default))
}

fn interpolate(points: &[AutomationPoint], frame: u64, default: f64) -> f64 {
    let samples: Vec<_> = points
        .iter()
        .map(|point| SamplePoint {
            frame: point.frame,
            value: point.value,
            interpolation: point.interpolation,
        })
        .collect();
    evaluate_automation(&samples, frame, default)
}

#[allow(clippy::cast_possible_truncation)]
fn add_metronome(plan: &RenderPlan, frame: u64, output: &mut [f32; 2]) {
    let tick = plan.timeline.frame_to_tick(frame);
    let position = plan.timeline.tick_to_position(tick);
    let beat_start = Tick(tick.0.saturating_sub(i64::from(position.tick)));
    let beat_frame = plan.timeline.tick_to_frame(beat_start);
    let offset = frame.saturating_sub(beat_frame);
    if offset >= 240 {
        return;
    }
    let frequency = if position.beat == 1 { 1_760.0 } else { 1_320.0 };
    #[allow(clippy::cast_precision_loss)]
    let phase = TAU * frequency * offset as f64 / f64::from(plan.sample_rate);
    #[allow(clippy::cast_precision_loss)]
    let envelope = 1.0 - offset as f32 / 240.0;
    let click = phase.sin() as f32 * envelope * 0.25;
    output[0] += click;
    output[1] += click;
}

#[allow(clippy::cast_possible_truncation)]
fn f64_to_f32(value: f64) -> f32 {
    value.clamp(f64::from(f32::MIN), f64::from(f32::MAX)) as f32
}

fn channel_automation(
    events: &[AutomationEvent],
    frame: u64,
    channel: ChannelId,
    volume: bool,
) -> f64 {
    automation_value(
        events,
        frame,
        |target| match target {
            AutomationTarget::ChannelVolume { channel_id } => volume && *channel_id == channel,
            AutomationTarget::ChannelPan { channel_id } => !volume && *channel_id == channel,
            _ => false,
        },
        if volume { 1.0 } else { 0.0 },
    )
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use radiant_domain::{
        AutomationInterpolation, AutomationPoint as DomainAutomationPoint, AutomationPointId,
        Channel, ClipId, MediaId, MediaSource, MixerInsert, Note, NoteId, Pattern, PatternLane,
        PlaylistClip, PlaylistClipContent, Rgba, Session, TrackAssociation,
    };
    use tempfile::tempdir;

    use super::*;
    use crate::{MediaPool, RenderPlan};

    #[allow(clippy::too_many_lines)]
    fn acceptance_session() -> (Session, MediaPool) {
        let mut session = Session::new("Milestone 2 acceptance");
        session.sample_rate = 48_000;
        session.tempo_map[0].bpm = 120.0;
        let master = session.master_insert().expect("master").id;
        let bus = MixerInsert::insert("Bus", master);
        let instrument_insert = MixerInsert::insert("Instrument", bus.id);
        let channel = Channel::sampler("Test oscillator", instrument_insert.id);

        let mut pattern = Pattern::new("Audible Pattern", Tick::beats(1));
        pattern.lanes.push(PatternLane {
            channel_id: channel.id,
            steps: Vec::new(),
            notes: vec![Note {
                id: NoteId::new(),
                start: Tick::ZERO,
                length: Tick(480),
                pitch: 69,
                velocity: 96,
                channel: 0,
            }],
        });

        let media_id = MediaId::new();
        session.media.push(MediaSource {
            id: media_id,
            path: PathBuf::from("acceptance.wav"),
            sample_rate: 24_000,
            channels: 1,
            frames: 12_000,
            import_mode: radiant_domain::MediaImportMode::Copy,
            fingerprint: String::new(),
            waveform: None,
        });
        let mut pool = MediaPool::default();
        pool.insert(
            media_id,
            AudioBuffer::new(24_000, vec![vec![0.2; 12_000]]).expect("test audio"),
        );

        let lane = &mut session.playlist_lanes[0];
        lane.association = Some(TrackAssociation {
            channel_id: channel.id,
            mixer_insert_id: bus.id,
        });
        lane.clips = vec![
            PlaylistClip {
                id: ClipId::new(),
                name: "Pattern".into(),
                start: Tick::ZERO,
                length: Tick::beats(1),
                color: Rgba::accent(),
                muted: false,
                content: PlaylistClipContent::Pattern {
                    pattern_id: pattern.id,
                },
            },
            PlaylistClip {
                id: ClipId::new(),
                name: "Audio".into(),
                start: Tick::beats(1),
                length: Tick::beats(1),
                color: Rgba::track(),
                muted: false,
                content: PlaylistClipContent::Audio {
                    media_id,
                    source_start_frame: 0,
                    gain_db: 0.0,
                },
            },
            PlaylistClip {
                id: ClipId::new(),
                name: "Bus gain".into(),
                start: Tick::ZERO,
                length: Tick::beats(2),
                color: Rgba::track(),
                muted: false,
                content: PlaylistClipContent::Automation {
                    target: AutomationTarget::MixerGain {
                        mixer_insert_id: bus.id,
                    },
                    points: vec![
                        DomainAutomationPoint {
                            id: AutomationPointId::new(),
                            at: Tick::ZERO,
                            value: 0.25,
                            interpolation: AutomationInterpolation::Linear,
                            tension: 0.0,
                        },
                        DomainAutomationPoint {
                            id: AutomationPointId::new(),
                            at: Tick::beats(2),
                            value: 1.0,
                            interpolation: AutomationInterpolation::Linear,
                            tension: 0.0,
                        },
                    ],
                },
            },
        ];
        session.channels.push(channel);
        session.patterns.push(pattern);
        session.mixer_inserts.push(bus);
        session.mixer_inserts.push(instrument_insert);
        (session, pool)
    }

    fn energy(audio: &AudioBuffer, range: std::ops::Range<usize>) -> f64 {
        range
            .map(|frame| f64::from(audio.sample(0, frame).abs()))
            .sum()
    }

    #[test]
    fn output_is_identical_across_render_block_sizes() {
        let (session, media) = acceptance_session();
        let plan = RenderPlan::compile(&session, &media).expect("plan");
        let small = render_offline(&plan, 17, RenderOptions::default()).expect("small blocks");
        let large = render_offline(&plan, 1_024, RenderOptions::default()).expect("large blocks");
        assert_eq!(small, large);
    }

    #[test]
    fn stem_rendering_preserves_timing_and_produces_one_buffer_per_insert() {
        let (session, media) = acceptance_session();
        let plan = RenderPlan::compile(&session, &media).expect("plan");
        let options = RenderOptions {
            output_frames: Some(2_048),
            ..RenderOptions::default()
        };
        let stems = render_stems(&plan, 127, options).expect("stems");
        assert_eq!(stems.len(), session.mixer_inserts.len());
        assert!(stems.iter().all(|stem| stem.frames() == 2_048));
        assert!(stems.iter().any(|stem| {
            stem.channel(0)
                .expect("left")
                .iter()
                .any(|sample| sample.abs() > 0.0)
        }));
    }

    #[test]
    fn freeze_and_bounce_preserve_the_render_snapshot() {
        let (session, media) = acceptance_session();
        let plan = RenderPlan::compile(&session, &media).expect("plan");
        let options = RenderOptions {
            output_frames: Some(2_048),
            ..RenderOptions::default()
        };
        let frozen = freeze(&plan, 97, options).expect("freeze");
        let bounced = bounce(&plan, 257, options).expect("bounce");
        assert_eq!(frozen, bounced);
    }

    #[test]
    fn solo_keeps_the_solo_path_and_its_buses_audible() {
        let (mut session, media) = acceptance_session();
        let instrument = session
            .mixer_inserts
            .iter_mut()
            .find(|insert| insert.name == "Instrument")
            .expect("instrument insert");
        instrument.solo = true;
        let plan = RenderPlan::compile(&session, &media).expect("plan");
        let rendered = render_offline(&plan, 128, RenderOptions::default()).expect("render");
        assert!(energy(&rendered, 0..rendered.frames()) > 1.0);
    }

    #[test]
    fn events_start_on_boundaries_and_instrument_tail_finishes() {
        let (mut session, media) = acceptance_session();
        session.playlist_lanes[0].clips.truncate(1);
        let clip = &mut session.playlist_lanes[0].clips[0];
        clip.start = Tick::beats(1);
        let plan = RenderPlan::compile(&session, &media).expect("plan");
        let rendered = render_offline(&plan, 127, RenderOptions::default()).expect("render");
        let start = usize::try_from(plan.timeline().tick_to_frame(Tick::beats(1))).expect("start");
        assert!(energy(&rendered, 0..start) < f64::EPSILON);
        assert!(energy(&rendered, start..start + 1_000) > 1.0);
        assert!(energy(&rendered, rendered.frames() - 1..rendered.frames()) < f64::EPSILON);
    }

    #[test]
    fn loop_range_repeats_the_same_source_samples() {
        let (session, media) = acceptance_session();
        let plan = RenderPlan::compile(&session, &media).expect("plan");
        let beat_frames = plan.timeline().tick_to_frame(Tick::beats(1));
        let rendered = render_offline(
            &plan,
            333,
            RenderOptions {
                output_frames: Some(beat_frames * 2),
                loop_range: Some((Tick::ZERO, Tick::beats(1))),
                metronome: false,
                ..RenderOptions::default()
            },
        )
        .expect("loop render");
        let beat = usize::try_from(beat_frames).expect("beat frames");
        assert_eq!(
            rendered.channel(0).expect("left")[..beat],
            rendered.channel(0).expect("left")[beat..beat * 2]
        );
    }

    #[test]
    fn pattern_audio_automation_and_route_reach_exported_wav() {
        let (session, media) = acceptance_session();
        let plan = RenderPlan::compile(&session, &media).expect("plan");
        let directory = tempdir().expect("temporary directory");
        let path = directory.path().join("milestone-two.wav");
        export_wav(&plan, &path, 256, RenderOptions::default()).expect("export");
        let decoded = radiant_media::decode(&path).expect("decode export");
        let beat = usize::try_from(plan.timeline().tick_to_frame(Tick::beats(1))).expect("beat");
        let pattern_energy = energy(&decoded, 0..beat);
        let audio_early = energy(&decoded, beat..beat + 4_000);
        let audio_late = energy(&decoded, beat * 2 - 4_000..beat * 2);
        assert!(
            pattern_energy > 10.0,
            "Pattern must traverse Instrument -> Bus -> Master"
        );
        assert!(audio_early > 10.0, "resampled Audio Clip must be audible");
        assert!(
            audio_late > audio_early,
            "linear Mixer automation must raise gain"
        );
    }

    #[test]
    fn non_finite_media_is_contained_at_output() {
        let (session, mut media) = acceptance_session();
        let media_id = session.media[0].id;
        media.insert(
            media_id,
            AudioBuffer::new(48_000, vec![vec![f32::NAN, f32::INFINITY, 1.0e-40]])
                .expect("invalid source samples"),
        );
        let plan = RenderPlan::compile(&session, &media).expect("plan");
        let rendered = render_offline(&plan, 64, RenderOptions::default()).expect("render");
        assert!(
            rendered
                .channel(0)
                .expect("left")
                .iter()
                .all(|sample| sample.is_finite())
        );
    }

    #[test]
    fn metronome_clicks_are_rendered_only_when_enabled() {
        let session = Session::new("Metronome");
        let plan = RenderPlan::compile(&session, &MediaPool::default()).expect("plan");
        let silent = render_offline(&plan, 64, RenderOptions::default()).expect("silent render");
        let clicked = render_offline(
            &plan,
            64,
            RenderOptions {
                metronome: true,
                ..RenderOptions::default()
            },
        )
        .expect("metronome render");
        assert!(energy(&silent, 0..silent.frames()) < f64::EPSILON);
        assert!(energy(&clicked, 0..clicked.frames()) > 1.0);
    }
}
