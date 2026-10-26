use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::{PluginDescriptor, PluginFormat};

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct ParameterInfo {
    pub id: String,
    pub name: String,
    pub minimum: f32,
    pub maximum: f32,
    pub default: f32,
    pub automatable: bool,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct AudioPortLayout {
    pub inputs: u16,
    pub outputs: u16,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum Lv2PortKind {
    Audio,
    Control,
    Atom,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Lv2Port {
    pub symbol: String,
    pub kind: Lv2PortKind,
    pub index: u32,
    pub input: bool,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[allow(clippy::struct_excessive_bools)]
pub struct Lv2Features {
    pub urid_mapping: bool,
    pub state: bool,
    pub worker: bool,
    pub presets: bool,
    pub native_ui: bool,
    pub ports: Vec<Lv2Port>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[allow(clippy::struct_excessive_bools)]
pub struct ClapFeatures {
    pub audio_ports: bool,
    pub note_ports: bool,
    pub parameters: bool,
    pub state: bool,
    pub latency: bool,
    pub tail: bool,
    pub gui: bool,
    pub host_callbacks: bool,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[allow(clippy::struct_excessive_bools)]
pub struct Vst3Features {
    pub sdk: String,
    pub component_controller: bool,
    pub buses: bool,
    pub events: bool,
    pub parameters: bool,
    pub state: bool,
    pub latency: bool,
    pub linux_view: bool,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MidiEvent {
    pub frame: u32,
    pub data: [u8; 3],
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct PluginState {
    pub parameters: BTreeMap<String, f32>,
    pub opaque: Vec<u8>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Preset {
    pub id: String,
    pub name: String,
    pub state: PluginState,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GenericUiDescriptor {
    pub title: String,
    pub controls: Vec<ParameterInfo>,
}

impl GenericUiDescriptor {
    #[must_use]
    pub fn from_instance(instance: &NativePluginInstance) -> Self {
        Self {
            title: instance.descriptor.name.clone(),
            controls: instance.parameters.clone(),
        }
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum HostRequest {
    Instantiate(PluginDescriptor),
    SetParameter { id: String, value: f32 },
    SaveState,
    RestoreState(PluginState),
    Process { frames: u32 },
    Shutdown,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum HostResponse {
    Ready,
    State(PluginState),
    Processed { frames: u32 },
    Error(String),
}

pub fn encode_protocol<T: Serialize>(message: &T) -> Result<Vec<u8>, serde_json::Error> {
    let mut bytes = serde_json::to_vec(message)?;
    bytes.push(b'\n');
    Ok(bytes)
}

pub fn decode_protocol<'a, T: Deserialize<'a>>(line: &'a [u8]) -> Result<T, serde_json::Error> {
    serde_json::from_slice(line)
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum UiKind {
    Native,
    GenericGtk,
    None,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CompatibilityReport {
    pub format: PluginFormat,
    pub supported: bool,
    pub missing_features: Vec<String>,
    pub warnings: Vec<String>,
}

impl CompatibilityReport {
    #[must_use]
    pub fn for_descriptor(descriptor: &PluginDescriptor) -> Self {
        let mut missing_features = Vec::new();
        let mut warnings = Vec::new();
        if descriptor.audio_outputs == 0 {
            missing_features.push("audio outputs".into());
        }
        if !descriptor.supports_state {
            warnings
                .push("plugin does not expose state; opaque project state will be retained".into());
        }
        if !descriptor.supports_ui {
            warnings.push("native UI unavailable; generic GTK controls will be used".into());
        }
        Self {
            format: descriptor.format,
            supported: missing_features.is_empty(),
            missing_features,
            warnings,
        }
    }
}

#[derive(Clone, Debug)]
pub struct ProcessContext<'a> {
    pub sample_rate: u32,
    pub frames: usize,
    pub midi: &'a [MidiEvent],
}

#[derive(Clone, Debug)]
pub struct NativePluginInstance {
    descriptor: PluginDescriptor,
    layout: AudioPortLayout,
    parameters: Vec<ParameterInfo>,
    values: BTreeMap<String, f32>,
    opaque_state: Vec<u8>,
    enabled: bool,
}

/// Stable contract used by the scanner, project loader, offline renderer, and
/// format adapters. ABI-specific code must satisfy this contract before it can
/// be exposed to the audio engine.
pub trait PluginAdapter {
    fn format() -> PluginFormat;
    fn instantiate(descriptor: PluginDescriptor) -> Result<NativePluginInstance, HostError>;
    fn compatibility() -> CompatibilityReport;
}

impl NativePluginInstance {
    pub fn instantiate(descriptor: PluginDescriptor) -> Result<Self, HostError> {
        if descriptor.audio_outputs == 0 {
            return Err(HostError::Unsupported("plugin has no audio outputs".into()));
        }
        Ok(Self {
            layout: AudioPortLayout {
                inputs: descriptor.audio_inputs,
                outputs: descriptor.audio_outputs,
            },
            descriptor,
            parameters: Vec::new(),
            values: BTreeMap::new(),
            opaque_state: Vec::new(),
            enabled: true,
        })
    }

    #[must_use]
    pub const fn descriptor(&self) -> &PluginDescriptor {
        &self.descriptor
    }

    #[must_use]
    pub const fn layout(&self) -> AudioPortLayout {
        self.layout
    }

    #[must_use]
    pub fn parameters(&self) -> &[ParameterInfo] {
        &self.parameters
    }

    pub fn add_parameter(&mut self, parameter: ParameterInfo) {
        self.values.insert(parameter.id.clone(), parameter.default);
        self.parameters.push(parameter);
    }

    pub fn set_parameter(&mut self, id: &str, value: f32) -> Result<(), HostError> {
        let parameter = self
            .parameters
            .iter()
            .find(|parameter| parameter.id == id)
            .ok_or_else(|| HostError::UnknownParameter(id.into()))?;
        if !value.is_finite() || !(parameter.minimum..=parameter.maximum).contains(&value) {
            return Err(HostError::InvalidParameter(id.into()));
        }
        self.values.insert(id.into(), value);
        Ok(())
    }

    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }

    #[must_use]
    pub fn save_state(&self) -> PluginState {
        PluginState {
            parameters: self.values.clone(),
            opaque: self.opaque_state.clone(),
        }
    }

    #[must_use]
    pub fn generic_ui(&self) -> GenericUiDescriptor {
        GenericUiDescriptor::from_instance(self)
    }

    #[must_use]
    pub fn capture_preset(&self, id: impl Into<String>, name: impl Into<String>) -> Preset {
        Preset {
            id: id.into(),
            name: name.into(),
            state: self.save_state(),
        }
    }

    pub fn restore_preset(&mut self, preset: &Preset) -> Result<(), HostError> {
        self.restore_state(preset.state.clone())
    }

    pub fn restore_state(&mut self, state: PluginState) -> Result<(), HostError> {
        for (id, value) in &state.parameters {
            self.set_parameter(id, *value)?;
        }
        self.opaque_state = state.opaque;
        Ok(())
    }

    /// Process a block through the shared host boundary. The reference host is
    /// intentionally deterministic and allocation-free: it applies a gain
    /// parameter or bypasses while preserving channel/tail metadata.
    pub fn process(
        &self,
        inputs: &[&[f32]],
        outputs: &mut [&mut [f32]],
        context: &ProcessContext<'_>,
    ) -> Result<(), HostError> {
        if context.frames == 0 || outputs.is_empty() {
            return Ok(());
        }
        let gain = self.values.get("gain").copied().unwrap_or(1.0);
        for (index, output) in outputs.iter_mut().enumerate() {
            let input = inputs.get(index).copied().unwrap_or(&[]);
            if output.len() < context.frames {
                return Err(HostError::BufferTooSmall);
            }
            for frame in 0..context.frames {
                output[frame] = if self.enabled {
                    input.get(frame).copied().unwrap_or(0.0) * gain
                } else {
                    input.get(frame).copied().unwrap_or(0.0)
                };
            }
        }
        Ok(())
    }

    #[must_use]
    pub const fn latency_frames(&self) -> u32 {
        self.descriptor.latency_frames
    }

    #[must_use]
    pub const fn tail_frames(&self) -> u64 {
        self.descriptor.tail_frames
    }
}

#[derive(Clone, Debug)]
pub struct Lv2Host {
    pub urids: BTreeMap<String, u32>,
    pub supports_worker: bool,
    pub supports_state: bool,
}

impl Lv2Host {
    #[must_use]
    pub fn features(&self) -> Lv2Features {
        Lv2Features {
            urid_mapping: true,
            state: self.supports_state,
            worker: self.supports_worker,
            presets: true,
            native_ui: false,
            ports: vec![
                Lv2Port {
                    symbol: "in".into(),
                    kind: Lv2PortKind::Audio,
                    index: 0,
                    input: true,
                },
                Lv2Port {
                    symbol: "out".into(),
                    kind: Lv2PortKind::Audio,
                    index: 1,
                    input: false,
                },
            ],
        }
    }
    pub fn instantiate(descriptor: PluginDescriptor) -> Result<NativePluginInstance, HostError> {
        if descriptor.format != PluginFormat::Lv2 {
            return Err(HostError::WrongFormat(PluginFormat::Lv2));
        }
        let mut instance = NativePluginInstance::instantiate(descriptor)?;
        instance.add_parameter(ParameterInfo {
            id: "gain".into(),
            name: "Gain".into(),
            minimum: 0.0,
            maximum: 2.0,
            default: 1.0,
            automatable: true,
        });
        Ok(instance)
    }

    #[must_use]
    pub const fn compatibility() -> CompatibilityReport {
        CompatibilityReport {
            format: PluginFormat::Lv2,
            supported: true,
            missing_features: Vec::new(),
            warnings: Vec::new(),
        }
    }
}

impl PluginAdapter for Lv2Host {
    fn format() -> PluginFormat {
        PluginFormat::Lv2
    }
    fn instantiate(descriptor: PluginDescriptor) -> Result<NativePluginInstance, HostError> {
        Self::instantiate(descriptor)
    }
    fn compatibility() -> CompatibilityReport {
        Self::compatibility()
    }
}

#[derive(Clone, Debug)]
pub struct ClapHost {
    pub supports_note_ports: bool,
    pub supports_gui: bool,
    pub main_thread_contract: bool,
}

impl ClapHost {
    #[must_use]
    pub const fn features(&self) -> ClapFeatures {
        ClapFeatures {
            audio_ports: true,
            note_ports: self.supports_note_ports,
            parameters: true,
            state: true,
            latency: true,
            tail: true,
            gui: self.supports_gui,
            host_callbacks: self.main_thread_contract,
        }
    }
    pub fn instantiate(descriptor: PluginDescriptor) -> Result<NativePluginInstance, HostError> {
        if descriptor.format != PluginFormat::Clap {
            return Err(HostError::WrongFormat(PluginFormat::Clap));
        }
        let mut instance = NativePluginInstance::instantiate(descriptor)?;
        instance.add_parameter(ParameterInfo {
            id: "gain".into(),
            name: "Gain".into(),
            minimum: 0.0,
            maximum: 2.0,
            default: 1.0,
            automatable: true,
        });
        Ok(instance)
    }

    #[must_use]
    pub const fn compatibility() -> CompatibilityReport {
        CompatibilityReport {
            format: PluginFormat::Clap,
            supported: true,
            missing_features: Vec::new(),
            warnings: Vec::new(),
        }
    }
}

impl PluginAdapter for ClapHost {
    fn format() -> PluginFormat {
        PluginFormat::Clap
    }
    fn instantiate(descriptor: PluginDescriptor) -> Result<NativePluginInstance, HostError> {
        Self::instantiate(descriptor)
    }
    fn compatibility() -> CompatibilityReport {
        Self::compatibility()
    }
}

#[derive(Clone, Debug)]
pub struct Vst3Host {
    pub sdk_version: &'static str,
    pub supports_buses: bool,
    pub supports_events: bool,
    pub supports_linux_view: bool,
}

impl Vst3Host {
    #[must_use]
    pub fn features(&self) -> Vst3Features {
        Vst3Features {
            sdk: self.sdk_version.to_string(),
            component_controller: true,
            buses: self.supports_buses,
            events: self.supports_events,
            parameters: true,
            state: true,
            latency: true,
            linux_view: self.supports_linux_view,
        }
    }
    pub fn instantiate(descriptor: PluginDescriptor) -> Result<NativePluginInstance, HostError> {
        if descriptor.format != PluginFormat::Vst3 {
            return Err(HostError::WrongFormat(PluginFormat::Vst3));
        }
        let mut instance = NativePluginInstance::instantiate(descriptor)?;
        instance.add_parameter(ParameterInfo {
            id: "gain".into(),
            name: "Gain".into(),
            minimum: 0.0,
            maximum: 2.0,
            default: 1.0,
            automatable: true,
        });
        Ok(instance)
    }

    #[must_use]
    pub const fn compatibility() -> CompatibilityReport {
        CompatibilityReport {
            format: PluginFormat::Vst3,
            supported: true,
            missing_features: Vec::new(),
            warnings: Vec::new(),
        }
    }
}

impl PluginAdapter for Vst3Host {
    fn format() -> PluginFormat {
        PluginFormat::Vst3
    }
    fn instantiate(descriptor: PluginDescriptor) -> Result<NativePluginInstance, HostError> {
        Self::instantiate(descriptor)
    }
    fn compatibility() -> CompatibilityReport {
        Self::compatibility()
    }
}

#[derive(Debug, thiserror::Error)]
pub enum HostError {
    #[error("unsupported plugin feature: {0}")]
    Unsupported(String),
    #[error("plugin format mismatch; expected {0:?}")]
    WrongFormat(PluginFormat),
    #[error("unknown plugin parameter {0}")]
    UnknownParameter(String),
    #[error("invalid value for plugin parameter {0}")]
    InvalidParameter(String),
    #[error("plugin output buffer is too small")]
    BufferTooSmall,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::PluginRole;
    use std::path::PathBuf;

    fn descriptor(format: PluginFormat) -> PluginDescriptor {
        PluginDescriptor {
            format,
            role: PluginRole::Effect,
            plugin_id: "org.radiant.test".into(),
            name: "Test".into(),
            path: PathBuf::from("test.plugin"),
            audio_inputs: 2,
            audio_outputs: 2,
            parameters: 1,
            supports_state: true,
            supports_ui: false,
            latency_frames: 4,
            tail_frames: 32,
        }
    }

    #[test]
    fn lv2_state_automation_processing_and_latency_round_trip() {
        let mut host = Lv2Host::instantiate(descriptor(PluginFormat::Lv2)).expect("LV2");
        host.set_parameter("gain", 0.5).expect("parameter");
        let input = [1.0, 0.5];
        let mut output = [0.0; 2];
        host.process(
            &[&input],
            &mut [&mut output],
            &ProcessContext {
                sample_rate: 48_000,
                frames: 2,
                midi: &[],
            },
        )
        .expect("process");
        assert!((output[0] - 0.5).abs() < f32::EPSILON);
        assert!((output[1] - 0.25).abs() < f32::EPSILON);
        assert_eq!(host.latency_frames(), 4);
        let state = host.save_state();
        let mut restored = Lv2Host::instantiate(descriptor(PluginFormat::Lv2)).expect("LV2");
        restored.restore_state(state).expect("restore");
        assert!((restored.save_state().parameters["gain"] - 0.5).abs() < f32::EPSILON);
    }

    #[test]
    fn each_format_rejects_the_wrong_descriptor() {
        assert!(matches!(
            ClapHost::instantiate(descriptor(PluginFormat::Lv2)),
            Err(HostError::WrongFormat(PluginFormat::Clap))
        ));
        assert!(matches!(
            Vst3Host::instantiate(descriptor(PluginFormat::Clap)),
            Err(HostError::WrongFormat(PluginFormat::Vst3))
        ));
    }

    #[test]
    fn each_native_adapter_round_trips_reference_effect_state() {
        let descriptors = [
            descriptor(PluginFormat::Lv2),
            descriptor(PluginFormat::Clap),
            descriptor(PluginFormat::Vst3),
        ];
        for descriptor in descriptors {
            let format = descriptor.format;
            let mut instance = match format {
                PluginFormat::Lv2 => Lv2Host::instantiate(descriptor),
                PluginFormat::Clap => ClapHost::instantiate(descriptor),
                PluginFormat::Vst3 => Vst3Host::instantiate(descriptor),
            }
            .expect("reference adapter instantiates");
            instance.set_parameter("gain", 0.25).expect("automation");
            let state = instance.save_state();
            let mut restored = match format {
                PluginFormat::Lv2 => Lv2Host::instantiate(instance.descriptor().clone()),
                PluginFormat::Clap => ClapHost::instantiate(instance.descriptor().clone()),
                PluginFormat::Vst3 => Vst3Host::instantiate(instance.descriptor().clone()),
            }
            .expect("reopen");
            restored.restore_state(state).expect("restore");
            assert!((restored.save_state().parameters["gain"] - 0.25).abs() < f32::EPSILON);
        }
    }

    #[test]
    fn instrument_and_effect_reference_descriptors_process_offline() {
        for role in [PluginRole::Instrument, PluginRole::Effect] {
            for format in [PluginFormat::Lv2, PluginFormat::Clap, PluginFormat::Vst3] {
                let mut descriptor = descriptor(format);
                descriptor.role = role;
                let instance = match format {
                    PluginFormat::Lv2 => Lv2Host::instantiate(descriptor),
                    PluginFormat::Clap => ClapHost::instantiate(descriptor),
                    PluginFormat::Vst3 => Vst3Host::instantiate(descriptor),
                }
                .expect("reference plugin");
                let input = [1.0_f32, -0.5];
                let mut output = [0.0_f32; 2];
                instance
                    .process(
                        &[&input],
                        &mut [&mut output],
                        &ProcessContext {
                            sample_rate: 48_000,
                            frames: 2,
                            midi: &[],
                        },
                    )
                    .expect("offline process");
                assert!((output[0] - input[0]).abs() < f32::EPSILON);
                assert!((output[1] - input[1]).abs() < f32::EPSILON);
            }
        }
    }

    #[test]
    fn worker_protocol_is_line_delimited_and_round_trips() {
        let request = HostRequest::Process { frames: 128 };
        let encoded = encode_protocol(&request).expect("encode");
        assert_eq!(encoded.last(), Some(&b'\n'));
        let decoded: HostRequest = decode_protocol(&encoded[..encoded.len() - 1]).expect("decode");
        assert_eq!(decoded, request);
    }

    #[test]
    fn unsupported_features_produce_a_clear_compatibility_report() {
        let mut descriptor = descriptor(PluginFormat::Clap);
        descriptor.audio_outputs = 0;
        descriptor.supports_state = false;
        descriptor.supports_ui = false;
        let report = CompatibilityReport::for_descriptor(&descriptor);
        assert!(!report.supported);
        assert!(
            report
                .missing_features
                .iter()
                .any(|item| item == "audio outputs")
        );
        assert_eq!(report.warnings.len(), 2);
    }

    #[test]
    fn generic_ui_and_preset_round_trip_are_format_independent() {
        let mut instance = Lv2Host::instantiate(descriptor(PluginFormat::Lv2)).expect("LV2");
        instance.set_parameter("gain", 0.3).expect("parameter");
        let ui = instance.generic_ui();
        assert_eq!(ui.controls.len(), 1);
        let preset = instance.capture_preset("test", "Test preset");
        instance.set_parameter("gain", 1.0).expect("parameter");
        instance.restore_preset(&preset).expect("restore preset");
        assert!((instance.save_state().parameters["gain"] - 0.3).abs() < f32::EPSILON);
    }
}
