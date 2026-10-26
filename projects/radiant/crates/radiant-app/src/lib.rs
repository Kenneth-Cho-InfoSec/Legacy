//! Application-level project, history, selection, and presentation state.

use std::{collections::HashSet, path::Path};

use radiant_commands::{CommandError, Transaction};
use radiant_domain::{
    ChannelId, ClipId, MixerInsertId, NoteId, PatternId, PlaylistLaneId, Session, ValidationIssue,
};
use radiant_session::{ProjectError, ProjectStore, RecoverableTake};
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct SelectionState {
    pub channel: Option<ChannelId>,
    pub pattern: Option<PatternId>,
    pub playlist_lane: Option<PlaylistLaneId>,
    pub clip: Option<ClipId>,
    pub note: Option<NoteId>,
    pub mixer_insert: Option<MixerInsertId>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct PresentationState {
    pub visible_panels: HashSet<WorkspacePanel>,
    pub primary_divider: i32,
    pub channel_mixer_divider: i32,
    pub picker_playlist_divider: i32,
    pub piano_roll_horizontal_zoom: f64,
    pub piano_roll_vertical_zoom: f64,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub enum WorkspacePanel {
    Browser,
    ChannelRack,
    Picker,
    Playlist,
    Mixer,
}

impl Default for PresentationState {
    fn default() -> Self {
        Self {
            visible_panels: [
                WorkspacePanel::Browser,
                WorkspacePanel::ChannelRack,
                WorkspacePanel::Picker,
                WorkspacePanel::Playlist,
                WorkspacePanel::Mixer,
            ]
            .into_iter()
            .collect(),
            primary_divider: 390,
            channel_mixer_divider: 330,
            picker_playlist_divider: 150,
            piano_roll_horizontal_zoom: 96.0,
            piano_roll_vertical_zoom: 22.0,
        }
    }
}

pub struct AppController {
    session: Session,
    clean_session: Session,
    store: Option<ProjectStore>,
    undo: Vec<Transaction>,
    redo: Vec<Transaction>,
    active_coalesce_key: Option<String>,
    coalesce_revision: u64,
    selection: SelectionState,
    presentation: PresentationState,
}

impl AppController {
    pub fn new(session: Session) -> Result<Self, AppError> {
        let issues = session.validate();
        if !issues.is_empty() {
            return Err(AppError::InvalidSession(issues));
        }
        Ok(Self {
            clean_session: session.clone(),
            session,
            store: None,
            undo: Vec::new(),
            redo: Vec::new(),
            active_coalesce_key: None,
            coalesce_revision: 0,
            selection: SelectionState::default(),
            presentation: PresentationState::default(),
        })
    }

    pub fn open(root: impl AsRef<Path>) -> Result<Self, AppError> {
        let (store, session) = ProjectStore::open(root)?;
        let presentation = store.load_presentation()?.unwrap_or_default();
        Ok(Self {
            clean_session: session.clone(),
            session,
            store: Some(store),
            undo: Vec::new(),
            redo: Vec::new(),
            active_coalesce_key: None,
            coalesce_revision: 0,
            selection: SelectionState::default(),
            presentation,
        })
    }

    #[must_use]
    pub const fn session(&self) -> &Session {
        &self.session
    }

    #[must_use]
    pub const fn selection(&self) -> &SelectionState {
        &self.selection
    }

    pub const fn selection_mut(&mut self) -> &mut SelectionState {
        &mut self.selection
    }

    #[must_use]
    pub const fn presentation(&self) -> &PresentationState {
        &self.presentation
    }

    pub const fn presentation_mut(&mut self) -> &mut PresentationState {
        &mut self.presentation
    }

    #[must_use]
    pub fn is_dirty(&self) -> bool {
        self.session != self.clean_session
    }

    #[must_use]
    pub fn can_undo(&self) -> bool {
        !self.undo.is_empty()
    }

    #[must_use]
    pub fn can_redo(&self) -> bool {
        !self.redo.is_empty()
    }

    #[must_use]
    pub fn project_root(&self) -> Option<&Path> {
        self.store.as_ref().map(ProjectStore::root)
    }

    /// Starts a durable staged take only for an on-disk project.
    pub fn begin_take(&self, sample_rate: u32, channels: u16) -> Result<RecoverableTake, AppError> {
        self.store
            .as_ref()
            .ok_or(AppError::NoProjectStore)?
            .begin_take(sample_rate, channels)
            .map_err(AppError::from)
    }

    pub fn execute(&mut self, transaction: &Transaction) -> Result<(), AppError> {
        self.active_coalesce_key = None;
        let inverse = transaction.execute(&mut self.session)?;
        self.undo.push(inverse);
        self.redo.clear();
        Ok(())
    }

    pub fn execute_coalesced(
        &mut self,
        key: impl Into<String>,
        transaction: &Transaction,
    ) -> Result<u64, AppError> {
        let key = key.into();
        let inverse = transaction.execute(&mut self.session)?;
        if self.active_coalesce_key.as_deref() != Some(&key) {
            self.undo.push(inverse);
        }
        self.active_coalesce_key = Some(key);
        self.coalesce_revision = self.coalesce_revision.wrapping_add(1);
        self.redo.clear();
        Ok(self.coalesce_revision)
    }

    pub fn end_coalescing(&mut self) {
        self.active_coalesce_key = None;
    }

    pub fn end_coalescing_if(&mut self, revision: u64) {
        if self.coalesce_revision == revision {
            self.end_coalescing();
        }
    }

    pub fn undo(&mut self) -> Result<bool, AppError> {
        self.active_coalesce_key = None;
        let Some(transaction) = self.undo.pop() else {
            return Ok(false);
        };
        let redo = transaction.execute(&mut self.session)?;
        self.redo.push(redo);
        Ok(true)
    }

    pub fn redo(&mut self) -> Result<bool, AppError> {
        self.active_coalesce_key = None;
        let Some(transaction) = self.redo.pop() else {
            return Ok(false);
        };
        let undo = transaction.execute(&mut self.session)?;
        self.undo.push(undo);
        Ok(true)
    }

    pub fn create_project(&mut self, root: impl AsRef<Path>) -> Result<(), AppError> {
        let mut store = ProjectStore::create(root, &self.session)?;
        store.save_presentation(&self.presentation)?;
        self.store = Some(store);
        self.clean_session.clone_from(&self.session);
        Ok(())
    }

    pub fn save(&mut self) -> Result<u64, AppError> {
        self.active_coalesce_key = None;
        let store = self.store.as_mut().ok_or(AppError::ProjectNotCreated)?;
        let revision = store.save_with_presentation(&self.session, &self.presentation)?;
        self.clean_session.clone_from(&self.session);
        Ok(revision)
    }

    pub fn persist_presentation(&mut self) -> Result<(), AppError> {
        if let Some(store) = self.store.as_mut() {
            store.save_presentation(&self.presentation)?;
        }
        Ok(())
    }

    pub fn replace_with_new(&mut self, session: Session) -> Result<(), AppError> {
        *self = Self::new(session)?;
        Ok(())
    }

    pub fn replace_with_open(&mut self, root: impl AsRef<Path>) -> Result<(), AppError> {
        *self = Self::open(root)?;
        Ok(())
    }
}

#[derive(Debug, Error)]
pub enum AppError {
    #[error("the project has not been created or opened")]
    ProjectNotCreated,
    #[error("the session is invalid: {0:?}")]
    InvalidSession(Vec<ValidationIssue>),
    #[error(transparent)]
    Command(#[from] CommandError),
    #[error(transparent)]
    Project(#[from] ProjectError),
    #[error("recording requires a saved project")]
    NoProjectStore,
}

#[cfg(test)]
mod tests {
    use radiant_commands::Command;
    use radiant_domain::{
        Channel, ClipId, MixerInsert, Note, NoteId, Pattern, PatternLane, PlaylistClip,
        PlaylistClipContent, Rgba, Step, Tick,
    };
    use tempfile::tempdir;

    use super::*;

    #[test]
    fn execute_undo_and_redo_track_dirty_state() {
        let mut app = AppController::new(Session::new("Original")).expect("controller");
        app.execute(&Transaction::one(
            "Rename",
            Command::RenameSession {
                name: "Changed".into(),
            },
        ))
        .expect("rename");
        assert_eq!(app.session().name, "Changed");
        assert!(app.is_dirty());
        assert!(app.can_undo());
        assert!(!app.can_redo());

        assert!(app.undo().expect("undo"));
        assert_eq!(app.session().name, "Original");
        assert!(!app.is_dirty());
        assert!(app.can_redo());

        assert!(app.redo().expect("redo"));
        assert_eq!(app.session().name, "Changed");
        assert!(app.is_dirty());
    }

    #[test]
    fn failed_transaction_does_not_enter_history() {
        let mut app = AppController::new(Session::new("Valid")).expect("controller");
        let result = app.execute(&Transaction::one(
            "Invalid tempo",
            Command::SetInitialTempo { bpm: -1.0 },
        ));
        assert!(result.is_err());
        assert!(!app.can_undo());
        assert!(!app.is_dirty());
    }

    #[test]
    fn playlist_resize_drag_is_one_history_entry() {
        let mut session = Session::new("Arrangement");
        let pattern = Pattern::new("Beat", Tick::beats(4));
        let lane_id = session.playlist_lanes[0].id;
        let clip_id = ClipId::new();
        session.playlist_lanes[0].clips.push(PlaylistClip {
            id: clip_id,
            name: "Beat".into(),
            start: Tick::ZERO,
            length: Tick::beats(4),
            color: Rgba::accent(),
            muted: false,
            content: PlaylistClipContent::Pattern {
                pattern_id: pattern.id,
            },
        });
        session.patterns.push(pattern);
        let mut app = AppController::new(session).expect("controller");

        app.execute(&Transaction::one(
            "Resize Playlist clip",
            Command::SetPlaylistClipRange {
                lane_id,
                clip_id,
                start: Tick::ZERO,
                length: Tick::beats(8),
            },
        ))
        .expect("resize");
        assert_eq!(
            app.session().playlist_lanes[0].clips[0].length,
            Tick::beats(8)
        );

        assert!(app.undo().expect("undo drag"));
        assert_eq!(
            app.session().playlist_lanes[0].clips[0].length,
            Tick::beats(4)
        );
        assert!(!app.can_undo());
        assert!(app.redo().expect("redo drag"));
        assert_eq!(
            app.session().playlist_lanes[0].clips[0].length,
            Tick::beats(8)
        );
    }

    #[test]
    fn continuous_parameter_updates_coalesce_until_gesture_end() {
        let mut app = AppController::new(Session::new("Coalescing")).expect("controller");
        for bpm in [120.0, 125.0, 130.0, 135.0] {
            app.execute_coalesced(
                "tempo-drag",
                &Transaction::one("Adjust tempo", Command::SetInitialTempo { bpm }),
            )
            .expect("coalesced update");
        }
        app.end_coalescing();
        assert!((app.session().tempo_map[0].bpm - 135.0).abs() < f64::EPSILON);
        assert!(app.undo().expect("undo gesture"));
        assert!((app.session().tempo_map[0].bpm - 140.0).abs() < f64::EPSILON);
        assert!(!app.can_undo());
    }

    #[test]
    fn save_and_reopen_preserve_controller_edits() {
        let directory = tempdir().expect("temporary directory");
        let root = directory.path().join("controller.radiant");
        let mut app = AppController::new(Session::new("Before")).expect("controller");
        app.create_project(&root).expect("create project");
        app.execute(&Transaction::one(
            "Rename",
            Command::RenameSession {
                name: "After".into(),
            },
        ))
        .expect("rename");
        app.presentation_mut().primary_divider = 512;
        app.presentation_mut()
            .visible_panels
            .remove(&WorkspacePanel::Picker);
        assert!(app.is_dirty());
        assert_eq!(app.save().expect("save"), 1);
        assert!(!app.is_dirty());

        let reopened = AppController::open(&root).expect("reopen");
        assert_eq!(reopened.session().name, "After");
        assert_eq!(reopened.presentation().primary_divider, 512);
        assert!(
            !reopened
                .presentation()
                .visible_panels
                .contains(&WorkspacePanel::Picker)
        );
        assert!(!reopened.is_dirty());
    }

    #[test]
    #[allow(clippy::too_many_lines)]
    fn milestone_one_workflow_round_trips_every_edit() {
        let directory = tempdir().expect("temporary directory");
        let root = directory.path().join("milestone-one.radiant");
        let mut app = AppController::new(Session::new("Milestone One")).expect("controller");
        app.create_project(&root).expect("create project");
        let original = app.session().clone();

        let master_id = app.session().master_insert().expect("master").id;
        let insert = MixerInsert::insert("Synth", master_id);
        let insert_id = insert.id;
        let channel = Channel::sampler("Synth", insert_id);
        let channel_id = channel.id;
        let pattern = Pattern {
            id: radiant_domain::PatternId::new(),
            name: "Melody".into(),
            length: Tick::beats(4),
            lanes: vec![PatternLane {
                channel_id,
                steps: Vec::new(),
                notes: Vec::new(),
            }],
        };
        let pattern_id = pattern.id;
        let note_id = NoteId::new();
        let lane_id = app.session().playlist_lanes[0].id;
        let clip_id = ClipId::new();
        let edits = vec![
            Transaction {
                label: "Create Channel and Pattern".into(),
                commands: vec![
                    Command::AddMixerInsert { insert },
                    Command::AddChannel { channel },
                    Command::AddPattern { pattern },
                ],
            },
            Transaction::one(
                "Paint step",
                Command::SetPatternStep {
                    pattern_id,
                    channel_id,
                    at: Tick::ZERO,
                    step: Some(Step {
                        at: Tick::ZERO,
                        velocity: 0.8,
                        probability: 1.0,
                        micro_shift: Tick::ZERO,
                        ratchets: 1,
                    }),
                },
            ),
            Transaction::one(
                "Add note",
                Command::SetPatternNote {
                    pattern_id,
                    channel_id,
                    note_id,
                    note: Some(Note {
                        id: note_id,
                        start: Tick::ZERO,
                        length: Tick::beats(1),
                        pitch: 60,
                        velocity: 100,
                        channel: 0,
                    }),
                },
            ),
            Transaction::one(
                "Resize note",
                Command::SetPatternNote {
                    pattern_id,
                    channel_id,
                    note_id,
                    note: Some(Note {
                        id: note_id,
                        start: Tick::ZERO,
                        length: Tick::beats(2),
                        pitch: 60,
                        velocity: 100,
                        channel: 0,
                    }),
                },
            ),
            Transaction::one(
                "Place Pattern Clip",
                Command::AddPlaylistClip {
                    lane_id,
                    clip: PlaylistClip {
                        id: clip_id,
                        name: "Melody".into(),
                        start: Tick::ZERO,
                        length: Tick::beats(4),
                        color: Rgba::accent(),
                        muted: false,
                        content: PlaylistClipContent::Pattern { pattern_id },
                    },
                },
            ),
            Transaction::one(
                "Move Pattern Clip",
                Command::SetPlaylistClipRange {
                    lane_id,
                    clip_id,
                    start: Tick::beats(4),
                    length: Tick::beats(4),
                },
            ),
            Transaction::one(
                "Resize Pattern Clip",
                Command::SetPlaylistClipRange {
                    lane_id,
                    clip_id,
                    start: Tick::beats(4),
                    length: Tick::beats(8),
                },
            ),
            Transaction::one(
                "Change Mixer fader",
                Command::SetMixerInsertControls {
                    mixer_insert_id: insert_id,
                    gain_db: -6.0,
                    pan: 0.0,
                    muted: false,
                },
            ),
        ];
        for edit in &edits {
            app.execute(edit).expect("execute representative edit");
        }
        let completed = app.session().clone();

        for _ in &edits {
            assert!(app.undo().expect("undo representative edit"));
        }
        assert_eq!(app.session(), &original);
        for _ in &edits {
            assert!(app.redo().expect("redo representative edit"));
        }
        assert_eq!(app.session(), &completed);
        app.save().expect("save completed workflow");
        let reopened = AppController::open(root).expect("reopen completed workflow");
        assert_eq!(reopened.session(), &completed);
    }
}
