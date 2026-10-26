use radiant_app::{PresentationState, WorkspacePanel};

pub fn panel_is_visible(presentation: &PresentationState, panel: WorkspacePanel) -> bool {
    presentation.visible_panels.contains(&panel)
}
