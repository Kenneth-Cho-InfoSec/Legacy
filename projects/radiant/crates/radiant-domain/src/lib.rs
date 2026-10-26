//! Persistent, UI-independent data model for Radiant sessions.

use std::collections::HashSet;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};
use thiserror::Error;
use uuid::Uuid;

pub const CURRENT_SCHEMA_VERSION: u32 = 3;
pub const TICKS_PER_QUARTER: i64 = 960;

macro_rules! object_id {
    ($name:ident) => {
        #[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
        #[serde(transparent)]
        pub struct $name(Uuid);

        impl $name {
            #[must_use]
            pub fn new() -> Self {
                Self(Uuid::new_v4())
            }

            #[must_use]
            pub const fn as_uuid(self) -> Uuid {
                self.0
            }
        }

        impl Default for $name {
            fn default() -> Self {
                Self::new()
            }
        }
    };
}

object_id!(SessionId);
object_id!(ChannelId);
object_id!(PatternId);
object_id!(PlaylistLaneId);
object_id!(ClipId);
object_id!(MixerInsertId);
object_id!(MediaId);
object_id!(NoteId);
object_id!(AutomationPointId);
object_id!(PlaylistMarkerId);
object_id!(ChannelGroupId);
object_id!(CompId);

/// A signed musical position in 960 PPQ ticks.
#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(transparent)]
pub struct Tick(pub i64);

impl Tick {
    pub const ZERO: Self = Self(0);

    #[must_use]
    pub const fn beats(beats: i64) -> Self {
        Self(beats * TICKS_PER_QUARTER)
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct TimeSignature {
    pub numerator: u8,
    pub denominator: u8,
}

impl Default for TimeSignature {
    fn default() -> Self {
        Self {
            numerator: 4,
            denominator: 4,
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Serialize)]
pub struct TempoPoint {
    pub at: Tick,
    pub bpm: f64,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Session {
    pub schema_version: u32,
    pub id: SessionId,
    pub name: String,
    pub sample_rate: u32,
    /// Calibrated device offsets in project sample frames.
    #[serde(default)]
    pub input_latency_frames: i64,
    #[serde(default)]
    pub output_latency_frames: i64,
    pub tempo_map: Vec<TempoPoint>,
    pub time_signature: TimeSignature,
    pub channels: Vec<Channel>,
    pub patterns: Vec<Pattern>,
    pub playlist_lanes: Vec<PlaylistLane>,
    pub mixer_inserts: Vec<MixerInsert>,
    pub media: Vec<MediaSource>,
    #[serde(default)]
    pub playlist_markers: Vec<PlaylistMarker>,
    #[serde(default)]
    pub playlist_clip_edits: Vec<PlaylistClipEdit>,
    #[serde(default)]
    pub channel_groups: Vec<ChannelGroup>,
    #[serde(default)]
    pub playlist_comps: Vec<PlaylistComp>,
}

impl Session {
    #[must_use]
    pub fn new(name: impl Into<String>) -> Self {
        let master = MixerInsert::master();
        Self {
            schema_version: CURRENT_SCHEMA_VERSION,
            id: SessionId::new(),
            name: name.into(),
            sample_rate: 48_000,
            input_latency_frames: 0,
            output_latency_frames: 0,
            tempo_map: vec![TempoPoint {
                at: Tick::ZERO,
                bpm: 140.0,
            }],
            time_signature: TimeSignature::default(),
            channels: Vec::new(),
            patterns: Vec::new(),
            playlist_lanes: vec![PlaylistLane::new("Track 1")],
            mixer_inserts: vec![master],
            media: Vec::new(),
            playlist_markers: Vec::new(),
            playlist_clip_edits: Vec::new(),
            channel_groups: Vec::new(),
            playlist_comps: Vec::new(),
        }
    }

    #[must_use]
    pub fn master_insert(&self) -> Option<&MixerInsert> {
        self.mixer_inserts.iter().find(|insert| insert.is_master)
    }

    #[must_use]
    pub fn validate(&self) -> Vec<ValidationIssue> {
        let mut issues = Vec::new();
        if self.schema_version != CURRENT_SCHEMA_VERSION {
            issues.push(ValidationIssue::UnsupportedSchema(self.schema_version));
        }
        if self.name.trim().is_empty() {
            issues.push(ValidationIssue::EmptySessionName);
        }
        if !(8_000..=768_000).contains(&self.sample_rate) {
            issues.push(ValidationIssue::InvalidSampleRate(self.sample_rate));
        }
        if self.tempo_map.is_empty()
            || self
                .tempo_map
                .iter()
                .any(|point| !point.bpm.is_finite() || !(1.0..=999.0).contains(&point.bpm))
        {
            issues.push(ValidationIssue::InvalidTempoMap);
        }

        validate_unique(
            self.channels.iter().map(|item| item.id),
            ObjectKind::Channel,
            &mut issues,
        );
        validate_unique(
            self.patterns.iter().map(|item| item.id),
            ObjectKind::Pattern,
            &mut issues,
        );
        validate_unique(
            self.playlist_lanes.iter().map(|item| item.id),
            ObjectKind::PlaylistLane,
            &mut issues,
        );
        validate_unique(
            self.mixer_inserts.iter().map(|item| item.id),
            ObjectKind::MixerInsert,
            &mut issues,
        );
        validate_unique(
            self.media.iter().map(|item| item.id),
            ObjectKind::Media,
            &mut issues,
        );
        validate_unique(
            self.playlist_lanes
                .iter()
                .flat_map(|lane| lane.clips.iter().map(|clip| clip.id)),
            ObjectKind::PlaylistClip,
            &mut issues,
        );
        validate_unique(
            self.patterns
                .iter()
                .flat_map(|pattern| pattern.lanes.iter())
                .flat_map(|lane| lane.notes.iter().map(|note| note.id)),
            ObjectKind::Note,
            &mut issues,
        );
        validate_unique(
            self.playlist_lanes
                .iter()
                .flat_map(|lane| &lane.clips)
                .filter_map(|clip| match &clip.content {
                    PlaylistClipContent::Automation { points, .. } => Some(points),
                    PlaylistClipContent::Pattern { .. } | PlaylistClipContent::Audio { .. } => None,
                })
                .flatten()
                .map(|point| point.id),
            ObjectKind::AutomationPoint,
            &mut issues,
        );

        if self
            .mixer_inserts
            .iter()
            .filter(|insert| insert.is_master)
            .count()
            != 1
        {
            issues.push(ValidationIssue::InvalidMasterCount);
        }

        let channel_ids: HashSet<_> = self.channels.iter().map(|channel| channel.id).collect();
        let mixer_ids: HashSet<_> = self.mixer_inserts.iter().map(|insert| insert.id).collect();
        let pattern_ids: HashSet<_> = self.patterns.iter().map(|pattern| pattern.id).collect();
        let media_ids: HashSet<_> = self.media.iter().map(|media| media.id).collect();

        for channel in &self.channels {
            validate_channel(channel, &mixer_ids, &mut issues);
        }
        for pattern in &self.patterns {
            validate_pattern(pattern, &channel_ids, &mut issues);
        }
        for lane in &self.playlist_lanes {
            for clip in &lane.clips {
                validate_playlist_clip(clip, &pattern_ids, &media_ids, &mut issues);
            }
        }
        for insert in &self.mixer_inserts {
            validate_mixer_insert(insert, &mixer_ids, &mut issues);
        }
        issues
    }
}

fn validate_channel(
    channel: &Channel,
    mixer_ids: &HashSet<MixerInsertId>,
    issues: &mut Vec<ValidationIssue>,
) {
    if !mixer_ids.contains(&channel.mixer_insert) {
        issues.push(ValidationIssue::MissingMixerInsert(channel.mixer_insert));
    }
    if !channel.volume.is_finite()
        || !(0.0..=1.25).contains(&channel.volume)
        || !channel.pan.is_finite()
        || !(-1.0..=1.0).contains(&channel.pan)
    {
        issues.push(ValidationIssue::InvalidChannelControls(channel.id));
    }
}

fn validate_pattern(
    pattern: &Pattern,
    channel_ids: &HashSet<ChannelId>,
    issues: &mut Vec<ValidationIssue>,
) {
    if pattern.length <= Tick::ZERO {
        issues.push(ValidationIssue::InvalidPatternLength(pattern.id));
    }
    for lane in &pattern.lanes {
        if !channel_ids.contains(&lane.channel_id) {
            issues.push(ValidationIssue::MissingChannel(lane.channel_id));
        }
        if lane.steps.iter().any(|step| {
            step.at < Tick::ZERO
                || !step.velocity.is_finite()
                || !(0.0..=1.0).contains(&step.velocity)
                || !step.probability.is_finite()
                || !(0.0..=1.0).contains(&step.probability)
                || step.ratchets == 0
        }) {
            issues.push(ValidationIssue::InvalidPatternSteps(pattern.id));
        }
        if lane
            .notes
            .iter()
            .any(|note| note.start < Tick::ZERO || note.length <= Tick::ZERO || note.channel > 15)
        {
            issues.push(ValidationIssue::InvalidPatternNotes(pattern.id));
        }
    }
}

fn validate_playlist_clip(
    clip: &PlaylistClip,
    pattern_ids: &HashSet<PatternId>,
    media_ids: &HashSet<MediaId>,
    issues: &mut Vec<ValidationIssue>,
) {
    if clip.start < Tick::ZERO || clip.length <= Tick::ZERO {
        issues.push(ValidationIssue::InvalidPlaylistClip(clip.id));
    }
    match &clip.content {
        PlaylistClipContent::Pattern { pattern_id } if !pattern_ids.contains(pattern_id) => {
            issues.push(ValidationIssue::MissingPattern(*pattern_id));
        }
        PlaylistClipContent::Audio { media_id, .. } if !media_ids.contains(media_id) => {
            issues.push(ValidationIssue::MissingMedia(*media_id));
        }
        PlaylistClipContent::Automation { points, .. }
            if points.iter().any(|point| {
                point.at < Tick::ZERO
                    || !point.value.is_finite()
                    || !point.tension.is_finite()
                    || !(-1.0..=1.0).contains(&point.tension)
            }) =>
        {
            issues.push(ValidationIssue::InvalidAutomationPoints(clip.id));
        }
        PlaylistClipContent::Pattern { .. }
        | PlaylistClipContent::Audio { .. }
        | PlaylistClipContent::Automation { .. } => {}
    }
}

fn validate_mixer_insert(
    insert: &MixerInsert,
    mixer_ids: &HashSet<MixerInsertId>,
    issues: &mut Vec<ValidationIssue>,
) {
    if !insert.gain_db.is_finite()
        || !(-120.0..=24.0).contains(&insert.gain_db)
        || !insert.pan.is_finite()
        || !(-1.0..=1.0).contains(&insert.pan)
        || insert.routes.iter().any(|route| {
            !mixer_ids.contains(&route.destination)
                || route.destination == insert.id
                || !route.gain.is_finite()
                || !(0.0..=4.0).contains(&route.gain)
        })
    {
        issues.push(ValidationIssue::InvalidMixerInsert(insert.id));
    }
}

fn validate_unique<T: Eq + std::hash::Hash>(
    ids: impl Iterator<Item = T>,
    kind: ObjectKind,
    issues: &mut Vec<ValidationIssue>,
) {
    let mut seen = HashSet::new();
    if ids.into_iter().any(|id| !seen.insert(id)) {
        issues.push(ValidationIssue::DuplicateId(kind));
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Channel {
    pub id: ChannelId,
    pub name: String,
    pub color: Rgba,
    pub source: ChannelSource,
    pub mixer_insert: MixerInsertId,
    pub muted: bool,
    pub volume: f32,
    pub pan: f32,
}

impl Channel {
    #[must_use]
    pub fn sampler(name: impl Into<String>, mixer_insert: MixerInsertId) -> Self {
        Self {
            id: ChannelId::new(),
            name: name.into(),
            color: Rgba::accent(),
            source: ChannelSource::Sampler { media_id: None },
            mixer_insert,
            muted: false,
            volume: 1.0,
            pan: 0.0,
        }
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ChannelSource {
    Sampler { media_id: Option<MediaId> },
    BuiltInInstrument { instrument: String },
    Plugin { plugin_id: String },
    AudioInput { ports: Vec<String> },
    AutomationController,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Pattern {
    pub id: PatternId,
    pub name: String,
    pub length: Tick,
    pub lanes: Vec<PatternLane>,
}

impl Pattern {
    #[must_use]
    pub fn new(name: impl Into<String>, length: Tick) -> Self {
        Self {
            id: PatternId::new(),
            name: name.into(),
            length,
            lanes: Vec::new(),
        }
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct PatternLane {
    pub channel_id: ChannelId,
    pub steps: Vec<Step>,
    pub notes: Vec<Note>,
}

#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Serialize)]
pub struct Step {
    pub at: Tick,
    pub velocity: f32,
    pub probability: f32,
    pub micro_shift: Tick,
    pub ratchets: u8,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Note {
    pub id: NoteId,
    pub start: Tick,
    pub length: Tick,
    pub pitch: u8,
    pub velocity: u8,
    pub channel: u8,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct PlaylistLane {
    pub id: PlaylistLaneId,
    pub name: String,
    pub color: Rgba,
    pub height: u16,
    pub muted: bool,
    pub clips: Vec<PlaylistClip>,
    pub association: Option<TrackAssociation>,
}

impl PlaylistLane {
    #[must_use]
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            id: PlaylistLaneId::new(),
            name: name.into(),
            color: Rgba::track(),
            height: 64,
            muted: false,
            clips: Vec::new(),
            association: None,
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct TrackAssociation {
    pub channel_id: ChannelId,
    pub mixer_insert_id: MixerInsertId,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct PlaylistClip {
    pub id: ClipId,
    pub name: String,
    pub start: Tick,
    pub length: Tick,
    pub color: Rgba,
    pub muted: bool,
    pub content: PlaylistClipContent,
}

/// A non-destructive timeline marker.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct PlaylistMarker {
    pub id: PlaylistMarkerId,
    pub at: Tick,
    pub name: String,
    pub color: Rgba,
}

/// Non-destructive production edits kept separately from immutable clip media.
#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Serialize)]
pub struct PlaylistClipEdit {
    pub clip_id: ClipId,
    pub fade_in: Tick,
    pub fade_out: Tick,
    #[serde(default)]
    pub crossfade_in: Tick,
    #[serde(default)]
    pub crossfade_out: Tick,
    pub stretch_ratio: f64,
    pub frozen: bool,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct ChannelGroup {
    pub id: ChannelGroupId,
    pub name: String,
    pub channel_ids: Vec<ChannelId>,
    pub collapsed: bool,
}

/// Comping keeps alternative takes and the active choice as project metadata;
/// source clips remain immutable and can be restored through undo.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct PlaylistComp {
    pub id: CompId,
    pub lane_id: PlaylistLaneId,
    pub take_clip_ids: Vec<ClipId>,
    pub active_take: usize,
}

impl PlaylistComp {
    #[must_use]
    pub fn active_clip_id(&self) -> Option<ClipId> {
        self.take_clip_ids.get(self.active_take).copied()
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum PlaylistClipContent {
    Pattern {
        pattern_id: PatternId,
    },
    Audio {
        media_id: MediaId,
        /// Frame offset into the immutable source.  Trimming and slipping
        /// change this value; they never alter the recorded/imported file.
        source_start_frame: i64,
        gain_db: f32,
    },
    Automation {
        target: AutomationTarget,
        points: Vec<AutomationPoint>,
    },
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum AutomationTarget {
    ChannelVolume {
        channel_id: ChannelId,
    },
    ChannelPan {
        channel_id: ChannelId,
    },
    MixerGain {
        mixer_insert_id: MixerInsertId,
    },
    PluginParameter {
        instance_id: Uuid,
        parameter_id: String,
    },
    Tempo,
}

#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Serialize)]
pub struct AutomationPoint {
    pub id: AutomationPointId,
    pub at: Tick,
    pub value: f64,
    pub interpolation: AutomationInterpolation,
    pub tension: f32,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum AutomationInterpolation {
    Hold,
    Linear,
    Smooth,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct MixerInsert {
    pub id: MixerInsertId,
    pub name: String,
    pub color: Rgba,
    pub is_master: bool,
    pub gain_db: f32,
    pub pan: f32,
    pub muted: bool,
    #[serde(default)]
    pub solo: bool,
    pub plugins: Vec<PluginSlot>,
    pub routes: Vec<MixerRoute>,
}

impl MixerInsert {
    #[must_use]
    pub fn master() -> Self {
        Self {
            id: MixerInsertId::new(),
            name: "Master".into(),
            color: Rgba::accent(),
            is_master: true,
            gain_db: 0.0,
            pan: 0.0,
            muted: false,
            solo: false,
            plugins: Vec::new(),
            routes: Vec::new(),
        }
    }

    #[must_use]
    pub fn insert(name: impl Into<String>, master_id: MixerInsertId) -> Self {
        Self {
            id: MixerInsertId::new(),
            name: name.into(),
            color: Rgba::track(),
            is_master: false,
            gain_db: 0.0,
            pan: 0.0,
            muted: false,
            solo: false,
            plugins: Vec::new(),
            routes: vec![MixerRoute {
                destination: master_id,
                gain: 1.0,
                pre_fader: false,
                sidechain: false,
            }],
        }
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct PluginSlot {
    pub instance_id: Uuid,
    pub format: PluginFormat,
    pub plugin_id: String,
    pub enabled: bool,
    pub opaque_state: Vec<u8>,
    /// Reported processing latency in project sample frames.
    #[serde(default)]
    pub latency_frames: u32,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum PluginFormat {
    Lv2,
    Clap,
    Vst3,
}

#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Serialize)]
pub struct MixerRoute {
    pub destination: MixerInsertId,
    pub gain: f32,
    pub pre_fader: bool,
    pub sidechain: bool,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct MediaSource {
    pub id: MediaId,
    pub path: PathBuf,
    pub sample_rate: u32,
    pub channels: u16,
    pub frames: u64,
    #[serde(default)]
    pub import_mode: MediaImportMode,
    /// Stable source fingerprint used to invalidate generated peak caches.
    #[serde(default)]
    pub fingerprint: String,
    #[serde(default)]
    pub waveform: Option<WaveformCache>,
}

#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum MediaImportMode {
    /// Project-owned copy in `audio/`; preferred for portable projects and takes.
    #[default]
    Copy,
    /// Reference an external asset, which can later be relinked.
    Link,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct WaveformCache {
    pub path: PathBuf,
    pub fingerprint: String,
    pub buckets: u32,
}

#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Serialize)]
pub struct Rgba {
    pub red: f32,
    pub green: f32,
    pub blue: f32,
    pub alpha: f32,
}

impl Rgba {
    #[must_use]
    pub const fn accent() -> Self {
        Self {
            red: 0.98,
            green: 0.46,
            blue: 0.24,
            alpha: 1.0,
        }
    }

    #[must_use]
    pub const fn track() -> Self {
        Self {
            red: 0.20,
            green: 0.66,
            blue: 0.62,
            alpha: 1.0,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ObjectKind {
    Channel,
    Pattern,
    PlaylistLane,
    PlaylistClip,
    MixerInsert,
    Media,
    Note,
    AutomationPoint,
}

#[derive(Clone, Copy, Debug, Error, PartialEq)]
pub enum ValidationIssue {
    #[error("unsupported project schema {0}")]
    UnsupportedSchema(u32),
    #[error("the session name is empty")]
    EmptySessionName,
    #[error("invalid sample rate {0}")]
    InvalidSampleRate(u32),
    #[error("the tempo map is missing or invalid")]
    InvalidTempoMap,
    #[error("duplicate {0:?} identifier")]
    DuplicateId(ObjectKind),
    #[error("the project must contain exactly one master insert")]
    InvalidMasterCount,
    #[error("missing mixer insert {0:?}")]
    MissingMixerInsert(MixerInsertId),
    #[error("missing channel {0:?}")]
    MissingChannel(ChannelId),
    #[error("missing pattern {0:?}")]
    MissingPattern(PatternId),
    #[error("missing media source {0:?}")]
    MissingMedia(MediaId),
    #[error("pattern {0:?} has an invalid length")]
    InvalidPatternLength(PatternId),
    #[error("pattern {0:?} has invalid steps")]
    InvalidPatternSteps(PatternId),
    #[error("pattern {0:?} has invalid notes")]
    InvalidPatternNotes(PatternId),
    #[error("playlist clip {0:?} has an invalid range")]
    InvalidPlaylistClip(ClipId),
    #[error("channel {0:?} has invalid controls")]
    InvalidChannelControls(ChannelId),
    #[error("mixer insert {0:?} has invalid controls or routes")]
    InvalidMixerInsert(MixerInsertId),
    #[error("automation clip {0:?} has invalid points")]
    InvalidAutomationPoints(ClipId),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_session_has_one_master_and_is_valid() {
        let session = Session::new("Test");
        assert_eq!(session.mixer_inserts.len(), 1);
        assert!(session.master_insert().is_some());
        assert_eq!(session.validate(), Vec::new());
    }

    #[test]
    fn playlist_lane_accepts_mixed_clip_types() {
        let mut session = Session::new("Mixed clips");
        let master = session.master_insert().expect("master").id;
        let channel = Channel::sampler("Kick", master);
        let pattern = Pattern::new("Beat", Tick::beats(4));
        let media = MediaSource {
            id: MediaId::new(),
            path: "audio/take.wav".into(),
            sample_rate: 48_000,
            channels: 2,
            frames: 48_000,
            import_mode: MediaImportMode::Copy,
            fingerprint: String::new(),
            waveform: None,
        };
        session.channels.push(channel.clone());
        session.patterns.push(pattern.clone());
        session.media.push(media.clone());
        let lane = &mut session.playlist_lanes[0];
        lane.clips.push(PlaylistClip {
            id: ClipId::new(),
            name: "Beat".into(),
            start: Tick::ZERO,
            length: pattern.length,
            color: Rgba::accent(),
            muted: false,
            content: PlaylistClipContent::Pattern {
                pattern_id: pattern.id,
            },
        });
        lane.clips.push(PlaylistClip {
            id: ClipId::new(),
            name: "Take".into(),
            start: Tick::beats(4),
            length: Tick::beats(4),
            color: Rgba::track(),
            muted: false,
            content: PlaylistClipContent::Audio {
                media_id: media.id,
                source_start_frame: 0,
                gain_db: 0.0,
            },
        });

        assert_eq!(lane.clips.len(), 2);
        assert_eq!(session.validate(), Vec::new());
    }

    #[test]
    fn missing_pattern_reference_is_reported() {
        let mut session = Session::new("Broken");
        session.playlist_lanes[0].clips.push(PlaylistClip {
            id: ClipId::new(),
            name: "Missing".into(),
            start: Tick::ZERO,
            length: Tick::beats(4),
            color: Rgba::accent(),
            muted: false,
            content: PlaylistClipContent::Pattern {
                pattern_id: PatternId::new(),
            },
        });
        assert!(
            session
                .validate()
                .iter()
                .any(|issue| matches!(issue, ValidationIssue::MissingPattern(_)))
        );
    }
}
pub mod editing;
