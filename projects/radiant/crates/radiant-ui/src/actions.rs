use radiant_app::AppController;

pub fn history_action_state(controller: &AppController) -> (bool, bool) {
    (controller.can_undo(), controller.can_redo())
}

pub fn project_directory_name(session_name: &str) -> String {
    let stem: String = session_name
        .chars()
        .map(|character| {
            if character.is_alphanumeric() || matches!(character, '-' | '_') {
                character
            } else {
                '_'
            }
        })
        .collect();
    format!("{}.radiant", stem.trim_matches('_').to_lowercase())
}
