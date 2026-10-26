#![allow(unsafe_code)]
use std::ffi::CString;
use std::path::Path;

use clack_host::prelude::PluginEntry;

use crate::{PluginDescriptor, PluginFormat, PluginRole};

pub struct NativeClapInstance {
    _entry: PluginEntry,
    instance: clack_host::plugin::PluginInstance<()>,
    processor: Option<clack_host::process::PluginAudioProcessor<()>>,
}

impl NativeClapInstance {
    pub fn load(path: &Path, plugin_id: &str) -> Result<Self, String> {
        let entry = unsafe { PluginEntry::load(path) }.map_err(|error| error.to_string())?;
        let id = CString::new(plugin_id).map_err(|error| error.to_string())?;
        let host = clack_host::host::HostInfo::new(
            "Radiant",
            "Radiant",
            "https://radiantdaw.org",
            "0.1.0",
        )
        .map_err(|error| error.to_string())?;
        let instance =
            clack_host::plugin::PluginInstance::<()>::new(|()| (), |()| (), &entry, &id, &host)
                .map_err(|error| error.to_string())?;
        Ok(Self {
            _entry: entry,
            instance,
            processor: None,
        })
    }

    pub fn activate(&mut self, sample_rate: f64, block_size: u32) -> Result<(), String> {
        let configuration = clack_host::prelude::PluginAudioConfiguration {
            sample_rate,
            min_frames_count: block_size,
            max_frames_count: block_size,
        };
        let processor = self
            .instance
            .activate(|(), ()| (), configuration)
            .map_err(|error| error.to_string())?;
        self.processor = Some(processor.into());
        Ok(())
    }

    pub fn process_silence(&mut self) -> Result<(), String> {
        let processor = self
            .processor
            .as_mut()
            .ok_or_else(|| "CLAP instance is not active".to_string())?;
        let started = processor
            .start_processing()
            .map_err(|error| error.to_string())?;
        let input = clack_host::prelude::InputAudioBuffers::empty();
        let mut output = clack_host::prelude::OutputAudioBuffers::empty();
        let input_events = clack_host::prelude::InputEvents::empty();
        let mut event_buffer = clack_host::prelude::EventBuffer::new();
        let mut output_events = clack_host::prelude::OutputEvents::from_buffer(&mut event_buffer);
        started
            .process(
                &input,
                &mut output,
                &input_events,
                &mut output_events,
                Some(0),
                None,
            )
            .map_err(|error| error.to_string())?;
        Ok(())
    }

    #[must_use]
    pub fn instance(&self) -> &clack_host::plugin::PluginInstance<()> {
        &self.instance
    }
}

/// Probe a CLAP entry from a short-lived scanner process. CLAP requires a
/// dynamic-library load, so this function is deliberately not used by the UI
/// or audio process directly.
pub fn discover(path: &Path) -> Result<Vec<PluginDescriptor>, String> {
    let entry = unsafe { PluginEntry::load(path) }.map_err(|error| error.to_string())?;
    let factory = entry
        .get_plugin_factory()
        .ok_or_else(|| "CLAP entry has no plugin factory".to_string())?;
    Ok(factory
        .plugin_descriptors()
        .filter_map(|plugin| {
            let plugin_id = plugin.id()?.to_string_lossy().into_owned();
            let name = plugin.name()?.to_string_lossy().into_owned();
            let role = if plugin
                .features()
                .any(|feature| feature.to_string_lossy().contains("instrument"))
            {
                PluginRole::Instrument
            } else {
                PluginRole::Effect
            };
            Some(PluginDescriptor {
                format: PluginFormat::Clap,
                role,
                plugin_id,
                name,
                path: path.to_path_buf(),
                audio_inputs: 2,
                audio_outputs: 2,
                parameters: 0,
                supports_state: true,
                supports_ui: false,
                latency_frames: 0,
                tail_frames: 0,
            })
        })
        .collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn missing_entry_is_rejected() {
        assert!(discover(Path::new("/definitely/missing/radiant.clap")).is_err());
    }

    #[test]
    fn native_instance_rejects_missing_entry() {
        assert!(
            NativeClapInstance::load(
                Path::new("/definitely/missing/radiant.clap"),
                "org.missing.plugin"
            )
            .is_err()
        );
    }
}
