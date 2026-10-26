//! Deterministic long-session recovery smoke/soak check for CI and release jobs.

use std::fs;
use std::time::{SystemTime, UNIX_EPOCH};

use radiant_domain::Session;
use radiant_session::ProjectStore;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let nonce = SystemTime::now().duration_since(UNIX_EPOCH)?.as_nanos();
    let root = std::env::temp_dir().join(format!("radiant-recovery-soak-{nonce}.radiant"));
    let result = (|| {
        let mut session = Session::new("Recovery soak");
        let store = ProjectStore::create(&root, &session)?;
        for iteration in 0..1_000_u32 {
            session.name = format!("Recovery soak {iteration}");
            store.autosave(&session)?;
            if iteration % 25 == 0 {
                let checkpoint = root.join("recovery-checkpoint.sqlite");
                store.checkpoint(&checkpoint)?;
                let recovered = store.load_autosave()?.ok_or("missing autosave")?;
                if recovered.name != session.name {
                    return Err("autosave mismatch".into());
                }
            }
        }
        let (_, durable) = ProjectStore::open(&root)?;
        if durable.name != "Recovery soak" {
            return Err("durable snapshot changed unexpectedly".into());
        }
        Ok::<(), Box<dyn std::error::Error>>(())
    })();
    let _ = fs::remove_dir_all(&root);
    result
}
