use std::sync::Arc;
use std::time::Instant;

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{Device, Host, HostId, SampleFormat, Stream, StreamConfig, StreamError};
use radiant_media::AudioBuffer;
use radiant_pipewire::PipeWireStream;

use crate::midi::AlsaMidiInput;
use crate::realtime::{RealtimeController, RealtimeDiagnostics, create_realtime};
use crate::{AudioError, LatencyCalibration, MidiPortStatus};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum NativeBackend {
    PipeWire,
    Jack,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum BackendPreference {
    #[default]
    Auto,
    PipeWire,
    Jack,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NativeDeviceInfo {
    pub backend: NativeBackend,
    pub name: String,
    pub is_default: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BackendDiagnostics {
    pub pipewire_available: bool,
    pub jack_available: bool,
    pub midi_device_access: bool,
    pub detail: String,
}

/// Reports native Linux backend prerequisites without starting an audio stream.
#[must_use]
pub fn diagnose_backends() -> BackendDiagnostics {
    let pipewire_available = std::env::var_os("PIPEWIRE_RUNTIME_DIR").is_some()
        || std::env::var_os("XDG_RUNTIME_DIR")
            .is_some_and(|path| std::path::Path::new(&path).join("pipewire-0").exists());
    let jack_available = open_jack_host().is_ok();
    let midi_device_access = std::path::Path::new("/dev/snd").is_dir();
    let detail =
        format!("PipeWire={pipewire_available}, JACK={jack_available}, MIDI={midi_device_access}");
    BackendDiagnostics {
        pipewire_available,
        jack_available,
        midi_device_access,
        detail,
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ServiceStatus {
    pub backend: Option<NativeBackend>,
    pub sample_rate: u32,
    pub channels: u16,
    pub quantum: u32,
    pub audio: RealtimeDiagnostics,
    pub midi: MidiPortStatus,
    pub reconnects: u64,
    pub calibration: LatencyCalibration,
}

pub fn available_devices(
    preference: BackendPreference,
) -> Result<Vec<NativeDeviceInfo>, AudioError> {
    let mut result = Vec::new();
    for backend in backend_order(preference) {
        if backend == NativeBackend::PipeWire {
            result.push(NativeDeviceInfo {
                backend,
                name: "PipeWire default audio sink".into(),
                is_default: true,
            });
            continue;
        }
        let Ok(host) = open_jack_host() else {
            continue;
        };
        let default = host.default_output_device();
        let default_id = default.as_ref().and_then(|device| device.id().ok());
        let devices = host
            .output_devices()
            .map_err(|error| AudioError::Stream(error.to_string()))?;
        for device in devices {
            let id = device.id().ok();
            result.push(NativeDeviceInfo {
                backend,
                name: device.description().map_or_else(
                    |_| "JACK output".into(),
                    |description| description.name().into(),
                ),
                is_default: id.is_some() && id == default_id,
            });
        }
    }
    Ok(result)
}

pub struct NativeAudioService {
    preference: BackendPreference,
    source: Arc<AudioBuffer>,
    stream: Option<NativeStream>,
    controller: RealtimeController,
    midi: AlsaMidiInput,
    backend: NativeBackend,
    sample_rate: u32,
    channels: u16,
    reconnects: u64,
    calibration: LatencyCalibration,
}

impl NativeAudioService {
    pub fn start(
        source: Arc<AudioBuffer>,
        preference: BackendPreference,
    ) -> Result<Self, AudioError> {
        let running = start_stream(&source, preference)?;
        Ok(Self {
            preference,
            source,
            stream: Some(running.stream),
            controller: running.controller,
            midi: running.midi,
            backend: running.backend,
            sample_rate: running.sample_rate,
            channels: running.channels,
            reconnects: 0,
            calibration: LatencyCalibration::default(),
        })
    }

    pub fn restart(&mut self) -> Result<(), AudioError> {
        let previous = self.controller.diagnostics();
        let previous_rate = self.sample_rate;
        self.stream.take();
        let mut running = start_stream(&self.source, self.preference)?;
        let restored_frame =
            scale_frame(previous.playhead_frame, previous_rate, running.sample_rate);
        running
            .controller
            .send(crate::AudioCommand::Seek(restored_frame))?;
        match previous.state {
            crate::RealtimeState::Playing => {
                running.controller.send(crate::AudioCommand::Play)?;
            }
            crate::RealtimeState::Paused => {
                running.controller.send(crate::AudioCommand::Play)?;
                running.controller.send(crate::AudioCommand::Pause)?;
            }
            crate::RealtimeState::Stopped => {}
        }
        self.stream = Some(running.stream);
        self.controller = running.controller;
        self.midi = running.midi;
        self.backend = running.backend;
        self.sample_rate = running.sample_rate;
        self.channels = running.channels;
        self.reconnects = self.reconnects.saturating_add(1);
        Ok(())
    }

    #[must_use]
    pub fn requires_restart(&self) -> bool {
        self.controller.diagnostics().faulted
    }

    pub fn controller_mut(&mut self) -> &mut RealtimeController {
        &mut self.controller
    }

    /// Drain post-mixer record-tap samples on a non-real-time thread.
    pub fn drain_record_tap(&mut self, destination: &mut Vec<f32>, maximum_samples: usize) {
        for _ in 0..maximum_samples {
            let Some(sample) = self.controller.pop_recorded_audio() else {
                break;
            };
            destination.push(sample);
        }
    }

    pub fn set_calibration(&mut self, calibration: LatencyCalibration) {
        self.calibration = calibration;
    }

    #[must_use]
    pub const fn calibration(&self) -> LatencyCalibration {
        self.calibration
    }

    #[must_use]
    pub fn status(&self) -> ServiceStatus {
        let audio = self.controller.diagnostics();
        ServiceStatus {
            backend: Some(self.backend),
            sample_rate: self.sample_rate,
            channels: self.channels,
            quantum: audio.quantum,
            audio,
            midi: self.midi.status(),
            reconnects: self.reconnects,
            calibration: self.calibration,
        }
    }
}

struct RunningStream {
    stream: NativeStream,
    controller: RealtimeController,
    midi: AlsaMidiInput,
    backend: NativeBackend,
    sample_rate: u32,
    channels: u16,
}

#[allow(dead_code)]
enum NativeStream {
    PipeWire(PipeWireStream),
    Jack(Stream),
}

fn start_stream(
    source: &Arc<AudioBuffer>,
    preference: BackendPreference,
) -> Result<RunningStream, AudioError> {
    let mut last_error = None;
    for backend in backend_order(preference) {
        match start_backend(source, backend) {
            Ok(stream) => return Ok(stream),
            Err(error) => last_error = Some(error),
        }
    }
    Err(last_error.unwrap_or(AudioError::NoOutputDevice))
}

fn start_backend(
    source: &Arc<AudioBuffer>,
    backend: NativeBackend,
) -> Result<RunningStream, AudioError> {
    match backend {
        NativeBackend::PipeWire => start_pipewire(source.clone()),
        NativeBackend::Jack => start_jack(source),
    }
}

fn start_pipewire(source: Arc<AudioBuffer>) -> Result<RunningStream, AudioError> {
    let sample_rate = source.sample_rate();
    let channels = 2_u16;
    let (controller, midi_ingress, mut processor) = create_realtime(source, 256, 2_048, 4_096);
    let counters = controller.counters();
    let error_counters = counters.clone();
    let clock_origin = Instant::now();
    let stream = PipeWireStream::start(
        sample_rate,
        u32::from(channels),
        move |output, output_channels| {
            let started = Instant::now();
            processor.set_callback_clock_nanos(elapsed_nanos(clock_origin));
            processor.process(output, output_channels);
            let frames = output.len() / output_channels.max(1);
            let budget_nanos = u128::try_from(frames)
                .unwrap_or(u128::MAX)
                .saturating_mul(1_000_000_000)
                / u128::from(sample_rate);
            if started.elapsed().as_nanos() > budget_nanos {
                processor.report_deadline_miss();
            }
        },
        move || error_counters.report_stream_error(false, true),
    )
    .map_err(|error| AudioError::Stream(error.to_string()))?;
    let midi = AlsaMidiInput::start(midi_ingress, counters, sample_rate, clock_origin);
    Ok(RunningStream {
        stream: NativeStream::PipeWire(stream),
        controller,
        midi,
        backend: NativeBackend::PipeWire,
        sample_rate,
        channels,
    })
}

fn start_jack(source: &AudioBuffer) -> Result<RunningStream, AudioError> {
    let host = open_jack_host()?;
    let device = host
        .default_output_device()
        .ok_or(AudioError::NoOutputDevice)?;
    let config = negotiate_config(&device, source.sample_rate())?;
    let sample_rate = config.sample_rate;
    let channels = config.channels;
    let source = Arc::new(source.resample_linear(sample_rate));
    let (controller, midi_ingress, mut processor) = create_realtime(source, 256, 2_048, 4_096);
    let counters = controller.counters();
    let error_counters = counters.clone();
    let clock_origin = Instant::now();
    let stream = device
        .build_output_stream(
            &config,
            move |output: &mut [f32], _| {
                let started = Instant::now();
                processor.set_callback_clock_nanos(elapsed_nanos(clock_origin));
                processor.process(output, usize::from(channels));
                let frames = output.len() / usize::from(channels);
                let budget_nanos = u128::try_from(frames)
                    .unwrap_or(u128::MAX)
                    .saturating_mul(1_000_000_000)
                    / u128::from(sample_rate);
                if started.elapsed().as_nanos() > budget_nanos {
                    processor.report_deadline_miss();
                }
            },
            move |error| {
                let xrun = error == StreamError::BufferUnderrun;
                let fault = matches!(
                    error,
                    StreamError::DeviceNotAvailable | StreamError::StreamInvalidated
                );
                error_counters.report_stream_error(xrun, fault);
            },
            None,
        )
        .map_err(|error| AudioError::Stream(error.to_string()))?;
    stream
        .play()
        .map_err(|error| AudioError::Stream(error.to_string()))?;
    let midi = AlsaMidiInput::start(midi_ingress, counters, sample_rate, clock_origin);
    Ok(RunningStream {
        stream: NativeStream::Jack(stream),
        controller,
        midi,
        backend: NativeBackend::Jack,
        sample_rate,
        channels,
    })
}

fn negotiate_config(device: &Device, requested_rate: u32) -> Result<StreamConfig, AudioError> {
    let requested = device
        .supported_output_configs()
        .map_err(|error| AudioError::Stream(error.to_string()))?
        .filter(|range| range.sample_format() == SampleFormat::F32 && range.channels() >= 2)
        .find_map(|range| range.try_with_sample_rate(requested_rate));
    if let Some(config) = requested {
        return Ok(config.into());
    }
    let default = device
        .default_output_config()
        .map_err(|error| AudioError::Stream(error.to_string()))?;
    if default.sample_format() != SampleFormat::F32 || default.channels() < 2 {
        return Err(AudioError::UnsupportedConfiguration);
    }
    Ok(default.into())
}

fn open_jack_host() -> Result<Host, AudioError> {
    cpal::host_from_id(HostId::Jack).map_err(|error| AudioError::HostUnavailable(error.to_string()))
}

fn backend_order(preference: BackendPreference) -> impl Iterator<Item = NativeBackend> {
    let backends = match preference {
        BackendPreference::Auto => [Some(NativeBackend::PipeWire), Some(NativeBackend::Jack)],
        BackendPreference::PipeWire => [Some(NativeBackend::PipeWire), None],
        BackendPreference::Jack => [Some(NativeBackend::Jack), None],
    };
    backends.into_iter().flatten()
}

fn elapsed_nanos(origin: Instant) -> u64 {
    u64::try_from(origin.elapsed().as_nanos()).unwrap_or(u64::MAX)
}

fn scale_frame(frame: u64, source_rate: u32, target_rate: u32) -> u64 {
    let scaled =
        u128::from(frame).saturating_mul(u128::from(target_rate)) / u128::from(source_rate.max(1));
    u64::try_from(scaled).unwrap_or(u64::MAX)
}

#[cfg(test)]
mod tests {
    use std::thread;
    use std::time::Duration;

    use super::*;
    use crate::AudioCommand;

    #[test]
    #[ignore = "requires a live PipeWire session"]
    fn native_pipewire_stream_processes_callbacks() {
        let audio = Arc::new(AudioBuffer::silence(48_000, 2, 480_000));
        let mut service = NativeAudioService::start(audio, BackendPreference::PipeWire)
            .expect("native PipeWire stream");
        service
            .controller_mut()
            .send(AudioCommand::Play)
            .expect("play");
        thread::sleep(Duration::from_secs(1));
        let status = service.status();
        assert_eq!(status.backend, Some(NativeBackend::PipeWire));
        assert!(status.audio.callback_cycles > 0);
        assert!(!status.audio.faulted);
        let frame_before_restart = status.audio.playhead_frame;
        service.restart().expect("restart PipeWire stream");
        thread::sleep(Duration::from_secs(1));
        let restarted = service.status();
        assert_eq!(restarted.reconnects, 1);
        assert!(restarted.audio.callback_cycles > 0);
        assert!(restarted.audio.playhead_frame > frame_before_restart);
        assert!(!restarted.audio.faulted);
    }
}
