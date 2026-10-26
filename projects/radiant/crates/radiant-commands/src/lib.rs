//! Atomic, reversible mutations for [`radiant_domain::Session`].

use radiant_domain::{
    AutomationPoint, Channel, ChannelId, ClipId, MediaId, MediaSource, MixerInsert, MixerInsertId,
    MixerRoute, Note, NoteId, Pattern, PatternId, PatternLane, PlaylistClip, PlaylistClipEdit,
    PlaylistComp, PlaylistLane, PlaylistLaneId, PlaylistMarker, PlaylistMarkerId, Session, Step,
    TempoPoint, Tick, ValidationIssue,
};
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Command {
    RenameSession {
        name: String,
    },
    SetInitialTempo {
        bpm: f64,
    },
    AddChannel {
        channel: Channel,
    },
    RemoveChannel {
        channel_id: ChannelId,
    },
    RestoreChannel {
        channel: Channel,
        index: usize,
    },
    AddPattern {
        pattern: Pattern,
    },
    RemovePattern {
        pattern_id: PatternId,
    },
    RestorePattern {
        pattern: Pattern,
        index: usize,
    },
    AddPlaylistLane {
        lane: PlaylistLane,
    },
    RemovePlaylistLane {
        lane_id: PlaylistLaneId,
    },
    RestorePlaylistLane {
        lane: PlaylistLane,
        index: usize,
    },
    AddPlaylistClip {
        lane_id: PlaylistLaneId,
        clip: PlaylistClip,
    },
    RemovePlaylistClip {
        lane_id: PlaylistLaneId,
        clip_id: ClipId,
    },
    RestorePlaylistClip {
        lane_id: PlaylistLaneId,
        clip: PlaylistClip,
        index: usize,
    },
    SetPlaylistClipEdit {
        edit: PlaylistClipEdit,
    },
    AddPlaylistMarker {
        marker: PlaylistMarker,
    },
    RemovePlaylistMarker {
        marker_id: PlaylistMarkerId,
    },
    SetPlaylistComp {
        comp: PlaylistComp,
    },
    RemovePlaylistComp {
        comp_id: radiant_domain::CompId,
    },
    AddMediaSource {
        media: MediaSource,
    },
    RemoveMediaSource {
        media_id: MediaId,
    },
    RestoreMediaSource {
        media: MediaSource,
        index: usize,
    },
    SetMediaSource {
        media: MediaSource,
    },
    AddMixerInsert {
        insert: MixerInsert,
    },
    RemoveMixerInsert {
        mixer_insert_id: MixerInsertId,
    },
    RestoreMixerInsert {
        insert: MixerInsert,
        index: usize,
    },
    SetPatternStep {
        pattern_id: PatternId,
        channel_id: ChannelId,
        at: Tick,
        step: Option<Step>,
    },
    SetPatternNote {
        pattern_id: PatternId,
        channel_id: ChannelId,
        note_id: NoteId,
        note: Option<Note>,
    },
    /// Applies a complete non-destructive note transform as one undo step.
    SetPatternNotes {
        pattern_id: PatternId,
        channel_id: ChannelId,
        notes: Vec<Note>,
    },
    SetPlaylistClipRange {
        lane_id: PlaylistLaneId,
        clip_id: ClipId,
        start: Tick,
        length: Tick,
    },
    MovePlaylistClip {
        from_lane_id: PlaylistLaneId,
        to_lane_id: PlaylistLaneId,
        clip_id: ClipId,
    },
    SetPlaylistClipMuted {
        lane_id: PlaylistLaneId,
        clip_id: ClipId,
        muted: bool,
    },
    /// Non-destructive slip/left-trim for audio clips.  The source file is
    /// never rewritten and right-edge resize remains a reveal/trim only.
    SetAudioClipSourceStart {
        lane_id: PlaylistLaneId,
        clip_id: ClipId,
        source_start_frame: i64,
    },
    SetChannelControls {
        channel_id: ChannelId,
        volume: f32,
        pan: f32,
        muted: bool,
    },
    SetChannelMixerInsert {
        channel_id: ChannelId,
        mixer_insert_id: MixerInsertId,
    },
    SetMixerInsertControls {
        mixer_insert_id: MixerInsertId,
        gain_db: f32,
        pan: f32,
        muted: bool,
    },
    SetMixerInsertSolo {
        mixer_insert_id: MixerInsertId,
        solo: bool,
    },
    SetMixerRoutes {
        mixer_insert_id: MixerInsertId,
        routes: Vec<MixerRoute>,
    },
    SetAutomationPoints {
        lane_id: PlaylistLaneId,
        clip_id: ClipId,
        points: Vec<AutomationPoint>,
    },
}

impl Command {
    // Keeping the exhaustive command interpreter in one place makes the
    // command/inverse relationship auditable.
    #[allow(clippy::too_many_lines)]
    pub fn apply(&self, session: &mut Session) -> Result<Self, CommandError> {
        match self {
            Self::RenameSession { name } => {
                if name.trim().is_empty() {
                    return Err(CommandError::InvalidName);
                }
                let previous = std::mem::replace(&mut session.name, name.trim().to_owned());
                Ok(Self::RenameSession { name: previous })
            }
            Self::SetInitialTempo { bpm } => {
                if !bpm.is_finite() || !(1.0..=999.0).contains(bpm) {
                    return Err(CommandError::InvalidTempo(*bpm));
                }
                let previous = session.tempo_map.first().map_or(140.0, |point| point.bpm);
                if let Some(point) = session.tempo_map.first_mut() {
                    point.at = Tick::ZERO;
                    point.bpm = *bpm;
                } else {
                    session.tempo_map.push(TempoPoint {
                        at: Tick::ZERO,
                        bpm: *bpm,
                    });
                }
                Ok(Self::SetInitialTempo { bpm: previous })
            }
            Self::AddChannel { channel } => {
                if session.channels.iter().any(|item| item.id == channel.id) {
                    return Err(CommandError::AlreadyExists("channel"));
                }
                session.channels.push(channel.clone());
                Ok(Self::RemoveChannel {
                    channel_id: channel.id,
                })
            }
            Self::RemoveChannel { channel_id } => {
                if session
                    .patterns
                    .iter()
                    .flat_map(|pattern| &pattern.lanes)
                    .any(|lane| lane.channel_id == *channel_id)
                {
                    return Err(CommandError::ObjectInUse("channel"));
                }
                let index = session
                    .channels
                    .iter()
                    .position(|channel| channel.id == *channel_id)
                    .ok_or(CommandError::NotFound("channel"))?;
                let channel = session.channels.remove(index);
                Ok(Self::RestoreChannel { channel, index })
            }
            Self::RestoreChannel { channel, index } => {
                if session.channels.iter().any(|item| item.id == channel.id) {
                    return Err(CommandError::AlreadyExists("channel"));
                }
                let index = (*index).min(session.channels.len());
                session.channels.insert(index, channel.clone());
                Ok(Self::RemoveChannel {
                    channel_id: channel.id,
                })
            }
            Self::AddPattern { pattern } => {
                if session.patterns.iter().any(|item| item.id == pattern.id) {
                    return Err(CommandError::AlreadyExists("pattern"));
                }
                if pattern.length <= Tick::ZERO {
                    return Err(CommandError::InvalidPatternLength);
                }
                session.patterns.push(pattern.clone());
                Ok(Self::RemovePattern {
                    pattern_id: pattern.id,
                })
            }
            Self::RemovePattern { pattern_id } => {
                if session.playlist_lanes.iter().any(|lane| {
                    lane.clips.iter().any(|clip| {
                        matches!(
                            clip.content,
                            radiant_domain::PlaylistClipContent::Pattern { pattern_id: id }
                                if id == *pattern_id
                        )
                    })
                }) {
                    return Err(CommandError::ObjectInUse("pattern"));
                }
                let index = session
                    .patterns
                    .iter()
                    .position(|pattern| pattern.id == *pattern_id)
                    .ok_or(CommandError::NotFound("pattern"))?;
                let pattern = session.patterns.remove(index);
                Ok(Self::RestorePattern { pattern, index })
            }
            Self::RestorePattern { pattern, index } => {
                if session.patterns.iter().any(|item| item.id == pattern.id) {
                    return Err(CommandError::AlreadyExists("pattern"));
                }
                let index = (*index).min(session.patterns.len());
                session.patterns.insert(index, pattern.clone());
                Ok(Self::RemovePattern {
                    pattern_id: pattern.id,
                })
            }
            Self::AddPlaylistLane { lane } => {
                if session.playlist_lanes.iter().any(|item| item.id == lane.id) {
                    return Err(CommandError::AlreadyExists("playlist lane"));
                }
                session.playlist_lanes.push(lane.clone());
                Ok(Self::RemovePlaylistLane { lane_id: lane.id })
            }
            Self::RemovePlaylistLane { lane_id } => {
                let index = session
                    .playlist_lanes
                    .iter()
                    .position(|lane| lane.id == *lane_id)
                    .ok_or(CommandError::NotFound("playlist lane"))?;
                let lane = session.playlist_lanes.remove(index);
                Ok(Self::RestorePlaylistLane { lane, index })
            }
            Self::RestorePlaylistLane { lane, index } => {
                if session.playlist_lanes.iter().any(|item| item.id == lane.id) {
                    return Err(CommandError::AlreadyExists("playlist lane"));
                }
                let index = (*index).min(session.playlist_lanes.len());
                session.playlist_lanes.insert(index, lane.clone());
                Ok(Self::RemovePlaylistLane { lane_id: lane.id })
            }
            Self::AddPlaylistClip { lane_id, clip } => {
                if session
                    .playlist_lanes
                    .iter()
                    .flat_map(|lane| &lane.clips)
                    .any(|item| item.id == clip.id)
                {
                    return Err(CommandError::AlreadyExists("playlist clip"));
                }
                let lane = playlist_lane_mut(session, *lane_id)?;
                lane.clips.push(clip.clone());
                Ok(Self::RemovePlaylistClip {
                    lane_id: *lane_id,
                    clip_id: clip.id,
                })
            }
            Self::RemovePlaylistClip { lane_id, clip_id } => {
                let lane = playlist_lane_mut(session, *lane_id)?;
                let index = lane
                    .clips
                    .iter()
                    .position(|clip| clip.id == *clip_id)
                    .ok_or(CommandError::NotFound("playlist clip"))?;
                let clip = lane.clips.remove(index);
                Ok(Self::RestorePlaylistClip {
                    lane_id: *lane_id,
                    clip,
                    index,
                })
            }
            Self::RestorePlaylistClip {
                lane_id,
                clip,
                index,
            } => {
                if session
                    .playlist_lanes
                    .iter()
                    .flat_map(|lane| &lane.clips)
                    .any(|item| item.id == clip.id)
                {
                    return Err(CommandError::AlreadyExists("playlist clip"));
                }
                let lane = playlist_lane_mut(session, *lane_id)?;
                let index = (*index).min(lane.clips.len());
                lane.clips.insert(index, clip.clone());
                Ok(Self::RemovePlaylistClip {
                    lane_id: *lane_id,
                    clip_id: clip.id,
                })
            }
            Self::SetPlaylistClipEdit { edit } => {
                if !session
                    .playlist_lanes
                    .iter()
                    .flat_map(|lane| &lane.clips)
                    .any(|clip| clip.id == edit.clip_id)
                {
                    return Err(CommandError::NotFound("playlist clip"));
                }
                let previous = session
                    .playlist_clip_edits
                    .iter()
                    .find(|item| item.clip_id == edit.clip_id)
                    .copied();
                session
                    .playlist_clip_edits
                    .retain(|item| item.clip_id != edit.clip_id);
                if !edit.frozen
                    && edit.fade_in == Tick::ZERO
                    && edit.fade_out == Tick::ZERO
                    && edit.crossfade_in == Tick::ZERO
                    && edit.crossfade_out == Tick::ZERO
                    && (edit.stretch_ratio - 1.0).abs() < f64::EPSILON
                {
                    // Identity edits are represented by absence, keeping the source non-destructive and compact.
                } else {
                    session.playlist_clip_edits.push(*edit);
                }
                Ok(previous.map_or(
                    Self::SetPlaylistClipEdit {
                        edit: PlaylistClipEdit {
                            clip_id: edit.clip_id,
                            fade_in: Tick::ZERO,
                            fade_out: Tick::ZERO,
                            crossfade_in: Tick::ZERO,
                            crossfade_out: Tick::ZERO,
                            stretch_ratio: 1.0,
                            frozen: false,
                        },
                    },
                    |old| Self::SetPlaylistClipEdit { edit: old },
                ))
            }
            Self::AddPlaylistMarker { marker } => {
                if session
                    .playlist_markers
                    .iter()
                    .any(|item| item.id == marker.id)
                {
                    return Err(CommandError::AlreadyExists("playlist marker"));
                }
                session.playlist_markers.push(marker.clone());
                Ok(Self::RemovePlaylistMarker {
                    marker_id: marker.id,
                })
            }
            Self::RemovePlaylistMarker { marker_id } => {
                let index = session
                    .playlist_markers
                    .iter()
                    .position(|marker| marker.id == *marker_id)
                    .ok_or(CommandError::NotFound("playlist marker"))?;
                let marker = session.playlist_markers.remove(index);
                Ok(Self::AddPlaylistMarker { marker })
            }
            Self::SetPlaylistComp { comp } => {
                if !session
                    .playlist_lanes
                    .iter()
                    .any(|lane| lane.id == comp.lane_id)
                {
                    return Err(CommandError::NotFound("playlist lane"));
                }
                if comp.take_clip_ids.is_empty() || comp.active_take >= comp.take_clip_ids.len() {
                    return Err(CommandError::InvalidComp);
                }
                let previous = session
                    .playlist_comps
                    .iter()
                    .find(|item| item.id == comp.id)
                    .cloned();
                session.playlist_comps.retain(|item| item.id != comp.id);
                session.playlist_comps.push(comp.clone());
                Ok(
                    previous.map_or(Self::RemovePlaylistComp { comp_id: comp.id }, |old| {
                        Self::SetPlaylistComp { comp: old }
                    }),
                )
            }
            Self::RemovePlaylistComp { comp_id } => {
                let index = session
                    .playlist_comps
                    .iter()
                    .position(|item| item.id == *comp_id)
                    .ok_or(CommandError::NotFound("playlist comp"))?;
                let comp = session.playlist_comps.remove(index);
                Ok(Self::SetPlaylistComp { comp })
            }
            Self::AddMediaSource { media } => {
                if session.media.iter().any(|item| item.id == media.id) {
                    return Err(CommandError::AlreadyExists("media"));
                }
                session.media.push(media.clone());
                Ok(Self::RemoveMediaSource { media_id: media.id })
            }
            Self::RemoveMediaSource { media_id } => {
                if session.playlist_lanes.iter().flat_map(|lane| &lane.clips).any(|clip| {
                    matches!(&clip.content, radiant_domain::PlaylistClipContent::Audio { media_id: id, .. } if id == media_id)
                }) {
                    return Err(CommandError::ObjectInUse("media"));
                }
                let index = session
                    .media
                    .iter()
                    .position(|media| media.id == *media_id)
                    .ok_or(CommandError::NotFound("media"))?;
                let media = session.media.remove(index);
                Ok(Self::RestoreMediaSource { media, index })
            }
            Self::RestoreMediaSource { media, index } => {
                if session.media.iter().any(|item| item.id == media.id) {
                    return Err(CommandError::AlreadyExists("media"));
                }
                let index = (*index).min(session.media.len());
                session.media.insert(index, media.clone());
                Ok(Self::RemoveMediaSource { media_id: media.id })
            }
            Self::SetMediaSource { media } => {
                let existing = session
                    .media
                    .iter_mut()
                    .find(|item| item.id == media.id)
                    .ok_or(CommandError::NotFound("media"))?;
                let previous = std::mem::replace(existing, media.clone());
                Ok(Self::SetMediaSource { media: previous })
            }
            Self::AddMixerInsert { insert } => {
                if session
                    .mixer_inserts
                    .iter()
                    .any(|item| item.id == insert.id)
                {
                    return Err(CommandError::AlreadyExists("mixer insert"));
                }
                session.mixer_inserts.push(insert.clone());
                Ok(Self::RemoveMixerInsert {
                    mixer_insert_id: insert.id,
                })
            }
            Self::RemoveMixerInsert { mixer_insert_id } => {
                let index = session
                    .mixer_inserts
                    .iter()
                    .position(|insert| insert.id == *mixer_insert_id)
                    .ok_or(CommandError::NotFound("mixer insert"))?;
                if session.mixer_inserts[index].is_master
                    || session
                        .channels
                        .iter()
                        .any(|channel| channel.mixer_insert == *mixer_insert_id)
                    || session.mixer_inserts.iter().any(|insert| {
                        insert
                            .routes
                            .iter()
                            .any(|route| route.destination == *mixer_insert_id)
                    })
                {
                    return Err(CommandError::ObjectInUse("mixer insert"));
                }
                let insert = session.mixer_inserts.remove(index);
                Ok(Self::RestoreMixerInsert { insert, index })
            }
            Self::RestoreMixerInsert { insert, index } => {
                if session
                    .mixer_inserts
                    .iter()
                    .any(|item| item.id == insert.id)
                {
                    return Err(CommandError::AlreadyExists("mixer insert"));
                }
                let index = (*index).min(session.mixer_inserts.len());
                session.mixer_inserts.insert(index, insert.clone());
                Ok(Self::RemoveMixerInsert {
                    mixer_insert_id: insert.id,
                })
            }
            Self::SetPatternStep {
                pattern_id,
                channel_id,
                at,
                step,
            } => {
                if *at < Tick::ZERO
                    || step.is_some_and(|step| {
                        step.at != *at
                            || !step.velocity.is_finite()
                            || !(0.0..=1.0).contains(&step.velocity)
                            || !step.probability.is_finite()
                            || !(0.0..=1.0).contains(&step.probability)
                            || step.ratchets == 0
                    })
                {
                    return Err(CommandError::InvalidStep);
                }
                ensure_channel_exists(session, *channel_id)?;
                let pattern = pattern_mut(session, *pattern_id)?;
                let lane_index = pattern
                    .lanes
                    .iter()
                    .position(|lane| lane.channel_id == *channel_id);
                let previous = if let Some(lane_index) = lane_index {
                    replace_step(&mut pattern.lanes[lane_index], *at, *step)
                } else if let Some(step) = step {
                    pattern.lanes.push(PatternLane {
                        channel_id: *channel_id,
                        steps: vec![*step],
                        notes: Vec::new(),
                    });
                    None
                } else {
                    None
                };
                Ok(Self::SetPatternStep {
                    pattern_id: *pattern_id,
                    channel_id: *channel_id,
                    at: *at,
                    step: previous,
                })
            }
            Self::SetPatternNote {
                pattern_id,
                channel_id,
                note_id,
                note,
            } => {
                if note.is_some_and(|note| {
                    note.id != *note_id
                        || note.start < Tick::ZERO
                        || note.length <= Tick::ZERO
                        || note.channel > 15
                }) {
                    return Err(CommandError::InvalidNote);
                }
                ensure_channel_exists(session, *channel_id)?;
                let pattern = pattern_mut(session, *pattern_id)?;
                let lane_index = pattern
                    .lanes
                    .iter()
                    .position(|lane| lane.channel_id == *channel_id);
                let previous = if let Some(lane_index) = lane_index {
                    replace_note(&mut pattern.lanes[lane_index], *note_id, *note)
                } else if let Some(note) = note {
                    pattern.lanes.push(PatternLane {
                        channel_id: *channel_id,
                        steps: Vec::new(),
                        notes: vec![*note],
                    });
                    None
                } else {
                    None
                };
                Ok(Self::SetPatternNote {
                    pattern_id: *pattern_id,
                    channel_id: *channel_id,
                    note_id: *note_id,
                    note: previous,
                })
            }
            Self::SetPatternNotes {
                pattern_id,
                channel_id,
                notes,
            } => {
                if notes.iter().any(|note| {
                    note.start < Tick::ZERO || note.length <= Tick::ZERO || note.channel > 15
                }) || notes.windows(2).any(|pair| pair[0].id == pair[1].id)
                {
                    return Err(CommandError::InvalidNote);
                }
                ensure_channel_exists(session, *channel_id)?;
                let pattern = pattern_mut(session, *pattern_id)?;
                let lane = pattern
                    .lanes
                    .iter_mut()
                    .find(|lane| lane.channel_id == *channel_id)
                    .ok_or(CommandError::NotFound("pattern lane"))?;
                let previous = std::mem::replace(&mut lane.notes, notes.clone());
                Ok(Self::SetPatternNotes {
                    pattern_id: *pattern_id,
                    channel_id: *channel_id,
                    notes: previous,
                })
            }
            Self::SetPlaylistClipRange {
                lane_id,
                clip_id,
                start,
                length,
            } => {
                if *start < Tick::ZERO || *length <= Tick::ZERO {
                    return Err(CommandError::InvalidClipRange);
                }
                let lane = playlist_lane_mut(session, *lane_id)?;
                let clip = lane
                    .clips
                    .iter_mut()
                    .find(|clip| clip.id == *clip_id)
                    .ok_or(CommandError::NotFound("playlist clip"))?;
                let previous_start = std::mem::replace(&mut clip.start, *start);
                let previous_length = std::mem::replace(&mut clip.length, *length);
                Ok(Self::SetPlaylistClipRange {
                    lane_id: *lane_id,
                    clip_id: *clip_id,
                    start: previous_start,
                    length: previous_length,
                })
            }
            Self::MovePlaylistClip {
                from_lane_id,
                to_lane_id,
                clip_id,
            } => {
                if from_lane_id == to_lane_id {
                    return Err(CommandError::InvalidClipRange);
                }
                let source = playlist_lane_mut(session, *from_lane_id)?;
                let index = source
                    .clips
                    .iter()
                    .position(|clip| clip.id == *clip_id)
                    .ok_or(CommandError::NotFound("playlist clip"))?;
                let clip = source.clips.remove(index);
                playlist_lane_mut(session, *to_lane_id)?.clips.push(clip);
                Ok(Self::MovePlaylistClip {
                    from_lane_id: *to_lane_id,
                    to_lane_id: *from_lane_id,
                    clip_id: *clip_id,
                })
            }
            Self::SetPlaylistClipMuted {
                lane_id,
                clip_id,
                muted,
            } => {
                let lane = playlist_lane_mut(session, *lane_id)?;
                let clip = lane
                    .clips
                    .iter_mut()
                    .find(|clip| clip.id == *clip_id)
                    .ok_or(CommandError::NotFound("playlist clip"))?;
                let previous = std::mem::replace(&mut clip.muted, *muted);
                Ok(Self::SetPlaylistClipMuted {
                    lane_id: *lane_id,
                    clip_id: *clip_id,
                    muted: previous,
                })
            }
            Self::SetAudioClipSourceStart {
                lane_id,
                clip_id,
                source_start_frame,
            } => {
                if *source_start_frame < 0 {
                    return Err(CommandError::InvalidAudioSourceStart);
                }
                let lane = playlist_lane_mut(session, *lane_id)?;
                let clip = lane
                    .clips
                    .iter_mut()
                    .find(|clip| clip.id == *clip_id)
                    .ok_or(CommandError::NotFound("playlist clip"))?;
                let radiant_domain::PlaylistClipContent::Audio {
                    source_start_frame: previous,
                    ..
                } = &mut clip.content
                else {
                    return Err(CommandError::InvalidAudioClip);
                };
                let previous = std::mem::replace(previous, *source_start_frame);
                Ok(Self::SetAudioClipSourceStart {
                    lane_id: *lane_id,
                    clip_id: *clip_id,
                    source_start_frame: previous,
                })
            }
            Self::SetChannelControls {
                channel_id,
                volume,
                pan,
                muted,
            } => {
                if !volume.is_finite()
                    || !(0.0..=1.25).contains(volume)
                    || !pan.is_finite()
                    || !(-1.0..=1.0).contains(pan)
                {
                    return Err(CommandError::InvalidChannelControls);
                }
                let channel = session
                    .channels
                    .iter_mut()
                    .find(|channel| channel.id == *channel_id)
                    .ok_or(CommandError::NotFound("channel"))?;
                let inverse = Self::SetChannelControls {
                    channel_id: *channel_id,
                    volume: channel.volume,
                    pan: channel.pan,
                    muted: channel.muted,
                };
                channel.volume = *volume;
                channel.pan = *pan;
                channel.muted = *muted;
                Ok(inverse)
            }
            Self::SetChannelMixerInsert {
                channel_id,
                mixer_insert_id,
            } => {
                if !session
                    .mixer_inserts
                    .iter()
                    .any(|insert| insert.id == *mixer_insert_id)
                {
                    return Err(CommandError::NotFound("mixer insert"));
                }
                let channel = session
                    .channels
                    .iter_mut()
                    .find(|channel| channel.id == *channel_id)
                    .ok_or(CommandError::NotFound("channel"))?;
                let previous = std::mem::replace(&mut channel.mixer_insert, *mixer_insert_id);
                Ok(Self::SetChannelMixerInsert {
                    channel_id: *channel_id,
                    mixer_insert_id: previous,
                })
            }
            Self::SetMixerInsertControls {
                mixer_insert_id,
                gain_db,
                pan,
                muted,
            } => {
                if !gain_db.is_finite()
                    || !(-120.0..=24.0).contains(gain_db)
                    || !pan.is_finite()
                    || !(-1.0..=1.0).contains(pan)
                {
                    return Err(CommandError::InvalidMixerControls);
                }
                let insert = session
                    .mixer_inserts
                    .iter_mut()
                    .find(|insert| insert.id == *mixer_insert_id)
                    .ok_or(CommandError::NotFound("mixer insert"))?;
                let inverse = Self::SetMixerInsertControls {
                    mixer_insert_id: *mixer_insert_id,
                    gain_db: insert.gain_db,
                    pan: insert.pan,
                    muted: insert.muted,
                };
                insert.gain_db = *gain_db;
                insert.pan = *pan;
                insert.muted = *muted;
                Ok(inverse)
            }
            Self::SetMixerInsertSolo {
                mixer_insert_id,
                solo,
            } => {
                let insert = session
                    .mixer_inserts
                    .iter_mut()
                    .find(|insert| insert.id == *mixer_insert_id)
                    .ok_or(CommandError::NotFound("mixer insert"))?;
                let previous = std::mem::replace(&mut insert.solo, *solo);
                Ok(Self::SetMixerInsertSolo {
                    mixer_insert_id: *mixer_insert_id,
                    solo: previous,
                })
            }
            Self::SetMixerRoutes {
                mixer_insert_id,
                routes,
            } => {
                if routes.iter().any(|route| {
                    route.destination == *mixer_insert_id
                        || !route.gain.is_finite()
                        || !(0.0..=4.0).contains(&route.gain)
                }) {
                    return Err(CommandError::InvalidMixerRoutes);
                }
                let destinations: std::collections::HashSet<_> = session
                    .mixer_inserts
                    .iter()
                    .map(|insert| insert.id)
                    .collect();
                if routes
                    .iter()
                    .any(|route| !destinations.contains(&route.destination))
                {
                    return Err(CommandError::InvalidMixerRoutes);
                }
                let insert = session
                    .mixer_inserts
                    .iter_mut()
                    .find(|insert| insert.id == *mixer_insert_id)
                    .ok_or(CommandError::NotFound("mixer insert"))?;
                let previous = std::mem::replace(&mut insert.routes, routes.clone());
                Ok(Self::SetMixerRoutes {
                    mixer_insert_id: *mixer_insert_id,
                    routes: previous,
                })
            }
            Self::SetAutomationPoints {
                lane_id,
                clip_id,
                points,
            } => {
                if points.iter().any(|point| {
                    point.at < Tick::ZERO
                        || !point.value.is_finite()
                        || !point.tension.is_finite()
                        || !(-1.0..=1.0).contains(&point.tension)
                }) {
                    return Err(CommandError::InvalidAutomationPoints);
                }
                let lane = playlist_lane_mut(session, *lane_id)?;
                let clip = lane
                    .clips
                    .iter_mut()
                    .find(|clip| clip.id == *clip_id)
                    .ok_or(CommandError::NotFound("playlist clip"))?;
                let radiant_domain::PlaylistClipContent::Automation {
                    points: clip_points,
                    ..
                } = &mut clip.content
                else {
                    return Err(CommandError::WrongObjectType("automation clip"));
                };
                let previous = std::mem::replace(clip_points, points.clone());
                Ok(Self::SetAutomationPoints {
                    lane_id: *lane_id,
                    clip_id: *clip_id,
                    points: previous,
                })
            }
        }
    }
}

fn ensure_channel_exists(session: &Session, channel_id: ChannelId) -> Result<(), CommandError> {
    if session
        .channels
        .iter()
        .any(|channel| channel.id == channel_id)
    {
        Ok(())
    } else {
        Err(CommandError::NotFound("channel"))
    }
}

fn pattern_mut(session: &mut Session, pattern_id: PatternId) -> Result<&mut Pattern, CommandError> {
    session
        .patterns
        .iter_mut()
        .find(|pattern| pattern.id == pattern_id)
        .ok_or(CommandError::NotFound("pattern"))
}

fn replace_step(lane: &mut PatternLane, at: Tick, step: Option<Step>) -> Option<Step> {
    let previous = lane
        .steps
        .iter()
        .position(|step| step.at == at)
        .map(|index| lane.steps.remove(index));
    if let Some(step) = step {
        lane.steps.push(step);
        lane.steps.sort_by_key(|step| step.at);
    }
    previous
}

fn replace_note(lane: &mut PatternLane, note_id: NoteId, note: Option<Note>) -> Option<Note> {
    let previous = lane
        .notes
        .iter()
        .position(|note| note.id == note_id)
        .map(|index| lane.notes.remove(index));
    if let Some(note) = note {
        lane.notes.push(note);
        lane.notes.sort_by_key(|note| note.start);
    }
    previous
}

fn playlist_lane_mut(
    session: &mut Session,
    lane_id: PlaylistLaneId,
) -> Result<&mut PlaylistLane, CommandError> {
    session
        .playlist_lanes
        .iter_mut()
        .find(|lane| lane.id == lane_id)
        .ok_or(CommandError::NotFound("playlist lane"))
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Transaction {
    pub label: String,
    pub commands: Vec<Command>,
}

impl Transaction {
    #[must_use]
    pub fn one(label: impl Into<String>, command: Command) -> Self {
        Self {
            label: label.into(),
            commands: vec![command],
        }
    }

    /// Applies all commands atomically and returns an undo transaction.
    pub fn execute(&self, session: &mut Session) -> Result<Self, CommandError> {
        let mut candidate = session.clone();
        let mut inverse = Vec::with_capacity(self.commands.len());
        for command in &self.commands {
            inverse.push(command.apply(&mut candidate)?);
        }
        let issues = candidate.validate();
        if !issues.is_empty() {
            return Err(CommandError::InvalidSession(issues));
        }
        *session = candidate;
        inverse.reverse();
        Ok(Self {
            label: format!("Undo {}", self.label),
            commands: inverse,
        })
    }
}

#[derive(Debug, Error, PartialEq)]
pub enum CommandError {
    #[error("{0} already exists")]
    AlreadyExists(&'static str),
    #[error("{0} was not found")]
    NotFound(&'static str),
    #[error("{0} is still in use")]
    ObjectInUse(&'static str),
    #[error("name cannot be empty")]
    InvalidName,
    #[error("tempo {0} is outside the supported range")]
    InvalidTempo(f64),
    #[error("pattern length must be positive")]
    InvalidPatternLength,
    #[error("step data is invalid")]
    InvalidStep,
    #[error("note data is invalid")]
    InvalidNote,
    #[error("playlist clip range is invalid")]
    InvalidClipRange,
    #[error("audio source start must be zero or greater")]
    InvalidAudioSourceStart,
    #[error("the requested playlist clip is not audio")]
    InvalidAudioClip,
    #[error("channel controls are invalid")]
    InvalidChannelControls,
    #[error("mixer controls are invalid")]
    InvalidMixerControls,
    #[error("mixer routes are invalid")]
    InvalidMixerRoutes,
    #[error("automation points are invalid")]
    InvalidAutomationPoints,
    #[error("comp take selection is invalid")]
    InvalidComp,
    #[error("the object is not a {0}")]
    WrongObjectType(&'static str),
    #[error("the command would produce an invalid session: {0:?}")]
    InvalidSession(Vec<ValidationIssue>),
}

#[cfg(test)]
mod tests {
    use radiant_domain::{
        AutomationInterpolation, AutomationPoint, AutomationPointId, AutomationTarget, Channel,
        MixerInsert, MixerRoute, Note, Pattern, PlaylistClip, PlaylistClipContent,
        PlaylistClipEdit, PlaylistComp, PlaylistMarker, PlaylistMarkerId, Rgba, Step, Tick,
    };

    use super::*;

    #[test]
    fn transaction_is_atomic_when_a_later_command_fails() {
        let mut session = Session::new("Original");
        let transaction = Transaction {
            label: "Broken rename".into(),
            commands: vec![
                Command::RenameSession {
                    name: "Changed".into(),
                },
                Command::SetInitialTempo { bpm: -2.0 },
            ],
        };
        assert!(transaction.execute(&mut session).is_err());
        assert_eq!(session.name, "Original");
    }

    #[test]
    fn returned_transaction_restores_prior_state() {
        let mut session = Session::new("Original");
        let undo = Transaction::one(
            "Rename",
            Command::RenameSession {
                name: "Renamed".into(),
            },
        )
        .execute(&mut session)
        .expect("rename");
        assert_eq!(session.name, "Renamed");
        undo.execute(&mut session).expect("undo");
        assert_eq!(session.name, "Original");
    }

    #[test]
    fn pattern_cannot_be_removed_while_placed() {
        let mut session = Session::new("Placed pattern");
        let master = session.master_insert().expect("master").id;
        let channel = Channel::sampler("Kick", master);
        let pattern = Pattern::new("Beat", Tick::beats(4));
        session.channels.push(channel);
        session.patterns.push(pattern.clone());
        session.playlist_lanes[0].clips.push(PlaylistClip {
            id: ClipId::new(),
            name: "Beat".into(),
            start: Tick::ZERO,
            length: Tick::beats(4),
            color: Rgba::accent(),
            muted: false,
            content: PlaylistClipContent::Pattern {
                pattern_id: pattern.id,
            },
        });
        let result = Command::RemovePattern {
            pattern_id: pattern.id,
        }
        .apply(&mut session);
        assert_eq!(result, Err(CommandError::ObjectInUse("pattern")));
    }

    #[test]
    fn pattern_step_edit_is_reversible() {
        let mut session = Session::new("Steps");
        let master = session.master_insert().expect("master").id;
        let channel = Channel::sampler("Kick", master);
        let channel_id = channel.id;
        let pattern = Pattern::new("Beat", Tick::beats(4));
        let pattern_id = pattern.id;
        session.channels.push(channel);
        session.patterns.push(pattern);
        let step = Step {
            at: Tick(240),
            velocity: 0.8,
            probability: 1.0,
            micro_shift: Tick::ZERO,
            ratchets: 1,
        };

        let undo = Transaction::one(
            "Enable step",
            Command::SetPatternStep {
                pattern_id,
                channel_id,
                at: step.at,
                step: Some(step),
            },
        )
        .execute(&mut session)
        .expect("enable step");
        assert_eq!(session.patterns[0].lanes[0].steps, vec![step]);

        undo.execute(&mut session).expect("undo step");
        assert!(session.patterns[0].lanes[0].steps.is_empty());
    }

    #[test]
    fn note_resize_is_reversible() {
        let mut session = Session::new("Notes");
        let master = session.master_insert().expect("master").id;
        let channel = Channel::sampler("Synth", master);
        let channel_id = channel.id;
        let pattern = Pattern::new("Melody", Tick::beats(4));
        let pattern_id = pattern.id;
        let note = Note {
            id: radiant_domain::NoteId::new(),
            start: Tick::ZERO,
            length: Tick::beats(1),
            pitch: 60,
            velocity: 100,
            channel: 0,
        };
        session.channels.push(channel);
        session.patterns.push(pattern);
        Command::SetPatternNote {
            pattern_id,
            channel_id,
            note_id: note.id,
            note: Some(note),
        }
        .apply(&mut session)
        .expect("add note");
        let resized = Note {
            length: Tick::beats(2),
            ..note
        };

        let undo = Transaction::one(
            "Resize note",
            Command::SetPatternNote {
                pattern_id,
                channel_id,
                note_id: note.id,
                note: Some(resized),
            },
        )
        .execute(&mut session)
        .expect("resize note");
        assert_eq!(session.patterns[0].lanes[0].notes[0], resized);

        undo.execute(&mut session).expect("undo resize");
        assert_eq!(session.patterns[0].lanes[0].notes[0], note);
    }

    #[test]
    fn transformed_note_selection_is_one_reversible_command() {
        let mut session = Session::new("Transform");
        let master = session.master_insert().expect("master").id;
        let channel = Channel::sampler("Lead", master);
        let channel_id = channel.id;
        let pattern = Pattern::new("Phrase", Tick::beats(4));
        let pattern_id = pattern.id;
        session.channels.push(channel);
        session.patterns.push(pattern);
        let note = Note {
            id: NoteId::new(),
            start: Tick::ZERO,
            length: Tick(120),
            pitch: 60,
            velocity: 100,
            channel: 0,
        };
        session.patterns[0].lanes.push(PatternLane {
            channel_id,
            steps: Vec::new(),
            notes: vec![note],
        });
        let transformed = Note {
            start: Tick(240),
            length: Tick(240),
            pitch: 64,
            ..note
        };
        let undo = Command::SetPatternNotes {
            pattern_id,
            channel_id,
            notes: vec![transformed],
        }
        .apply(&mut session)
        .expect("transform");
        assert_eq!(session.patterns[0].lanes[0].notes, vec![transformed]);
        undo.apply(&mut session).expect("undo transform");
        assert_eq!(session.patterns[0].lanes[0].notes, vec![note]);
    }

    #[test]
    fn mixer_controls_are_validated_and_reversible() {
        let mut session = Session::new("Mixer");
        let master = session.master_insert().expect("master").id;
        let insert = MixerInsert::insert("Drums", master);
        let insert_id = insert.id;
        session.mixer_inserts.push(insert);

        let undo = Transaction::one(
            "Adjust mixer",
            Command::SetMixerInsertControls {
                mixer_insert_id: insert_id,
                gain_db: -6.0,
                pan: 0.25,
                muted: true,
            },
        )
        .execute(&mut session)
        .expect("adjust mixer");
        assert!((session.mixer_inserts[1].gain_db - -6.0).abs() < f32::EPSILON);
        assert!(session.mixer_inserts[1].muted);

        undo.execute(&mut session).expect("undo mixer");
        assert!(session.mixer_inserts[1].gain_db.abs() < f32::EPSILON);
        assert!(!session.mixer_inserts[1].muted);
    }

    #[test]
    fn mixer_routes_are_reversible() {
        let mut session = Session::new("Routes");
        let master_id = session.master_insert().expect("master").id;
        let insert = MixerInsert::insert("Bus", master_id);
        let insert_id = insert.id;
        session.mixer_inserts.push(insert);
        let route = MixerRoute {
            destination: master_id,
            gain: 0.5,
            pre_fader: true,
            sidechain: false,
        };
        let undo = Command::SetMixerRoutes {
            mixer_insert_id: insert_id,
            routes: vec![route],
        }
        .apply(&mut session)
        .expect("set routes");
        assert_eq!(session.mixer_inserts[1].routes, vec![route]);
        undo.apply(&mut session).expect("undo routes");
        assert!((session.mixer_inserts[1].routes[0].gain - 1.0).abs() < f32::EPSILON);
    }

    #[test]
    fn automation_points_are_reversible() {
        let mut session = Session::new("Automation");
        let lane_id = session.playlist_lanes[0].id;
        let clip_id = ClipId::new();
        session.playlist_lanes[0].clips.push(PlaylistClip {
            id: clip_id,
            name: "Tempo".into(),
            start: Tick::ZERO,
            length: Tick::beats(4),
            color: Rgba::accent(),
            muted: false,
            content: PlaylistClipContent::Automation {
                target: AutomationTarget::Tempo,
                points: Vec::new(),
            },
        });
        let point = AutomationPoint {
            id: AutomationPointId::new(),
            at: Tick::beats(1),
            value: 0.75,
            interpolation: AutomationInterpolation::Smooth,
            tension: 0.0,
        };
        let undo = Command::SetAutomationPoints {
            lane_id,
            clip_id,
            points: vec![point],
        }
        .apply(&mut session)
        .expect("set points");
        let PlaylistClipContent::Automation { points, .. } =
            &session.playlist_lanes[0].clips[0].content
        else {
            panic!("automation clip");
        };
        assert_eq!(points, &vec![point]);
        undo.apply(&mut session).expect("undo points");
        let PlaylistClipContent::Automation { points, .. } =
            &session.playlist_lanes[0].clips[0].content
        else {
            panic!("automation clip");
        };
        assert!(points.is_empty());
    }

    #[test]
    fn playlist_marker_and_clip_production_edits_are_reversible() {
        let mut session = Session::new("Playlist production");
        let clip_id = radiant_domain::ClipId::new();
        session.playlist_lanes[0].clips.push(PlaylistClip {
            id: clip_id,
            name: "Audio".into(),
            start: Tick::ZERO,
            length: Tick::beats(4),
            color: Rgba::accent(),
            muted: false,
            content: PlaylistClipContent::Pattern {
                pattern_id: Pattern::new("P", Tick::beats(4)).id,
            },
        });
        let edit = PlaylistClipEdit {
            clip_id,
            fade_in: Tick(10),
            fade_out: Tick(20),
            crossfade_in: Tick(5),
            crossfade_out: Tick(8),
            stretch_ratio: 1.25,
            frozen: true,
        };
        let undo_edit = Command::SetPlaylistClipEdit { edit }
            .apply(&mut session)
            .expect("edit");
        assert_eq!(session.playlist_clip_edits, vec![edit]);
        undo_edit.apply(&mut session).expect("undo edit");
        assert!(session.playlist_clip_edits.is_empty());
        let marker = PlaylistMarker {
            id: PlaylistMarkerId::new(),
            at: Tick::beats(2),
            name: "Drop".into(),
            color: Rgba::track(),
        };
        let undo_marker = Command::AddPlaylistMarker {
            marker: marker.clone(),
        }
        .apply(&mut session)
        .expect("marker");
        assert_eq!(session.playlist_markers, vec![marker]);
        undo_marker.apply(&mut session).expect("undo marker");
        assert!(session.playlist_markers.is_empty());
    }

    #[test]
    fn comp_selection_is_reversible_and_validated() {
        let mut session = Session::new("Comping");
        let lane_id = session.playlist_lanes[0].id;
        let comp = PlaylistComp {
            id: radiant_domain::CompId::new(),
            lane_id,
            take_clip_ids: vec![ClipId::new(), ClipId::new()],
            active_take: 1,
        };
        let undo = Command::SetPlaylistComp { comp: comp.clone() }
            .apply(&mut session)
            .expect("comp");
        assert_eq!(
            session.playlist_comps[0].active_clip_id(),
            Some(comp.take_clip_ids[1])
        );
        undo.apply(&mut session).expect("undo comp");
        assert!(session.playlist_comps.is_empty());
    }
}
