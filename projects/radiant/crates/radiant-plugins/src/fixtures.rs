use crate::{
    ClapHost, HostError, Lv2Host, MissingPlugin, PluginDescriptor, PluginFormat, PluginRole,
    ProcessContext, Vst3Host,
};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

#[derive(Clone, Debug)]
pub struct ReferenceFixture {
    pub descriptor: PluginDescriptor,
    pub expected_gain: f32,
}

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct LiveFixtureAvailability {
    pub lv2: Vec<PathBuf>,
    pub clap: Vec<PathBuf>,
    pub vst3: Vec<PathBuf>,
}

#[must_use]
pub fn audit_live_fixtures(search_paths: &[PathBuf]) -> LiveFixtureAvailability {
    let mut availability = LiveFixtureAvailability::default();
    for root in search_paths {
        collect_fixture_paths(root, &mut availability);
    }
    availability.lv2.sort();
    availability.clap.sort();
    availability.vst3.sort();
    availability
}

fn collect_fixture_paths(root: &Path, availability: &mut LiveFixtureAvailability) {
    let Ok(entries) = std::fs::read_dir(root) else {
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            collect_fixture_paths(&path, availability);
        }
        match path.extension().and_then(|extension| extension.to_str()) {
            Some(extension)
                if extension.eq_ignore_ascii_case("lv2")
                    && path.is_dir()
                    && contains_shared_object(&path) =>
            {
                availability.lv2.push(path);
            }
            Some(extension) if extension.eq_ignore_ascii_case("clap") => {
                availability.clap.push(path);
            }
            Some(extension) if extension.eq_ignore_ascii_case("vst3") => {
                availability.vst3.push(path);
            }
            _ => {}
        }
    }
}

fn contains_shared_object(root: &Path) -> bool {
    let Ok(entries) = std::fs::read_dir(root) else {
        return false;
    };
    entries.flatten().any(|entry| {
        let path = entry.path();
        (path.is_file()
            && path
                .extension()
                .and_then(|extension| extension.to_str())
                .is_some_and(|extension| extension.eq_ignore_ascii_case("so")))
            || (path.is_dir() && contains_shared_object(&path))
    })
}

#[must_use]
pub fn reference_fixtures() -> Vec<ReferenceFixture> {
    [PluginFormat::Lv2, PluginFormat::Clap, PluginFormat::Vst3]
        .into_iter()
        .flat_map(|format| {
            [PluginRole::Instrument, PluginRole::Effect]
                .into_iter()
                .map(move |role| ReferenceFixture {
                    descriptor: PluginDescriptor {
                        format,
                        role,
                        plugin_id: format!(
                            "org.radiant.reference.{}.{}",
                            format_name(format),
                            role_name(role)
                        ),
                        name: format!("Radiant {} {}", format_name(format), role_name(role)),
                        path: PathBuf::from(format!("reference.{}", extension(format))),
                        audio_inputs: 2,
                        audio_outputs: 2,
                        parameters: 1,
                        supports_state: true,
                        supports_ui: false,
                        latency_frames: 0,
                        tail_frames: 0,
                    },
                    expected_gain: if role == PluginRole::Instrument {
                        0.75
                    } else {
                        0.5
                    },
                })
        })
        .collect()
}

pub fn run_reference_exit_gate(fixture: &ReferenceFixture) -> Result<(), HostError> {
    let mut instance = match fixture.descriptor.format {
        PluginFormat::Lv2 => Lv2Host::instantiate(fixture.descriptor.clone())?,
        PluginFormat::Clap => ClapHost::instantiate(fixture.descriptor.clone())?,
        PluginFormat::Vst3 => Vst3Host::instantiate(fixture.descriptor.clone())?,
    };
    instance.set_parameter("gain", fixture.expected_gain)?;
    let state = instance.save_state();
    let input = [1.0_f32, -0.5, 0.25, -0.125];
    let mut output = [0.0_f32; 4];
    instance.process(
        &[&input],
        &mut [&mut output],
        &ProcessContext {
            sample_rate: 48_000,
            frames: input.len(),
            midi: &[],
        },
    )?;
    let mut reopened = match fixture.descriptor.format {
        PluginFormat::Lv2 => Lv2Host::instantiate(fixture.descriptor.clone())?,
        PluginFormat::Clap => ClapHost::instantiate(fixture.descriptor.clone())?,
        PluginFormat::Vst3 => Vst3Host::instantiate(fixture.descriptor.clone())?,
    };
    reopened.restore_state(state)?;
    if (reopened.save_state().parameters["gain"] - fixture.expected_gain).abs() >= f32::EPSILON {
        return Err(HostError::Unsupported(
            "reference state did not restore".into(),
        ));
    }
    if output
        .iter()
        .zip(input.iter().map(|sample| sample * fixture.expected_gain))
        .any(|(actual, expected)| (actual - expected).abs() >= f32::EPSILON)
    {
        return Err(HostError::Unsupported(
            "offline reference render mismatch".into(),
        ));
    }
    let missing =
        MissingPlugin::from_descriptor(&fixture.descriptor, vec![1, 2, 3], "fixture unavailable");
    let encoded =
        serde_json::to_vec(&missing).map_err(|error| HostError::Unsupported(error.to_string()))?;
    let decoded: MissingPlugin = serde_json::from_slice(&encoded)
        .map_err(|error| HostError::Unsupported(error.to_string()))?;
    if decoded.plugin_id != fixture.descriptor.plugin_id || decoded.opaque_state != vec![1, 2, 3] {
        return Err(HostError::Unsupported(
            "missing-plugin state did not round-trip".into(),
        ));
    }
    Ok(())
}

const fn format_name(format: PluginFormat) -> &'static str {
    match format {
        PluginFormat::Lv2 => "LV2",
        PluginFormat::Clap => "CLAP",
        PluginFormat::Vst3 => "VST3",
    }
}
const fn extension(format: PluginFormat) -> &'static str {
    match format {
        PluginFormat::Lv2 => "lv2",
        PluginFormat::Clap => "clap",
        PluginFormat::Vst3 => "vst3",
    }
}
const fn role_name(role: PluginRole) -> &'static str {
    match role {
        PluginRole::Instrument => "instrument",
        PluginRole::Effect => "effect",
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn every_format_and_role_passes_the_reference_exit_gate() {
        let fixtures = reference_fixtures();
        assert_eq!(fixtures.len(), 6);
        for fixture in &fixtures {
            run_reference_exit_gate(fixture).expect("reference fixture");
        }
    }

    #[test]
    fn live_fixture_audit_classifies_native_extensions() {
        let directory = tempfile::tempdir().expect("directory");
        let lv2 = directory.path().join("demo.lv2");
        std::fs::create_dir(&lv2).expect("lv2");
        std::fs::write(lv2.join("plugin.so"), b"").expect("binary");
        std::fs::write(directory.path().join("demo.clap"), b"").expect("clap");
        std::fs::create_dir(directory.path().join("demo.vst3")).expect("vst3");
        let report = audit_live_fixtures(&[directory.path().to_path_buf()]);
        assert_eq!(report.lv2.len(), 1);
        assert_eq!(report.clap.len(), 1);
        assert_eq!(report.vst3.len(), 1);
    }
}
