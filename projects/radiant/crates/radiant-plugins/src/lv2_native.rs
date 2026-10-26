#![allow(unsafe_code)]

use crate::{Lv2Port, Lv2PortKind, PluginDescriptor, PluginFormat, PluginRole};
use std::path::Path;

pub struct NativeLv2Instance {
    active: lilv::instance::ActiveInstance,
    inputs: Vec<Vec<f32>>,
    outputs: Vec<Vec<f32>>,
}

impl NativeLv2Instance {
    pub fn load(path: &Path, sample_rate: f64, block_size: usize) -> Result<Self, String> {
        let world = lilv::World::with_load_all();
        let audio = world.new_uri("http://lv2plug.in/ns/lv2core#AudioPort");
        let input_uri = world.new_uri("http://lv2plug.in/ns/lv2core#InputPort");
        let Some(plugin) = world.plugins().into_iter().find(|plugin| {
            plugin
                .bundle_uri()
                .as_uri()
                .is_some_and(|uri| uri.contains(&path.to_string_lossy().to_string()))
        }) else {
            return Err("LV2 bundle was not found in Lilv world".into());
        };
        let audio_indices: Vec<usize> = plugin
            .iter_ports()
            .enumerate()
            .filter_map(|(index, port)| port.is_a(&audio).then_some(index))
            .collect();
        let input_indices: Vec<usize> = plugin
            .iter_ports()
            .enumerate()
            .filter_map(|(index, port)| {
                (port.is_a(&audio) && port.is_a(&input_uri)).then_some(index)
            })
            .collect();
        let mut inputs = vec![vec![0.0; block_size]; input_indices.len()];
        let mut outputs =
            vec![vec![0.0; block_size]; audio_indices.len().saturating_sub(input_indices.len())];
        let Some(instance) = (unsafe { plugin.instantiate(sample_rate, std::iter::empty()) })
        else {
            return Err("LV2 instantiate failed".into());
        };
        let mut active = unsafe { instance.activate() };
        let mut in_cursor = 0;
        let mut out_cursor = 0;
        for index in audio_indices {
            if input_indices.contains(&index) {
                unsafe {
                    active
                        .instance_mut()
                        .connect_port_mut(index, inputs[in_cursor].as_mut_ptr());
                };
                in_cursor += 1;
            } else {
                unsafe {
                    active
                        .instance_mut()
                        .connect_port_mut(index, outputs[out_cursor].as_mut_ptr());
                };
                out_cursor += 1;
            }
        }
        Ok(Self {
            active,
            inputs,
            outputs,
        })
    }

    pub fn process(&mut self, frames: usize) -> &[Vec<f32>] {
        unsafe { self.active.run(frames) };
        &self.outputs
    }

    #[must_use]
    pub fn inputs(&self) -> &[Vec<f32>] {
        &self.inputs
    }
}

/// Discover LV2 metadata through Lilv. This is called only by the external scanner.
#[must_use]
pub fn discover(path: &Path) -> Vec<PluginDescriptor> {
    let world = lilv::World::with_load_all();
    let audio = world.new_uri("http://lv2plug.in/ns/lv2core#AudioPort");
    let control = world.new_uri("http://lv2plug.in/ns/lv2core#ControlPort");
    let atom = world.new_uri("http://lv2plug.in/ns/ext/atom#AtomPort");
    let input = world.new_uri("http://lv2plug.in/ns/lv2core#InputPort");
    world
        .plugins()
        .into_iter()
        .filter(|plugin| {
            plugin
                .bundle_uri()
                .as_uri()
                .is_some_and(|uri| uri.contains(&path.to_string_lossy().to_string()))
        })
        .map(|plugin| {
            let ports: Vec<Lv2Port> = plugin
                .iter_ports()
                .enumerate()
                .filter_map(|(index, port)| {
                    let kind = if port.is_a(&audio) {
                        Lv2PortKind::Audio
                    } else if port.is_a(&control) {
                        Lv2PortKind::Control
                    } else if port.is_a(&atom) {
                        Lv2PortKind::Atom
                    } else {
                        return None;
                    };
                    Some(Lv2Port {
                        symbol: port
                            .symbol()
                            .and_then(|symbol| symbol.as_str().map(str::to_owned))
                            .unwrap_or_else(|| "port".into()),
                        kind,
                        index: u32::try_from(index).unwrap_or(u32::MAX),
                        input: port.is_a(&input),
                    })
                })
                .collect();
            let audio_inputs = ports
                .iter()
                .filter(|p| p.kind == Lv2PortKind::Audio && p.input)
                .count();
            let audio_outputs = ports
                .iter()
                .filter(|p| p.kind == Lv2PortKind::Audio && !p.input)
                .count();
            let controls = ports
                .iter()
                .filter(|p| p.kind == Lv2PortKind::Control)
                .count();
            PluginDescriptor {
                format: PluginFormat::Lv2,
                role: PluginRole::Effect,
                plugin_id: plugin.uri().as_uri().unwrap_or("unknown").into(),
                name: plugin.name().as_str().unwrap_or("Unknown").into(),
                path: path.to_path_buf(),
                audio_inputs: u16::try_from(audio_inputs).unwrap_or(u16::MAX),
                audio_outputs: u16::try_from(audio_outputs).unwrap_or(u16::MAX),
                parameters: u32::try_from(controls).unwrap_or(u32::MAX),
                supports_state: true,
                supports_ui: plugin.uis().iter().next().is_some(),
                latency_frames: 0,
                tail_frames: 0,
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn missing_bundle_returns_no_descriptors() {
        assert!(discover(Path::new("/definitely/missing/radiant.lv2")).is_empty());
    }

    #[test]
    fn native_instance_rejects_missing_bundle() {
        assert!(
            NativeLv2Instance::load(Path::new("/definitely/missing/radiant.lv2"), 48_000.0, 64)
                .is_err()
        );
    }
}
