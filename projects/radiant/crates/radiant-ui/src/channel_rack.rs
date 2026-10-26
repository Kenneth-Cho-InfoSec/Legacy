use radiant_domain::Channel;

pub fn merged_controls(
    channel: &Channel,
    volume: Option<f32>,
    pan: Option<f32>,
    muted: Option<bool>,
) -> (f32, f32, bool) {
    (
        volume.unwrap_or(channel.volume),
        pan.unwrap_or(channel.pan),
        muted.unwrap_or(channel.muted),
    )
}
