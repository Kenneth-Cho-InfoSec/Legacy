//! Native Linux real-time audio and ALSA Sequencer MIDI integration.

mod backend;
mod capture;
mod midi;
mod realtime;

pub use backend::{
    BackendDiagnostics, BackendPreference, NativeAudioService, NativeBackend, NativeDeviceInfo,
    ServiceStatus, available_devices, diagnose_backends,
};
pub use capture::NativeCapture;
pub use midi::{AlsaMidiInput, MidiPortStatus, TimestampMapper};
pub use realtime::{
    AudioCommand, MidiIngress, MidiMessage, MidiMessageKind, RealtimeController,
    RealtimeDiagnostics, RealtimeProcessor, RealtimeState, RecordedMidiEvent, create_realtime,
};

/// Measured device offsets used to align recorded takes with the transport.
/// Positive input values move captured audio later; positive output values
/// compensate the reported playback position earlier.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct LatencyCalibration {
    pub input_frames: i64,
    pub output_frames: i64,
}

impl LatencyCalibration {
    #[must_use]
    pub const fn new(input_frames: i64, output_frames: i64) -> Self {
        Self {
            input_frames,
            output_frames,
        }
    }

    #[must_use]
    pub const fn captured_frame(self, device_frame: i64) -> i64 {
        device_frame.saturating_add(self.input_frames)
    }

    #[must_use]
    pub const fn playback_frame(self, transport_frame: i64) -> i64 {
        transport_frame.saturating_sub(self.output_frames)
    }
}

#[cfg(test)]
mod calibration_tests {
    use super::LatencyCalibration;

    #[test]
    fn offsets_are_applied_in_the_expected_direction() {
        let calibration = LatencyCalibration::new(128, 64);
        assert_eq!(calibration.captured_frame(1_000), 1_128);
        assert_eq!(calibration.playback_frame(1_000), 936);
    }
}

use thiserror::Error;

#[derive(Debug, Error)]
pub enum AudioError {
    #[error("native audio host is unavailable: {0}")]
    HostUnavailable(String),
    #[error("no suitable native output device is available")]
    NoOutputDevice,
    #[error("no native input device is available")]
    NoInputDevice,
    #[error("no stereo f32 output configuration supports the requested sample rate")]
    UnsupportedConfiguration,
    #[error("no native f32 input configuration is available")]
    UnsupportedInputConfiguration,
    #[error("real-time queue is full")]
    QueueFull,
    #[error("native stream failed: {0}")]
    Stream(String),
    #[error("ALSA Sequencer failed: {0}")]
    Midi(String),
}
