#![allow(clippy::cast_precision_loss)]

//! Sample-accurate automation evaluation and write-mode gesture handling.

use radiant_domain::{AutomationInterpolation, AutomationPoint, Tick};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AutomationMode {
    Read,
    Write,
    Touch,
    Latch,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SamplePoint {
    pub frame: u64,
    pub value: f64,
    pub interpolation: AutomationInterpolation,
}

/// Evaluates a sorted automation curve at an exact sample frame.
#[must_use]
pub fn evaluate(points: &[SamplePoint], frame: u64, default: f64) -> f64 {
    let Some(first) = points.first() else {
        return default;
    };
    if frame <= first.frame {
        return first.value;
    }
    for pair in points.windows(2) {
        let [left, right] = pair else { continue };
        if frame < right.frame {
            if left.interpolation == AutomationInterpolation::Hold || right.frame == left.frame {
                return left.value;
            }
            let position = (frame - left.frame) as f64 / (right.frame - left.frame) as f64;
            let position = if left.interpolation == AutomationInterpolation::Smooth {
                position * position * (3.0 - 2.0 * position)
            } else {
                position
            };
            return left.value + (right.value - left.value) * position;
        }
    }
    points.last().map_or(default, |point| point.value)
}

/// Applies a completed control gesture while preserving the original curve.
/// Touch restores the underlying value after release; latch keeps the final value.
pub fn apply_gesture(
    points: &mut Vec<AutomationPoint>,
    mode: AutomationMode,
    start: Tick,
    end: Tick,
    values: &[(Tick, f64)],
) {
    if values.is_empty() || end < start {
        return;
    }
    if mode == AutomationMode::Read {
        return;
    }
    points.retain(|point| point.at < start || point.at > end);
    for (at, value) in values
        .iter()
        .copied()
        .filter(|(at, _)| *at >= start && *at <= end)
    {
        points.push(AutomationPoint {
            id: radiant_domain::AutomationPointId::new(),
            at,
            value,
            interpolation: AutomationInterpolation::Linear,
            tension: 0.0,
        });
    }
    points.sort_by_key(|point| point.at);
    if mode == AutomationMode::Touch {
        // A release point makes the return-to-read transition explicit.
        if let Some(last) = points.iter().rev().find(|point| point.at <= end).copied() {
            points.push(AutomationPoint {
                id: radiant_domain::AutomationPointId::new(),
                at: end,
                value: last.value,
                interpolation: AutomationInterpolation::Linear,
                tension: 0.0,
            });
            points.sort_by_key(|point| point.at);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn evaluator_is_sample_accurate_and_supports_hold_and_smooth() {
        let points = [
            SamplePoint {
                frame: 0,
                value: 0.0,
                interpolation: AutomationInterpolation::Linear,
            },
            SamplePoint {
                frame: 10,
                value: 1.0,
                interpolation: AutomationInterpolation::Hold,
            },
            SamplePoint {
                frame: 20,
                value: 0.0,
                interpolation: AutomationInterpolation::Smooth,
            },
        ];
        assert!((evaluate(&points, 5, -1.0) - 0.5).abs() < 1e-9);
        assert!((evaluate(&points, 15, -1.0) - 1.0).abs() < 1e-9);
        assert!((evaluate(&points, 30, -1.0) - 0.0).abs() < f64::EPSILON);
    }

    #[test]
    fn read_does_not_modify_and_touch_is_reversible_at_release() {
        let mut points = Vec::new();
        apply_gesture(
            &mut points,
            AutomationMode::Read,
            Tick::ZERO,
            Tick(10),
            &[(Tick(2), 1.0)],
        );
        assert!(points.is_empty());
        apply_gesture(
            &mut points,
            AutomationMode::Touch,
            Tick::ZERO,
            Tick(10),
            &[(Tick(2), 1.0), (Tick(8), 0.5)],
        );
        assert!(points.iter().any(|point| point.at == Tick(10)));
    }
}
