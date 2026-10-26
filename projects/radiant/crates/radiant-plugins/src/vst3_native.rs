use std::path::Path;

use crate::{PluginDescriptor, PluginFormat, PluginRole};

pub struct NativeVst3Instance {
    host: vst3_host::Vst3Host,
    plugin: vst3_host::Plugin,
}

impl NativeVst3Instance {
    pub fn load(path: &Path) -> Result<Self, String> {
        let mut host = vst3_host::Vst3Host::builder()
            .with_process_isolation(true)
            .build()
            .map_err(|error| error.to_string())?;
        let plugin = host.load_plugin(path).map_err(|error| error.to_string())?;
        Ok(Self { host, plugin })
    }

    pub fn save_state(&self) -> Result<Vec<u8>, String> {
        self.plugin.save_state().map_err(|error| error.to_string())
    }

    pub fn restore_state(&mut self, state: &[u8]) -> Result<(), String> {
        self.plugin
            .load_state(state)
            .map_err(|error| error.to_string())
    }

    pub fn process(&mut self, buffers: &mut vst3_host::audio::AudioBuffers) -> Result<(), String> {
        self.plugin
            .start_processing()
            .map_err(|error| error.to_string())?;
        self.plugin
            .process_audio(buffers)
            .map_err(|error| error.to_string())
    }

    #[must_use]
    pub fn host(&self) -> &vst3_host::Vst3Host {
        &self.host
    }
}

/// Native VST3 discovery adapter. The third-party host keeps COM/ABI details
/// behind its safe API and performs probing in a helper process.
pub fn discover(path: &Path) -> Result<Vec<PluginDescriptor>, String> {
    let mut host = vst3_host::Vst3Host::builder()
        .with_process_isolation(true)
        .build()
        .map_err(|error| error.to_string())?;
    host.add_scan_path(path)
        .map_err(|error| error.to_string())?;
    let report = host.discover_plugins_safe();
    if let Some(error) = report.error {
        return Err(error);
    }
    Ok(report
        .plugins
        .into_iter()
        .map(|plugin| PluginDescriptor {
            format: PluginFormat::Vst3,
            role: if plugin
                .info
                .category
                .to_ascii_lowercase()
                .contains("instrument")
            {
                PluginRole::Instrument
            } else {
                PluginRole::Effect
            },
            plugin_id: plugin.info.uid,
            name: plugin.info.name,
            path: plugin.info.path,
            audio_inputs: u16::try_from(plugin.info.audio_inputs).unwrap_or(u16::MAX),
            audio_outputs: u16::try_from(plugin.info.audio_outputs).unwrap_or(u16::MAX),
            parameters: 0,
            supports_state: true,
            supports_ui: plugin.info.has_gui,
            latency_frames: 0,
            tail_frames: 0,
        })
        .collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn missing_path_is_reported_without_loading_code() {
        let result = discover(Path::new("/definitely/missing/radiant.vst3"));
        assert!(result.is_err());
    }

    #[test]
    fn native_instance_rejects_missing_binary() {
        assert!(NativeVst3Instance::load(Path::new("/definitely/missing/radiant.vst3")).is_err());
    }
}
