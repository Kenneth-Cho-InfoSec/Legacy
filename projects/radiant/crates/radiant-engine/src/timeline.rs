use radiant_domain::{TICKS_PER_QUARTER, TempoPoint, Tick, TimeSignature};

use crate::EngineError;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MeterPoint {
    pub at: Tick,
    pub signature: TimeSignature,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MusicalPosition {
    pub bar: u32,
    pub beat: u8,
    pub tick: u16,
}

#[derive(Clone, Debug)]
struct TempoSegment {
    at: Tick,
    start_frame: f64,
    frames_per_tick: f64,
}

#[derive(Clone, Debug)]
struct MeterSegment {
    at: Tick,
    start_bar: u32,
    signature: TimeSignature,
}

#[derive(Clone, Debug)]
pub struct MusicalTimeline {
    sample_rate: u32,
    tempo: Vec<TempoSegment>,
    meter: Vec<MeterSegment>,
}

impl MusicalTimeline {
    pub fn new(
        sample_rate: u32,
        tempo: &[TempoPoint],
        meter: &[MeterPoint],
    ) -> Result<Self, EngineError> {
        if sample_rate == 0
            || tempo.is_empty()
            || tempo[0].at != Tick::ZERO
            || tempo.windows(2).any(|points| points[0].at >= points[1].at)
            || tempo
                .iter()
                .any(|point| !point.bpm.is_finite() || point.bpm <= 0.0)
        {
            return Err(EngineError::InvalidTimeline);
        }
        if meter.is_empty()
            || meter[0].at != Tick::ZERO
            || meter.windows(2).any(|points| points[0].at >= points[1].at)
            || meter.iter().any(|point| !valid_signature(point.signature))
        {
            return Err(EngineError::InvalidTimeline);
        }

        let mut tempo_segments: Vec<TempoSegment> = Vec::with_capacity(tempo.len());
        let mut start_frame = 0.0;
        for (index, point) in tempo.iter().enumerate() {
            if index > 0 {
                let previous = &tempo_segments[index - 1];
                #[allow(clippy::cast_precision_loss)]
                let ticks = (point.at.0 - previous.at.0) as f64;
                start_frame += ticks * previous.frames_per_tick;
            }
            tempo_segments.push(TempoSegment {
                at: point.at,
                start_frame,
                frames_per_tick: f64::from(sample_rate) * 60.0 / (point.bpm * 960.0),
            });
        }

        let mut meter_segments: Vec<MeterSegment> = Vec::with_capacity(meter.len());
        let mut start_bar = 1_u32;
        for (index, point) in meter.iter().enumerate() {
            if index > 0 {
                let previous = &meter_segments[index - 1];
                let ticks = point.at.0 - previous.at.0;
                let bars = ticks.div_euclid(ticks_per_bar(previous.signature));
                start_bar = start_bar.saturating_add(u32::try_from(bars).unwrap_or(u32::MAX));
            }
            meter_segments.push(MeterSegment {
                at: point.at,
                start_bar,
                signature: point.signature,
            });
        }
        Ok(Self {
            sample_rate,
            tempo: tempo_segments,
            meter: meter_segments,
        })
    }

    #[must_use]
    pub const fn sample_rate(&self) -> u32 {
        self.sample_rate
    }

    #[must_use]
    pub fn tick_to_frame(&self, tick: Tick) -> u64 {
        if tick <= Tick::ZERO {
            return 0;
        }
        let segment = self
            .tempo
            .iter()
            .rfind(|segment| segment.at <= tick)
            .unwrap_or(&self.tempo[0]);
        #[allow(clippy::cast_precision_loss)]
        let offset = (tick.0 - segment.at.0) as f64;
        f64_to_u64_round(segment.start_frame + offset * segment.frames_per_tick)
    }

    #[must_use]
    pub fn frame_to_tick(&self, frame: u64) -> Tick {
        #[allow(clippy::cast_precision_loss)]
        let frame = frame as f64;
        let segment = self
            .tempo
            .iter()
            .rfind(|segment| segment.start_frame <= frame)
            .unwrap_or(&self.tempo[0]);
        let ticks = (frame - segment.start_frame) / segment.frames_per_tick;
        Tick(segment.at.0.saturating_add(f64_to_i64_round(ticks)))
    }

    #[must_use]
    pub fn tick_to_position(&self, tick: Tick) -> MusicalPosition {
        let tick = Tick(tick.0.max(0));
        let segment = self
            .meter
            .iter()
            .rfind(|segment| segment.at <= tick)
            .unwrap_or(&self.meter[0]);
        let offset = tick.0 - segment.at.0;
        let beat_ticks = ticks_per_beat(segment.signature);
        let bar_ticks = beat_ticks * i64::from(segment.signature.numerator);
        let bar_offset = offset.div_euclid(bar_ticks);
        let within_bar = offset.rem_euclid(bar_ticks);
        let beat = within_bar.div_euclid(beat_ticks);
        let tick = within_bar.rem_euclid(beat_ticks);
        MusicalPosition {
            bar: segment
                .start_bar
                .saturating_add(u32::try_from(bar_offset).unwrap_or(u32::MAX)),
            beat: u8::try_from(beat + 1).unwrap_or(u8::MAX),
            tick: u16::try_from(tick).unwrap_or(u16::MAX),
        }
    }

    #[must_use]
    pub fn position_to_tick(&self, position: MusicalPosition) -> Tick {
        let segment = self
            .meter
            .iter()
            .rfind(|segment| segment.start_bar <= position.bar)
            .unwrap_or(&self.meter[0]);
        let bar_offset = position.bar.saturating_sub(segment.start_bar);
        let beat_offset = position.beat.saturating_sub(1);
        let beat_ticks = ticks_per_beat(segment.signature);
        Tick(
            segment
                .at
                .0
                .saturating_add(i64::from(bar_offset) * ticks_per_bar(segment.signature))
                .saturating_add(i64::from(beat_offset) * beat_ticks)
                .saturating_add(i64::from(position.tick)),
        )
    }
}

fn valid_signature(signature: TimeSignature) -> bool {
    signature.numerator > 0
        && signature.denominator.is_power_of_two()
        && signature.denominator <= 32
}

fn ticks_per_beat(signature: TimeSignature) -> i64 {
    TICKS_PER_QUARTER * 4 / i64::from(signature.denominator)
}

fn ticks_per_bar(signature: TimeSignature) -> i64 {
    ticks_per_beat(signature) * i64::from(signature.numerator)
}

#[allow(
    clippy::cast_possible_truncation,
    clippy::cast_precision_loss,
    clippy::cast_sign_loss
)]
fn f64_to_u64_round(value: f64) -> u64 {
    value.round().clamp(0.0, u64::MAX as f64) as u64
}

#[allow(clippy::cast_possible_truncation, clippy::cast_precision_loss)]
fn f64_to_i64_round(value: f64) -> i64 {
    value.round().clamp(i64::MIN as f64, i64::MAX as f64) as i64
}

#[cfg(test)]
mod tests {
    use super::*;

    fn timeline() -> MusicalTimeline {
        MusicalTimeline::new(
            48_000,
            &[
                TempoPoint {
                    at: Tick::ZERO,
                    bpm: 120.0,
                },
                TempoPoint {
                    at: Tick::beats(4),
                    bpm: 60.0,
                },
            ],
            &[
                MeterPoint {
                    at: Tick::ZERO,
                    signature: TimeSignature {
                        numerator: 4,
                        denominator: 4,
                    },
                },
                MeterPoint {
                    at: Tick::beats(8),
                    signature: TimeSignature {
                        numerator: 3,
                        denominator: 4,
                    },
                },
            ],
        )
        .expect("timeline")
    }

    #[test]
    fn tempo_boundaries_map_to_exact_frames() {
        let timeline = timeline();
        assert_eq!(timeline.tick_to_frame(Tick::beats(4)), 96_000);
        assert_eq!(timeline.tick_to_frame(Tick::beats(5)), 144_000);
        assert_eq!(timeline.frame_to_tick(144_000), Tick::beats(5));
    }

    #[test]
    fn meter_changes_preserve_bar_positions() {
        let timeline = timeline();
        assert_eq!(
            timeline.tick_to_position(Tick::beats(11)),
            MusicalPosition {
                bar: 4,
                beat: 1,
                tick: 0
            }
        );
        assert_eq!(
            timeline.position_to_tick(MusicalPosition {
                bar: 4,
                beat: 1,
                tick: 0
            }),
            Tick::beats(11)
        );
    }
}
