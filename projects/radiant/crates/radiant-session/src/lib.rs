//! Schema-versioned `SQLite` persistence for Radiant projects.

mod crash;
mod recording;
pub use crash::{CrashReport, write_opt_in_report};
pub use recording::RecoverableTake;

use std::fs;
use std::path::{Path, PathBuf};

use radiant_commands::{CommandError, Transaction};
use radiant_domain::{
    CURRENT_SCHEMA_VERSION, MediaId, MediaImportMode, MediaSource, Session, ValidationIssue,
    WaveformCache,
};
use radiant_media::{WaveformPeaks, decode};
use rusqlite::{Connection, OptionalExtension, params};
use serde::{Serialize, de::DeserializeOwned};
use thiserror::Error;

const DATABASE_NAME: &str = "project.sqlite";
const AUTOSAVE_NAME: &str = "autosave.json";

fn media_fingerprint(path: &Path) -> Result<String, std::io::Error> {
    let metadata = fs::metadata(path)?;
    let modified = metadata
        .modified()
        .ok()
        .and_then(|time| time.duration_since(std::time::UNIX_EPOCH).ok())
        .map_or(0, |duration| duration.as_nanos());
    Ok(format!("{:x}-{:x}", metadata.len(), modified))
}

pub struct ProjectStore {
    root: PathBuf,
    connection: Connection,
}

impl ProjectStore {
    pub fn create(root: impl AsRef<Path>, session: &Session) -> Result<Self, ProjectError> {
        validate_session(session)?;
        let root = root.as_ref().to_path_buf();
        fs::create_dir_all(root.join("audio"))?;
        fs::create_dir_all(root.join("renders"))?;
        fs::create_dir_all(root.join("freeze"))?;
        fs::create_dir_all(root.join("cache/waveforms"))?;
        fs::create_dir_all(root.join("cache/plugins"))?;
        fs::create_dir_all(root.join("recovery"))?;

        let database = root.join(DATABASE_NAME);
        if database.exists() {
            return Err(ProjectError::AlreadyExists(database));
        }
        let connection = Connection::open(&database)?;
        let mut store = Self { root, connection };
        store.initialize()?;
        store.save_snapshot(session)?;
        Ok(store)
    }

    pub fn open(root: impl AsRef<Path>) -> Result<(Self, Session), ProjectError> {
        let root = root.as_ref().to_path_buf();
        let database = root.join(DATABASE_NAME);
        if !database.is_file() {
            return Err(ProjectError::NotFound(database));
        }
        let connection = Connection::open(database)?;
        let mut store = Self { root, connection };
        store.configure()?;
        store.migrate_schema()?;
        let session = store.load()?;
        Ok((store, session))
    }

    #[must_use]
    pub fn root(&self) -> &Path {
        &self.root
    }

    /// Begins a staged recording. The caller feeds it from a writer thread and
    /// finalizes it to a project-owned source once capture stops.
    pub fn begin_take(
        &self,
        sample_rate: u32,
        channels: u16,
    ) -> Result<RecoverableTake, ProjectError> {
        RecoverableTake::begin(&self.root, sample_rate, channels)
    }

    /// Adds a source without modifying it.  `Copy` imports are stored in the
    /// project-owned `audio/` directory; `Link` keeps an absolute external path.
    /// A compact peak cache is generated separately from the original media.
    pub fn import_media(
        &self,
        source: impl AsRef<Path>,
        mode: MediaImportMode,
    ) -> Result<MediaSource, ProjectError> {
        let source = source.as_ref();
        let input = decode(source).map_err(ProjectError::Media)?;
        let id = MediaId::new();
        let path = match mode {
            MediaImportMode::Copy => {
                let filename = source.file_name().ok_or(ProjectError::InvalidMediaPath)?;
                let extension = source
                    .extension()
                    .and_then(|item| item.to_str())
                    .unwrap_or("wav");
                let destination = self.root.join("audio").join(format!(
                    "{}-{}.{}",
                    id.as_uuid(),
                    source
                        .file_stem()
                        .and_then(|item| item.to_str())
                        .unwrap_or("media"),
                    extension
                ));
                fs::copy(source, &destination)?;
                let _ = filename;
                destination
                    .strip_prefix(&self.root)
                    .unwrap_or(&destination)
                    .to_path_buf()
            }
            MediaImportMode::Link => fs::canonicalize(source)?,
        };
        let fingerprint = media_fingerprint(source)?;
        let peaks = WaveformPeaks::from_audio(&input, 2_048);
        let peak_path = self
            .root
            .join("cache/waveforms")
            .join(format!("{}.json", id.as_uuid()));
        fs::write(&peak_path, serde_json::to_vec(&peaks)?)?;
        Ok(MediaSource {
            id,
            path,
            sample_rate: input.sample_rate(),
            channels: u16::try_from(input.channel_count())
                .map_err(|_| ProjectError::InvalidMediaPath)?,
            frames: u64::try_from(input.frames()).map_err(|_| ProjectError::InvalidMediaPath)?,
            import_mode: mode,
            fingerprint: fingerprint.clone(),
            waveform: Some(WaveformCache {
                path: peak_path
                    .strip_prefix(&self.root)
                    .unwrap_or(&peak_path)
                    .to_path_buf(),
                fingerprint,
                buckets: peaks.buckets,
            }),
        })
    }

    /// Repoints a linked asset after it has moved, regenerating metadata and
    /// peaks. Project-owned copies deliberately cannot be relinked.
    pub fn relink_media(
        &self,
        media: &MediaSource,
        source: impl AsRef<Path>,
    ) -> Result<MediaSource, ProjectError> {
        if media.import_mode != MediaImportMode::Link {
            return Err(ProjectError::CannotRelinkOwnedMedia);
        }
        let source = fs::canonicalize(source)?;
        let input = decode(&source).map_err(ProjectError::Media)?;
        let fingerprint = media_fingerprint(&source)?;
        let peaks = WaveformPeaks::from_audio(&input, 2_048);
        let peak_path = self
            .root
            .join("cache/waveforms")
            .join(format!("{}.json", media.id.as_uuid()));
        fs::write(&peak_path, serde_json::to_vec(&peaks)?)?;
        Ok(MediaSource {
            path: source,
            sample_rate: input.sample_rate(),
            channels: u16::try_from(input.channel_count())
                .map_err(|_| ProjectError::InvalidMediaPath)?,
            frames: u64::try_from(input.frames()).map_err(|_| ProjectError::InvalidMediaPath)?,
            fingerprint: fingerprint.clone(),
            waveform: Some(WaveformCache {
                path: peak_path
                    .strip_prefix(&self.root)
                    .unwrap_or(&peak_path)
                    .to_path_buf(),
                fingerprint,
                buckets: peaks.buckets,
            }),
            ..media.clone()
        })
    }

    pub fn load(&self) -> Result<Session, ProjectError> {
        let json: Option<String> = self
            .connection
            .query_row(
                "SELECT json FROM session_state WHERE singleton = 1",
                [],
                |row| row.get(0),
            )
            .optional()?;
        let json = json.ok_or(ProjectError::MissingSessionState)?;
        let session: Session = serde_json::from_str(&json)?;
        validate_session(&session)?;
        Ok(session)
    }

    /// Executes and durably records a command transaction before publishing it
    /// to the caller's in-memory session.
    pub fn apply(
        &mut self,
        session: &mut Session,
        transaction: &Transaction,
    ) -> Result<Transaction, ProjectError> {
        let mut candidate = session.clone();
        let inverse = transaction.execute(&mut candidate)?;
        let session_json = serde_json::to_string(&candidate)?;
        let command_json = serde_json::to_string(transaction)?;
        let database = self.connection.transaction()?;
        let revision: i64 = database.query_row(
            "SELECT revision FROM session_state WHERE singleton = 1",
            [],
            |row| row.get(0),
        )?;
        let next_revision = revision.saturating_add(1);
        database.execute(
            "UPDATE session_state SET revision = ?1, json = ?2 WHERE singleton = 1",
            params![next_revision, session_json],
        )?;
        database.execute(
            "INSERT INTO command_log (revision, label, json) VALUES (?1, ?2, ?3)",
            params![next_revision, transaction.label, command_json],
        )?;
        database.commit()?;
        *session = candidate;
        Ok(inverse)
    }

    pub fn revision(&self) -> Result<u64, ProjectError> {
        let revision: i64 = self.connection.query_row(
            "SELECT revision FROM session_state WHERE singleton = 1",
            [],
            |row| row.get(0),
        )?;
        u64::try_from(revision).map_err(|_| ProjectError::InvalidRevision(revision))
    }

    /// Replays every committed transaction onto a caller-provided base session.
    /// The candidate is only published after all log entries validate.
    pub fn replay_command_log(&self, base: &Session) -> Result<(Session, usize), ProjectError> {
        let mut candidate = base.clone();
        let mut statement = self
            .connection
            .prepare("SELECT json FROM command_log ORDER BY revision ASC")?;
        let entries = statement.query_map([], |row| row.get::<_, String>(0))?;
        let mut count = 0;
        for entry in entries {
            let json = entry?;
            let transaction: Transaction = serde_json::from_str(&json)?;
            transaction.execute(&mut candidate)?;
            count += 1;
        }
        validate_session(&candidate)?;
        Ok((candidate, count))
    }

    /// Replaces the durable project snapshot at an explicit user save point.
    pub fn save(&mut self, session: &Session) -> Result<u64, ProjectError> {
        self.save_internal(session, None)
    }

    /// Writes a crash-recoverable session snapshot using an atomic rename.
    pub fn autosave(&self, session: &Session) -> Result<(), ProjectError> {
        validate_session(session)?;
        let temporary = self.root.join("recovery").join("autosave.json.tmp");
        let target = self.root.join("recovery").join(AUTOSAVE_NAME);
        fs::write(&temporary, serde_json::to_vec(session)?)?;
        fs::rename(temporary, target)?;
        Ok(())
    }

    /// Loads the last autosave candidate, if one exists, without modifying the
    /// durable `SQLite` project snapshot.
    pub fn load_autosave(&self) -> Result<Option<Session>, ProjectError> {
        let path = self.root.join("recovery").join(AUTOSAVE_NAME);
        if !path.is_file() {
            return Ok(None);
        }
        let session: Session = serde_json::from_slice(&fs::read(path)?)?;
        validate_session(&session)?;
        Ok(Some(session))
    }

    /// Creates a consistent portable `SQLite` checkpoint copy.
    pub fn checkpoint(&self, destination: impl AsRef<Path>) -> Result<(), ProjectError> {
        let destination = destination.as_ref();
        if let Some(parent) = destination.parent() {
            fs::create_dir_all(parent)?;
        }
        let temporary = destination.with_extension("tmp");
        let _ = fs::remove_file(&temporary);
        self.connection.backup("main", &temporary, None)?;
        fs::rename(temporary, destination)?;
        Ok(())
    }

    /// Copies the project into a self-contained portable directory, including
    /// owned media and durable state but excluding disposable caches.
    pub fn package_portable(&self, destination: impl AsRef<Path>) -> Result<(), ProjectError> {
        let destination = destination.as_ref();
        if destination.starts_with(&self.root) {
            return Err(ProjectError::InvalidPackageDestination);
        }
        fs::create_dir_all(destination)?;
        copy_project_tree(
            &self.root,
            destination,
            &std::collections::HashSet::from(["cache", "recovery"]),
        )?;
        Ok(())
    }

    pub fn save_with_presentation<T: Serialize>(
        &mut self,
        session: &Session,
        presentation: &T,
    ) -> Result<u64, ProjectError> {
        let json = serde_json::to_string(presentation)?;
        self.save_internal(session, Some(&json))
    }

    pub fn save_presentation<T: Serialize>(
        &mut self,
        presentation: &T,
    ) -> Result<(), ProjectError> {
        let json = serde_json::to_string(presentation)?;
        self.connection.execute(
            "INSERT INTO presentation_state (singleton, json) VALUES (1, ?1)
             ON CONFLICT(singleton) DO UPDATE SET json = excluded.json",
            [json],
        )?;
        Ok(())
    }

    pub fn load_presentation<T: DeserializeOwned>(&self) -> Result<Option<T>, ProjectError> {
        let json: Option<String> = self
            .connection
            .query_row(
                "SELECT json FROM presentation_state WHERE singleton = 1",
                [],
                |row| row.get(0),
            )
            .optional()?;
        json.map(|json| serde_json::from_str(&json).map_err(ProjectError::from))
            .transpose()
    }

    fn save_internal(
        &mut self,
        session: &Session,
        presentation_json: Option<&str>,
    ) -> Result<u64, ProjectError> {
        validate_session(session)?;
        let session_json = serde_json::to_string(session)?;
        let database = self.connection.transaction()?;
        let revision: i64 = database.query_row(
            "SELECT revision FROM session_state WHERE singleton = 1",
            [],
            |row| row.get(0),
        )?;
        let next_revision = revision.saturating_add(1);
        database.execute(
            "UPDATE session_state SET revision = ?1, json = ?2 WHERE singleton = 1",
            params![next_revision, session_json],
        )?;
        if let Some(presentation_json) = presentation_json {
            database.execute(
                "INSERT INTO presentation_state (singleton, json) VALUES (1, ?1)
                 ON CONFLICT(singleton) DO UPDATE SET json = excluded.json",
                [presentation_json],
            )?;
        }
        database.commit()?;
        u64::try_from(next_revision).map_err(|_| ProjectError::InvalidRevision(next_revision))
    }

    fn initialize(&mut self) -> Result<(), ProjectError> {
        self.configure()?;
        self.connection.execute_batch(
            "BEGIN IMMEDIATE;
             CREATE TABLE metadata (
                 key TEXT PRIMARY KEY NOT NULL,
                 value TEXT NOT NULL
             );
             CREATE TABLE session_state (
                 singleton INTEGER PRIMARY KEY CHECK (singleton = 1),
                 revision INTEGER NOT NULL,
                 json TEXT NOT NULL
             );
             CREATE TABLE command_log (
                 revision INTEGER PRIMARY KEY,
                 label TEXT NOT NULL,
                 json TEXT NOT NULL,
                 committed_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
             );
             CREATE TABLE presentation_state (
                 singleton INTEGER PRIMARY KEY CHECK (singleton = 1),
                 json TEXT NOT NULL
             );
             INSERT INTO metadata (key, value) VALUES ('schema_version', '3');
             COMMIT;",
        )?;
        Ok(())
    }

    fn configure(&mut self) -> Result<(), ProjectError> {
        self.connection.execute_batch(
            "PRAGMA journal_mode = WAL;
             PRAGMA synchronous = FULL;
             PRAGMA foreign_keys = ON;
             PRAGMA busy_timeout = 5000;",
        )?;
        Ok(())
    }

    fn migrate_schema(&mut self) -> Result<(), ProjectError> {
        let raw: Option<String> = self
            .connection
            .query_row(
                "SELECT value FROM metadata WHERE key = 'schema_version'",
                [],
                |row| row.get(0),
            )
            .optional()?;
        let found = raw
            .ok_or(ProjectError::MissingSchema)?
            .parse::<u32>()
            .map_err(|_| ProjectError::MissingSchema)?;
        if found > CURRENT_SCHEMA_VERSION || found == 0 {
            return Err(ProjectError::UnsupportedSchema {
                found,
                supported: CURRENT_SCHEMA_VERSION,
            });
        }
        if found == 1 {
            let session_json: String = self.connection.query_row(
                "SELECT json FROM session_state WHERE singleton = 1",
                [],
                |row| row.get(0),
            )?;
            let mut session: serde_json::Value = serde_json::from_str(&session_json)?;
            let Some(object) = session.as_object_mut() else {
                return Err(ProjectError::MissingSessionState);
            };
            object.insert("schema_version".into(), serde_json::Value::from(2));
            let migrated_json = serde_json::to_string(&session)?;
            let migration = self.connection.transaction()?;
            migration.execute_batch(
                "CREATE TABLE presentation_state (
                     singleton INTEGER PRIMARY KEY CHECK (singleton = 1),
                     json TEXT NOT NULL
                 );",
            )?;
            migration.execute(
                "UPDATE session_state SET json = ?1 WHERE singleton = 1",
                [migrated_json],
            )?;
            migration.execute(
                "UPDATE metadata SET value = '2' WHERE key = 'schema_version'",
                [],
            )?;
            migration.commit()?;
        }
        if found <= 2 {
            let session_json: String = self.connection.query_row(
                "SELECT json FROM session_state WHERE singleton = 1",
                [],
                |row| row.get(0),
            )?;
            let mut session: serde_json::Value = serde_json::from_str(&session_json)?;
            let Some(object) = session.as_object_mut() else {
                return Err(ProjectError::MissingSessionState);
            };
            object.insert(
                "schema_version".into(),
                serde_json::Value::from(CURRENT_SCHEMA_VERSION),
            );
            if let Some(media) = object
                .get_mut("media")
                .and_then(serde_json::Value::as_array_mut)
            {
                for item in media {
                    if let Some(item) = item.as_object_mut() {
                        item.entry("import_mode")
                            .or_insert(serde_json::Value::from("copy"));
                        item.entry("fingerprint")
                            .or_insert(serde_json::Value::from(""));
                        item.entry("waveform").or_insert(serde_json::Value::Null);
                    }
                }
            }
            let migration = self.connection.transaction()?;
            migration.execute(
                "UPDATE session_state SET json = ?1 WHERE singleton = 1",
                [serde_json::to_string(&session)?],
            )?;
            migration.execute(
                "UPDATE metadata SET value = '3' WHERE key = 'schema_version'",
                [],
            )?;
            migration.commit()?;
        }
        Ok(())
    }

    fn save_snapshot(&mut self, session: &Session) -> Result<(), ProjectError> {
        let json = serde_json::to_string(session)?;
        self.connection.execute(
            "INSERT INTO session_state (singleton, revision, json) VALUES (1, 0, ?1)",
            params![json],
        )?;
        Ok(())
    }
}

fn validate_session(session: &Session) -> Result<(), ProjectError> {
    let issues = session.validate();
    if issues.is_empty() {
        Ok(())
    } else {
        Err(ProjectError::InvalidSession(issues))
    }
}

fn copy_project_tree(
    source: &Path,
    destination: &Path,
    excluded: &std::collections::HashSet<&str>,
) -> Result<(), ProjectError> {
    for entry in fs::read_dir(source)? {
        let entry = entry?;
        let name = entry.file_name();
        if name.to_str().is_some_and(|name| excluded.contains(name)) {
            continue;
        }
        let target = destination.join(&name);
        if entry.file_type()?.is_dir() {
            fs::create_dir_all(&target)?;
            copy_project_tree(&entry.path(), &target, excluded)?;
        } else {
            fs::copy(entry.path(), target)?;
        }
    }
    Ok(())
}

#[derive(Debug, Error)]
pub enum ProjectError {
    #[error("a project already exists at {0}")]
    AlreadyExists(PathBuf),
    #[error("no Radiant project exists at {0}")]
    NotFound(PathBuf),
    #[error("project schema metadata is missing")]
    MissingSchema,
    #[error("project state is missing")]
    MissingSessionState,
    #[error("project schema {found} is unsupported; this build supports {supported}")]
    UnsupportedSchema { found: u32, supported: u32 },
    #[error("invalid project revision {0}")]
    InvalidRevision(i64),
    #[error("the session is invalid: {0:?}")]
    InvalidSession(Vec<ValidationIssue>),
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Database(#[from] rusqlite::Error),
    #[error(transparent)]
    Json(#[from] serde_json::Error),
    #[error(transparent)]
    Command(#[from] CommandError),
    #[error("media path has no usable dimensions")]
    InvalidMediaPath,
    #[error("only linked media can be relinked")]
    CannotRelinkOwnedMedia,
    #[error(transparent)]
    Media(#[from] radiant_media::MediaError),
    #[error("recording take has an invalid sample layout")]
    InvalidTakeFormat,
    #[error("portable package destination must be outside the project")]
    InvalidPackageDestination,
}

#[cfg(test)]
mod tests {
    use radiant_commands::Command;
    use radiant_domain::MediaImportMode;
    use radiant_media::{AudioBuffer, write_wav};
    use tempfile::tempdir;

    use super::*;

    #[test]
    fn project_round_trip_preserves_session() {
        let directory = tempdir().expect("temporary directory");
        let root = directory.path().join("song.radiant");
        let original = Session::new("Round trip");
        ProjectStore::create(&root, &original).expect("create project");
        let (_store, loaded) = ProjectStore::open(&root).expect("open project");
        assert_eq!(loaded, original);
    }

    #[test]
    fn command_and_project_commit_share_one_boundary() {
        let directory = tempdir().expect("temporary directory");
        let root = directory.path().join("song.radiant");
        let mut session = Session::new("Before");
        let mut store = ProjectStore::create(&root, &session).expect("create project");
        let transaction = Transaction::one(
            "Rename project",
            Command::RenameSession {
                name: "After".into(),
            },
        );
        store
            .apply(&mut session, &transaction)
            .expect("commit rename");

        assert_eq!(session.name, "After");
        assert_eq!(store.revision().expect("revision"), 1);
        let (_reopened, loaded) = ProjectStore::open(&root).expect("reopen project");
        assert_eq!(loaded.name, "After");
        let (replayed, count) = store
            .replay_command_log(&Session::new("Before"))
            .expect("replay log");
        assert_eq!(count, 1);
        assert_eq!(replayed.name, "After");
    }

    #[test]
    fn schema_one_project_migrates_through_current_schema() {
        let directory = tempdir().expect("temporary directory");
        let root = directory.path().join("legacy.radiant");
        let session = Session::new("Legacy");
        let store = ProjectStore::create(&root, &session).expect("create project");
        let json: String = store
            .connection
            .query_row(
                "SELECT json FROM session_state WHERE singleton = 1",
                [],
                |row| row.get(0),
            )
            .expect("session json");
        let legacy_json = json.replace("\"schema_version\":2", "\"schema_version\":1");
        store
            .connection
            .execute("UPDATE session_state SET json = ?1", [legacy_json])
            .expect("downgrade session");
        store
            .connection
            .execute("UPDATE metadata SET value = '1'", [])
            .expect("downgrade metadata");
        store
            .connection
            .execute("DROP TABLE presentation_state", [])
            .expect("remove v2 table");
        drop(store);

        let (migrated, loaded) = ProjectStore::open(&root).expect("migrate project");
        assert_eq!(loaded.schema_version, CURRENT_SCHEMA_VERSION);
        let version: String = migrated
            .connection
            .query_row(
                "SELECT value FROM metadata WHERE key = 'schema_version'",
                [],
                |row| row.get(0),
            )
            .expect("schema version");
        assert_eq!(version, CURRENT_SCHEMA_VERSION.to_string());
    }

    #[test]
    fn future_schema_fails_without_modification() {
        let directory = tempdir().expect("temporary directory");
        let root = directory.path().join("future.radiant");
        let session = Session::new("Future");
        let store = ProjectStore::create(&root, &session).expect("create project");
        store
            .connection
            .execute("UPDATE metadata SET value = '99'", [])
            .expect("set future schema");
        drop(store);

        assert!(matches!(
            ProjectStore::open(&root),
            Err(ProjectError::UnsupportedSchema {
                found: 99,
                supported: CURRENT_SCHEMA_VERSION
            })
        ));
        let connection = Connection::open(root.join(DATABASE_NAME)).expect("reopen database");
        let version: String = connection
            .query_row(
                "SELECT value FROM metadata WHERE key = 'schema_version'",
                [],
                |row| row.get(0),
            )
            .expect("future version");
        assert_eq!(version, "99");
    }

    #[test]
    fn malformed_session_fails_without_rewriting_it() {
        let directory = tempdir().expect("temporary directory");
        let root = directory.path().join("malformed.radiant");
        let session = Session::new("Malformed");
        let store = ProjectStore::create(&root, &session).expect("create project");
        store
            .connection
            .execute(
                "UPDATE session_state SET json = 'not valid json' WHERE singleton = 1",
                [],
            )
            .expect("corrupt session");
        drop(store);

        assert!(matches!(
            ProjectStore::open(&root),
            Err(ProjectError::Json(_))
        ));
        let connection = Connection::open(root.join(DATABASE_NAME)).expect("reopen database");
        let json: String = connection
            .query_row(
                "SELECT json FROM session_state WHERE singleton = 1",
                [],
                |row| row.get(0),
            )
            .expect("malformed json");
        assert_eq!(json, "not valid json");
    }

    #[test]
    fn copied_media_is_portable_and_linked_media_can_be_relinked() {
        let directory = tempdir().expect("temporary directory");
        let source = directory.path().join("source.wav");
        write_wav(
            &source,
            &AudioBuffer::new(48_000, vec![vec![0.0, 0.5], vec![0.0, -0.5]]).expect("audio"),
        )
        .expect("write source");
        let root = directory.path().join("song.radiant");
        let store = ProjectStore::create(&root, &Session::new("Media")).expect("project");

        let copied = store
            .import_media(&source, MediaImportMode::Copy)
            .expect("copy import");
        assert!(root.join(&copied.path).is_file());
        assert!(
            copied
                .waveform
                .as_ref()
                .is_some_and(|cache| root.join(&cache.path).is_file())
        );

        let linked = store
            .import_media(&source, MediaImportMode::Link)
            .expect("link import");
        let moved = directory.path().join("moved.wav");
        fs::rename(&source, &moved).expect("move source");
        let relinked = store.relink_media(&linked, &moved).expect("relink");
        assert_eq!(
            relinked.path,
            fs::canonicalize(&moved).expect("canonical path")
        );
        assert!(matches!(
            store.relink_media(&copied, &moved),
            Err(ProjectError::CannotRelinkOwnedMedia)
        ));
    }

    #[test]
    fn staged_take_survives_before_finalization_and_becomes_owned_wav() {
        let directory = tempdir().expect("temporary directory");
        let root = directory.path().join("recording.radiant");
        let store = ProjectStore::create(&root, &Session::new("Recording")).expect("project");
        let mut take = store.begin_take(48_000, 2).expect("begin take");
        take.append_interleaved(&[0.0, 0.0, 0.5, -0.5])
            .expect("append");
        assert!(
            fs::read_dir(root.join("recovery"))
                .expect("recovery directory")
                .next()
                .is_some()
        );
        let media = take.finalize("Guitar take").expect("finalize");
        assert!(root.join(&media.path).is_file());
        assert!(
            fs::read_dir(root.join("recovery"))
                .expect("recovery directory")
                .next()
                .is_none()
        );
    }

    #[test]
    fn autosave_and_checkpoint_survive_forced_shutdown_boundary() {
        let directory = tempdir().expect("temporary directory");
        let root = directory.path().join("recovery.radiant");
        let mut session = Session::new("Before crash");
        let store = ProjectStore::create(&root, &session).expect("project");
        session.name = "Autosaved edit".into();
        store.autosave(&session).expect("autosave");
        let recovered = store
            .load_autosave()
            .expect("load autosave")
            .expect("candidate");
        assert_eq!(recovered.name, "Autosaved edit");
        let checkpoint = directory.path().join("checkpoint.sqlite");
        store.checkpoint(&checkpoint).expect("checkpoint");
        assert!(checkpoint.is_file());
        let (reopened, durable) = ProjectStore::open(&root).expect("reopen");
        assert_eq!(durable.name, "Before crash");
        assert_eq!(reopened.revision().expect("revision"), 0);
    }

    #[test]
    fn portable_package_contains_durable_project_state_without_caches() {
        let directory = tempdir().expect("temporary directory");
        let root = directory.path().join("source.radiant");
        let store = ProjectStore::create(&root, &Session::new("Portable")).expect("project");
        fs::write(root.join("cache/waveforms/temporary"), b"cache").expect("cache");
        let destination = directory.path().join("portable.radiant");
        store.package_portable(&destination).expect("package");
        assert!(destination.join("project.sqlite").is_file());
        assert!(!destination.join("cache").exists());
    }

    #[test]
    fn malformed_session_inputs_fail_without_panicking_or_mutating_state() {
        let baseline = Session::new("Safe");
        let samples = [
            b"".as_slice(),
            b"{".as_slice(),
            b"[]".as_slice(),
            br#"{"schema_version":"bad"}"#.as_slice(),
            br#"{"channels":[null]}"#.as_slice(),
        ];
        for sample in samples {
            assert!(serde_json::from_slice::<Session>(sample).is_err());
        }
        assert_eq!(baseline.name, "Safe");
    }
}
