use radiant_domain::Tick;

#[allow(clippy::cast_precision_loss)]
pub fn ticks_to_beats(ticks: Tick) -> f64 {
    ticks.0 as f64 / 960.0
}

#[allow(clippy::cast_possible_truncation, clippy::cast_precision_loss)]
pub fn beats_to_ticks(beats: f64) -> Tick {
    let ticks = (beats * 960.0)
        .round()
        .clamp(i64::MIN as f64, i64::MAX as f64) as i64;
    Tick(ticks.max(1))
}

#[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
pub fn normalized_velocity_to_midi(value: f64) -> u8 {
    (value.clamp(0.0, 1.0) * 127.0)
        .round()
        .clamp(0.0, f64::from(u8::MAX)) as u8
}
