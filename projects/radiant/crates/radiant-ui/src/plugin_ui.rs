use gtk::prelude::*;
use gtk4 as gtk;
use radiant_plugins::GenericUiDescriptor;

/// Build a native GTK4 compatibility editor for a plugin without an embedded
/// native view. It exposes the scanner/host descriptors as real GTK controls.
#[must_use]
pub fn build_generic_plugin_ui(descriptor: &GenericUiDescriptor) -> gtk::Box {
    let panel = gtk::Box::new(gtk::Orientation::Vertical, 8);
    panel.set_margin_top(12);
    panel.set_margin_bottom(12);
    panel.set_margin_start(12);
    panel.set_margin_end(12);
    panel.append(&gtk::Label::new(Some(&descriptor.title)));
    for parameter in &descriptor.controls {
        let row = gtk::Box::new(gtk::Orientation::Horizontal, 8);
        let label = gtk::Label::new(Some(&parameter.name));
        label.set_hexpand(true);
        label.set_xalign(0.0);
        let scale = gtk::Scale::with_range(
            gtk::Orientation::Horizontal,
            f64::from(parameter.minimum),
            f64::from(parameter.maximum),
            0.001,
        );
        scale.set_value(f64::from(parameter.default));
        scale.set_sensitive(parameter.automatable);
        scale.set_hexpand(true);
        row.append(&label);
        row.append(&scale);
        panel.append(&row);
    }
    panel
}
