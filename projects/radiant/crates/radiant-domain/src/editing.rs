#![allow(
    clippy::cast_possible_truncation,
    clippy::cast_precision_loss,
    clippy::cast_sign_loss,
    clippy::cast_possible_wrap
)]

use crate::{Note, PatternLane, Tick};

/// Applies a deterministic probability value to every step in a selection.
pub fn set_probability(lane: &mut PatternLane, probability: f32) {
    let probability = if probability.is_finite() {
        probability.clamp(0.0, 1.0)
    } else {
        1.0
    };
    for step in &mut lane.steps {
        step.probability = probability;
    }
}

/// Sets ratchet count, clamping to the UI-supported range.
pub fn set_ratchets(lane: &mut PatternLane, ratchets: u8) {
    for step in &mut lane.steps {
        step.ratchets = ratchets.clamp(1, 8);
    }
}

/// Applies a signed micro-timing offset without changing the step's musical position.
pub fn shift_steps(lane: &mut PatternLane, offset: Tick) {
    for step in &mut lane.steps {
        step.micro_shift = Tick(step.micro_shift.0.saturating_add(offset.0));
    }
}

/// Converts the step sequence into notes while retaining the source steps for undo.
pub fn burn_steps_to_pattern(lane: &mut PatternLane, note_length: Tick) {
    let length = note_length.max(Tick(1));
    lane.notes = lane
        .steps
        .iter()
        .map(|step| Note {
            id: crate::NoteId::new(),
            start: Tick(step.at.0.saturating_add(step.micro_shift.0).max(0)),
            length,
            pitch: 60,
            velocity: (step.velocity.clamp(0.0, 1.0) * 127.0) as u8,
            channel: 0,
        })
        .collect();
}

/// Quantize note starts to the nearest positive grid, preserving note lengths.
pub fn quantize_notes(notes: &mut [Note], grid: Tick) {
    if grid <= Tick::ZERO {
        return;
    }
    for note in notes {
        let remainder = note.start.0.rem_euclid(grid.0);
        let snapped = if remainder.saturating_mul(2) < grid.0 {
            note.start.0 - remainder
        } else {
            note.start.0 + grid.0 - remainder
        };
        note.start = Tick(snapped.max(0));
    }
}

/// Deterministic humanization suitable for undoable editing and repeatable renders.
pub fn humanize_notes(notes: &mut [Note], max_ticks: i64, seed: u64) {
    if max_ticks <= 0 {
        return;
    }
    let span = max_ticks.saturating_mul(2).saturating_add(1) as u64;
    for (index, note) in notes.iter_mut().enumerate() {
        let value = splitmix64(seed.wrapping_add(index as u64)) % span;
        let offset = value as i64 - max_ticks;
        note.start = Tick(note.start.0.saturating_add(offset).max(0));
    }
}

/// Extend each note to the next note in pitch/time order without overlap.
pub fn legato(notes: &mut [Note]) {
    let mut order: Vec<_> = (0..notes.len()).collect();
    order.sort_by_key(|&index| (notes[index].start, notes[index].pitch));
    for pair in order.windows(2) {
        let (current, next) = (pair[0], pair[1]);
        let end = notes[next].start.0.saturating_sub(notes[current].start.0);
        if end > 0 {
            notes[current].length = Tick(end);
        }
    }
}

/// Offset successive notes to create a strum while retaining their pitches.
pub fn strum(notes: &mut [Note], spacing: Tick) {
    if spacing <= Tick::ZERO {
        return;
    }
    let mut order: Vec<_> = (0..notes.len()).collect();
    order.sort_by_key(|&index| (notes[index].start, notes[index].pitch));
    for (position, index) in order.into_iter().enumerate() {
        notes[index].start = Tick(
            notes[index]
                .start
                .0
                .saturating_add(spacing.0.saturating_mul(position as i64)),
        );
    }
}

/// Reorders a selected chord into a deterministic arpeggio pattern.
pub fn arpeggiate(notes: &mut [Note], spacing: Tick, pattern: &[usize]) {
    if notes.is_empty() || spacing <= Tick::ZERO || pattern.is_empty() {
        return;
    }
    let mut order: Vec<_> = (0..notes.len()).collect();
    order.sort_by_key(|&index| (notes[index].start, notes[index].pitch));
    let pitches: Vec<_> = order.iter().map(|&index| notes[index].pitch).collect();
    let origin = notes[order[0]].start;
    for (position, &index) in order.iter().enumerate() {
        let source = pattern[position % pattern.len()] % pitches.len();
        notes[index].pitch = pitches[source];
        notes[index].start = Tick(
            origin
                .0
                .saturating_add(spacing.0.saturating_mul(position as i64)),
        );
    }
}

/// Constrain pitches to a scale represented by pitch classes (0..=11).
pub fn constrain_scale(notes: &mut [Note], scale: &[u8]) {
    if scale.is_empty() {
        return;
    }
    for note in notes {
        let mut pitch = note.pitch;
        while !scale.contains(&(pitch % 12)) && pitch > 0 {
            pitch -= 1;
        }
        note.pitch = pitch;
    }
}

/// Stretch selected notes around an origin by a fixed ratio in Q16 precision.
pub fn stretch_notes(notes: &mut [Note], origin: Tick, numerator: i64, denominator: i64) {
    if denominator <= 0 || numerator < 0 {
        return;
    }
    for note in notes {
        let start = note.start.0.saturating_sub(origin.0).max(0);
        note.start = Tick(
            origin
                .0
                .saturating_add(start.saturating_mul(numerator) / denominator),
        );
        note.length = Tick(note.length.0.saturating_mul(numerator) / denominator).max(Tick(1));
    }
}

pub fn apply_swing(lane: &mut PatternLane, subdivision: Tick, amount: f32) {
    if subdivision <= Tick::ZERO || !amount.is_finite() {
        return;
    }
    let amount = amount.clamp(-1.0, 1.0);
    for step in &mut lane.steps {
        if (step.at.0 / subdivision.0) % 2 != 0 {
            step.micro_shift = Tick((subdivision.0 as f32 * 0.5 * amount) as i64);
        }
    }
}

fn splitmix64(mut value: u64) -> u64 {
    value = value.wrapping_add(0x9E37_79B9_7F4A_7C15);
    let mut z = value;
    z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
    z ^ (z >> 31)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{NoteId, Step};

    fn notes() -> Vec<Note> {
        vec![
            Note {
                id: NoteId::new(),
                start: Tick(11),
                length: Tick(4),
                pitch: 61,
                velocity: 100,
                channel: 0,
            },
            Note {
                id: NoteId::new(),
                start: Tick(31),
                length: Tick(4),
                pitch: 64,
                velocity: 100,
                channel: 0,
            },
        ]
    }

    #[test]
    fn note_tools_are_deterministic_and_non_destructive_to_identity() {
        let mut value = notes();
        quantize_notes(&mut value, Tick(10));
        assert_eq!(value[0].start, Tick(10));
        humanize_notes(&mut value, 2, 7);
        let first = value.clone();
        let mut repeat = notes();
        quantize_notes(&mut repeat, Tick(10));
        humanize_notes(&mut repeat, 2, 7);
        assert_eq!(first[0].start, repeat[0].start);
        legato(&mut value);
        strum(&mut value, Tick(2));
        constrain_scale(&mut value, &[0, 2, 4, 5, 7, 9, 11]);
        stretch_notes(&mut value, Tick::ZERO, 2, 1);
        arpeggiate(&mut value, Tick(2), &[0, 1]);
        assert_eq!(value[0].id, first[0].id);
    }

    #[test]
    fn swing_only_moves_offbeat_steps() {
        let mut lane = PatternLane {
            channel_id: crate::ChannelId::new(),
            steps: vec![
                Step {
                    at: Tick(0),
                    velocity: 1.0,
                    probability: 1.0,
                    micro_shift: Tick::ZERO,
                    ratchets: 1,
                },
                Step {
                    at: Tick(120),
                    velocity: 1.0,
                    probability: 1.0,
                    micro_shift: Tick::ZERO,
                    ratchets: 1,
                },
            ],
            notes: Vec::new(),
        };
        apply_swing(&mut lane, Tick(120), 1.0);
        assert_eq!(lane.steps[0].micro_shift, Tick::ZERO);
        assert!(lane.steps[1].micro_shift > Tick::ZERO);
    }
}
