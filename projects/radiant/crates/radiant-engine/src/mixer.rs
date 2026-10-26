//! Allocation-free mixer primitives shared by real-time and offline paths.

use crate::{DelayLine, SmoothedValue};

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct MeterSnapshot {
    pub peak: [f32; 2],
    pub rms: [f32; 2],
    pub clipped: bool,
}

#[derive(Clone, Debug)]
pub struct StereoMeter {
    peak: [f32; 2],
    sum_squares: [f64; 2],
    frames: u32,
    clip_hold: u32,
    decimation: u32,
    snapshot: MeterSnapshot,
}

impl StereoMeter {
    #[must_use]
    pub fn new(decimation: u32) -> Self {
        Self {
            decimation: decimation.max(1),
            ..Self::default()
        }
    }

    #[allow(clippy::cast_possible_truncation)]
    pub fn push(&mut self, sample: [f32; 2]) {
        for (index, value) in sample.into_iter().enumerate() {
            self.peak[index] = self.peak[index].max(value.abs());
            self.sum_squares[index] += f64::from(value) * f64::from(value);
        }
        self.frames = self.frames.saturating_add(1);
        if sample.into_iter().any(|value| value.abs() >= 1.0) {
            self.clip_hold = 3;
        } else {
            self.clip_hold = self.clip_hold.saturating_sub(1);
        }
        if self.frames >= self.decimation {
            let count = f64::from(self.frames);
            self.snapshot = MeterSnapshot {
                peak: self.peak,
                rms: [
                    (self.sum_squares[0] / count).sqrt() as f32,
                    (self.sum_squares[1] / count).sqrt() as f32,
                ],
                clipped: self.clip_hold > 0,
            };
            self.peak = [0.0; 2];
            self.sum_squares = [0.0; 2];
            self.frames = 0;
        }
    }

    #[must_use]
    pub const fn snapshot(&self) -> MeterSnapshot {
        self.snapshot
    }
}

impl Default for StereoMeter {
    fn default() -> Self {
        Self {
            peak: [0.0; 2],
            sum_squares: [0.0; 2],
            frames: 0,
            clip_hold: 0,
            decimation: 64,
            snapshot: MeterSnapshot {
                peak: [0.0; 2],
                rms: [0.0; 2],
                clipped: false,
            },
        }
    }
}

#[derive(Clone, Debug)]
pub struct ChannelStrip {
    gain: SmoothedValue,
    pan: SmoothedValue,
    pub phase_inverted: bool,
    pub stereo_width: f32,
    pub meter: StereoMeter,
    delay: Option<(DelayLine, DelayLine)>,
    eq_low: f32,
    eq_state: [f32; 2],
    compressor_threshold: f32,
    compressor_ratio: f32,
}

impl ChannelStrip {
    #[must_use]
    pub fn new(gain: f32, pan: f32, meter_decimation: u32) -> Self {
        Self {
            gain: SmoothedValue::new(gain),
            pan: SmoothedValue::new(pan),
            phase_inverted: false,
            stereo_width: 1.0,
            meter: StereoMeter::new(meter_decimation),
            delay: None,
            eq_low: 0.0,
            eq_state: [0.0; 2],
            compressor_threshold: 1.0,
            compressor_ratio: 1.0,
        }
    }

    pub fn set_delay(&mut self, frames: usize) {
        self.delay = (frames > 0).then(|| (DelayLine::new(frames), DelayLine::new(frames)));
    }

    pub fn prepare_delay_capacity(&mut self, capacity: usize) {
        self.delay = Some((
            DelayLine::with_capacity(capacity),
            DelayLine::with_capacity(capacity),
        ));
    }

    pub fn set_delay_bounded(&mut self, frames: usize) -> bool {
        let Some((left, right)) = &mut self.delay else {
            return frames == 0;
        };
        left.set_delay(frames) && right.set_delay(frames)
    }

    #[must_use]
    pub fn delay_frames(&self) -> usize {
        self.delay.as_ref().map_or(0, |(left, _)| left.delay())
    }

    /// Set a gentle low-shelf amount (-1..1) and a simple dynamics curve.
    pub fn set_utility(&mut self, low_shelf: f32, threshold: f32, ratio: f32) {
        self.eq_low = low_shelf.clamp(-1.0, 1.0);
        self.compressor_threshold = threshold.max(0.001);
        self.compressor_ratio = ratio.max(1.0);
    }

    pub fn set_targets(&mut self, gain: f32, pan: f32, frames: u32) {
        if (self.gain.target() - gain).abs() > f32::EPSILON {
            self.gain.set_target(gain, frames);
        }
        if (self.pan.target() - pan).abs() > f32::EPSILON {
            self.pan.set_target(pan, frames);
        }
    }

    pub fn process(&mut self, mut input: [f32; 2]) -> [f32; 2] {
        // Preserve bit-for-bit stereo unity for the live default strip. This
        // is also useful for bypass: no unnecessary mid/side round trip.
        if self.delay.is_none()
            && self.eq_low == 0.0
            && (self.compressor_ratio - 1.0).abs() < f32::EPSILON
            && !self.phase_inverted
            && (self.stereo_width - 1.0).abs() < f32::EPSILON
            && self.pan.target() == 0.0
            && self.pan.value() == 0.0
            && (self.gain.target() - 1.0).abs() < f32::EPSILON
            && (self.gain.value() - 1.0).abs() < f32::EPSILON
        {
            self.meter.push(input);
            return input;
        }
        if let Some((left, right)) = &mut self.delay {
            let mut l = [input[0]];
            let mut r = [input[1]];
            left.process(&mut l);
            right.process(&mut r);
            input = [l[0], r[0]];
        }
        for (index, value) in input.iter_mut().enumerate() {
            self.eq_state[index] += (self.eq_low * *value - self.eq_state[index]) * 0.02;
            *value += self.eq_state[index];
            let magnitude = value.abs();
            if magnitude > self.compressor_threshold {
                let excess = magnitude - self.compressor_threshold;
                *value =
                    value.signum() * (self.compressor_threshold + excess / self.compressor_ratio);
            }
        }
        let gain = self.gain.advance();
        let pan = self.pan.advance().clamp(-1.0, 1.0);
        let mid = (input[0] + input[1]) * 0.5;
        let side = (input[0] - input[1]) * 0.5 * self.stereo_width.clamp(0.0, 2.0);
        let (pan_left, pan_right) = (((1.0 - pan) * 0.5).sqrt(), ((1.0 + pan) * 0.5).sqrt());
        let phase = if self.phase_inverted { -1.0 } else { 1.0 };
        let panned = if pan.abs() < f32::EPSILON {
            input
        } else {
            [
                mid * pan_left * 2.0_f32.sqrt() + side,
                mid * pan_right * 2.0_f32.sqrt() - side,
            ]
        };
        let output = [panned[0] * gain * phase, panned[1] * gain * phase];
        self.meter.push(output);
        output
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn meter_reports_peak_rms_and_clip_hold() {
        let mut meter = StereoMeter::new(2);
        meter.push([1.2, 0.5]);
        meter.push([0.0, 0.5]);
        let snapshot = meter.snapshot();
        assert!(snapshot.peak[0] >= 1.2);
        assert!(snapshot.rms[1] > 0.4);
        assert!(snapshot.clipped);
    }

    #[test]
    fn strip_smooths_gain_and_supports_phase_and_width() {
        let mut strip = ChannelStrip::new(0.0, 0.0, 1);
        strip.stereo_width = 2.0;
        strip.phase_inverted = true;
        strip.set_targets(1.0, 0.0, 4);
        assert!(strip.process([1.0, 0.0])[0] < 0.0);
    }

    #[test]
    fn built_in_utility_dynamics_and_delay_have_observable_behavior() {
        let mut strip = ChannelStrip::new(1.0, 0.0, 1);
        strip.set_delay(1);
        strip.set_utility(0.5, 0.25, 4.0);
        let first = strip.process([1.0, 1.0]);
        assert!(first.iter().all(|sample| sample.abs() < f32::EPSILON));
        let second = strip.process([0.0, 0.0]);
        assert!(second.iter().all(|sample| sample.is_finite()));
        assert!(second[0].abs() < 1.0);
        let before_bypass = strip.process([0.0, 0.0]);
        assert!(before_bypass.iter().all(|sample| sample.is_finite()));
    }
}
