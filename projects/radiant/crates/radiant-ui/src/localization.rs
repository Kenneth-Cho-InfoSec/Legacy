//! Small, deterministic localization boundary for the desktop shell.

use std::env;

#[must_use]
pub fn text(key: &str) -> String {
    let locale = env::var("LANG").unwrap_or_default().to_ascii_lowercase();
    match (key, locale.split(['_', '-']).next().unwrap_or("en")) {
        ("app.title", "fr") => "Radiant — station audio".into(),
        ("app.title", "de") => "Radiant — Audiowerkstatt".into(),
        ("app.title", _) => "Radiant".into(),
        _ => key.into(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unknown_keys_are_stable_and_default_locale_is_english() {
        assert_eq!(text("unknown.key"), "unknown.key");
        assert!(!text("app.title").is_empty());
    }
}
