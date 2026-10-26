//! Crash-safe staging for captured audio takes.
//!
//! The capture callback feeds a non-real-time writer. Until finalization, a
//! take is a raw f32 stream plus a flushed JSON manifest in `recovery/`; a
//! power loss therefore leaves recoverable material rather than a corrupt WAV.

use std::fs::{self, File, OpenOptions};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};

use radiant_domain::{MediaId, MediaImportMode, MediaSource};
use radiant_media::{AudioBuffer, write_wav};
use serde::{Deserialize, Serialize};

use crate::ProjectError;

#[derive(Debug, Serialize, Deserialize)]
struct TakeManifest {
    id: MediaId,
    sample_rate: u32,
    channels: u16,
    frames: u64,
}

pub struct RecoverableTake {
    root: PathBuf,
    id: MediaId,
    sample_rate: u32,
    channels: u16,
    frames: u64,
    raw_path: PathBuf,
    manifest_path: PathBuf,
}

impl RecoverableTake {
    pub(crate) fn begin(
        root: &Path,
        sample_rate: u32,
        channels: u16,
    ) -> Result<Self, ProjectError> {
        if sample_rate == 0 || channels == 0 {
            return Err(ProjectError::InvalidTakeFormat);
        }
        let id = MediaId::new();
        let raw_path = root.join("recovery").join(format!("{}.f32", id.as_uuid()));
        let manifest_path = root.join("recovery").join(format!("{}.json", id.as_uuid()));
        let take = Self {
            root: root.to_path_buf(),
            id,
            sample_rate,
            channels,
            frames: 0,
            raw_path,
            manifest_path,
        };
        take.flush_manifest()?;
        Ok(take)
    }

    /// Called by the writer thread, never by a real-time callback.
    pub fn append_interleaved(&mut self, samples: &[f32]) -> Result<(), ProjectError> {
        let channels = usize::from(self.channels);
        if !samples.len().is_multiple_of(channels) {
            return Err(ProjectError::InvalidTakeFormat);
        }
        let mut output = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.raw_path)?;
        for sample in samples {
            output.write_all(&sample.to_le_bytes())?;
        }
        output.sync_data()?;
        self.frames = self
            .frames
            .saturating_add(u64::try_from(samples.len() / channels).unwrap_or(u64::MAX));
        self.flush_manifest()
    }

    /// Converts a fully captured staged take to a project-owned WAV, then
    /// removes only the staging files after its final output exists.
    pub fn finalize(self, name: &str) -> Result<MediaSource, ProjectError> {
        let mut bytes = Vec::new();
        File::open(&self.raw_path)?.read_to_end(&mut bytes)?;
        if bytes.len() % 4 != 0 {
            return Err(ProjectError::InvalidTakeFormat);
        }
        let mut channels = vec![Vec::new(); usize::from(self.channels)];
        let channel_count = channels.len();
        for (index, bytes) in bytes.chunks_exact(4).enumerate() {
            let sample = f32::from_le_bytes(
                bytes
                    .try_into()
                    .map_err(|_| ProjectError::InvalidTakeFormat)?,
            );
            channels[index % channel_count].push(sample);
        }
        let audio = AudioBuffer::new(self.sample_rate, channels).map_err(ProjectError::Media)?;
        let safe_name: String = name
            .chars()
            .filter(|character| {
                character.is_ascii_alphanumeric() || *character == '-' || *character == '_'
            })
            .collect();
        let destination = self.root.join("audio").join(format!(
            "{}-{}.wav",
            self.id.as_uuid(),
            if safe_name.is_empty() {
                "take"
            } else {
                &safe_name
            }
        ));
        write_wav(&destination, &audio).map_err(ProjectError::Media)?;
        fs::remove_file(&self.raw_path)?;
        fs::remove_file(&self.manifest_path)?;
        Ok(MediaSource {
            id: self.id,
            path: destination
                .strip_prefix(&self.root)
                .unwrap_or(&destination)
                .to_path_buf(),
            sample_rate: self.sample_rate,
            channels: self.channels,
            frames: self.frames,
            import_mode: MediaImportMode::Copy,
            fingerprint: format!("recorded-{}", self.frames),
            waveform: None,
        })
    }

    fn flush_manifest(&self) -> Result<(), ProjectError> {
        let manifest = TakeManifest {
            id: self.id,
            sample_rate: self.sample_rate,
            channels: self.channels,
            frames: self.frames,
        };
        let temporary = self.manifest_path.with_extension("json.tmp");
        fs::write(&temporary, serde_json::to_vec(&manifest)?)?;
        fs::rename(temporary, &self.manifest_path)?;
        Ok(())
    }
}
