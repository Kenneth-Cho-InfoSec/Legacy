use radiant_domain::Tick;

use crate::MusicalTimeline;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TransportState {
    Stopped,
    Playing,
    Paused,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TransportCommand {
    Play,
    Pause,
    Stop,
    Seek(Tick),
    SetLoop(Option<(Tick, Tick)>),
}

#[derive(Clone, Debug)]
pub struct Transport {
    state: TransportState,
    frame: u64,
    loop_frames: Option<(u64, u64)>,
}

impl Default for Transport {
    fn default() -> Self {
        Self {
            state: TransportState::Stopped,
            frame: 0,
            loop_frames: None,
        }
    }
}

impl Transport {
    pub fn command(&mut self, command: TransportCommand, timeline: &MusicalTimeline) {
        match command {
            TransportCommand::Play => self.state = TransportState::Playing,
            TransportCommand::Pause => self.state = TransportState::Paused,
            TransportCommand::Stop => {
                self.state = TransportState::Stopped;
                self.frame = 0;
            }
            TransportCommand::Seek(tick) => self.frame = timeline.tick_to_frame(tick),
            TransportCommand::SetLoop(range) => {
                self.loop_frames = range.and_then(|(start, end)| {
                    let start = timeline.tick_to_frame(start);
                    let end = timeline.tick_to_frame(end);
                    (end > start).then_some((start, end))
                });
            }
        }
    }

    pub fn advance(&mut self, frames: u64) {
        if self.state != TransportState::Playing {
            return;
        }
        self.frame = self.frame.saturating_add(frames);
        if let Some((start, end)) = self.loop_frames
            && self.frame >= end
        {
            let length = end - start;
            self.frame = start + (self.frame - start) % length;
        }
    }

    #[must_use]
    pub const fn state(&self) -> TransportState {
        self.state
    }

    #[must_use]
    pub const fn frame(&self) -> u64 {
        self.frame
    }

    #[must_use]
    pub fn tick(&self, timeline: &MusicalTimeline) -> Tick {
        timeline.frame_to_tick(self.frame)
    }
}

#[cfg(test)]
mod tests {
    use radiant_domain::{TempoPoint, TimeSignature};

    use crate::MeterPoint;

    use super::*;

    fn timeline() -> MusicalTimeline {
        MusicalTimeline::new(
            48_000,
            &[TempoPoint {
                at: Tick::ZERO,
                bpm: 120.0,
            }],
            &[MeterPoint {
                at: Tick::ZERO,
                signature: TimeSignature::default(),
            }],
        )
        .expect("timeline")
    }

    #[test]
    fn play_pause_seek_stop_are_explicit() {
        let timeline = timeline();
        let mut transport = Transport::default();
        transport.command(TransportCommand::Seek(Tick::beats(2)), &timeline);
        transport.command(TransportCommand::Play, &timeline);
        transport.advance(100);
        assert_eq!(transport.frame(), 48_100);
        transport.command(TransportCommand::Pause, &timeline);
        transport.advance(100);
        assert_eq!(transport.frame(), 48_100);
        transport.command(TransportCommand::Stop, &timeline);
        assert_eq!(transport.frame(), 0);
    }

    #[test]
    fn loop_wrap_is_sample_exact() {
        let timeline = timeline();
        let mut transport = Transport::default();
        transport.command(
            TransportCommand::SetLoop(Some((Tick::beats(1), Tick::beats(2)))),
            &timeline,
        );
        transport.command(TransportCommand::Seek(Tick::beats(1)), &timeline);
        transport.command(TransportCommand::Play, &timeline);
        transport.advance(24_100);
        assert_eq!(transport.frame(), 24_100);
    }

    #[test]
    fn playhead_matches_tempo_and_meter_timeline() {
        let timeline = MusicalTimeline::new(
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
                    signature: TimeSignature::default(),
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
        .expect("timeline");
        let mut transport = Transport::default();
        transport.command(TransportCommand::Seek(Tick::beats(11)), &timeline);
        assert_eq!(transport.frame(), 432_000);
        assert_eq!(
            timeline.tick_to_position(transport.tick(&timeline)),
            crate::MusicalPosition {
                bar: 4,
                beat: 1,
                tick: 0,
            }
        );
    }
}
