use radiant_domain::Tick;

#[allow(clippy::cast_precision_loss)]
pub fn ticks_to_bars(ticks: Tick) -> f64 {
    ticks.0 as f64 / 3_840.0
}

#[allow(clippy::cast_possible_truncation, clippy::cast_precision_loss)]
pub fn bars_to_ticks(bars: f64) -> Tick {
    let ticks = (bars * 3_840.0)
        .round()
        .clamp(i64::MIN as f64, i64::MAX as f64) as i64;
    Tick(ticks.max(1))
}
