use std::collections::HashMap;
use std::path::Path;
use std::sync::Arc;

use radiant_domain::{
    AutomationInterpolation, AutomationTarget, ChannelId, MediaId, MixerInsertId, Pattern,
    PlaylistClip, PlaylistClipContent, Session, Tick,
};
use radiant_graph::CompiledGraph;
use radiant_media::AudioBuffer;

use crate::{EngineError, MeterPoint, MusicalTimeline};

const RELEASE_FRAMES: u64 = 480;
const STEP_TICKS: i64 = 240;

#[derive(Clone, Debug, Default)]
pub struct MediaPool {
    buffers: HashMap<MediaId, Arc<AudioBuffer>>,
}

impl MediaPool {
    pub fn insert(&mut self, id: MediaId, buffer: AudioBuffer) {
        self.buffers.insert(id, Arc::new(buffer));
    }

    #[must_use]
    pub fn get(&self, id: MediaId) -> Option<&Arc<AudioBuffer>> {
        self.buffers.get(&id)
    }

    pub fn load_session(session: &Session, project_root: &Path) -> Result<Self, EngineError> {
        let mut pool = Self::default();
        for source in &session.media {
            let path = if source.path.is_absolute() {
                source.path.clone()
            } else {
                project_root.join(&source.path)
            };
            pool.insert(source.id, radiant_media::decode(path)?);
        }
        Ok(pool)
    }
}

#[derive(Clone, Debug)]
pub struct RenderPlan {
    pub(crate) sample_rate: u32,
    pub(crate) timeline: MusicalTimeline,
    pub(crate) length_frames: u64,
    pub(crate) master_index: usize,
    pub(crate) graph: CompiledGraph,
    pub(crate) inserts: Vec<CompiledInsert>,
    pub(crate) voices: Vec<Voice>,
    pub(crate) audio: Vec<AudioEvent>,
    pub(crate) automation: Vec<AutomationEvent>,
}

impl RenderPlan {
    #[allow(clippy::too_many_lines)]
    pub fn compile(session: &Session, media: &MediaPool) -> Result<Arc<Self>, EngineError> {
        let issues = session.validate();
        if !issues.is_empty() {
            return Err(EngineError::InvalidSession(issues));
        }
        let timeline = MusicalTimeline::new(
            session.sample_rate,
            &session.tempo_map,
            &[MeterPoint {
                at: Tick::ZERO,
                signature: session.time_signature,
            }],
        )?;
        let insert_indices: HashMap<_, _> = session
            .mixer_inserts
            .iter()
            .enumerate()
            .map(|(index, insert)| (insert.id, index))
            .collect();
        let master_index = session
            .mixer_inserts
            .iter()
            .position(|insert| insert.is_master)
            .ok_or(EngineError::MissingMaster)?;
        let solo_active = session
            .mixer_inserts
            .iter()
            .any(|insert| !insert.is_master && insert.solo);
        let mut solo_path = vec![false; session.mixer_inserts.len()];
        if solo_active {
            let mut changed = true;
            for (index, insert) in session.mixer_inserts.iter().enumerate() {
                solo_path[index] = insert.solo;
            }
            while changed {
                changed = false;
                for (source, insert) in session.mixer_inserts.iter().enumerate() {
                    if !solo_path[source] {
                        continue;
                    }
                    for route in &insert.routes {
                        if let Some(destination) = insert_indices.get(&route.destination).copied()
                            && !solo_path[destination]
                        {
                            solo_path[destination] = true;
                            changed = true;
                        }
                    }
                }
            }
        }
        let inserts: Vec<_> = session
            .mixer_inserts
            .iter()
            .enumerate()
            .map(|(index, insert)| {
                let routes = insert
                    .routes
                    .iter()
                    .map(|route| {
                        Ok(CompiledRoute {
                            destination: *insert_indices
                                .get(&route.destination)
                                .ok_or(EngineError::MissingMixerInsert(route.destination))?,
                            gain: route.gain,
                            pre_fader: route.pre_fader,
                            sidechain: route.sidechain,
                        })
                    })
                    .collect::<Result<_, EngineError>>()?;
                Ok(CompiledInsert {
                    id: insert.id,
                    gain: db_to_gain(insert.gain_db),
                    pan: insert.pan,
                    muted: insert.muted || (solo_active && !solo_path[index] && !insert.is_master),
                    latency_frames: insert
                        .plugins
                        .iter()
                        .map(|plugin| plugin.latency_frames)
                        .sum(),
                    compensation_delay: 0,
                    routes,
                })
            })
            .collect::<Result<_, EngineError>>()?;
        let edges: Vec<_> = inserts
            .iter()
            .enumerate()
            .flat_map(|(source, insert)| {
                insert
                    .routes
                    .iter()
                    .map(move |route| (source, route.destination))
            })
            .collect();
        let latencies: Vec<_> = inserts.iter().map(|insert| insert.latency_frames).collect();
        let graph = CompiledGraph::compile(inserts.len(), &edges, &latencies)?;
        let compensation: Vec<_> = (0..inserts.len())
            .map(|index| graph.compensation_delay(index).unwrap_or(0))
            .collect();
        let inserts = inserts
            .into_iter()
            .enumerate()
            .map(|(index, mut insert)| {
                insert.compensation_delay = compensation[index];
                insert
            })
            .collect();

        let mut plan = Self {
            sample_rate: session.sample_rate,
            timeline,
            length_frames: 0,
            master_index,
            graph,
            inserts,
            voices: Vec::new(),
            audio: Vec::new(),
            automation: Vec::new(),
        };
        compile_playlist(session, media, &insert_indices, &mut plan)?;
        plan.length_frames = plan
            .length_frames
            .max(plan.timeline.tick_to_frame(Tick::beats(4)));
        Ok(Arc::new(plan))
    }

    /// Per-insert delay needed to align every route with the longest graph
    /// path. The order matches the session's mixer insert order.
    #[must_use]
    pub fn compensation_delays(&self) -> Vec<u32> {
        self.inserts
            .iter()
            .map(|insert| insert.compensation_delay)
            .collect()
    }

    #[must_use]
    pub const fn sample_rate(&self) -> u32 {
        self.sample_rate
    }

    #[must_use]
    pub const fn length_frames(&self) -> u64 {
        self.length_frames
    }

    #[must_use]
    pub const fn timeline(&self) -> &MusicalTimeline {
        &self.timeline
    }

    #[must_use]
    pub fn graph_order(&self) -> &[usize] {
        self.graph.order()
    }
}

#[allow(clippy::too_many_lines)]
fn compile_playlist(
    session: &Session,
    media: &MediaPool,
    insert_indices: &HashMap<MixerInsertId, usize>,
    plan: &mut RenderPlan,
) -> Result<(), EngineError> {
    let patterns: HashMap<_, _> = session
        .patterns
        .iter()
        .map(|pattern| (pattern.id, pattern))
        .collect();
    let channels: HashMap<_, _> = session
        .channels
        .iter()
        .map(|channel| (channel.id, channel))
        .collect();
    for lane in &session.playlist_lanes {
        if lane.muted {
            continue;
        }
        for clip in &lane.clips {
            if clip.muted {
                continue;
            }
            if session.playlist_comps.iter().any(|comp| {
                comp.lane_id == lane.id
                    && comp.take_clip_ids.contains(&clip.id)
                    && comp.active_clip_id() != Some(clip.id)
            }) {
                continue;
            }
            let clip_end = Tick(clip.start.0.saturating_add(clip.length.0));
            plan.length_frames = plan
                .length_frames
                .max(plan.timeline.tick_to_frame(clip_end));
            match &clip.content {
                PlaylistClipContent::Pattern { pattern_id } => {
                    let pattern = patterns
                        .get(pattern_id)
                        .ok_or(EngineError::MissingPattern(*pattern_id))?;
                    compile_pattern_clip(clip, pattern, &channels, insert_indices, plan)?;
                }
                PlaylistClipContent::Audio {
                    media_id,
                    source_start_frame,
                    gain_db,
                } => {
                    let source = media
                        .get(*media_id)
                        .ok_or(EngineError::MissingMedia(*media_id))?;
                    let original_rate = source.sample_rate();
                    let source = Arc::new(source.resample_linear(session.sample_rate));
                    let source_start =
                        scale_frame(*source_start_frame, original_rate, session.sample_rate);
                    let mixer = lane
                        .association
                        .and_then(|association| {
                            insert_indices.get(&association.mixer_insert_id).copied()
                        })
                        .unwrap_or(plan.master_index);
                    let edit = session
                        .playlist_clip_edits
                        .iter()
                        .find(|edit| edit.clip_id == clip.id);
                    let stretch_ratio = edit.map_or(1.0, |edit| {
                        if edit.stretch_ratio.is_finite() && edit.stretch_ratio > 0.0 {
                            edit.stretch_ratio
                        } else {
                            1.0
                        }
                    });
                    plan.audio.push(AudioEvent {
                        start: plan.timeline.tick_to_frame(clip.start),
                        end: plan.timeline.tick_to_frame(clip_end),
                        source_start,
                        gain: db_to_gain(*gain_db),
                        fade_in: edit.map_or(0, |edit| plan.timeline.tick_to_frame(edit.fade_in)),
                        fade_out: edit.map_or(0, |edit| plan.timeline.tick_to_frame(edit.fade_out)),
                        crossfade_in: edit
                            .map_or(0, |edit| plan.timeline.tick_to_frame(edit.crossfade_in)),
                        crossfade_out: edit
                            .map_or(0, |edit| plan.timeline.tick_to_frame(edit.crossfade_out)),
                        stretch_ratio,
                        mixer,
                        buffer: source,
                    });
                }
                PlaylistClipContent::Automation { target, points } => {
                    let mut compiled_points: Vec<_> = points
                        .iter()
                        .map(|point| AutomationPoint {
                            frame: plan
                                .timeline
                                .tick_to_frame(Tick(clip.start.0.saturating_add(point.at.0))),
                            value: point.value,
                            interpolation: point.interpolation,
                        })
                        .collect();
                    compiled_points.sort_by_key(|point| point.frame);
                    plan.automation.push(AutomationEvent {
                        start: plan.timeline.tick_to_frame(clip.start),
                        end: plan.timeline.tick_to_frame(clip_end),
                        target: target.clone(),
                        points: compiled_points,
                    });
                }
            }
        }
    }
    Ok(())
}

fn compile_pattern_clip(
    clip: &PlaylistClip,
    pattern: &Pattern,
    channels: &HashMap<ChannelId, &radiant_domain::Channel>,
    insert_indices: &HashMap<MixerInsertId, usize>,
    plan: &mut RenderPlan,
) -> Result<(), EngineError> {
    let clip_end = Tick(clip.start.0.saturating_add(clip.length.0));
    let mut repeat = Tick::ZERO;
    let mut repeat_index = 0_u64;
    while repeat < clip.length {
        for (lane_index, lane) in pattern.lanes.iter().enumerate() {
            let channel = channels
                .get(&lane.channel_id)
                .ok_or(EngineError::MissingChannel(lane.channel_id))?;
            if channel.muted {
                continue;
            }
            let mixer = *insert_indices
                .get(&channel.mixer_insert)
                .ok_or(EngineError::MissingMixerInsert(channel.mixer_insert))?;
            for note in &lane.notes {
                let start_tick = Tick(
                    clip.start
                        .0
                        .saturating_add(repeat.0)
                        .saturating_add(note.start.0),
                );
                if start_tick >= clip_end {
                    continue;
                }
                let end_tick = Tick(start_tick.0.saturating_add(note.length.0).min(clip_end.0));
                push_voice(
                    plan,
                    lane.channel_id,
                    mixer,
                    note.pitch,
                    f32::from(note.velocity) / 127.0 * channel.volume,
                    channel.pan,
                    start_tick,
                    end_tick,
                    channel_timbre(&channel.name),
                );
            }
            for (step_index, step) in lane.steps.iter().enumerate() {
                if !step_fires(step.probability, lane_index, step_index, repeat_index) {
                    continue;
                }
                for ratchet in 0..step.ratchets {
                    let spacing = STEP_TICKS / i64::from(step.ratchets);
                    let start_tick = Tick(
                        clip.start
                            .0
                            .saturating_add(repeat.0)
                            .saturating_add(step.at.0)
                            .saturating_add(step.micro_shift.0)
                            .saturating_add(i64::from(ratchet) * spacing),
                    );
                    if start_tick < clip.start || start_tick >= clip_end {
                        continue;
                    }
                    let end_tick =
                        Tick(start_tick.0.saturating_add(spacing.max(1)).min(clip_end.0));
                    let pitch = 36_u8.saturating_add(u8::try_from(lane_index % 24).unwrap_or(0));
                    push_voice(
                        plan,
                        lane.channel_id,
                        mixer,
                        pitch,
                        step.velocity * channel.volume,
                        channel.pan,
                        start_tick,
                        end_tick,
                        channel_timbre(&channel.name),
                    );
                }
            }
        }
        repeat = Tick(repeat.0.saturating_add(pattern.length.0));
        repeat_index = repeat_index.saturating_add(1);
    }
    Ok(())
}

fn step_fires(probability: f32, lane: usize, step: usize, repeat: u64) -> bool {
    if probability <= 0.0 {
        return false;
    }
    if probability >= 1.0 {
        return true;
    }
    let mut hash = u64::try_from(lane)
        .unwrap_or(u64::MAX)
        .wrapping_add(0x9E37_79B9);
    hash ^= u64::try_from(step)
        .unwrap_or(u64::MAX)
        .wrapping_mul(0x85EB_CA6B);
    hash ^= repeat.wrapping_mul(0xC2B2_AE35);
    hash ^= hash >> 16;
    hash = hash.wrapping_mul(0x7FEB_352D);
    hash ^= hash >> 15;
    #[allow(clippy::cast_possible_truncation, clippy::cast_precision_loss)]
    let normalized = (hash & 0x00FF_FFFF) as f32 / 16_777_215.0;
    normalized < probability
}

#[allow(clippy::too_many_arguments)]
fn push_voice(
    plan: &mut RenderPlan,
    channel_id: ChannelId,
    mixer: usize,
    pitch: u8,
    velocity: f32,
    stereo_position: f32,
    start_tick: Tick,
    end_tick: Tick,
    timbre: u8,
) {
    let start = plan.timeline.tick_to_frame(start_tick);
    let end = plan.timeline.tick_to_frame(end_tick).max(start + 1);
    let release_end = end.saturating_add(RELEASE_FRAMES);
    plan.length_frames = plan.length_frames.max(release_end);
    plan.voices.push(Voice {
        start,
        end,
        release_end,
        frequency: midi_frequency(pitch),
        velocity,
        pan: stereo_position,
        mixer,
        channel_id,
        timbre,
    });
}

fn channel_timbre(name: &str) -> u8 {
    let name = name.to_ascii_lowercase();
    if name.contains("kick") {
        1
    } else if name.contains("clap") || name.contains("snare") {
        2
    } else if name.contains("hat") || name.contains("perc") {
        3
    } else if name.contains("bass") {
        4
    } else if name.contains("lead") {
        5
    } else if name.contains("pluck") {
        6
    } else if name.contains("pad") {
        7
    } else if name.contains("riser") {
        8
    } else if name.contains("impact") {
        9
    } else {
        0
    }
}

fn scale_frame(frame: i64, source_rate: u32, target_rate: u32) -> usize {
    if frame <= 0 {
        return 0;
    }
    let value = u128::from(u64::try_from(frame).unwrap_or(u64::MAX))
        .saturating_mul(u128::from(target_rate))
        / u128::from(source_rate);
    usize::try_from(value).unwrap_or(usize::MAX)
}

fn midi_frequency(pitch: u8) -> f64 {
    440.0 * 2.0_f64.powf((f64::from(pitch) - 69.0) / 12.0)
}

pub(crate) fn db_to_gain(db: f32) -> f32 {
    10.0_f32.powf(db / 20.0)
}

#[derive(Clone, Debug)]
pub(crate) struct CompiledInsert {
    pub id: MixerInsertId,
    pub gain: f32,
    pub pan: f32,
    pub muted: bool,
    pub latency_frames: u32,
    pub compensation_delay: u32,
    pub routes: Vec<CompiledRoute>,
}

#[derive(Clone, Copy, Debug)]
pub(crate) struct CompiledRoute {
    pub destination: usize,
    pub gain: f32,
    pub pre_fader: bool,
    pub sidechain: bool,
}

#[derive(Clone, Copy, Debug)]
pub(crate) struct Voice {
    pub start: u64,
    pub end: u64,
    pub release_end: u64,
    pub frequency: f64,
    pub velocity: f32,
    pub pan: f32,
    pub mixer: usize,
    pub channel_id: ChannelId,
    pub timbre: u8,
}

#[derive(Clone, Debug)]
pub(crate) struct AudioEvent {
    pub start: u64,
    pub end: u64,
    pub source_start: usize,
    pub gain: f32,
    pub fade_in: u64,
    pub fade_out: u64,
    pub crossfade_in: u64,
    pub crossfade_out: u64,
    pub stretch_ratio: f64,
    pub mixer: usize,
    pub buffer: Arc<AudioBuffer>,
}

#[derive(Clone, Debug)]
pub(crate) struct AutomationEvent {
    pub start: u64,
    pub end: u64,
    pub target: AutomationTarget,
    pub points: Vec<AutomationPoint>,
}

#[derive(Clone, Copy, Debug)]
pub(crate) struct AutomationPoint {
    pub frame: u64,
    pub value: f64,
    pub interpolation: AutomationInterpolation,
}
