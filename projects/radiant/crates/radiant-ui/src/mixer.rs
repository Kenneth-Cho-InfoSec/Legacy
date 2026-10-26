use radiant_domain::MixerInsert;

pub fn merged_controls(
    insert: &MixerInsert,
    gain_db: Option<f32>,
    pan: Option<f32>,
    muted: Option<bool>,
) -> (f32, f32, bool) {
    (
        gain_db.unwrap_or(insert.gain_db),
        pan.unwrap_or(insert.pan),
        muted.unwrap_or(insert.muted),
    )
}
