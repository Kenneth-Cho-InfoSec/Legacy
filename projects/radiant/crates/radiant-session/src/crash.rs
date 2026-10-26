//! Opt-in, privacy-preserving crash diagnostics.

use std::fs;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct CrashReport {
    pub app_version: String,
    pub platform: String,
    pub backend: String,
    pub message: String,
}

/// Writes a report only when the caller has explicit opt-in enabled.
pub fn write_opt_in_report(
    root: impl AsRef<Path>,
    enabled: bool,
    report: &CrashReport,
) -> Result<Option<PathBuf>, std::io::Error> {
    if !enabled {
        return Ok(None);
    }
    let directory = root.as_ref().join("recovery/crash-reports");
    fs::create_dir_all(&directory)?;
    let path = directory.join(format!("report-{}.json", uuid::Uuid::new_v4()));
    fs::write(
        &path,
        serde_json::to_vec_pretty(report).expect("report serialization"),
    )?;
    Ok(Some(path))
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn crash_reporting_is_explicitly_opt_in() {
        let directory = tempdir().expect("temporary");
        let report = CrashReport {
            app_version: "0.1".into(),
            platform: "linux".into(),
            backend: "PipeWire".into(),
            message: "test".into(),
        };
        assert!(
            write_opt_in_report(directory.path(), false, &report)
                .expect("disabled")
                .is_none()
        );
        let path = write_opt_in_report(directory.path(), true, &report)
            .expect("enabled")
            .expect("path");
        assert!(path.is_file());
        assert_eq!(
            serde_json::from_slice::<CrashReport>(&fs::read(path).expect("report")).expect("json"),
            report
        );
    }
}
