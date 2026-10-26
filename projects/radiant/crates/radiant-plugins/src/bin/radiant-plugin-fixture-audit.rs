use radiant_plugins::audit_live_fixtures;
use std::env;
use std::path::PathBuf;

fn main() {
    let paths: Vec<PathBuf> = env::args().skip(1).map(PathBuf::from).collect();
    let paths = if paths.is_empty() {
        vec![PathBuf::from("/usr/lib"), PathBuf::from("/usr/local/lib")]
    } else {
        paths
    };
    println!(
        "{}",
        serde_json::to_string_pretty(&audit_live_fixtures(&paths)).expect("fixture report")
    );
}
