#![no_main]

use libfuzzer_sys::fuzz_target;
use radiant_domain::Session;

fuzz_target!(|bytes: &[u8]| {
    // Deserialization must never panic or mutate any existing project state.
    let _ = serde_json::from_slice::<Session>(bytes);
});
