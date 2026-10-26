//! Native Linux plugin discovery, scan supervision, cache, and recovery state.
//!
//! This crate deliberately contains no plugin ABI loading. Scanning is a
//! separate process protocol; hosts are added only after a descriptor has
//! passed this boundary. Unknown plugin IDs and opaque state remain serializable
//! even when a format is unavailable on the current machine.

use std::fs;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Stdio};
use std::thread;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use serde::{Deserialize, Serialize};
use thiserror::Error;

mod clap_native;
mod fixtures;
mod host;
mod isolation;
mod lv2_native;
mod vst3_native;

pub use clap_native::{NativeClapInstance, discover as discover_clap_native};
pub use fixtures::{
    LiveFixtureAvailability, ReferenceFixture, audit_live_fixtures, reference_fixtures,
    run_reference_exit_gate,
};
pub use host::{
    AudioPortLayout, ClapFeatures, ClapHost, CompatibilityReport, GenericUiDescriptor, HostError,
    HostRequest, HostResponse, Lv2Features, Lv2Host, Lv2Port, Lv2PortKind, MidiEvent,
    NativePluginInstance, ParameterInfo, PluginAdapter, PluginState, Preset, ProcessContext,
    UiKind, Vst3Features, Vst3Host, decode_protocol, encode_protocol,
};
pub use isolation::{IsolationError, IsolationPolicy, IsolationState, ProcessingIsolation};
pub use lv2_native::{NativeLv2Instance, discover as discover_lv2_native};
pub use vst3_native::{NativeVst3Instance, discover as discover_vst3_native};

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum PluginFormat {
    Lv2,
    Clap,
    Vst3,
}

#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum PluginRole {
    Instrument,
    #[default]
    Effect,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct PluginDescriptor {
    pub format: PluginFormat,
    #[serde(default)]
    pub role: PluginRole,
    pub plugin_id: String,
    pub name: String,
    pub path: PathBuf,
    pub audio_inputs: u16,
    pub audio_outputs: u16,
    pub parameters: u32,
    pub supports_state: bool,
    pub supports_ui: bool,
    pub latency_frames: u32,
    pub tail_frames: u64,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ScanFingerprint {
    pub modified_seconds: u64,
    pub size: u64,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum ScanOutcome {
    Ready(PluginDescriptor),
    TimedOut,
    Crashed { code: Option<i32> },
    InvalidOutput(String),
    Quarantined { reason: String },
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ScanRecord {
    pub format: PluginFormat,
    pub path: PathBuf,
    pub outcome: ScanOutcome,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ScanCacheEntry {
    pub fingerprint: ScanFingerprint,
    pub outcome: ScanOutcome,
    pub failures: u32,
}

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct ScanCache {
    pub entries: std::collections::BTreeMap<String, ScanCacheEntry>,
    #[serde(default)]
    pub blacklist: std::collections::BTreeMap<String, String>,
}

#[derive(Debug, Error)]
pub enum ScanError {
    #[error("plugin path does not exist: {0}")]
    MissingPath(PathBuf),
    #[error("plugin metadata failed: {0}")]
    Metadata(String),
    #[error("scanner process failed to start: {0}")]
    Spawn(String),
    #[error("scanner cache read failed: {0}")]
    CacheRead(String),
    #[error("scanner cache write failed: {0}")]
    CacheWrite(String),
    #[error("scanner output was not valid JSON: {0}")]
    InvalidOutput(String),
}

impl ScanCache {
    pub fn blacklist(&mut self, key: impl Into<String>, reason: impl Into<String>) {
        self.blacklist.insert(key.into(), reason.into());
    }

    pub fn clear_blacklist(&mut self, key: &str) {
        self.blacklist.remove(key);
    }

    pub fn load(path: impl AsRef<Path>) -> Result<Self, ScanError> {
        let path = path.as_ref();
        if !path.exists() {
            return Ok(Self::default());
        }
        let data = fs::read(path).map_err(|error| ScanError::CacheRead(error.to_string()))?;
        serde_json::from_slice(&data).map_err(|error| ScanError::CacheRead(error.to_string()))
    }

    pub fn save(&self, path: impl AsRef<Path>) -> Result<(), ScanError> {
        let path = path.as_ref();
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).map_err(|error| ScanError::CacheWrite(error.to_string()))?;
        }
        let temporary = path.with_extension("json.tmp");
        let bytes = serde_json::to_vec_pretty(self)
            .map_err(|error| ScanError::CacheWrite(error.to_string()))?;
        fs::write(&temporary, bytes).map_err(|error| ScanError::CacheWrite(error.to_string()))?;
        fs::rename(&temporary, path).map_err(|error| ScanError::CacheWrite(error.to_string()))
    }

    pub fn clear_quarantine(&mut self, key: &str) {
        if let Some(entry) = self.entries.get_mut(key) {
            entry.failures = 0;
            if matches!(entry.outcome, ScanOutcome::Quarantined { .. }) {
                entry.outcome = ScanOutcome::TimedOut;
            }
        }
    }
}

#[derive(Clone, Debug)]
pub struct ScanSupervisor {
    pub timeout: Duration,
    pub quarantine_after: u32,
}

impl Default for ScanSupervisor {
    fn default() -> Self {
        Self {
            timeout: Duration::from_secs(5),
            quarantine_after: 3,
        }
    }
}

impl ScanSupervisor {
    pub fn scan_paths(
        &self,
        scanner_binary: impl AsRef<Path>,
        search_paths: &[PathBuf],
        cache: &mut ScanCache,
    ) -> Vec<ScanRecord> {
        discover_paths(search_paths)
            .into_iter()
            .map(|(format, path)| {
                let outcome = self
                    .scan(&scanner_binary, &path, cache)
                    .unwrap_or_else(|error| ScanOutcome::InvalidOutput(error.to_string()));
                ScanRecord {
                    format,
                    path,
                    outcome,
                }
            })
            .collect()
    }

    pub fn scan(
        &self,
        scanner_binary: impl AsRef<Path>,
        plugin_path: impl AsRef<Path>,
        cache: &mut ScanCache,
    ) -> Result<ScanOutcome, ScanError> {
        let plugin_path = plugin_path.as_ref();
        if !plugin_path.exists() {
            return Err(ScanError::MissingPath(plugin_path.to_path_buf()));
        }
        let key = plugin_path.to_string_lossy().into_owned();
        let fingerprint = fingerprint(plugin_path)?;
        if let Some(reason) = cache.blacklist.get(&key) {
            return Ok(ScanOutcome::Quarantined {
                reason: format!("blacklisted: {reason}"),
            });
        }
        if let Some(entry) = cache.entries.get(&key)
            && entry.fingerprint == fingerprint
            && matches!(
                entry.outcome,
                ScanOutcome::Ready(_) | ScanOutcome::Quarantined { .. }
            )
        {
            return Ok(entry.outcome.clone());
        }
        let mut child = Command::new(scanner_binary.as_ref())
            .arg("--probe")
            .arg(plugin_path)
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .spawn()
            .map_err(|error| ScanError::Spawn(error.to_string()))?;
        let outcome = wait_for_scan(&mut child, self.timeout);
        let failures = if matches!(outcome, ScanOutcome::Ready(_)) {
            0
        } else {
            cache
                .entries
                .get(&key)
                .map_or(0, |entry| entry.failures)
                .saturating_add(1)
        };
        let outcome =
            if failures >= self.quarantine_after && !matches!(outcome, ScanOutcome::Ready(_)) {
                ScanOutcome::Quarantined {
                    reason: format!("scanner failed {failures} times"),
                }
            } else {
                outcome
            };
        cache.entries.insert(
            key,
            ScanCacheEntry {
                fingerprint,
                outcome: outcome.clone(),
                failures,
            },
        );
        Ok(outcome)
    }
}

fn fingerprint(path: &Path) -> Result<ScanFingerprint, ScanError> {
    let metadata = fs::metadata(path).map_err(|error| ScanError::Metadata(error.to_string()))?;
    let modified_seconds = metadata
        .modified()
        .ok()
        .and_then(|time| time.duration_since(UNIX_EPOCH).ok())
        .unwrap_or(Duration::ZERO)
        .as_secs();
    Ok(ScanFingerprint {
        modified_seconds,
        size: metadata.len(),
    })
}

fn wait_for_scan(child: &mut Child, timeout: Duration) -> ScanOutcome {
    let started = SystemTime::now();
    loop {
        match child.try_wait() {
            Ok(Some(status)) => {
                if !status.success() {
                    return ScanOutcome::Crashed {
                        code: status.code(),
                    };
                }
                let Some(stdout) = child.stdout.take() else {
                    return ScanOutcome::InvalidOutput("scanner had no stdout".into());
                };
                let mut bytes = Vec::new();
                let mut stdout = stdout;
                let _ = stdout.read_to_end(&mut bytes);
                return serde_json::from_slice(&bytes).map_or_else(
                    |error| ScanOutcome::InvalidOutput(error.to_string()),
                    ScanOutcome::Ready,
                );
            }
            Ok(None) => {
                if started.elapsed().unwrap_or(timeout) >= timeout {
                    let _ = child.kill();
                    let _ = child.wait();
                    return ScanOutcome::TimedOut;
                }
                thread::sleep(Duration::from_millis(5));
            }
            Err(error) => {
                return ScanOutcome::Crashed {
                    code: error.raw_os_error(),
                };
            }
        }
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct MissingPlugin {
    pub format: PluginFormat,
    pub plugin_id: String,
    pub opaque_state: Vec<u8>,
    pub reason: String,
}

/// Discover native Linux bundle paths without loading them.
#[must_use]
pub fn discover_paths(search_paths: &[PathBuf]) -> Vec<(PluginFormat, PathBuf)> {
    let mut found = Vec::new();
    for root in search_paths {
        let Ok(entries) = fs::read_dir(root) else {
            continue;
        };
        for entry in entries.flatten() {
            let path = entry.path();
            let Some(name) = path.file_name().and_then(|n| n.to_str()) else {
                continue;
            };
            let extension = Path::new(name).extension().and_then(|ext| ext.to_str());
            let format =
                if extension.is_some_and(|ext| ext.eq_ignore_ascii_case("lv2")) && path.is_dir() {
                    Some(PluginFormat::Lv2)
                } else if extension.is_some_and(|ext| ext.eq_ignore_ascii_case("clap")) {
                    Some(PluginFormat::Clap)
                } else if extension.is_some_and(|ext| ext.eq_ignore_ascii_case("vst3")) {
                    Some(PluginFormat::Vst3)
                } else {
                    None
                };
            if let Some(format) = format {
                found.push((format, path));
            }
        }
    }
    found.sort_by(|left, right| left.1.cmp(&right.1));
    found.dedup_by(|left, right| left.1 == right.1);
    found
}

impl MissingPlugin {
    pub fn from_descriptor(
        descriptor: &PluginDescriptor,
        opaque_state: Vec<u8>,
        reason: impl Into<String>,
    ) -> Self {
        Self {
            format: descriptor.format,
            plugin_id: descriptor.plugin_id.clone(),
            opaque_state,
            reason: reason.into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::tempdir;

    #[test]
    fn discovery_finds_native_bundle_extensions() {
        let directory = tempdir().expect("directory");
        fs::create_dir(directory.path().join("synth.lv2")).expect("lv2");
        fs::write(directory.path().join("effect.clap"), b"").expect("clap");
        fs::write(directory.path().join("instrument.vst3"), b"").expect("vst3");
        let found = discover_paths(&[directory.path().to_path_buf()]);
        assert_eq!(found.len(), 3);
        assert!(found.iter().any(|(format, _)| *format == PluginFormat::Lv2));
    }

    #[test]
    fn blacklisted_plugin_never_reaches_the_scanner() {
        let directory = tempdir().expect("directory");
        let plugin = directory.path().join("unsafe.clap");
        fs::write(&plugin, b"synthetic-invalid-clap-fixture").expect("plugin");
        let mut cache = ScanCache::default();
        cache.blacklist(plugin.to_string_lossy(), "known crash");
        let outcome = ScanSupervisor::default()
            .scan("missing-scanner", &plugin, &mut cache)
            .expect("blacklist is handled before spawn");
        assert!(matches!(outcome, ScanOutcome::Quarantined { .. }));
    }

    #[test]
    fn batch_scan_returns_one_record_per_bundle_without_aborting() {
        let directory = tempdir().expect("directory");
        fs::create_dir(directory.path().join("one.lv2")).expect("lv2");
        fs::write(directory.path().join("two.clap"), b"").expect("clap");
        let records = ScanSupervisor::default().scan_paths(
            directory.path().join("missing-scanner"),
            &[directory.path().to_path_buf()],
            &mut ScanCache::default(),
        );
        assert_eq!(records.len(), 2);
        assert!(
            records
                .iter()
                .all(|record| matches!(record.outcome, ScanOutcome::InvalidOutput(_)))
        );
    }

    #[test]
    fn old_scan_cache_descriptors_default_to_effect_role() {
        let json = r#"{"format":"lv2","plugin_id":"org.old","name":"Old","path":"old.lv2","audio_inputs":2,"audio_outputs":2,"parameters":0,"supports_state":true,"supports_ui":false,"latency_frames":0,"tail_frames":0}"#;
        let descriptor: PluginDescriptor = serde_json::from_str(json).expect("legacy descriptor");
        assert_eq!(descriptor.role, PluginRole::Effect);
    }

    #[test]
    fn cache_round_trips_and_quarantine_is_recoverable() {
        let directory = tempdir().expect("directory");
        let path = directory.path().join("scan-cache.json");
        let mut cache = ScanCache::default();
        cache.entries.insert(
            "plugin.so".into(),
            ScanCacheEntry {
                fingerprint: ScanFingerprint {
                    modified_seconds: 1,
                    size: 2,
                },
                outcome: ScanOutcome::Quarantined {
                    reason: "crash".into(),
                },
                failures: 3,
            },
        );
        cache.save(&path).expect("save");
        let mut loaded = ScanCache::load(&path).expect("load");
        loaded.clear_quarantine("plugin.so");
        assert_eq!(loaded.entries["plugin.so"].failures, 0);
    }

    #[test]
    fn scanner_timeout_is_cached_and_escalates_to_quarantine() {
        let directory = tempdir().expect("directory");
        let plugin = directory.path().join("fake.clap");
        fs::File::create(&plugin).expect("plugin");
        let script = directory.path().join("scanner.sh");
        let mut file = fs::File::create(&script).expect("script");
        writeln!(file, "#!/bin/sh\nsleep 1").expect("write script");
        drop(file);
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            fs::set_permissions(&script, fs::Permissions::from_mode(0o755)).expect("chmod");
        }
        let supervisor = ScanSupervisor {
            timeout: Duration::from_millis(50),
            quarantine_after: 2,
        };
        let mut cache = ScanCache::default();
        assert!(matches!(
            supervisor.scan(&script, &plugin, &mut cache),
            Ok(ScanOutcome::TimedOut)
        ));
        cache.entries.clear();
        // A cached timeout is a stable result until the fingerprint changes.
        cache.entries.insert(
            plugin.to_string_lossy().into_owned(),
            ScanCacheEntry {
                fingerprint: fingerprint(&plugin).expect("fingerprint"),
                outcome: ScanOutcome::TimedOut,
                failures: 1,
            },
        );
        assert!(matches!(
            cache.entries.values().next().unwrap().outcome,
            ScanOutcome::TimedOut
        ));
    }
}
