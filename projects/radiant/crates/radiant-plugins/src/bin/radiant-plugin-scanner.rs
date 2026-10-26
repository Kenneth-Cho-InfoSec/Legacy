use std::env;
use std::path::Path;

use radiant_plugins::{
    PluginDescriptor, PluginFormat, PluginRole, discover_clap_native, discover_lv2_native,
    discover_vst3_native,
};

fn main() {
    let Some(path) = env::args().skip_while(|arg| arg != "--probe").nth(1) else {
        std::process::exit(2);
    };
    let path_ref = Path::new(&path);
    let format = match path_ref
        .extension()
        .and_then(|extension| extension.to_str())
    {
        Some("clap") => PluginFormat::Clap,
        Some("vst3") => PluginFormat::Vst3,
        _ => PluginFormat::Lv2,
    };
    let plugin_id = path_ref.to_string_lossy().into_owned();
    if format == PluginFormat::Clap
        && let Ok(mut descriptors) = discover_clap_native(path_ref)
        && let Some(descriptor) = descriptors.pop()
    {
        println!(
            "{}",
            serde_json::to_string(&descriptor).expect("descriptor JSON")
        );
        return;
    }
    if format == PluginFormat::Lv2
        && let Some(descriptor) = discover_lv2_native(path_ref).into_iter().next()
    {
        println!(
            "{}",
            serde_json::to_string(&descriptor).expect("descriptor JSON")
        );
        return;
    }
    if format == PluginFormat::Vst3
        && let Ok(mut descriptors) = discover_vst3_native(path_ref)
        && let Some(descriptor) = descriptors.pop()
    {
        println!(
            "{}",
            serde_json::to_string(&descriptor).expect("descriptor JSON")
        );
        return;
    }
    let descriptor = PluginDescriptor {
        format,
        role: PluginRole::Effect,
        plugin_id: plugin_id.clone(),
        name: path_ref
            .file_stem()
            .and_then(|name| name.to_str())
            .unwrap_or("Unknown")
            .into(),
        path: path_ref.to_path_buf(),
        audio_inputs: 2,
        audio_outputs: 2,
        parameters: 0,
        supports_state: true,
        supports_ui: false,
        latency_frames: 0,
        tail_frames: 0,
    };
    println!(
        "{}",
        serde_json::to_string(&descriptor).expect("descriptor JSON")
    );
}
