//! Radiant's native GTK4/libadwaita desktop shell.

mod actions;
mod channel_rack;
mod drawing;
mod localization;
mod mixer;
mod piano_roll;
mod playlist;
mod workspace;

use std::{
    cell::{Cell, RefCell},
    collections::HashSet,
    path::Path,
    rc::Rc,
    sync::Arc,
};

use adw::prelude::*;
use gtk::cairo;
use gtk::gdk;
use gtk::gio;
use gtk4 as gtk;
use libadwaita as adw;
use radiant_app::{AppController, WorkspacePanel};
use radiant_audio::{
    AudioCommand, BackendPreference, NativeAudioService, NativeCapture, RealtimeState,
    available_devices,
};
use radiant_commands::{Command, Transaction};
use radiant_domain::{
    Channel, ChannelId, ClipId, MixerInsert, MixerInsertId, Note, NoteId, Pattern, PatternId,
    PatternLane, PlaylistClip, PlaylistClipContent, PlaylistLaneId, Rgba, Session, Step, Tick,
};
use radiant_engine::{MediaPool, RenderOptions, RenderPlan};
use radiant_session::RecoverableTake;

use actions::{history_action_state, project_directory_name};
use drawing::set_cairo_color;
use localization::text as localized_text;
use piano_roll::{beats_to_ticks, normalized_velocity_to_midi, ticks_to_beats};
use playlist::{bars_to_ticks as playlist_bars_to_ticks, ticks_to_bars as playlist_ticks_to_bars};

const APP_ID: &str = "org.radiantdaw.Radiant";

fn main() -> gtk::glib::ExitCode {
    let application = adw::Application::builder().application_id(APP_ID).build();
    application.connect_activate(build_application);
    application.run()
}

fn build_starter_session() -> Session {
    let mut session = Session::new("Untitled");
    let master_id = session.master_insert().expect("starter master insert").id;
    let mut pattern = Pattern::new("Pattern 1", Tick::beats(4));

    for (name, active_steps) in [
        ("Kick", &[0_usize, 4, 8, 12][..]),
        ("Bass", &[0_usize, 3, 7, 10, 14][..]),
        ("Hat", &[0_usize, 2, 4, 6, 8, 10, 12, 14][..]),
        ("Snare", &[4_usize, 12][..]),
    ] {
        let insert = MixerInsert::insert(name, master_id);
        let mut channel = Channel::sampler(name, insert.id);
        channel.volume = 0.8;
        let steps = active_steps
            .iter()
            .map(|step| Step {
                at: Tick(i64::try_from(*step).expect("step index") * 240),
                velocity: 0.8,
                probability: 1.0,
                micro_shift: Tick::ZERO,
                ratchets: 1,
            })
            .collect();
        let notes = if name == "Bass" {
            vec![
                Note {
                    id: NoteId::new(),
                    start: Tick::ZERO,
                    length: Tick::beats(1),
                    pitch: 48,
                    velocity: 96,
                    channel: 0,
                },
                Note {
                    id: NoteId::new(),
                    start: Tick::beats(2),
                    length: Tick::beats(1),
                    pitch: 52,
                    velocity: 104,
                    channel: 0,
                },
            ]
        } else {
            Vec::new()
        };
        pattern.lanes.push(PatternLane {
            channel_id: channel.id,
            steps,
            notes,
        });
        session.channels.push(channel);
        session.mixer_inserts.push(insert);
    }

    let pattern_id = pattern.id;
    session.patterns.push(pattern);
    session.playlist_lanes[0]
        .clips
        .push(radiant_domain::PlaylistClip {
            id: radiant_domain::ClipId::new(),
            name: "Pattern 1".into(),
            start: Tick::ZERO,
            length: Tick::beats(4),
            color: Rgba::accent(),
            muted: false,
            content: PlaylistClipContent::Pattern { pattern_id },
        });
    session
}

fn build_application(application: &adw::Application) {
    let mut app_controller =
        AppController::new(build_starter_session()).expect("valid starter session");
    if let Ok(project_path) = std::env::var("RADIANT_PROJECT_PATH") {
        let path = Path::new(&project_path);
        if path.is_dir() {
            app_controller
                .replace_with_open(path)
                .expect("open project supplied on command line");
        }
    }
    let controller = Rc::new(RefCell::new(app_controller));
    let window = adw::ApplicationWindow::builder()
        .application(application)
        .title(localized_text("app.title"))
        .default_width(1_360)
        .default_height(820)
        .resizable(true)
        .build();

    let sections = build_workspace(&controller);
    let audio_preference = Rc::new(Cell::new(BackendPreference::Auto));
    let toolbar_view = adw::ToolbarView::new();
    let header_bar = build_header_bar(application, &window, &audio_preference);
    let transport_bars = build_transport_bars(
        &window,
        &controller,
        &audio_preference,
        &sections.channel_rack_bindings,
        &sections.mixer_bindings,
        &sections.pattern_selector_bindings,
        &sections.playlist_bindings,
    );
    let status_bar = build_status_bar();
    toolbar_view.add_top_bar(&header_bar);
    toolbar_view.add_top_bar(&transport_bars.primary);
    toolbar_view.add_top_bar(&transport_bars.overflow_views);
    toolbar_view.add_bottom_bar(&status_bar);
    toolbar_view.set_content(Some(&sections.sidebar));
    window.set_content(Some(&toolbar_view));

    install_window_actions(application, &window, &sections, &controller);
    install_breakpoints(&window, &sections, &transport_bars);
    window.present();
}

#[derive(Clone)]
struct WorkspaceSections {
    sidebar: adw::OverlaySplitView,
    primary_split: gtk::Paned,
    left_column: gtk::Paned,
    arrangement: gtk::Paned,
    channel_rack: gtk::Frame,
    mixer: gtk::Frame,
    picker: gtk::Frame,
    playlist: gtk::Frame,
    channel_rack_bindings: ChannelRackBindings,
    playlist_bindings: PlaylistBindings,
    mixer_bindings: MixerBindings,
    picker_bindings: PickerBindings,
    pattern_selector_bindings: Rc<RefCell<Option<PatternSelectorBindings>>>,
}

impl WorkspaceSections {
    fn sync_layout_visibility(&self) {
        self.left_column
            .set_visible(self.channel_rack.is_visible() || self.mixer.is_visible());
        self.arrangement
            .set_visible(self.picker.is_visible() || self.playlist.is_visible());
    }

    fn sync_from_controller(&self, controller: &Rc<RefCell<AppController>>) {
        let presentation = {
            let controller_ref = controller.borrow();
            self.channel_rack_bindings.sync(&controller_ref);
            self.playlist_bindings.sync(&controller_ref);
            self.mixer_bindings.sync(&controller_ref);
            self.picker_bindings.sync(&controller_ref);
            if let Some(bindings) = self.pattern_selector_bindings.borrow().as_ref() {
                bindings.sync(&controller_ref);
            }
            controller_ref.presentation().clone()
        };
        self.sidebar.set_show_sidebar(workspace::panel_is_visible(
            &presentation,
            WorkspacePanel::Browser,
        ));
        self.channel_rack.set_visible(workspace::panel_is_visible(
            &presentation,
            WorkspacePanel::ChannelRack,
        ));
        self.picker.set_visible(workspace::panel_is_visible(
            &presentation,
            WorkspacePanel::Picker,
        ));
        self.playlist.set_visible(workspace::panel_is_visible(
            &presentation,
            WorkspacePanel::Playlist,
        ));
        self.mixer.set_visible(workspace::panel_is_visible(
            &presentation,
            WorkspacePanel::Mixer,
        ));
        self.primary_split
            .set_position(presentation.primary_divider);
        self.left_column
            .set_position(presentation.channel_mixer_divider);
        self.arrangement
            .set_position(presentation.picker_playlist_divider);
        self.sync_layout_visibility();
    }
}

fn build_workspace(controller: &Rc<RefCell<AppController>>) -> WorkspaceSections {
    let pattern_selector_bindings = Rc::new(RefCell::new(None));
    let sidebar = adw::OverlaySplitView::new();
    sidebar.set_sidebar(Some(&build_browser()));
    sidebar.set_min_sidebar_width(210.0);
    sidebar.set_max_sidebar_width(320.0);
    sidebar.set_sidebar_width_fraction(0.22);
    sidebar.set_enable_hide_gesture(true);
    sidebar.set_enable_show_gesture(true);

    let channel_rack_bindings = ChannelRackBindings::new(controller);
    let channel_rack = section_frame(
        "Channel Rack",
        build_channel_rack(controller, &channel_rack_bindings),
        "win.toggle-channel-rack",
    );
    let (mixer_content, mixer_bindings) = build_mixer(controller);
    let mixer = section_frame("Mixer", mixer_content, "win.toggle-mixer");
    let (playlist_content, playlist_bindings) = build_playlist(controller);
    let (picker_content, picker_bindings) = build_picker(controller, &playlist_bindings);
    let picker = section_frame("Picker", picker_content, "win.toggle-picker");
    let playlist = section_frame(
        "Playlist · Arrangement",
        playlist_content,
        "win.toggle-playlist",
    );

    let left_column = gtk::Paned::new(gtk::Orientation::Vertical);
    left_column.set_start_child(Some(&channel_rack));
    left_column.set_end_child(Some(&mixer));
    left_column.set_resize_start_child(true);
    left_column.set_resize_end_child(true);
    left_column.set_shrink_start_child(false);
    left_column.set_shrink_end_child(false);
    left_column.set_position(330);

    let arrangement = gtk::Paned::new(gtk::Orientation::Horizontal);
    arrangement.set_start_child(Some(&picker));
    arrangement.set_end_child(Some(&playlist));
    arrangement.set_resize_start_child(false);
    arrangement.set_resize_end_child(true);
    arrangement.set_shrink_start_child(true);
    arrangement.set_shrink_end_child(false);
    arrangement.set_position(150);

    let primary_split = gtk::Paned::new(gtk::Orientation::Horizontal);
    primary_split.set_start_child(Some(&left_column));
    primary_split.set_end_child(Some(&arrangement));
    primary_split.set_resize_start_child(false);
    primary_split.set_resize_end_child(true);
    primary_split.set_shrink_start_child(true);
    primary_split.set_shrink_end_child(false);
    primary_split.set_position(390);
    sidebar.set_content(Some(&primary_split));

    let sections = WorkspaceSections {
        sidebar,
        primary_split,
        left_column,
        arrangement,
        channel_rack,
        mixer,
        picker,
        playlist,
        channel_rack_bindings,
        playlist_bindings,
        mixer_bindings,
        picker_bindings,
        pattern_selector_bindings,
    };
    install_presentation_tracking(&sections, controller);
    sections.sync_from_controller(controller);
    sections
}

fn install_presentation_tracking(
    sections: &WorkspaceSections,
    controller: &Rc<RefCell<AppController>>,
) {
    sections.primary_split.connect_position_notify({
        let controller = controller.clone();
        move |paned| controller.borrow_mut().presentation_mut().primary_divider = paned.position()
    });
    sections.left_column.connect_position_notify({
        let controller = controller.clone();
        move |paned| {
            controller
                .borrow_mut()
                .presentation_mut()
                .channel_mixer_divider = paned.position();
        }
    });
    sections.arrangement.connect_position_notify({
        let controller = controller.clone();
        move |paned| {
            controller
                .borrow_mut()
                .presentation_mut()
                .picker_playlist_divider = paned.position();
        }
    });
}

fn build_header_bar(
    application: &adw::Application,
    window: &adw::ApplicationWindow,
    audio_preference: &Rc<Cell<BackendPreference>>,
) -> adw::HeaderBar {
    let header = adw::HeaderBar::new();
    let title = adw::WindowTitle::new("Radiant", "Untitled");
    header.set_title_widget(Some(&title));

    let browser = gtk::Button::builder()
        .icon_name("sidebar-show-symbolic")
        .tooltip_text("Show or hide the Browser")
        .build();
    browser.set_action_name(Some("win.toggle-browser"));
    header.pack_start(&browser);

    let new_button = gtk::Button::builder()
        .icon_name("document-new-symbolic")
        .tooltip_text("New Project")
        .action_name("win.new-project")
        .build();
    header.pack_start(&new_button);
    let open_button = gtk::Button::builder()
        .icon_name("document-open-symbolic")
        .tooltip_text("Open Project")
        .action_name("win.open-project")
        .build();
    header.pack_start(&open_button);
    let save_button = gtk::Button::builder()
        .icon_name("document-save-symbolic")
        .tooltip_text("Save Project")
        .action_name("win.save-project")
        .build();
    header.pack_start(&save_button);

    let menu = gio::Menu::new();
    let project = gio::Menu::new();
    project.append(Some("New Project"), Some("win.new-project"));
    project.append(Some("Open Project…"), Some("win.open-project"));
    project.append(Some("Save"), Some("win.save-project"));
    project.append(Some("Save As…"), Some("win.save-project-as"));
    project.append(Some("Export Audio…"), Some("win.export-audio"));
    menu.append_submenu(Some("Project"), &project);
    let edit = gio::Menu::new();
    edit.append(Some("Undo"), Some("win.undo"));
    edit.append(Some("Redo"), Some("win.redo"));
    menu.append_submenu(Some("Edit"), &edit);
    let views = gio::Menu::new();
    views.append(Some("Browser"), Some("win.toggle-browser"));
    views.append(Some("Channel Rack"), Some("win.toggle-channel-rack"));
    views.append(Some("Picker"), Some("win.toggle-picker"));
    views.append(Some("Playlist"), Some("win.toggle-playlist"));
    views.append(Some("Mixer"), Some("win.toggle-mixer"));
    menu.append_submenu(Some("View"), &views);
    menu.append(Some("Audio & MIDI Setup…"), Some("app.audio-setup"));
    menu.append(Some("Keyboard Shortcuts"), Some("app.shortcuts"));
    menu.append(Some("About Radiant"), Some("app.about"));
    let menu_button = gtk::MenuButton::builder()
        .icon_name("open-menu-symbolic")
        .tooltip_text("Main Menu")
        .menu_model(&menu)
        .build();
    header.pack_end(&menu_button);

    let about = gio::SimpleAction::new("about", None);
    let about_window = window.clone();
    about.connect_activate(move |_, _| {
        let dialog = adw::AboutDialog::builder()
            .application_name("Radiant")
            .application_icon(APP_ID)
            .developer_name("Radiant contributors")
            .version(env!("CARGO_PKG_VERSION"))
            .license_type(gtk::License::Gpl30)
            .comments("A Linux-first, pattern-based digital audio workstation.")
            .website("https://github.com/radiantdaw/radiant")
            .build();
        dialog.present(Some(&about_window));
    });
    application.add_action(&about);

    let shortcuts = gio::SimpleAction::new("shortcuts", None);
    let shortcuts_window = window.clone();
    shortcuts.connect_activate(move |_, _| show_shortcuts(&shortcuts_window));
    application.add_action(&shortcuts);

    let audio_setup = gio::SimpleAction::new("audio-setup", None);
    let setup_window = window.clone();
    let setup_preference = audio_preference.clone();
    audio_setup.connect_activate(move |_, _| {
        show_audio_setup(&setup_window, &setup_preference);
    });
    application.add_action(&audio_setup);
    header
}

fn show_audio_setup(window: &adw::ApplicationWindow, preference: &Rc<Cell<BackendPreference>>) {
    let dialog = adw::PreferencesDialog::builder()
        .title("Audio & MIDI Setup")
        .content_width(620)
        .content_height(520)
        .build();
    let page = adw::PreferencesPage::new();

    let backend_group = adw::PreferencesGroup::builder()
        .title("Native audio backend")
        .description("PipeWire is preferred; JACK is the studio fallback.")
        .build();
    let backend_row = adw::ActionRow::builder()
        .title("Backend")
        .subtitle("Applied when the next playback stream starts")
        .build();
    let backend = gtk::DropDown::from_strings(&["Automatic", "PipeWire", "JACK"]);
    backend.set_selected(match preference.get() {
        BackendPreference::Auto => 0,
        BackendPreference::PipeWire => 1,
        BackendPreference::Jack => 2,
    });
    backend.set_valign(gtk::Align::Center);
    backend_row.add_suffix(&backend);
    backend_group.add(&backend_row);
    backend.connect_selected_notify({
        let preference = preference.clone();
        move |backend| {
            preference.set(match backend.selected() {
                1 => BackendPreference::PipeWire,
                2 => BackendPreference::Jack,
                _ => BackendPreference::Auto,
            });
        }
    });

    match available_devices(BackendPreference::Auto) {
        Ok(devices) => {
            for device in devices {
                let row = adw::ActionRow::builder()
                    .title(&device.name)
                    .subtitle(format!(
                        "{:?}{}",
                        device.backend,
                        if device.is_default { " · default" } else { "" }
                    ))
                    .build();
                backend_group.add(&row);
            }
        }
        Err(error) => {
            backend_group.add(
                &adw::ActionRow::builder()
                    .title("Audio discovery failed")
                    .subtitle(error.to_string())
                    .build(),
            );
        }
    }
    page.add(&backend_group);

    let midi_group = adw::PreferencesGroup::builder()
        .title("MIDI")
        .description("Native ALSA Sequencer input with subscription and hot-plug announcements.")
        .build();
    midi_group.add(
        &adw::ActionRow::builder()
            .title("Radiant MIDI In")
            .subtitle("Created when playback starts; connect devices with the desktop MIDI graph")
            .build(),
    );
    page.add(&midi_group);
    dialog.add(&page);
    dialog.present(Some(window));
}

struct TransportBars {
    primary: gtk::ScrolledWindow,
    inline_views: gtk::Box,
    overflow_views: gtk::Box,
}

#[derive(Default)]
struct UiTransportRuntime {
    plan: Option<Arc<RenderPlan>>,
    audio: Option<NativeAudioService>,
    last_restart_attempt: Option<std::time::Instant>,
    capture: Option<NativeCapture>,
    take: Option<RecoverableTake>,
    capture_buffer: Vec<f32>,
}

#[allow(clippy::similar_names, clippy::too_many_lines)]
fn build_transport_bars(
    window: &adw::ApplicationWindow,
    controller: &Rc<RefCell<AppController>>,
    audio_preference: &Rc<Cell<BackendPreference>>,
    channel_rack_bindings: &ChannelRackBindings,
    mixer_bindings: &MixerBindings,
    pattern_selector_bindings: &Rc<RefCell<Option<PatternSelectorBindings>>>,
    playlist_bindings: &PlaylistBindings,
) -> TransportBars {
    let controls = gtk::Box::new(gtk::Orientation::Horizontal, 6);
    controls.set_margin_top(6);
    controls.set_margin_bottom(6);
    controls.set_margin_start(8);
    controls.set_margin_end(8);
    controls.add_css_class("toolbar");

    let pattern_mode = gtk::ToggleButton::with_label("Pattern");
    pattern_mode.set_active(true);
    pattern_mode.set_tooltip_text(Some("Play the selected pattern"));
    let song_mode = gtk::ToggleButton::with_label("Song");
    song_mode.set_group(Some(&pattern_mode));
    song_mode.set_tooltip_text(Some("Play the Playlist arrangement"));
    controls.append(&pattern_mode);
    controls.append(&song_mode);
    controls.append(&gtk::Separator::new(gtk::Orientation::Vertical));

    let play = gtk::ToggleButton::builder()
        .icon_name("media-playback-start-symbolic")
        .tooltip_text("Play or pause the transport")
        .build();
    let stop = gtk::Button::builder()
        .icon_name("media-playback-stop-symbolic")
        .tooltip_text("Stop")
        .build();
    let record = gtk::ToggleButton::builder()
        .icon_name("media-record-symbolic")
        .tooltip_text("Record")
        .build();
    record.add_css_class("destructive-action");
    controls.append(&play);
    controls.append(&stop);
    controls.append(&record);

    let tempo = gtk::SpinButton::with_range(1.0, 999.0, 0.1);
    tempo.set_value(controller.borrow().session().tempo_map[0].bpm);
    tempo.set_digits(1);
    tempo.set_width_chars(6);
    tempo.set_tooltip_text(Some("Tempo in beats per minute"));
    bind_tempo(&tempo, controller);
    controls.append(&tempo);
    controls.append(&gtk::Label::new(Some("BPM")));

    let metronome = gtk::ToggleButton::builder()
        .icon_name("audio-volume-high-symbolic")
        .tooltip_text("Metronome")
        .build();
    controls.append(&metronome);
    controls.append(&gtk::Separator::new(gtk::Orientation::Vertical));

    let position = gtk::Label::new(Some("1:01:000"));
    position.add_css_class("title-3");
    position.set_tooltip_text(Some("Song position"));
    controls.append(&position);
    controls.append(&gtk::Separator::new(gtk::Orientation::Vertical));

    controls.append(&gtk::Label::new(Some("Snap")));
    let snap = gtk::DropDown::from_strings(&["Line", "Beat", "1/2 beat", "Step", "None"]);
    snap.set_selected(3);
    controls.append(&snap);
    controls.append(&build_pattern_selector(
        controller,
        channel_rack_bindings,
        pattern_selector_bindings,
    ));
    let mass_select = gtk::ToggleButton::with_label("MS");
    mass_select.set_tooltip_text(Some(
        "Mass select Playlist clips: drag a rectangle, then press Delete",
    ));
    mass_select.connect_toggled({
        let playlist_bindings = playlist_bindings.clone();
        move |button| playlist_bindings.set_mass_select(button.is_active())
    });
    controls.append(&mass_select);
    let inline_views = gtk::Box::new(gtk::Orientation::Horizontal, 4);
    inline_views.append(&gtk::Separator::new(gtk::Orientation::Vertical));
    inline_views.append(&build_window_toggle_buttons());
    controls.append(&inline_views);

    let spacer = gtk::Box::new(gtk::Orientation::Horizontal, 0);
    spacer.set_hexpand(true);
    controls.append(&spacer);
    let diagnostics = gtk::Label::new(Some("CPU 0% · 0 MB"));
    diagnostics.add_css_class("dim-label");
    controls.append(&diagnostics);

    let primary = gtk::ScrolledWindow::new();
    primary.set_policy(gtk::PolicyType::Automatic, gtk::PolicyType::Never);
    primary.set_propagate_natural_height(true);
    primary.set_child(Some(&controls));

    let overflow_views = gtk::Box::new(gtk::Orientation::Horizontal, 4);
    overflow_views.set_margin_top(4);
    overflow_views.set_margin_bottom(4);
    overflow_views.set_margin_start(8);
    overflow_views.set_margin_end(8);
    overflow_views.set_halign(gtk::Align::Center);
    overflow_views.add_css_class("toolbar");
    overflow_views.append(&build_window_toggle_buttons());
    overflow_views.set_visible(false);

    let runtime = Rc::new(RefCell::new(UiTransportRuntime::default()));
    play.connect_toggled({
        let controller = controller.clone();
        let runtime = runtime.clone();
        let window = window.clone();
        let metronome = metronome.clone();
        let record = record.clone();
        let audio_preference = audio_preference.clone();
        move |button| {
            if !button.is_active() {
                let mut runtime = runtime.borrow_mut();
                if let Some(audio) = runtime.audio.as_mut()
                    && audio.status().audio.state == RealtimeState::Playing
                {
                    let _ = audio.controller_mut().send(AudioCommand::Pause);
                }
                return;
            }
            let result = compile_current_plan(&controller);
            match result {
                Ok(plan) => {
                    let mut runtime = runtime.borrow_mut();
                    let needs_snapshot = runtime
                        .audio
                        .as_ref()
                        .is_none_or(|audio| audio.status().audio.state == RealtimeState::Stopped);
                    if needs_snapshot {
                        let rendered = match radiant_engine::render_offline(
                            &plan,
                            512,
                            RenderOptions {
                                metronome: metronome.is_active(),
                                ..RenderOptions::default()
                            },
                        ) {
                            Ok(rendered) => rendered,
                            Err(error) => {
                                button.set_active(false);
                                show_project_error(&window, &error.to_string());
                                return;
                            }
                        };
                        runtime.audio = match NativeAudioService::start(
                            Arc::new(rendered),
                            audio_preference.get(),
                        ) {
                            Ok(audio) => Some(audio),
                            Err(error) => {
                                button.set_active(false);
                                show_project_error(&window, &error.to_string());
                                return;
                            }
                        };
                        if let Some(audio) = runtime.audio.as_mut() {
                            let (input_latency, output_latency) = {
                                let session = controller.borrow();
                                (
                                    session.session().input_latency_frames,
                                    session.session().output_latency_frames,
                                )
                            };
                            audio.set_calibration(radiant_audio::LatencyCalibration::new(
                                input_latency,
                                output_latency,
                            ));
                        }
                    }
                    if let Some(audio) = runtime.audio.as_mut() {
                        let _ = audio
                            .controller_mut()
                            .send(AudioCommand::SetRecording(record.is_active()));
                        let _ = audio.controller_mut().send(AudioCommand::Play);
                    }
                    runtime.plan = Some(plan);
                }
                Err(error) => {
                    button.set_active(false);
                    show_project_error(&window, &error);
                }
            }
        }
    });
    if std::env::var_os("RADIANT_AUTOPLAY_TEST").is_some() {
        let play = play.clone();
        gtk::glib::timeout_add_local_once(std::time::Duration::from_millis(500), move || {
            eprintln!("[radiant-playback-test] activating Play");
            play.set_active(true);
        });
    }
    stop.connect_clicked({
        let runtime = runtime.clone();
        let play = play.clone();
        let position = position.clone();
        let playlist_bindings = playlist_bindings.clone();
        move |_| {
            if let Some(audio) = runtime.borrow_mut().audio.as_mut() {
                let _ = audio.controller_mut().send(AudioCommand::Stop);
            }
            play.set_active(false);
            position.set_label("1:01:000");
            playlist_bindings.set_playhead_bars(0.0);
        }
    });
    gtk::glib::timeout_add_local(std::time::Duration::from_millis(30), {
        let runtime = runtime.clone();
        let play = play.clone();
        let position = position.clone();
        let diagnostics = diagnostics.clone();
        let master_meters = mixer_bindings.master_meters.clone();
        let playlist_bindings = playlist_bindings.clone();
        move || {
            let update = {
                let mut runtime = runtime.borrow_mut();
                if let Some(capture) = runtime.capture.take() {
                    capture.drain(&mut runtime.capture_buffer, 8_192);
                    runtime.capture = Some(capture);
                }
                if !runtime.capture_buffer.is_empty() {
                    let samples = std::mem::take(&mut runtime.capture_buffer);
                    if let Some(take) = runtime.take.as_mut() {
                        let _ = take.append_interleaved(&samples);
                    }
                }
                if runtime
                    .audio
                    .as_ref()
                    .is_some_and(NativeAudioService::requires_restart)
                    && runtime.last_restart_attempt.is_none_or(|attempt| {
                        attempt.elapsed() >= std::time::Duration::from_secs(1)
                    })
                {
                    runtime.last_restart_attempt = Some(std::time::Instant::now());
                    if let Some(audio) = runtime.audio.as_mut() {
                        let _ = audio.restart();
                    }
                }
                let plan = runtime.plan.clone();
                plan.and_then(|plan| {
                    runtime.audio.as_ref().map(|audio| {
                        let status = audio.status();
                        let playhead_tick =
                            plan.timeline().frame_to_tick(status.audio.playhead_frame);
                        let musical = plan.timeline().tick_to_position(playhead_tick);
                        (
                            format!("{}:{:02}:{:03}", musical.bar, musical.beat, musical.tick),
                            status,
                            playlist_ticks_to_bars(playhead_tick),
                        )
                    })
                })
            };
            if let Some((label, status, playhead_bars)) = update {
                playlist_bindings.set_playhead_bars(playhead_bars);
                if std::env::var_os("RADIANT_AUTOPLAY_TEST").is_some() {
                    eprintln!(
                        "[radiant-playback-test] position={label} state={:?} backend={:?} quantum={} xruns={} deadline_misses={} callbacks={} faulted={} peak={:.3}",
                        status.audio.state,
                        status.backend,
                        status.quantum,
                        status.audio.xruns,
                        status.audio.deadline_misses,
                        status.audio.callback_cycles,
                        status.audio.faulted,
                        status.audio.peak_left.max(status.audio.peak_right),
                    );
                }
                position.set_label(&label);
                diagnostics.set_label(&format!(
                    "{:?} · {} Hz · Q{} · XRuns {} · Peak {:.2}{}",
                    status
                        .backend
                        .unwrap_or(radiant_audio::NativeBackend::PipeWire),
                    status.sample_rate,
                    status.quantum,
                    status.audio.xruns,
                    status.audio.peak_left.max(status.audio.peak_right),
                    if status.audio.clipped { " · CLIP" } else { "" },
                ));
                for (meter, value) in master_meters
                    .borrow()
                    .iter()
                    .zip([status.audio.peak_left, status.audio.peak_right])
                {
                    meter.set_value(f64::from(value));
                }
                if status.audio.state == RealtimeState::Stopped && play.is_active() {
                    play.set_active(false);
                }
            }
            gtk::glib::ControlFlow::Continue
        }
    });
    record.connect_toggled({
        let runtime = runtime.clone();
        let controller = controller.clone();
        let window = window.clone();
        move |record| {
            if record.is_active() {
                let (sample_rate, channels) =
                    runtime
                        .borrow()
                        .audio
                        .as_ref()
                        .map_or((48_000, 2), |audio| {
                            let status = audio.status();
                            (status.sample_rate, status.channels)
                        });
                let take = controller.borrow().begin_take(sample_rate, channels);
                let capture = NativeCapture::start();
                match (take, capture) {
                    (Ok(take), Ok(capture)) => {
                        let mut runtime = runtime.borrow_mut();
                        runtime.take = Some(take);
                        runtime.capture = Some(capture);
                    }
                    (Err(error), _) => {
                        record.set_active(false);
                        show_project_error(&window, &error.to_string());
                    }
                    (_, Err(error)) => {
                        record.set_active(false);
                        show_project_error(&window, &error.to_string());
                    }
                }
            } else {
                let mut runtime = runtime.borrow_mut();
                if let Some(capture) = runtime.capture.take() {
                    capture.drain(&mut runtime.capture_buffer, usize::MAX);
                }
                if let Some(mut take) = runtime.take.take() {
                    if !runtime.capture_buffer.is_empty() {
                        let samples = std::mem::take(&mut runtime.capture_buffer);
                        let _ = take.append_interleaved(&samples);
                    }
                    match take.finalize("take") {
                        Ok(media) => {
                            let edit = Transaction::one(
                                "Add recorded take",
                                Command::AddMediaSource { media },
                            );
                            if let Err(error) = controller.borrow_mut().execute(&edit) {
                                show_project_error(&window, &error.to_string());
                            }
                        }
                        Err(error) => show_project_error(&window, &error.to_string()),
                    }
                }
            }
            if let Some(audio) = runtime.borrow_mut().audio.as_mut() {
                let _ = audio
                    .controller_mut()
                    .send(AudioCommand::SetRecording(record.is_active()));
            }
        }
    });

    TransportBars {
        primary,
        inline_views,
        overflow_views,
    }
}

fn compile_current_plan(
    controller: &Rc<RefCell<AppController>>,
) -> Result<Arc<RenderPlan>, String> {
    let controller = controller.borrow();
    let root = controller.project_root().unwrap_or_else(|| Path::new("."));
    let media =
        MediaPool::load_session(controller.session(), root).map_err(|error| error.to_string())?;
    RenderPlan::compile(controller.session(), &media).map_err(|error| error.to_string())
}

fn bind_tempo(tempo: &gtk::SpinButton, controller: &Rc<RefCell<AppController>>) {
    tempo.connect_value_changed({
        let controller = controller.clone();
        move |tempo| {
            let transaction = Transaction::one(
                "Adjust tempo",
                Command::SetInitialTempo { bpm: tempo.value() },
            );
            if let Ok(revision) = controller
                .borrow_mut()
                .execute_coalesced("transport-tempo", &transaction)
            {
                schedule_end_coalescing(&controller, revision);
            }
        }
    });
}

#[derive(Clone)]
struct PatternSelectorBindings {
    syncing: Rc<Cell<bool>>,
    model: gtk::StringList,
    dropdown: gtk::DropDown,
}

impl PatternSelectorBindings {
    fn sync(&self, controller: &AppController) {
        self.syncing.set(true);
        let names: Vec<_> = controller
            .session()
            .patterns
            .iter()
            .map(|pattern| pattern.name.as_str())
            .collect();
        self.model.splice(0, self.model.n_items(), &names);
        let selected = controller
            .selection()
            .pattern
            .and_then(|id| {
                controller
                    .session()
                    .patterns
                    .iter()
                    .position(|pattern| pattern.id == id)
            })
            .unwrap_or(0);
        self.dropdown
            .set_selected(u32::try_from(selected).unwrap_or(gtk::INVALID_LIST_POSITION));
        self.syncing.set(false);
    }
}

fn build_pattern_selector(
    controller: &Rc<RefCell<AppController>>,
    channel_rack_bindings: &ChannelRackBindings,
    binding_slot: &Rc<RefCell<Option<PatternSelectorBindings>>>,
) -> gtk::Widget {
    let controls = gtk::Box::new(gtk::Orientation::Horizontal, 4);
    let pattern_model = gtk::StringList::new(&[]);
    for pattern in &controller.borrow().session().patterns {
        pattern_model.append(&pattern.name);
    }
    let pattern = gtk::DropDown::new(Some(pattern_model.clone()), None::<gtk::Expression>);
    let bindings = PatternSelectorBindings {
        syncing: Rc::new(Cell::new(false)),
        model: pattern_model.clone(),
        dropdown: pattern.clone(),
    };
    *binding_slot.borrow_mut() = Some(bindings.clone());
    pattern.set_tooltip_text(Some("Current pattern"));
    let selector_syncing = bindings.syncing.clone();
    pattern.connect_selected_notify({
        let controller = controller.clone();
        let bindings = channel_rack_bindings.clone();
        move |dropdown| {
            if selector_syncing.get() {
                return;
            }
            let selected = usize::try_from(dropdown.selected()).ok();
            let pattern_id = selected.and_then(|index| {
                controller
                    .borrow()
                    .session()
                    .patterns
                    .get(index)
                    .map(|pattern| pattern.id)
            });
            controller.borrow_mut().selection_mut().pattern = pattern_id;
            bindings.sync(&controller.borrow());
        }
    });
    controls.append(&pattern);
    let add_pattern = gtk::Button::builder()
        .icon_name("list-add-symbolic")
        .tooltip_text("Add Pattern")
        .build();
    add_pattern.connect_clicked({
        let controller = controller.clone();
        let pattern_model = pattern_model.clone();
        let pattern = pattern.clone();
        let bindings = channel_rack_bindings.clone();
        move |_| {
            let number = controller.borrow().session().patterns.len() + 1;
            let mut new_pattern = Pattern::new(format!("Pattern {number}"), Tick::beats(4));
            new_pattern.lanes = controller
                .borrow()
                .session()
                .channels
                .iter()
                .map(|channel| PatternLane {
                    channel_id: channel.id,
                    steps: Vec::new(),
                    notes: Vec::new(),
                })
                .collect();
            let pattern_id = new_pattern.id;
            let name = new_pattern.name.clone();
            let edit = Transaction::one(
                "Add Pattern",
                Command::AddPattern {
                    pattern: new_pattern,
                },
            );
            if controller.borrow_mut().execute(&edit).is_ok() {
                controller.borrow_mut().selection_mut().pattern = Some(pattern_id);
                pattern_model.append(&name);
                pattern.set_selected(u32::try_from(number - 1).unwrap_or(u32::MAX));
                bindings.sync(&controller.borrow());
            }
        }
    });
    controls.append(&add_pattern);
    controls.upcast()
}

fn build_window_toggle_buttons() -> gtk::Box {
    let buttons = gtk::Box::new(gtk::Orientation::Horizontal, 2);
    for (label, tooltip, action) in [
        ("BR", "Browser (Ctrl+B)", "win.toggle-browser"),
        ("PL", "Playlist (F5)", "win.toggle-playlist"),
        ("CR", "Channel Rack (F6)", "win.toggle-channel-rack"),
        ("PR", "Piano Roll (F7)", "win.open-piano-roll"),
        ("MX", "Mixer (F9)", "win.toggle-mixer"),
        ("PK", "Picker (Ctrl+Shift+P)", "win.toggle-picker"),
    ] {
        let button = gtk::Button::with_label(label);
        button.set_tooltip_text(Some(tooltip));
        button.set_action_name(Some(action));
        buttons.append(&button);
    }
    buttons
}

fn build_status_bar() -> gtk::ActionBar {
    let bar = gtk::ActionBar::new();
    let status = gtk::Label::new(Some(
        "Ready · Native PipeWire/JACK audio · ALSA Sequencer MIDI",
    ));
    status.add_css_class("dim-label");
    status.set_xalign(0.0);
    bar.pack_start(&status);
    bar
}

fn install_window_actions(
    application: &adw::Application,
    window: &adw::ApplicationWindow,
    sections: &WorkspaceSections,
    controller: &Rc<RefCell<AppController>>,
) {
    add_browser_action(window, sections, application, controller);
    add_visibility_action(
        window,
        "toggle-playlist",
        &sections.playlist,
        sections,
        application,
        ("F5", WorkspacePanel::Playlist),
        controller,
    );
    add_visibility_action(
        window,
        "toggle-channel-rack",
        &sections.channel_rack,
        sections,
        application,
        ("F6", WorkspacePanel::ChannelRack),
        controller,
    );
    add_visibility_action(
        window,
        "toggle-mixer",
        &sections.mixer,
        sections,
        application,
        ("F9", WorkspacePanel::Mixer),
        controller,
    );
    add_visibility_action(
        window,
        "toggle-picker",
        &sections.picker,
        sections,
        application,
        ("<Control><Shift>p", WorkspacePanel::Picker),
        controller,
    );
    add_history_actions(application, window, sections, controller);
    add_project_actions(application, window, sections, controller);
    add_export_action(application, window, controller);
    install_close_guard(window, sections, controller);
    install_action_state_monitor(window, controller);

    let piano_roll = gio::SimpleAction::new("open-piano-roll", None);
    let application_for_piano = application.clone();
    let piano_controller = controller.clone();
    piano_roll.connect_activate(move |_, _| {
        open_piano_roll(&application_for_piano, &piano_controller);
    });
    window.add_action(&piano_roll);
    application.set_accels_for_action("win.open-piano-roll", &["F7"]);

    let maximize = gio::SimpleAction::new("maximize", None);
    let target = window.clone();
    maximize.connect_activate(move |_, _| {
        if target.is_maximized() {
            target.unmaximize();
        } else {
            target.maximize();
        }
    });
    window.add_action(&maximize);
    application.set_accels_for_action("win.maximize", &["F11"]);
}

fn add_history_actions(
    application: &adw::Application,
    window: &adw::ApplicationWindow,
    sections: &WorkspaceSections,
    controller: &Rc<RefCell<AppController>>,
) {
    let undo = gio::SimpleAction::new("undo", None);
    let undo_controller = Rc::clone(controller);
    let undo_sections = sections.clone();
    undo.connect_activate(move |_, _| {
        if matches!(undo_controller.borrow_mut().undo(), Ok(true)) {
            undo_sections.sync_from_controller(&undo_controller);
        }
    });
    window.add_action(&undo);
    application.set_accels_for_action("win.undo", &["<Control>z"]);

    let redo = gio::SimpleAction::new("redo", None);
    let redo_controller = Rc::clone(controller);
    let redo_sections = sections.clone();
    redo.connect_activate(move |_, _| {
        if matches!(redo_controller.borrow_mut().redo(), Ok(true)) {
            redo_sections.sync_from_controller(&redo_controller);
        }
    });
    window.add_action(&redo);
    application.set_accels_for_action("win.redo", &["<Control><Shift>z", "<Control>y"]);
}

fn install_action_state_monitor(
    window: &adw::ApplicationWindow,
    controller: &Rc<RefCell<AppController>>,
) {
    sync_action_state(window, &controller.borrow());
    let window = window.downgrade();
    let controller = controller.clone();
    gtk::glib::timeout_add_local(std::time::Duration::from_millis(100), move || {
        let Some(window) = window.upgrade() else {
            return gtk::glib::ControlFlow::Break;
        };
        sync_action_state(&window, &controller.borrow());
        gtk::glib::ControlFlow::Continue
    });
}

fn sync_action_state(window: &adw::ApplicationWindow, controller: &AppController) {
    let (can_undo, can_redo) = history_action_state(controller);
    if let Some(action) = window
        .lookup_action("undo")
        .and_downcast::<gio::SimpleAction>()
    {
        action.set_enabled(can_undo);
    }
    if let Some(action) = window
        .lookup_action("redo")
        .and_downcast::<gio::SimpleAction>()
    {
        action.set_enabled(can_redo);
    }
    update_window_title(window, controller);
}

type UiContinuation = Rc<dyn Fn()>;

fn add_project_actions(
    application: &adw::Application,
    window: &adw::ApplicationWindow,
    sections: &WorkspaceSections,
    controller: &Rc<RefCell<AppController>>,
) {
    let new_project = gio::SimpleAction::new("new-project", None);
    new_project.connect_activate({
        let window = window.clone();
        let sections = sections.clone();
        let controller = controller.clone();
        move |_, _| {
            let reset: UiContinuation = Rc::new({
                let window = window.clone();
                let sections = sections.clone();
                let controller = controller.clone();
                move || {
                    if let Err(error) = controller
                        .borrow_mut()
                        .replace_with_new(Session::new("Untitled"))
                    {
                        show_project_error(&window, &error.to_string());
                        return;
                    }
                    sections.sync_from_controller(&controller);
                    update_window_title(&window, &controller.borrow());
                }
            });
            confirm_dirty_then(&window, &sections, &controller, reset);
        }
    });
    window.add_action(&new_project);
    application.set_accels_for_action("win.new-project", &["<Control>n"]);

    let open_project = gio::SimpleAction::new("open-project", None);
    open_project.connect_activate({
        let window = window.clone();
        let sections = sections.clone();
        let controller = controller.clone();
        move |_, _| {
            let open: UiContinuation = Rc::new({
                let window = window.clone();
                let sections = sections.clone();
                let controller = controller.clone();
                move || choose_project_to_open(&window, &sections, &controller)
            });
            confirm_dirty_then(&window, &sections, &controller, open);
        }
    });
    window.add_action(&open_project);
    application.set_accels_for_action("win.open-project", &["<Control>o"]);

    let save_project = gio::SimpleAction::new("save-project", None);
    save_project.connect_activate({
        let window = window.clone();
        let controller = controller.clone();
        move |_, _| save_project_to_disk(&window, &controller, false, None)
    });
    window.add_action(&save_project);
    application.set_accels_for_action("win.save-project", &["<Control>s"]);

    let save_project_as = gio::SimpleAction::new("save-project-as", None);
    save_project_as.connect_activate({
        let window = window.clone();
        let controller = controller.clone();
        move |_, _| save_project_to_disk(&window, &controller, true, None)
    });
    window.add_action(&save_project_as);
    application.set_accels_for_action("win.save-project-as", &["<Control><Shift>s"]);
}

fn add_export_action(
    application: &adw::Application,
    window: &adw::ApplicationWindow,
    controller: &Rc<RefCell<AppController>>,
) {
    let export = gio::SimpleAction::new("export-audio", None);
    export.connect_activate({
        let window = window.clone();
        let controller = controller.clone();
        move |_, _| {
            let initial_name = format!(
                "{}.wav",
                actions::project_directory_name(&controller.borrow().session().name)
                    .trim_end_matches(".radiant")
            );
            let dialog = gtk::FileDialog::builder()
                .title("Export Radiant Audio")
                .initial_name(&initial_name)
                .modal(true)
                .build();
            let window_for_result = window.clone();
            let controller = controller.clone();
            dialog.save(Some(&window), None::<&gio::Cancellable>, move |result| {
                let Ok(file) = result else {
                    return;
                };
                let Some(path) = file.path() else {
                    show_project_error(
                        &window_for_result,
                        "The selected export destination has no local path.",
                    );
                    return;
                };
                let plan = match compile_current_plan(&controller) {
                    Ok(plan) => plan,
                    Err(error) => {
                        show_project_error(&window_for_result, &error);
                        return;
                    }
                };
                if let Err(error) =
                    radiant_engine::export_wav(&plan, path, 512, RenderOptions::default())
                {
                    show_project_error(&window_for_result, &error.to_string());
                }
            });
        }
    });
    window.add_action(&export);
    application.set_accels_for_action("win.export-audio", &["<Control><Shift>e"]);
}

fn choose_project_to_open(
    window: &adw::ApplicationWindow,
    sections: &WorkspaceSections,
    controller: &Rc<RefCell<AppController>>,
) {
    let dialog = gtk::FileDialog::builder()
        .title("Open Radiant Project Folder")
        .modal(true)
        .build();
    let window_for_result = window.clone();
    let sections = sections.clone();
    let controller = controller.clone();
    dialog.select_folder(Some(window), None::<&gio::Cancellable>, move |result| {
        let Ok(folder) = result else {
            return;
        };
        let Some(path) = folder.path() else {
            show_project_error(&window_for_result, "The selected folder has no local path.");
            return;
        };
        if let Err(error) = controller.borrow_mut().replace_with_open(path) {
            show_project_error(&window_for_result, &error.to_string());
            return;
        }
        sections.sync_from_controller(&controller);
        update_window_title(&window_for_result, &controller.borrow());
    });
}

fn save_project_to_disk(
    window: &adw::ApplicationWindow,
    controller: &Rc<RefCell<AppController>>,
    save_as: bool,
    after: Option<UiContinuation>,
) {
    if !save_as && controller.borrow().project_root().is_some() {
        match controller.borrow_mut().save() {
            Ok(_) => {
                update_window_title(window, &controller.borrow());
                if let Some(after) = after {
                    after();
                }
            }
            Err(error) => show_project_error(window, &error.to_string()),
        }
        return;
    }
    let dialog = gtk::FileDialog::builder()
        .title("Choose Where to Create the Radiant Project")
        .modal(true)
        .build();
    let window_for_result = window.clone();
    let controller = controller.clone();
    dialog.select_folder(Some(window), None::<&gio::Cancellable>, move |result| {
        let Ok(folder) = result else {
            return;
        };
        let Some(parent) = folder.path() else {
            show_project_error(&window_for_result, "The selected folder has no local path.");
            return;
        };
        let project_name = project_directory_name(&controller.borrow().session().name);
        let root = parent.join(project_name);
        if let Err(error) = controller.borrow_mut().create_project(root) {
            show_project_error(&window_for_result, &error.to_string());
            return;
        }
        update_window_title(&window_for_result, &controller.borrow());
        if let Some(after) = after {
            after();
        }
    });
}

fn confirm_dirty_then(
    window: &adw::ApplicationWindow,
    _sections: &WorkspaceSections,
    controller: &Rc<RefCell<AppController>>,
    continuation: UiContinuation,
) {
    if !controller.borrow().is_dirty() {
        if let Err(error) = controller.borrow_mut().persist_presentation() {
            show_project_error(window, &error.to_string());
            return;
        }
        continuation();
        return;
    }
    let dialog = adw::AlertDialog::builder()
        .heading("Save changes to this project?")
        .body("Unsaved musical edits will be lost if you continue.")
        .build();
    dialog.add_responses(&[
        ("cancel", "Cancel"),
        ("discard", "Discard"),
        ("save", "Save"),
    ]);
    dialog.set_close_response("cancel");
    dialog.set_default_response(Some("save"));
    dialog.set_response_appearance("discard", adw::ResponseAppearance::Destructive);
    let window_for_response = window.clone();
    let controller = controller.clone();
    dialog.choose(
        Some(window),
        None::<&gio::Cancellable>,
        move |response| match response.as_str() {
            "discard" => {
                if let Err(error) = controller.borrow_mut().persist_presentation() {
                    show_project_error(&window_for_response, &error.to_string());
                } else {
                    continuation();
                }
            }
            "save" => {
                save_project_to_disk(&window_for_response, &controller, false, Some(continuation));
            }
            _ => {}
        },
    );
}

fn install_close_guard(
    window: &adw::ApplicationWindow,
    sections: &WorkspaceSections,
    controller: &Rc<RefCell<AppController>>,
) {
    let close_allowed = Rc::new(Cell::new(false));
    window.connect_close_request({
        let window = window.clone();
        let sections = sections.clone();
        let controller = controller.clone();
        let close_allowed = close_allowed.clone();
        move |_| {
            if close_allowed.get() {
                return gtk::glib::Propagation::Proceed;
            }
            if !controller.borrow().is_dirty() {
                if let Err(error) = controller.borrow_mut().persist_presentation() {
                    show_project_error(&window, &error.to_string());
                    return gtk::glib::Propagation::Stop;
                }
                return gtk::glib::Propagation::Proceed;
            }
            let close: UiContinuation = Rc::new({
                let window = window.clone();
                let close_allowed = close_allowed.clone();
                move || {
                    close_allowed.set(true);
                    window.close();
                }
            });
            confirm_dirty_then(&window, &sections, &controller, close);
            gtk::glib::Propagation::Stop
        }
    });
}

fn update_window_title(window: &adw::ApplicationWindow, controller: &AppController) {
    let dirty = if controller.is_dirty() { " •" } else { "" };
    window.set_title(Some(&format!(
        "Radiant — {}{dirty}",
        controller.session().name
    )));
}

fn show_project_error(window: &adw::ApplicationWindow, message: &str) {
    let dialog = adw::AlertDialog::builder()
        .heading("Project operation failed")
        .body(message)
        .build();
    dialog.add_response("close", "Close");
    dialog.present(Some(window));
}

fn add_visibility_action(
    window: &adw::ApplicationWindow,
    name: &str,
    widget: &impl IsA<gtk::Widget>,
    sections: &WorkspaceSections,
    application: &adw::Application,
    binding: (&str, WorkspacePanel),
    controller: &Rc<RefCell<AppController>>,
) {
    let (accelerator, panel) = binding;
    let target = widget.as_ref().clone();
    let action = gio::SimpleAction::new_stateful(name, None, &target.is_visible().to_variant());
    let sections = sections.clone();
    let controller = controller.clone();
    action.connect_activate(move |action, _| {
        let visible = !target.is_visible();
        target.set_visible(visible);
        sections.sync_layout_visibility();
        let mut controller = controller.borrow_mut();
        if visible {
            controller.presentation_mut().visible_panels.insert(panel);
        } else {
            controller.presentation_mut().visible_panels.remove(&panel);
        }
        action.set_state(&visible.to_variant());
    });
    window.add_action(&action);
    application.set_accels_for_action(&format!("win.{name}"), &[accelerator]);
}

fn add_browser_action(
    window: &adw::ApplicationWindow,
    sections: &WorkspaceSections,
    application: &adw::Application,
    controller: &Rc<RefCell<AppController>>,
) {
    let sidebar = sections.sidebar.clone();
    let action = gio::SimpleAction::new_stateful(
        "toggle-browser",
        None,
        &sidebar.shows_sidebar().to_variant(),
    );
    let controller = controller.clone();
    action.connect_activate(move |action, _| {
        let visible = !sidebar.shows_sidebar();
        sidebar.set_show_sidebar(visible);
        let mut controller = controller.borrow_mut();
        if visible {
            controller
                .presentation_mut()
                .visible_panels
                .insert(WorkspacePanel::Browser);
        } else {
            controller
                .presentation_mut()
                .visible_panels
                .remove(&WorkspacePanel::Browser);
        }
        action.set_state(&visible.to_variant());
    });
    let action_for_notify = action.clone();
    sections
        .sidebar
        .connect_show_sidebar_notify(move |sidebar| {
            action_for_notify.set_state(&sidebar.shows_sidebar().to_variant());
        });
    window.add_action(&action);
    application.set_accels_for_action("win.toggle-browser", &["<Control>b"]);
}

fn install_breakpoints(
    window: &adw::ApplicationWindow,
    sections: &WorkspaceSections,
    transport_bars: &TransportBars,
) {
    let wrapped_toolbar = adw::Breakpoint::new(
        adw::BreakpointCondition::parse("max-width: 1180px")
            .expect("valid toolbar wrap breakpoint"),
    );
    wrapped_toolbar.add_setter(
        &transport_bars.inline_views,
        "visible",
        Some(&false.to_value()),
    );
    wrapped_toolbar.add_setter(
        &transport_bars.overflow_views,
        "visible",
        Some(&true.to_value()),
    );
    window.add_breakpoint(wrapped_toolbar);

    let compact = adw::Breakpoint::new(
        adw::BreakpointCondition::parse("max-width: 900px").expect("valid compact breakpoint"),
    );
    compact.add_setter(&sections.sidebar, "collapsed", Some(&true.to_value()));
    compact.add_setter(
        &sections.primary_split,
        "orientation",
        Some(&gtk::Orientation::Vertical.to_value()),
    );
    window.add_breakpoint(compact);
}

fn section_frame(title: &str, content: impl IsA<gtk::Widget>, close_action: &str) -> gtk::Frame {
    let frame = gtk::Frame::new(None);
    let layout = gtk::Box::new(gtk::Orientation::Vertical, 0);
    let title_bar = gtk::Box::new(gtk::Orientation::Horizontal, 6);
    let heading = gtk::Label::new(Some(title));
    heading.add_css_class("heading");
    heading.set_xalign(0.0);
    heading.set_hexpand(true);
    heading.set_margin_top(8);
    heading.set_margin_bottom(8);
    heading.set_margin_start(12);
    title_bar.append(&heading);
    let close = gtk::Button::builder()
        .icon_name("window-close-symbolic")
        .tooltip_text(format!("Close {title}"))
        .action_name(close_action)
        .margin_end(6)
        .valign(gtk::Align::Center)
        .build();
    close.add_css_class("flat");
    title_bar.append(&close);
    layout.append(&title_bar);
    layout.append(&gtk::Separator::new(gtk::Orientation::Horizontal));
    content.as_ref().set_hexpand(true);
    content.as_ref().set_vexpand(true);
    layout.append(&content);
    frame.set_child(Some(&layout));
    frame
}

fn build_browser() -> gtk::Widget {
    let container = gtk::Box::new(gtk::Orientation::Vertical, 6);
    container.add_css_class("sidebar");
    container.set_margin_top(8);
    container.set_margin_bottom(8);
    container.set_margin_start(8);
    container.set_margin_end(8);

    let heading = gtk::Box::new(gtk::Orientation::Horizontal, 6);
    let title = gtk::Label::new(Some("Browser"));
    title.add_css_class("title-3");
    title.set_xalign(0.0);
    title.set_hexpand(true);
    heading.append(&title);
    let close = gtk::Button::builder()
        .icon_name("window-close-symbolic")
        .tooltip_text("Close Browser")
        .action_name("win.toggle-browser")
        .build();
    close.add_css_class("flat");
    heading.append(&close);
    container.append(&heading);
    let search = gtk::SearchEntry::new();
    search.set_placeholder_text(Some("Search sounds and plugins"));
    container.append(&search);

    let strings = gtk::StringList::new(&[
        "Current Project",
        "Plugin Database",
        "Packs",
        "Samples",
        "Presets",
        "Recorded",
        "Rendered",
    ]);
    let selection = gtk::SingleSelection::new(Some(strings));
    let factory = gtk::SignalListItemFactory::new();
    factory.connect_setup(|_, item| {
        let item = item.downcast_ref::<gtk::ListItem>().expect("list item");
        let label = gtk::Label::new(None);
        label.set_xalign(0.0);
        label.set_margin_top(8);
        label.set_margin_bottom(8);
        label.set_margin_start(8);
        label.set_margin_end(8);
        item.set_child(Some(&label));
    });
    factory.connect_bind(|_, item| {
        let item = item.downcast_ref::<gtk::ListItem>().expect("list item");
        let label = item
            .child()
            .and_downcast::<gtk::Label>()
            .expect("list label");
        let value = item
            .item()
            .and_downcast::<gtk::StringObject>()
            .expect("string item");
        label.set_label(&value.string());
    });
    let list = gtk::ListView::new(Some(selection), Some(factory));
    list.add_css_class("navigation-sidebar");
    let scroller = gtk::ScrolledWindow::new();
    scroller.set_vexpand(true);
    scroller.set_child(Some(&list));
    container.append(&scroller);
    container.upcast()
}

#[derive(Clone)]
struct ChannelRackBindings {
    controller: Rc<RefCell<AppController>>,
    syncing: Rc<Cell<bool>>,
    steps: Rc<RefCell<Vec<ChannelStepBinding>>>,
    controls: Rc<RefCell<Vec<ChannelControlBinding>>>,
    rack: Rc<RefCell<Option<gtk::Grid>>>,
    rendered_pattern: Rc<Cell<Option<PatternId>>>,
    rendered_channels: Rc<RefCell<Vec<ChannelId>>>,
}

impl ChannelRackBindings {
    fn new(controller: &Rc<RefCell<AppController>>) -> Self {
        Self {
            controller: controller.clone(),
            syncing: Rc::new(Cell::new(false)),
            steps: Rc::new(RefCell::new(Vec::new())),
            controls: Rc::new(RefCell::new(Vec::new())),
            rack: Rc::new(RefCell::new(None)),
            rendered_pattern: Rc::new(Cell::new(None)),
            rendered_channels: Rc::new(RefCell::new(Vec::new())),
        }
    }

    fn sync(&self, controller: &AppController) {
        let pattern_id = selected_pattern(controller).map(|pattern| pattern.id);
        let channel_ids: Vec<_> = controller
            .session()
            .channels
            .iter()
            .map(|channel| channel.id)
            .collect();
        if self.rendered_pattern.get() != pattern_id
            || *self.rendered_channels.borrow() != channel_ids
        {
            self.rebuild();
        }
        self.syncing.set(true);
        for binding in self.steps.borrow().iter() {
            let active = controller
                .session()
                .patterns
                .iter()
                .find(|pattern| pattern.id == binding.pattern_id)
                .and_then(|pattern| {
                    pattern
                        .lanes
                        .iter()
                        .find(|lane| lane.channel_id == binding.channel_id)
                })
                .is_some_and(|lane| lane.steps.iter().any(|step| step.at == binding.at));
            binding.button.set_active(active);
            update_step_appearance(&binding.button, binding.group_index);
        }
        for binding in self.controls.borrow().iter() {
            if let Some(channel) = controller
                .session()
                .channels
                .iter()
                .find(|channel| channel.id == binding.channel_id)
            {
                binding.enabled.set_active(!channel.muted);
                binding.pan.set_value(f64::from(channel.pan));
                binding.volume.set_value(f64::from(channel.volume));
                let mixer_index = controller
                    .session()
                    .mixer_inserts
                    .iter()
                    .position(|insert| insert.id == channel.mixer_insert)
                    .and_then(|index| u32::try_from(index).ok())
                    .map_or(0.0, f64::from);
                binding.mixer_track.set_value(mixer_index);
            }
        }
        self.syncing.set(false);
    }

    fn rebuild(&self) {
        let Some(rack) = self.rack.borrow().clone() else {
            return;
        };
        while let Some(child) = rack.first_child() {
            rack.remove(&child);
        }
        self.steps.borrow_mut().clear();
        self.controls.borrow_mut().clear();
        let channels = channel_row_specs(&self.controller);
        let mut channel_group = None;
        for (row, spec) in (0_i32..).zip(channels) {
            build_channel_row(
                &rack,
                row,
                &spec,
                &mut channel_group,
                &self.controller,
                self,
            );
        }
        let controller = self.controller.borrow();
        self.rendered_pattern
            .set(selected_pattern(&controller).map(|pattern| pattern.id));
        *self.rendered_channels.borrow_mut() = controller
            .session()
            .channels
            .iter()
            .map(|channel| channel.id)
            .collect();
    }
}

#[derive(Clone)]
struct ChannelStepBinding {
    pattern_id: PatternId,
    channel_id: ChannelId,
    at: Tick,
    group_index: usize,
    button: gtk::ToggleButton,
}

#[derive(Clone)]
struct ChannelControlBinding {
    channel_id: ChannelId,
    enabled: gtk::CheckButton,
    pan: gtk::ScaleButton,
    volume: gtk::ScaleButton,
    mixer_track: gtk::SpinButton,
}

fn build_channel_rack(
    controller: &Rc<RefCell<AppController>>,
    bindings: &ChannelRackBindings,
) -> gtk::Widget {
    let container = gtk::Box::new(gtk::Orientation::Vertical, 6);
    container.set_margin_top(8);
    container.set_margin_bottom(8);
    container.set_margin_start(8);
    container.set_margin_end(8);

    let controls = gtk::Box::new(gtk::Orientation::Horizontal, 4);
    let filter = gtk::DropDown::from_strings(&["All", "Audio", "Instruments", "Automation"]);
    filter.set_tooltip_text(Some("Channel group filter"));
    controls.append(&filter);
    let spacer = gtk::Box::new(gtk::Orientation::Horizontal, 0);
    spacer.set_hexpand(true);
    controls.append(&spacer);

    let loop_popover = build_loop_type_popover();
    let loop_menu = gtk::MenuButton::builder()
        .icon_name("media-playlist-repeat-symbolic")
        .tooltip_text("Channel Rack loop options (right-click also opens this menu)")
        .popover(&loop_popover)
        .build();
    attach_secondary_click_popover(&loop_menu, &loop_popover);
    controls.append(&loop_menu);
    let options_popover = build_channel_options_popover();
    let options_menu = gtk::MenuButton::builder()
        .icon_name("view-more-symbolic")
        .tooltip_text("Channel Rack options (right-click also opens this menu)")
        .popover(&options_popover)
        .build();
    attach_secondary_click_popover(&options_menu, &options_popover);
    controls.append(&options_menu);
    container.append(&controls);

    let rack = gtk::Grid::builder()
        .column_spacing(4)
        .row_spacing(3)
        .build();
    *bindings.rack.borrow_mut() = Some(rack.clone());
    bindings.rebuild();

    let rack_scroller = gtk::ScrolledWindow::new();
    rack_scroller.set_policy(gtk::PolicyType::Automatic, gtk::PolicyType::Automatic);
    rack_scroller.set_propagate_natural_height(true);
    rack_scroller.set_child(Some(&rack));
    container.append(&rack_scroller);

    let footer = gtk::Box::new(gtk::Orientation::Horizontal, 6);
    let add = gtk::Button::builder()
        .label("Add Channel")
        .icon_name("list-add-symbolic")
        .tooltip_text("Add an instrument, sampler, or controller channel")
        .build();
    add.connect_clicked({
        let controller = controller.clone();
        let bindings = bindings.clone();
        move |_| {
            let channel_number = controller.borrow().session().channels.len() + 1;
            let Some(master_id) = controller
                .borrow()
                .session()
                .master_insert()
                .map(|item| item.id)
            else {
                return;
            };
            let name = format!("Channel {channel_number}");
            let insert = MixerInsert::insert(&name, master_id);
            let channel = Channel::sampler(&name, insert.id);
            let channel_id = channel.id;
            let mixer_insert_id = insert.id;
            let edit = Transaction {
                label: "Add Channel".into(),
                commands: vec![
                    Command::AddMixerInsert { insert },
                    Command::AddChannel { channel },
                ],
            };
            if controller.borrow_mut().execute(&edit).is_ok() {
                let mut controller = controller.borrow_mut();
                controller.selection_mut().channel = Some(channel_id);
                controller.selection_mut().mixer_insert = Some(mixer_insert_id);
            }
            bindings.sync(&controller.borrow());
        }
    });
    footer.append(&add);
    let footer_spacer = gtk::Box::new(gtk::Orientation::Horizontal, 0);
    footer_spacer.set_hexpand(true);
    footer.append(&footer_spacer);
    let length = gtk::Label::new(Some("16 steps"));
    length.add_css_class("dim-label");
    footer.append(&length);
    let loop_point_popover = build_loop_point_popover();
    let loop_point = gtk::MenuButton::builder()
        .label("Step")
        .tooltip_text("Selected channel loop point (right-click also opens this menu)")
        .popover(&loop_point_popover)
        .build();
    attach_secondary_click_popover(&loop_point, &loop_point_popover);
    footer.append(&loop_point);
    container.append(&footer);

    container.upcast()
}

#[derive(Clone)]
struct ChannelRowSpec {
    channel_id: ChannelId,
    pattern_id: PatternId,
    name: String,
    mixer_track: f64,
    active_steps: Vec<usize>,
    active: bool,
    muted: bool,
    volume: f32,
    pan: f32,
}

fn channel_row_specs(controller: &Rc<RefCell<AppController>>) -> Vec<ChannelRowSpec> {
    let controller = controller.borrow();
    let session = controller.session();
    let Some(pattern) = selected_pattern(&controller) else {
        return Vec::new();
    };
    session
        .channels
        .iter()
        .enumerate()
        .map(|(index, channel)| {
            let active_steps = pattern
                .lanes
                .iter()
                .find(|lane| lane.channel_id == channel.id)
                .map_or_else(Vec::new, |lane| {
                    lane.steps
                        .iter()
                        .filter_map(|step| usize::try_from(step.at.0 / 240).ok())
                        .filter(|step| *step < 16)
                        .collect()
                });
            let mixer_track = session
                .mixer_inserts
                .iter()
                .position(|insert| insert.id == channel.mixer_insert)
                .and_then(|position| u32::try_from(position).ok())
                .map_or(0.0, f64::from);
            ChannelRowSpec {
                channel_id: channel.id,
                pattern_id: pattern.id,
                name: channel.name.clone(),
                mixer_track,
                active_steps,
                active: index == 0,
                muted: channel.muted,
                volume: channel.volume,
                pan: channel.pan,
            }
        })
        .collect()
}

fn selected_pattern(controller: &AppController) -> Option<&Pattern> {
    controller
        .selection()
        .pattern
        .and_then(|id| {
            controller
                .session()
                .patterns
                .iter()
                .find(|pattern| pattern.id == id)
        })
        .or_else(|| controller.session().patterns.first())
}

fn build_channel_row(
    rack: &gtk::Grid,
    row: i32,
    spec: &ChannelRowSpec,
    channel_group: &mut Option<gtk::ToggleButton>,
    controller: &Rc<RefCell<AppController>>,
    bindings: &ChannelRackBindings,
) {
    let controls = build_channel_controls(rack, row, spec, controller, bindings);

    let mixer_track = gtk::SpinButton::with_range(0.0, 125.0, 1.0);
    mixer_track.set_value(spec.mixer_track);
    mixer_track.set_digits(0);
    mixer_track.set_width_chars(2);
    mixer_track.set_tooltip_text(Some(&format!("{} Mixer insert", spec.name)));
    mixer_track.connect_value_changed({
        let controller = controller.clone();
        let syncing = bindings.syncing.clone();
        let channel_id = spec.channel_id;
        move |spin| {
            if syncing.get() {
                return;
            }
            let index = usize::try_from(f64_to_i32_round(spin.value())).ok();
            let mixer_insert_id = index.and_then(|index| {
                controller
                    .borrow()
                    .session()
                    .mixer_inserts
                    .get(index)
                    .map(|insert| insert.id)
            });
            let Some(mixer_insert_id) = mixer_insert_id else {
                return;
            };
            let edit = Transaction::one(
                "Assign Channel to Mixer",
                Command::SetChannelMixerInsert {
                    channel_id,
                    mixer_insert_id,
                },
            );
            let _ = controller.borrow_mut().execute(&edit);
        }
    });
    rack.attach(&mixer_track, 3, row, 1, 1);

    let channel = gtk::ToggleButton::with_label(&spec.name);
    channel.set_size_request(116, -1);
    channel.set_tooltip_text(Some(&format!(
        "Select {} and open its instrument",
        spec.name
    )));
    if let Some(group) = channel_group.as_ref() {
        channel.set_group(Some(group));
    } else {
        channel.set_active(true);
        *channel_group = Some(channel.clone());
    }
    channel.connect_toggled({
        let controller = controller.clone();
        let channel_id = spec.channel_id;
        let pattern_id = spec.pattern_id;
        move |button| {
            if button.is_active() {
                let mut controller = controller.borrow_mut();
                controller.selection_mut().channel = Some(channel_id);
                controller.selection_mut().pattern = Some(pattern_id);
            }
        }
    });
    rack.attach(&channel, 4, row, 1, 1);

    let activity = gtk::LevelBar::for_interval(0.0, 1.0);
    activity.set_orientation(gtk::Orientation::Vertical);
    activity.set_value(if spec.active { 0.75 } else { 0.0 });
    activity.set_tooltip_text(Some(&format!("{} activity", spec.name)));
    rack.attach(&activity, 5, row, 1, 1);

    bindings.controls.borrow_mut().push(ChannelControlBinding {
        channel_id: spec.channel_id,
        enabled: controls.enabled,
        pan: controls.pan,
        volume: controls.volume,
        mixer_track: mixer_track.clone(),
    });

    rack.attach(
        &build_step_sequence(
            &spec.active_steps,
            controller,
            spec.pattern_id,
            spec.channel_id,
            bindings,
        ),
        6,
        row,
        1,
        1,
    );
}

struct ChannelControlWidgets {
    enabled: gtk::CheckButton,
    pan: gtk::ScaleButton,
    volume: gtk::ScaleButton,
}

fn build_channel_controls(
    rack: &gtk::Grid,
    row: i32,
    spec: &ChannelRowSpec,
    controller: &Rc<RefCell<AppController>>,
    bindings: &ChannelRackBindings,
) -> ChannelControlWidgets {
    let enabled = gtk::CheckButton::new();
    enabled.set_active(!spec.muted);
    enabled.set_tooltip_text(Some(&format!("Enable {}", spec.name)));
    enabled.connect_toggled({
        let controller = controller.clone();
        let syncing = bindings.syncing.clone();
        let channel_id = spec.channel_id;
        move |button| {
            if syncing.get() {
                return;
            }
            update_channel_muted(&controller, channel_id, !button.is_active());
        }
    });
    rack.attach(&enabled, 0, row, 1, 1);

    let pan = gtk::ScaleButton::new(
        -1.0,
        1.0,
        0.01,
        &["audio-volume-low-symbolic", "audio-volume-high-symbolic"],
    );
    pan.set_value(f64::from(spec.pan));
    pan.set_tooltip_text(Some(&format!("{} pan", spec.name)));
    pan.connect_value_changed({
        let controller = controller.clone();
        let syncing = bindings.syncing.clone();
        let channel_id = spec.channel_id;
        move |_, value| {
            if syncing.get() {
                return;
            }
            update_channel_pan(&controller, channel_id, f64_to_f32(value));
        }
    });
    rack.attach(&pan, 1, row, 1, 1);

    let volume = gtk::ScaleButton::new(
        0.0,
        1.25,
        0.01,
        &[
            "audio-volume-muted-symbolic",
            "audio-volume-low-symbolic",
            "audio-volume-high-symbolic",
        ],
    );
    volume.set_value(f64::from(spec.volume));
    volume.set_tooltip_text(Some(&format!("{} volume", spec.name)));
    volume.connect_value_changed({
        let controller = controller.clone();
        let syncing = bindings.syncing.clone();
        let channel_id = spec.channel_id;
        move |_, value| {
            if syncing.get() {
                return;
            }
            update_channel_volume(&controller, channel_id, f64_to_f32(value));
        }
    });
    rack.attach(&volume, 2, row, 1, 1);

    ChannelControlWidgets {
        enabled,
        pan,
        volume,
    }
}

fn build_step_sequence(
    active_steps: &[usize],
    controller: &Rc<RefCell<AppController>>,
    pattern_id: PatternId,
    channel_id: ChannelId,
    bindings: &ChannelRackBindings,
) -> gtk::Box {
    let sequence = gtk::Box::new(gtk::Orientation::Horizontal, 4);
    for group_index in 0..4 {
        let beat = gtk::Box::new(gtk::Orientation::Horizontal, 1);
        for step_in_group in 0..4 {
            let step_index = group_index * 4 + step_in_group;
            let step = gtk::ToggleButton::new();
            step.set_size_request(22, 28);
            step.set_tooltip_text(Some(&format!("Step {}", step_index + 1)));
            step.set_active(active_steps.contains(&step_index));
            update_step_appearance(&step, group_index);
            let at = Tick(i64::try_from(step_index).expect("step index") * 240);
            bindings.steps.borrow_mut().push(ChannelStepBinding {
                pattern_id,
                channel_id,
                at,
                group_index,
                button: step.clone(),
            });
            step.connect_toggled({
                let controller = controller.clone();
                let syncing = bindings.syncing.clone();
                move |button| {
                    update_step_appearance(button, group_index);
                    if syncing.get() {
                        return;
                    }
                    let value = button.is_active().then_some(Step {
                        at,
                        velocity: 0.8,
                        probability: 1.0,
                        micro_shift: Tick::ZERO,
                        ratchets: 1,
                    });
                    let transaction = Transaction::one(
                        if value.is_some() {
                            "Enable Channel Rack step"
                        } else {
                            "Disable Channel Rack step"
                        },
                        Command::SetPatternStep {
                            pattern_id,
                            channel_id,
                            at,
                            step: value,
                        },
                    );
                    if let Err(error) = controller.borrow_mut().execute(&transaction) {
                        button.set_tooltip_text(Some(&format!("Step edit failed: {error}")));
                    }
                }
            });
            beat.append(&step);
        }
        sequence.append(&beat);
    }
    sequence
}

fn update_channel_volume(
    controller: &Rc<RefCell<AppController>>,
    channel_id: ChannelId,
    volume: f32,
) {
    let (volume, pan, muted) = {
        let controller = controller.borrow();
        let Some(channel) = controller
            .session()
            .channels
            .iter()
            .find(|channel| channel.id == channel_id)
        else {
            return;
        };
        channel_rack::merged_controls(channel, Some(volume), None, None)
    };
    execute_channel_controls(
        controller,
        channel_id,
        volume,
        pan,
        muted,
        Some(format!("channel-volume-{channel_id:?}")),
    );
}

fn update_channel_pan(controller: &Rc<RefCell<AppController>>, channel_id: ChannelId, pan: f32) {
    let (volume, pan, muted) = {
        let controller = controller.borrow();
        let Some(channel) = controller
            .session()
            .channels
            .iter()
            .find(|channel| channel.id == channel_id)
        else {
            return;
        };
        channel_rack::merged_controls(channel, None, Some(pan), None)
    };
    execute_channel_controls(
        controller,
        channel_id,
        volume,
        pan,
        muted,
        Some(format!("channel-pan-{channel_id:?}")),
    );
}

fn update_channel_muted(
    controller: &Rc<RefCell<AppController>>,
    channel_id: ChannelId,
    muted: bool,
) {
    let (volume, pan, muted) = {
        let controller = controller.borrow();
        let Some(channel) = controller
            .session()
            .channels
            .iter()
            .find(|channel| channel.id == channel_id)
        else {
            return;
        };
        channel_rack::merged_controls(channel, None, None, Some(muted))
    };
    execute_channel_controls(controller, channel_id, volume, pan, muted, None);
}

fn execute_channel_controls(
    controller: &Rc<RefCell<AppController>>,
    channel_id: ChannelId,
    volume: f32,
    pan: f32,
    muted: bool,
    coalesce_key: Option<String>,
) {
    let transaction = Transaction::one(
        "Adjust Channel",
        Command::SetChannelControls {
            channel_id,
            volume,
            pan,
            muted,
        },
    );
    if let Some(key) = coalesce_key {
        if let Ok(revision) = controller.borrow_mut().execute_coalesced(key, &transaction) {
            schedule_end_coalescing(controller, revision);
        }
    } else {
        let _ = controller.borrow_mut().execute(&transaction);
    }
}

fn schedule_end_coalescing(controller: &Rc<RefCell<AppController>>, revision: u64) {
    let controller = controller.clone();
    gtk::glib::timeout_add_local_once(std::time::Duration::from_millis(250), move || {
        controller.borrow_mut().end_coalescing_if(revision);
    });
}

fn update_step_appearance(step: &gtk::ToggleButton, group_index: usize) {
    if step.is_active() {
        step.set_opacity(1.0);
        step.add_css_class("suggested-action");
    } else {
        step.remove_css_class("suggested-action");
        step.set_opacity(if group_index.is_multiple_of(2) {
            0.82
        } else {
            0.64
        });
    }
}

fn build_loop_type_popover() -> gtk::Popover {
    let content = popover_content("Loop Type");
    let loop_steps = gtk::CheckButton::with_label("Loop step channels");
    loop_steps.set_active(true);
    let loop_all = gtk::CheckButton::with_label("Loop all channels");
    loop_all.set_group(Some(&loop_steps));
    let advanced = gtk::CheckButton::with_label("Advanced looping");
    advanced.set_group(Some(&loop_steps));
    content.append(&loop_steps);
    content.append(&loop_all);
    content.append(&advanced);
    content.append(&gtk::Separator::new(gtk::Orientation::Horizontal));
    content.append(&popover_button("Burn to Pattern", "document-send-symbolic"));
    let colorful = gtk::CheckButton::with_label("Colorful loop controls");
    colorful.set_active(true);
    content.append(&colorful);
    let controls = gtk::CheckButton::with_label("Always show advanced controls");
    controls.set_active(true);
    content.append(&controls);
    popover_with_child(&content)
}

fn build_channel_options_popover() -> gtk::Popover {
    let content = popover_content("Channel Rack");
    for (label, icon) in [
        ("Add Channel", "list-add-symbolic"),
        ("Clone Selected", "edit-copy-symbolic"),
        ("Delete Selected", "user-trash-symbolic"),
        ("Assign to Mixer", "audio-card-symbolic"),
    ] {
        content.append(&popover_button(label, icon));
    }
    content.append(&gtk::Separator::new(gtk::Orientation::Horizontal));
    content.append(&popover_button("Burn to Pattern", "document-send-symbolic"));
    popover_with_child(&content)
}

fn build_loop_point_popover() -> gtk::Popover {
    let content = popover_content("Loop Point");
    let no_loop = gtk::CheckButton::with_label("Don't loop");
    let step = gtk::CheckButton::with_label("Step");
    step.set_group(Some(&no_loop));
    step.set_active(true);
    let beat = gtk::CheckButton::with_label("Beat");
    beat.set_group(Some(&no_loop));
    let bar = gtk::CheckButton::with_label("Bar");
    bar.set_group(Some(&no_loop));
    content.append(&no_loop);
    content.append(&step);
    content.append(&beat);
    content.append(&bar);
    content.append(&gtk::Separator::new(gtk::Orientation::Horizontal));
    content.append(&popover_button("Set to Next Beat", "go-next-symbolic"));
    content.append(&popover_button("Set to Next Bar", "go-last-symbolic"));
    content.append(&popover_button("Burn to Pattern", "document-send-symbolic"));
    popover_with_child(&content)
}

fn popover_content(title: &str) -> gtk::Box {
    let content = gtk::Box::new(gtk::Orientation::Vertical, 4);
    content.set_margin_top(10);
    content.set_margin_bottom(10);
    content.set_margin_start(10);
    content.set_margin_end(10);
    let heading = gtk::Label::new(Some(title));
    heading.add_css_class("heading");
    heading.set_xalign(0.0);
    content.append(&heading);
    content
}

fn popover_button(label: &str, icon: &str) -> gtk::Button {
    let button = gtk::Button::builder()
        .label(label)
        .icon_name(icon)
        .halign(gtk::Align::Fill)
        .build();
    button.add_css_class("flat");
    button
}

fn popover_with_child(child: &impl IsA<gtk::Widget>) -> gtk::Popover {
    let popover = gtk::Popover::new();
    popover.set_child(Some(child));
    popover
}

fn attach_secondary_click_popover(target: &impl IsA<gtk::Widget>, popover: &gtk::Popover) {
    let secondary_click = gtk::GestureClick::new();
    secondary_click.set_button(gtk::gdk::BUTTON_SECONDARY);
    secondary_click.connect_pressed({
        let popover = popover.clone();
        move |gesture, _, _, _| {
            gesture.set_state(gtk::EventSequenceState::Claimed);
            popover.popup();
        }
    });
    target.add_controller(secondary_click);
}

#[derive(Clone)]
struct PickerBindings {
    model: gtk::StringList,
}

impl PickerBindings {
    fn sync(&self, controller: &AppController) {
        let mut names: Vec<_> = controller
            .session()
            .patterns
            .iter()
            .map(|pattern| pattern.name.as_str())
            .collect();
        names.extend(["Audio Clips", "Automation"]);
        self.model.splice(0, self.model.n_items(), &names);
    }
}

fn build_picker(
    controller: &Rc<RefCell<AppController>>,
    playlist_bindings: &PlaylistBindings,
) -> (gtk::Widget, PickerBindings) {
    let strings = gtk::StringList::new(&[]);
    let bindings = PickerBindings {
        model: strings.clone(),
    };
    bindings.sync(&controller.borrow());
    let selection = gtk::SingleSelection::new(Some(strings));
    let factory = gtk::SignalListItemFactory::new();
    factory.connect_setup(|_, item| {
        let item = item.downcast_ref::<gtk::ListItem>().expect("list item");
        let label = gtk::Label::new(None);
        label.set_xalign(0.0);
        label.set_margin_top(8);
        label.set_margin_bottom(8);
        label.set_margin_start(8);
        label.set_margin_end(8);
        item.set_child(Some(&label));
    });
    factory.connect_bind(|_, item| {
        let item = item.downcast_ref::<gtk::ListItem>().expect("list item");
        let label = item
            .child()
            .and_downcast::<gtk::Label>()
            .expect("picker label");
        let value = item
            .item()
            .and_downcast::<gtk::StringObject>()
            .expect("picker item");
        label.set_label(&value.string());
    });
    let list = gtk::ListView::new(Some(selection), Some(factory));
    list.set_single_click_activate(false);
    list.connect_activate({
        let controller = controller.clone();
        let playlist_bindings = playlist_bindings.clone();
        move |_, position| {
            let pattern_id = usize::try_from(position).ok().and_then(|index| {
                controller
                    .borrow()
                    .session()
                    .patterns
                    .get(index)
                    .map(|pattern| pattern.id)
            });
            let Some(pattern_id) = pattern_id else {
                return;
            };
            controller.borrow_mut().selection_mut().pattern = Some(pattern_id);
            place_current_pattern(&controller, &playlist_bindings);
        }
    });
    list.set_tooltip_text(Some(
        "Double-click a Pattern to place it after the arrangement",
    ));
    let scroller = gtk::ScrolledWindow::new();
    scroller.set_child(Some(&list));
    (scroller.upcast(), bindings)
}

fn place_current_pattern(
    controller: &Rc<RefCell<AppController>>,
    playlist_bindings: &PlaylistBindings,
) {
    let placement = {
        let controller = controller.borrow();
        let session = controller.session();
        let pattern = controller
            .selection()
            .pattern
            .and_then(|id| session.patterns.iter().find(|pattern| pattern.id == id))
            .or_else(|| session.patterns.first());
        let lane = controller
            .selection()
            .playlist_lane
            .and_then(|id| session.playlist_lanes.iter().find(|lane| lane.id == id))
            .or_else(|| session.playlist_lanes.first());
        pattern.zip(lane).map(|(pattern, lane)| {
            let start = lane
                .clips
                .iter()
                .map(|clip| Tick(clip.start.0 + clip.length.0))
                .max()
                .unwrap_or(Tick::ZERO);
            (pattern.clone(), lane.id, start)
        })
    };
    let Some((pattern, lane_id, start)) = placement else {
        return;
    };
    let clip = PlaylistClip {
        id: ClipId::new(),
        name: pattern.name,
        start,
        length: pattern.length,
        color: Rgba::accent(),
        muted: false,
        content: PlaylistClipContent::Pattern {
            pattern_id: pattern.id,
        },
    };
    let edit = Transaction::one(
        "Place Pattern Clip",
        Command::AddPlaylistClip {
            lane_id,
            clip: clip.clone(),
        },
    );
    if controller.borrow_mut().execute(&edit).is_ok() {
        let mut controller = controller.borrow_mut();
        controller.selection_mut().playlist_lane = Some(lane_id);
        controller.selection_mut().clip = Some(clip.id);
    }
    playlist_bindings.sync(&controller.borrow());
}

#[derive(Clone, Copy)]
enum PlaylistClipKind {
    Pattern,
    Audio,
    Automation,
}

#[derive(Clone)]
struct PlaylistClipView {
    lane: i32,
    lane_id: PlaylistLaneId,
    clip_id: ClipId,
    start: Tick,
    length: Tick,
    source_length: Tick,
    name: String,
    kind: PlaylistClipKind,
    muted: bool,
    selected: bool,
    /// A compact piano-roll preview for pattern clips, so the playlist cell
    /// visibly carries the same musical material as the channel rack.
    pattern_notes: Vec<(Tick, Tick, u8)>,
    pattern_steps: Vec<Tick>,
}

#[derive(Clone, Copy)]
struct ClipResize {
    lane_id: PlaylistLaneId,
    clip_id: ClipId,
    start: Tick,
    original_length: Tick,
}

#[derive(Clone, Copy)]
struct ClipMove {
    lane_id: PlaylistLaneId,
    target_lane_id: PlaylistLaneId,
    target_lane_index: usize,
    original_lane_index: usize,
    clip_id: ClipId,
    original_start: Tick,
    length: Tick,
}

#[derive(Clone)]
struct PlaylistBindings {
    clips: Rc<RefCell<Vec<PlaylistClipView>>>,
    drawing: gtk::DrawingArea,
    playhead_bars: Rc<Cell<f64>>,
    mass_select: Rc<Cell<bool>>,
    mass_selected: Rc<RefCell<HashSet<ClipId>>>,
    selection_rect: Rc<Cell<Option<(f64, f64, f64, f64)>>>,
}

impl PlaylistBindings {
    fn sync(&self, controller: &AppController) {
        *self.clips.borrow_mut() = playlist_clip_views(controller);
        self.drawing.queue_draw();
    }

    fn set_playhead_bars(&self, bars: f64) {
        self.playhead_bars.set(bars.max(0.0));
        self.drawing.queue_draw();
    }

    fn set_mass_select(&self, enabled: bool) {
        self.mass_select.set(enabled);
    }
}

const PLAYLIST_BAR_WIDTH: f64 = 96.0;
const PLAYLIST_HEADER_HEIGHT: f64 = 34.0;
const PLAYLIST_LANE_HEIGHT: f64 = 54.0;

fn playlist_clip_views(controller: &AppController) -> Vec<PlaylistClipView> {
    let session = controller.session();
    session
        .playlist_lanes
        .iter()
        .enumerate()
        .filter_map(|(lane_index, lane)| {
            let lane_index = i32::try_from(lane_index).ok()?;
            Some(lane.clips.iter().map(move |clip| {
                let (kind, source_length, pattern_notes, pattern_steps) = match &clip.content {
                    PlaylistClipContent::Pattern { pattern_id } => {
                        let pattern = session
                            .patterns
                            .iter()
                            .find(|pattern| pattern.id == *pattern_id);
                        let notes = pattern.map_or_else(Vec::new, |pattern| {
                            pattern
                                .lanes
                                .iter()
                                .flat_map(|lane| {
                                    lane.notes
                                        .iter()
                                        .map(|note| (note.start, note.length, note.pitch))
                                })
                                .collect()
                        });
                        let steps = pattern.map_or_else(Vec::new, |pattern| {
                            pattern
                                .lanes
                                .iter()
                                .flat_map(|lane| lane.steps.iter().map(|step| step.at))
                                .collect()
                        });
                        (
                            PlaylistClipKind::Pattern,
                            pattern.map_or(clip.length, |pattern| pattern.length),
                            notes,
                            steps,
                        )
                    }
                    PlaylistClipContent::Audio { .. } => {
                        (PlaylistClipKind::Audio, clip.length, Vec::new(), Vec::new())
                    }
                    PlaylistClipContent::Automation { .. } => (
                        PlaylistClipKind::Automation,
                        clip.length,
                        Vec::new(),
                        Vec::new(),
                    ),
                };
                PlaylistClipView {
                    lane: lane_index,
                    lane_id: lane.id,
                    clip_id: clip.id,
                    start: clip.start,
                    length: clip.length,
                    source_length,
                    name: clip.name.clone(),
                    kind,
                    muted: clip.muted,
                    selected: controller.selection().clip == Some(clip.id),
                    pattern_notes,
                    pattern_steps,
                }
            }))
        })
        .flatten()
        .collect()
}

fn build_playlist(controller: &Rc<RefCell<AppController>>) -> (gtk::Widget, PlaylistBindings) {
    let clips = Rc::new(RefCell::new(playlist_clip_views(&controller.borrow())));
    let playhead_bars = Rc::new(Cell::new(0.0));
    let mass_select = Rc::new(Cell::new(false));
    let mass_selected = Rc::new(RefCell::new(HashSet::new()));
    let selection_rect = Rc::new(Cell::new(None));
    let drawing = gtk::DrawingArea::new();
    drawing.set_content_width(2_304);
    drawing.set_content_height(1_000);
    drawing.set_focusable(true);
    drawing.set_accessible_role(gtk::AccessibleRole::Grid);
    drawing.set_tooltip_text(Some(
        "Playlist arrangement. Use arrow keys to move clips; drag clip edges to resize.",
    ));
    drawing.set_draw_func({
        let clips = clips.clone();
        let playhead_bars = playhead_bars.clone();
        let selection_rect = selection_rect.clone();
        move |widget, context, width, height| {
            draw_playlist(
                widget,
                context,
                width,
                height,
                &clips.borrow(),
                playhead_bars.get(),
            );
            if let Some((x, y, w, h)) = selection_rect.get() {
                let color = adw::StyleManager::for_display(&widget.display()).accent_color_rgba();
                set_cairo_color(context, &color, 0.18);
                context.rectangle(x, y, w, h);
                context.fill().expect("fill selection rectangle");
                set_cairo_color(context, &color, 0.85);
                context.rectangle(x, y, w, h);
                context.stroke().expect("stroke selection rectangle");
            }
        }
    });
    let bindings = PlaylistBindings {
        clips: clips.clone(),
        drawing: drawing.clone(),
        playhead_bars,
        mass_select,
        mass_selected,
        selection_rect,
    };
    install_playlist_selection(&drawing, &clips, controller);
    install_playlist_mass_selection(&drawing, &clips, &bindings);
    install_playlist_resize(&drawing, &clips, controller, &bindings);
    install_playlist_move(&drawing, &clips, controller, &bindings);
    install_playlist_resize_cursor(&drawing, &clips);
    install_playlist_clip_shortcuts(&drawing, controller, &bindings);

    let scroller = gtk::ScrolledWindow::new();
    scroller.set_child(Some(&drawing));
    (scroller.upcast(), bindings)
}

fn install_playlist_mass_selection(
    drawing: &gtk::DrawingArea,
    clips: &Rc<RefCell<Vec<PlaylistClipView>>>,
    bindings: &PlaylistBindings,
) {
    let drag = gtk::GestureDrag::new();
    drag.set_button(gdk::BUTTON_PRIMARY);
    let origin = Rc::new(Cell::new((0.0, 0.0)));
    drag.connect_drag_begin({
        let origin = origin.clone();
        let bindings = bindings.clone();
        move |_, x, y| {
            if bindings.mass_select.get() {
                origin.set((x, y));
                bindings.selection_rect.set(Some((x, y, 0.0, 0.0)));
            }
        }
    });
    drag.connect_drag_update({
        let origin = origin.clone();
        let bindings = bindings.clone();
        move |_, dx, dy| {
            if bindings.mass_select.get() {
                let (x, y) = origin.get();
                let (rx, ry) = (x.min(x + dx), y.min(y + dy));
                bindings
                    .selection_rect
                    .set(Some((rx, ry, dx.abs(), dy.abs())));
                bindings.drawing.queue_draw();
            }
        }
    });
    drag.connect_drag_end({
        let origin = origin.clone();
        let clips = clips.clone();
        let bindings = bindings.clone();
        move |_, dx, dy| {
            if !bindings.mass_select.get() {
                return;
            }
            let (x, y) = origin.get();
            let (rx, ry) = (x.min(x + dx), y.min(y + dy));
            let right = rx + dx.abs();
            let bottom = ry + dy.abs();
            let mut selected = bindings.mass_selected.borrow_mut();
            selected.clear();
            for clip in clips.borrow().iter() {
                let left = playlist_ticks_to_bars(clip.start) * PLAYLIST_BAR_WIDTH;
                let top = PLAYLIST_HEADER_HEIGHT + f64::from(clip.lane) * PLAYLIST_LANE_HEIGHT;
                let clip_right =
                    playlist_ticks_to_bars(Tick(clip.start.0 + clip.length.0)) * PLAYLIST_BAR_WIDTH;
                let clip_bottom = top + PLAYLIST_LANE_HEIGHT;
                if left < right && clip_right > rx && top < bottom && clip_bottom > ry {
                    selected.insert(clip.clip_id);
                }
            }
            for clip in clips.borrow_mut().iter_mut() {
                clip.selected = selected.contains(&clip.clip_id);
            }
            bindings.selection_rect.set(None);
            bindings.drawing.queue_draw();
        }
    });
    drawing.add_controller(drag);
}

/// Deliberately keeps duplicate/delete at the command layer: the view only
/// chooses the target, and undo sees one coherent edit in either case.
fn install_playlist_clip_shortcuts(
    drawing: &gtk::DrawingArea,
    controller: &Rc<RefCell<AppController>>,
    bindings: &PlaylistBindings,
) {
    let keys = gtk::EventControllerKey::new();
    keys.connect_key_pressed({
        let controller = controller.clone();
        let bindings = bindings.clone();
        move |_, key, _, modifiers| {
            if (key == gdk::Key::Delete || key == gdk::Key::BackSpace)
                && !bindings.mass_selected.borrow().is_empty()
            {
                let ids = bindings.mass_selected.borrow().clone();
                let commands = controller
                    .borrow()
                    .session()
                    .playlist_lanes
                    .iter()
                    .flat_map(|lane| {
                        lane.clips.iter().filter_map(|clip| {
                            ids.contains(&clip.id)
                                .then_some(Command::RemovePlaylistClip {
                                    lane_id: lane.id,
                                    clip_id: clip.id,
                                })
                        })
                    })
                    .collect();
                let edit = Transaction {
                    label: "Delete selected Playlist clips".into(),
                    commands,
                };
                if controller.borrow_mut().execute(&edit).is_ok() {
                    bindings.mass_selected.borrow_mut().clear();
                    bindings.sync(&controller.borrow());
                }
                return gtk::glib::Propagation::Stop;
            }
            let selected = {
                let controller = controller.borrow();
                let Some(lane_id) = controller.selection().playlist_lane else {
                    return gtk::glib::Propagation::Proceed;
                };
                let Some(clip_id) = controller.selection().clip else {
                    return gtk::glib::Propagation::Proceed;
                };
                controller
                    .session()
                    .playlist_lanes
                    .iter()
                    .find(|lane| lane.id == lane_id)
                    .and_then(|lane| lane.clips.iter().find(|clip| clip.id == clip_id).cloned())
                    .map(|clip| (lane_id, clip))
            };
            let Some((lane_id, clip)) = selected else {
                return gtk::glib::Propagation::Proceed;
            };
            let edit = if modifiers.contains(gdk::ModifierType::ALT_MASK)
                && (key == gdk::Key::Left || key == gdk::Key::Right)
            {
                let radiant_domain::PlaylistClipContent::Audio {
                    source_start_frame, ..
                } = clip.content
                else {
                    return gtk::glib::Propagation::Proceed;
                };
                let delta = if key == gdk::Key::Left { -240 } else { 240 };
                Transaction::one(
                    "Slip audio clip",
                    Command::SetAudioClipSourceStart {
                        lane_id,
                        clip_id: clip.id,
                        source_start_frame: (source_start_frame + delta).max(0),
                    },
                )
            } else if key == gdk::Key::m && modifiers.is_empty() {
                Transaction::one(
                    "Toggle Playlist clip mute",
                    Command::SetPlaylistClipMuted {
                        lane_id,
                        clip_id: clip.id,
                        muted: !clip.muted,
                    },
                )
            } else if modifiers.contains(gdk::ModifierType::CONTROL_MASK)
                && (key == gdk::Key::Left || key == gdk::Key::Right)
            {
                let delta = if key == gdk::Key::Left { -240 } else { 240 };
                let length = Tick((clip.length.0 + delta).max(240));
                Transaction::one(
                    "Nudge Playlist clip edge",
                    Command::SetPlaylistClipRange {
                        lane_id,
                        clip_id: clip.id,
                        start: clip.start,
                        length,
                    },
                )
            } else if key == gdk::Key::Delete || key == gdk::Key::BackSpace {
                Transaction::one(
                    "Delete Playlist clip",
                    Command::RemovePlaylistClip {
                        lane_id,
                        clip_id: clip.id,
                    },
                )
            } else if key == gdk::Key::d && modifiers.contains(gdk::ModifierType::CONTROL_MASK) {
                let copy = PlaylistClip {
                    id: ClipId::new(),
                    start: Tick(clip.start.0 + clip.length.0),
                    ..clip
                };
                Transaction::one(
                    "Duplicate Playlist clip",
                    Command::AddPlaylistClip {
                        lane_id,
                        clip: copy,
                    },
                )
            } else {
                return gtk::glib::Propagation::Proceed;
            };
            if controller.borrow_mut().execute(&edit).is_ok() {
                if key == gdk::Key::Delete || key == gdk::Key::BackSpace {
                    let mut controller = controller.borrow_mut();
                    controller.selection_mut().clip = None;
                }
                bindings.sync(&controller.borrow());
            }
            gtk::glib::Propagation::Stop
        }
    });
    drawing.add_controller(keys);
}

fn install_playlist_move(
    drawing: &gtk::DrawingArea,
    clips: &Rc<RefCell<Vec<PlaylistClipView>>>,
    controller: &Rc<RefCell<AppController>>,
    bindings: &PlaylistBindings,
) {
    let moving = Rc::new(RefCell::new(None::<ClipMove>));
    let drag = gtk::GestureDrag::new();
    drag.set_button(gdk::BUTTON_PRIMARY);
    drag.connect_drag_begin({
        let clips = clips.clone();
        let moving = moving.clone();
        let bindings = bindings.clone();
        move |gesture, x, y| {
            if bindings.mass_select.get() {
                return;
            }
            if playlist_clip_resize_target(&clips.borrow(), x, y).is_some() {
                return;
            }
            let target = playlist_clip_at(&clips.borrow(), x, y).map(|index| {
                let clip = &clips.borrow()[index];
                ClipMove {
                    lane_id: clip.lane_id,
                    target_lane_id: clip.lane_id,
                    target_lane_index: usize::try_from(clip.lane).unwrap_or(0),
                    original_lane_index: usize::try_from(clip.lane).unwrap_or(0),
                    clip_id: clip.clip_id,
                    original_start: clip.start,
                    length: clip.length,
                }
            });
            *moving.borrow_mut() = target;
            if target.is_some() {
                gesture.set_state(gtk::EventSequenceState::Claimed);
            }
        }
    });
    drag.connect_drag_update({
        let clips = clips.clone();
        let moving = moving.clone();
        let drawing = drawing.clone();
        let controller_for_lane = controller.clone();
        move |gesture, offset_x, offset_y| {
            let Some(moving_snapshot) = *moving.borrow() else {
                return;
            };
            let bypass_snap = gesture
                .current_event_state()
                .contains(gdk::ModifierType::ALT_MASK);
            let raw_start = playlist_ticks_to_bars(moving_snapshot.original_start)
                + offset_x / PLAYLIST_BAR_WIDTH;
            let start = snap_timeline_value(raw_start, 0.25, bypass_snap).max(0.0);
            if let Some(clip) = clips
                .borrow_mut()
                .iter_mut()
                .find(|clip| clip.clip_id == moving_snapshot.clip_id)
            {
                clip.start = if start == 0.0 {
                    Tick::ZERO
                } else {
                    playlist_bars_to_ticks(start)
                };
                clip.selected = true;
            }
            let target_row = ((moving_snapshot.original_lane_index as f64
                + offset_y / PLAYLIST_LANE_HEIGHT)
                .floor()
                .max(0.0)) as usize;
            if let Some(lane) = controller_for_lane
                .borrow()
                .session()
                .playlist_lanes
                .get(target_row)
            {
                if let Some(current) = moving.borrow_mut().as_mut() {
                    current.target_lane_id = lane.id;
                    current.target_lane_index = target_row;
                }
                if let Some(clip) = clips
                    .borrow_mut()
                    .iter_mut()
                    .find(|clip| clip.clip_id == moving_snapshot.clip_id)
                {
                    clip.lane = i32::try_from(target_row).unwrap_or(clip.lane);
                    clip.lane_id = lane.id;
                }
            } else if let Some(current) = moving.borrow_mut().as_mut() {
                current.target_lane_index = target_row;
            }
            drawing.queue_draw();
        }
    });
    drag.connect_drag_end({
        let clips = clips.clone();
        let moving = moving.clone();
        let controller = controller.clone();
        let bindings = bindings.clone();
        move |_, _, _| {
            let Some(mut moving) = moving.borrow_mut().take() else {
                return;
            };
            let mut controller_ref = controller.borrow_mut();
            while controller_ref.session().playlist_lanes.len() <= moving.target_lane_index {
                let number = controller_ref.session().playlist_lanes.len() + 1;
                let lane = radiant_domain::PlaylistLane::new(format!("Playlist {number}"));
                let edit = Transaction::one("Add Playlist lane", Command::AddPlaylistLane { lane });
                if controller_ref.execute(&edit).is_err() {
                    return;
                }
            }
            moving.target_lane_id =
                controller_ref.session().playlist_lanes[moving.target_lane_index].id;
            drop(controller_ref);
            let new_start = clips
                .borrow()
                .iter()
                .find(|clip| clip.clip_id == moving.clip_id)
                .map_or(moving.original_start, |clip| clip.start);
            if new_start != moving.original_start || moving.target_lane_id != moving.lane_id {
                if moving.target_lane_id != moving.lane_id {
                    let edit = Transaction::one(
                        "Move Playlist clip to lane",
                        Command::MovePlaylistClip {
                            from_lane_id: moving.lane_id,
                            to_lane_id: moving.target_lane_id,
                            clip_id: moving.clip_id,
                        },
                    );
                    let _ = controller.borrow_mut().execute(&edit);
                }
                let edit = Transaction::one(
                    "Move Playlist clip",
                    Command::SetPlaylistClipRange {
                        lane_id: moving.target_lane_id,
                        clip_id: moving.clip_id,
                        start: new_start,
                        length: moving.length,
                    },
                );
                let _ = controller.borrow_mut().execute(&edit);
            }
            bindings.sync(&controller.borrow());
        }
    });
    drawing.add_controller(drag);
}

fn install_playlist_selection(
    drawing: &gtk::DrawingArea,
    clips: &Rc<RefCell<Vec<PlaylistClipView>>>,
    controller: &Rc<RefCell<AppController>>,
) {
    let click = gtk::GestureClick::new();
    click.set_button(gdk::BUTTON_PRIMARY);
    click.connect_pressed({
        let clips = clips.clone();
        let drawing = drawing.clone();
        let controller = controller.clone();
        move |_, _, x, y| {
            if playlist_clip_resize_target(&clips.borrow(), x, y).is_some() {
                return;
            }
            let selected = playlist_clip_at(&clips.borrow(), x, y);
            for clip in clips.borrow_mut().iter_mut() {
                clip.selected = false;
            }
            if let Some(index) = selected {
                clips.borrow_mut()[index].selected = true;
            }
            let selected_ids = selected.map(|index| {
                let clips = clips.borrow();
                (clips[index].lane_id, clips[index].clip_id)
            });
            let mut controller = controller.borrow_mut();
            controller.selection_mut().playlist_lane = selected_ids.map(|ids| ids.0);
            controller.selection_mut().clip = selected_ids.map(|ids| ids.1);
            drawing.grab_focus();
            drawing.queue_draw();
        }
    });
    drawing.add_controller(click);
}

fn install_playlist_resize(
    drawing: &gtk::DrawingArea,
    clips: &Rc<RefCell<Vec<PlaylistClipView>>>,
    controller: &Rc<RefCell<AppController>>,
    bindings: &PlaylistBindings,
) {
    let resize = Rc::new(RefCell::new(None::<ClipResize>));
    let resize_drag = gtk::GestureDrag::new();
    resize_drag.set_button(gdk::BUTTON_PRIMARY);
    resize_drag.connect_drag_begin({
        let clips = clips.clone();
        let resize = resize.clone();
        let controller = controller.clone();
        move |gesture, x, y| {
            let target = playlist_clip_resize_target(&clips.borrow(), x, y).map(|index| {
                let clip = &clips.borrow()[index];
                ClipResize {
                    lane_id: clip.lane_id,
                    clip_id: clip.clip_id,
                    start: clip.start,
                    original_length: clip.length,
                }
            });
            *resize.borrow_mut() = target;
            if let Some(target) = target {
                let mut controller = controller.borrow_mut();
                controller.selection_mut().playlist_lane = Some(target.lane_id);
                controller.selection_mut().clip = Some(target.clip_id);
                gesture.set_state(gtk::EventSequenceState::Claimed);
            }
        }
    });
    resize_drag.connect_drag_update({
        let clips = clips.clone();
        let resize = resize.clone();
        let drawing = drawing.clone();
        move |gesture, offset_x, _| {
            let Some(resize) = *resize.borrow() else {
                return;
            };
            let mut clips = clips.borrow_mut();
            let Some(clip) = clips.iter_mut().find(|clip| clip.clip_id == resize.clip_id) else {
                return;
            };
            let raw_length =
                playlist_ticks_to_bars(resize.original_length) + offset_x / PLAYLIST_BAR_WIDTH;
            let bypass_snap = gesture
                .current_event_state()
                .contains(gdk::ModifierType::ALT_MASK);
            let minimum = if bypass_snap { 0.02 } else { 0.25 };
            let snapped_length = snap_timeline_value(raw_length, 0.25, bypass_snap).max(minimum);
            clip.length = playlist_bars_to_ticks(snapped_length);
            clip.selected = true;
            drop(clips);
            drawing.queue_draw();
        }
    });
    resize_drag.connect_drag_end({
        let resize = resize.clone();
        let clips = clips.clone();
        let controller = controller.clone();
        let bindings = bindings.clone();
        move |_, _, _| {
            let Some(resize) = resize.borrow_mut().take() else {
                return;
            };
            let new_length = clips
                .borrow()
                .iter()
                .find(|clip| clip.clip_id == resize.clip_id)
                .map_or(resize.original_length, |clip| clip.length);
            if new_length != resize.original_length {
                let edit = Transaction::one(
                    "Resize Playlist clip",
                    Command::SetPlaylistClipRange {
                        lane_id: resize.lane_id,
                        clip_id: resize.clip_id,
                        start: resize.start,
                        length: new_length,
                    },
                );
                let _ = controller.borrow_mut().execute(&edit);
            }
            bindings.sync(&controller.borrow());
        }
    });
    drawing.add_controller(resize_drag);
}

fn install_playlist_resize_cursor(
    drawing: &gtk::DrawingArea,
    clips: &Rc<RefCell<Vec<PlaylistClipView>>>,
) {
    let resize_cursor = gtk::EventControllerMotion::new();
    resize_cursor.connect_motion({
        let clips = clips.clone();
        move |controller, x, y| {
            let cursor = playlist_clip_resize_target(&clips.borrow(), x, y)
                .is_some()
                .then_some("ew-resize");
            if let Some(widget) = controller.widget() {
                widget.set_cursor_from_name(cursor);
            }
        }
    });
    resize_cursor.connect_leave(|controller| {
        if let Some(widget) = controller.widget() {
            widget.set_cursor_from_name(None);
        }
    });
    drawing.add_controller(resize_cursor);
}

fn draw_playlist(
    widget: &gtk::DrawingArea,
    context: &cairo::Context,
    width: i32,
    height: i32,
    clips: &[PlaylistClipView],
    playhead_bars: f64,
) {
    let style = adw::StyleManager::for_display(&widget.display());
    let background = if style.is_dark() {
        gdk::RGBA::new(0.12, 0.12, 0.12, 1.0)
    } else {
        gdk::RGBA::new(0.98, 0.98, 0.98, 1.0)
    };
    let foreground = widget.color();
    let accent = style.accent_color_rgba();
    set_cairo_color(context, &background, 1.0);
    context.paint().expect("paint Playlist background");

    context.set_line_width(1.0);
    for x in (0..width).step_by(24) {
        let alpha = if x % 96 == 0 { 0.38 } else { 0.14 };
        set_cairo_color(context, &foreground, alpha);
        context.move_to(f64::from(x) + 0.5, 0.0);
        context.line_to(f64::from(x) + 0.5, f64::from(height));
    }
    for y in (34..height).step_by(54) {
        set_cairo_color(context, &foreground, 0.25);
        context.move_to(0.0, f64::from(y) + 0.5);
        context.line_to(f64::from(width), f64::from(y) + 0.5);
    }
    context.stroke().expect("draw Playlist grid");

    set_cairo_color(context, &foreground, 0.07);
    context.rectangle(0.0, 0.0, f64::from(width), PLAYLIST_HEADER_HEIGHT);
    context.fill().expect("paint Playlist ruler");
    set_cairo_color(context, &foreground, 0.82);
    context.select_font_face("Sans", cairo::FontSlant::Normal, cairo::FontWeight::Normal);
    context.set_font_size(11.0);
    for bar in 0..24 {
        context.move_to(f64::from(bar) * PLAYLIST_BAR_WIDTH + 8.0, 22.0);
        context
            .show_text(&(bar + 1).to_string())
            .expect("draw bar number");
    }

    for clip in clips {
        draw_playlist_clip(context, &foreground, &accent, clip);
    }
    set_cairo_color(context, &accent, 1.0);
    context.rectangle(
        playhead_bars * PLAYLIST_BAR_WIDTH,
        33.0,
        2.0,
        f64::from(height - 33),
    );
    context.fill().expect("draw Playlist playhead");
}

fn draw_playlist_clip(
    context: &cairo::Context,
    foreground: &gdk::RGBA,
    accent: &gdk::RGBA,
    clip: &PlaylistClipView,
) {
    let start = playlist_ticks_to_bars(clip.start);
    let length = playlist_ticks_to_bars(clip.length);
    let source_length = playlist_ticks_to_bars(clip.source_length);
    let x = start * PLAYLIST_BAR_WIDTH + 2.0;
    let y = PLAYLIST_HEADER_HEIGHT + f64::from(clip.lane) * PLAYLIST_LANE_HEIGHT + 3.0;
    let width = (length * PLAYLIST_BAR_WIDTH - 4.0).max(8.0);
    set_cairo_color(
        context,
        accent,
        if clip.muted {
            0.18
        } else if clip.selected {
            0.86
        } else {
            0.54
        },
    );
    context.rectangle(x, y, width, PLAYLIST_LANE_HEIGHT - 6.0);
    context.fill().expect("draw Playlist clip");

    set_cairo_color(context, foreground, 0.88);
    context.set_font_size(10.0);
    context.move_to(x + 6.0, y + 14.0);
    context.show_text(&clip.name).expect("draw clip name");
    match clip.kind {
        PlaylistClipKind::Pattern => {
            let pattern_ticks = (clip.source_length.0).max(1) as f64;
            for (start_tick, note_length, pitch) in &clip.pattern_notes {
                let note_x = x
                    + (f64::from(start_tick.0 as i32) / pattern_ticks)
                        * source_length
                        * PLAYLIST_BAR_WIDTH;
                let note_width = ((f64::from(note_length.0 as i32) / pattern_ticks)
                    * source_length
                    * PLAYLIST_BAR_WIDTH)
                    .max(2.0);
                let normalized_pitch = f64::from(*pitch) / 127.0;
                let note_y = y + 19.0 + (1.0 - normalized_pitch) * 25.0;
                context.rectangle(
                    note_x,
                    note_y,
                    note_width.min((x + width - note_x).max(2.0)),
                    3.0,
                );
                context.fill().expect("draw pattern piano-roll preview");
            }
            set_cairo_color(context, foreground, 0.58);
            for step in &clip.pattern_steps {
                let step_x =
                    x + (step.0 as f64 / pattern_ticks) * source_length * PLAYLIST_BAR_WIDTH;
                context.rectangle(step_x, y + PLAYLIST_LANE_HEIGHT - 13.0, 3.0, 5.0);
                context.fill().expect("draw pattern step preview");
            }
            let mut loop_at = source_length;
            while loop_at < length {
                let loop_x = x + loop_at * PLAYLIST_BAR_WIDTH;
                set_cairo_color(context, foreground, 0.36);
                context.move_to(loop_x, y + 18.0);
                context.line_to(loop_x, y + PLAYLIST_LANE_HEIGHT - 8.0);
                context.stroke().expect("draw pattern loop boundary");
                loop_at += source_length;
            }
        }
        PlaylistClipKind::Audio => {
            set_cairo_color(context, foreground, 0.62);
            for sample in 0..f64_to_i32_floor(width / 8.0) {
                let wave_x = x + f64::from(sample) * 8.0;
                let amplitude = f64::from((sample * 17).rem_euclid(13)) + 3.0;
                context.move_to(wave_x, y + 34.0 - amplitude * 0.5);
                context.line_to(wave_x, y + 34.0 + amplitude * 0.5);
            }
            context.stroke().expect("draw audio waveform preview");
        }
        PlaylistClipKind::Automation => {
            set_cairo_color(context, foreground, 0.72);
            context.set_line_width(2.0);
            context.move_to(x + 4.0, y + 36.0);
            context.curve_to(
                x + width * 0.3,
                y + 36.0,
                x + width * 0.6,
                y + 18.0,
                x + width - 4.0,
                y + 22.0,
            );
            context.stroke().expect("draw automation preview");
        }
    }
    if clip.selected {
        set_cairo_color(context, foreground, 0.68);
        context.rectangle(x + width - 4.0, y + 2.0, 4.0, PLAYLIST_LANE_HEIGHT - 10.0);
        context.fill().expect("draw clip resize handle");
    }
}

fn playlist_clip_at(clips: &[PlaylistClipView], x: f64, y: f64) -> Option<usize> {
    if y < PLAYLIST_HEADER_HEIGHT {
        return None;
    }
    let lane = f64_to_i32_floor((y - PLAYLIST_HEADER_HEIGHT) / PLAYLIST_LANE_HEIGHT);
    clips.iter().rposition(|clip| {
        let left = playlist_ticks_to_bars(clip.start) * PLAYLIST_BAR_WIDTH;
        let right = playlist_ticks_to_bars(Tick(clip.start.0 + clip.length.0)) * PLAYLIST_BAR_WIDTH;
        clip.lane == lane && x >= left && x <= right
    })
}

fn playlist_clip_resize_target(clips: &[PlaylistClipView], x: f64, y: f64) -> Option<usize> {
    let index = playlist_clip_at(clips, x, y)?;
    let clip = &clips[index];
    let right = playlist_ticks_to_bars(Tick(clip.start.0 + clip.length.0)) * PLAYLIST_BAR_WIDTH;
    (x >= right - 8.0 && x <= right + 3.0).then_some(index)
}

#[derive(Clone)]
struct MixerBindings {
    controller: Rc<RefCell<AppController>>,
    syncing: Rc<Cell<bool>>,
    controls: Rc<RefCell<Vec<MixerControlBinding>>>,
    strips: Rc<RefCell<Option<gtk::Box>>>,
    selected_track: Rc<RefCell<Option<gtk::Label>>>,
    master_meters: Rc<RefCell<Vec<gtk::LevelBar>>>,
}

impl MixerBindings {
    fn new(controller: &Rc<RefCell<AppController>>) -> Self {
        Self {
            controller: controller.clone(),
            syncing: Rc::new(Cell::new(false)),
            controls: Rc::new(RefCell::new(Vec::new())),
            strips: Rc::new(RefCell::new(None)),
            selected_track: Rc::new(RefCell::new(None)),
            master_meters: Rc::new(RefCell::new(Vec::new())),
        }
    }

    fn sync(&self, controller: &AppController) {
        let insert_ids: Vec<_> = controller
            .session()
            .mixer_inserts
            .iter()
            .map(|insert| insert.id)
            .collect();
        let rendered_ids: Vec<_> = self
            .controls
            .borrow()
            .iter()
            .map(|binding| binding.mixer_insert_id)
            .collect();
        if insert_ids != rendered_ids {
            self.rebuild();
        }
        self.syncing.set(true);
        for binding in self.controls.borrow().iter() {
            if let Some(insert) = controller
                .session()
                .mixer_inserts
                .iter()
                .find(|insert| insert.id == binding.mixer_insert_id)
            {
                binding.fader.set_value(f64::from(insert.gain_db));
                binding.pan.set_value(f64::from(insert.pan));
                binding.mute.set_active(insert.muted);
            }
        }
        self.syncing.set(false);
    }

    fn rebuild(&self) {
        let Some(strips) = self.strips.borrow().clone() else {
            return;
        };
        let Some(selected_track) = self.selected_track.borrow().clone() else {
            return;
        };
        while let Some(child) = strips.first_child() {
            strips.remove(&child);
        }
        self.controls.borrow_mut().clear();
        let specs = mixer_strip_specs(&self.controller.borrow());
        let mut selection_group = None;
        for (index, spec) in specs.iter().enumerate() {
            if index == 1 {
                strips.append(&gtk::Separator::new(gtk::Orientation::Vertical));
            }
            strips.append(&build_mixer_strip(
                spec,
                &mut selection_group,
                &selected_track,
                &self.controller,
                self,
            ));
        }
    }
}

#[derive(Clone)]
struct MixerControlBinding {
    mixer_insert_id: MixerInsertId,
    fader: gtk::Scale,
    pan: gtk::ScaleButton,
    mute: gtk::ToggleButton,
}

fn build_mixer(controller: &Rc<RefCell<AppController>>) -> (gtk::Widget, MixerBindings) {
    let bindings = MixerBindings::new(controller);
    let inspector_split = adw::OverlaySplitView::new();
    inspector_split.set_sidebar_position(gtk::PackType::End);
    inspector_split.set_min_sidebar_width(250.0);
    inspector_split.set_max_sidebar_width(340.0);
    inspector_split.set_sidebar_width_fraction(0.32);
    inspector_split.set_pin_sidebar(true);
    inspector_split.set_enable_hide_gesture(true);
    inspector_split.set_enable_show_gesture(true);

    let (inspector, selected_track) = build_mixer_inspector(&inspector_split);
    inspector_split.set_sidebar(Some(&inspector));
    inspector_split.set_content(Some(&build_mixer_deck(&selected_track, &bindings)));

    let mixer = gtk::Box::new(gtk::Orientation::Vertical, 0);
    mixer.append(&build_mixer_toolbar(&inspector_split));
    mixer.append(&gtk::Separator::new(gtk::Orientation::Horizontal));
    inspector_split.set_vexpand(true);
    mixer.append(&inspector_split);

    let responsive = adw::BreakpointBin::new();
    responsive.set_width_request(1);
    responsive.set_height_request(1);
    responsive.set_child(Some(&mixer));
    let compact = adw::Breakpoint::new(
        adw::BreakpointCondition::parse("max-width: 720px")
            .expect("valid compact mixer breakpoint"),
    );
    compact.add_setter(&inspector_split, "collapsed", Some(&true.to_value()));
    compact.add_setter(&inspector_split, "show-sidebar", Some(&false.to_value()));
    responsive.add_breakpoint(compact);
    (responsive.upcast(), bindings)
}

fn build_mixer_toolbar(inspector_split: &adw::OverlaySplitView) -> gtk::Widget {
    let toolbar = gtk::Box::new(gtk::Orientation::Horizontal, 4);
    toolbar.set_margin_top(4);
    toolbar.set_margin_bottom(4);
    toolbar.set_margin_start(6);
    toolbar.set_margin_end(6);
    toolbar.add_css_class("toolbar");

    let menu = gtk::MenuButton::builder()
        .icon_name("open-menu-symbolic")
        .tooltip_text("Mixer menu")
        .popover(&build_mixer_menu_popover())
        .build();
    toolbar.append(&menu);

    let show_inspector = gtk::ToggleButton::builder()
        .icon_name("sidebar-show-symbolic")
        .tooltip_text("Show or hide the FX and track inspector")
        .active(true)
        .build();
    show_inspector.connect_toggled({
        let inspector_split = inspector_split.clone();
        move |button| inspector_split.set_show_sidebar(button.is_active())
    });
    inspector_split.connect_show_sidebar_notify({
        let show_inspector = show_inspector.clone();
        move |split| show_inspector.set_active(split.shows_sidebar())
    });
    toolbar.append(&show_inspector);

    let add_track = gtk::Button::builder()
        .icon_name("list-add-symbolic")
        .tooltip_text("Add Mixer track")
        .build();
    toolbar.append(&add_track);
    let layout = gtk::DropDown::from_strings(&["Compact", "Wide", "Extra large"]);
    layout.set_tooltip_text(Some("Mixer strip layout"));
    toolbar.append(&layout);

    let spacer = gtk::Box::new(gtk::Orientation::Horizontal, 0);
    spacer.set_hexpand(true);
    toolbar.append(&spacer);
    toolbar.upcast()
}

fn build_mixer_menu_popover() -> gtk::Popover {
    let content = popover_content("Mixer");
    for (label, icon) in [
        ("Add Track", "list-add-symbolic"),
        ("Dock Inspector Right", "sidebar-show-symbolic"),
        ("Reset Selected Track", "edit-clear-all-symbolic"),
        ("Save Mixer State", "document-save-symbolic"),
    ] {
        content.append(&popover_button(label, icon));
    }
    popover_with_child(&content)
}

#[derive(Clone)]
#[allow(clippy::struct_excessive_bools)]
struct MixerStripSpec {
    mixer_insert_id: MixerInsertId,
    number: String,
    name: String,
    level: f64,
    fader: f64,
    pan: f64,
    muted: bool,
    solo: bool,
    selected: bool,
    master: bool,
}

fn mixer_strip_specs(controller: &AppController) -> Vec<MixerStripSpec> {
    controller
        .session()
        .mixer_inserts
        .iter()
        .enumerate()
        .map(|(index, insert)| MixerStripSpec {
            mixer_insert_id: insert.id,
            number: if insert.is_master {
                "M".into()
            } else {
                index.to_string()
            },
            name: insert.name.clone(),
            level: 0.0,
            fader: f64::from(insert.gain_db),
            pan: f64::from(insert.pan),
            muted: insert.muted,
            solo: insert.solo,
            selected: controller.selection().mixer_insert == Some(insert.id)
                || (controller.selection().mixer_insert.is_none() && insert.is_master),
            master: insert.is_master,
        })
        .collect()
}

fn build_mixer_deck(selected_track: &gtk::Label, bindings: &MixerBindings) -> gtk::Widget {
    let deck = gtk::Box::new(gtk::Orientation::Horizontal, 0);
    deck.append(&build_selected_meter(bindings));
    deck.append(&gtk::Separator::new(gtk::Orientation::Vertical));

    let strip_area = gtk::Box::new(gtk::Orientation::Vertical, 0);
    strip_area.set_hexpand(true);
    let strips = gtk::Box::new(gtk::Orientation::Horizontal, 3);
    strips.set_margin_top(6);
    strips.set_margin_bottom(6);
    strips.set_margin_start(6);
    strips.set_margin_end(6);
    *bindings.strips.borrow_mut() = Some(strips.clone());
    *bindings.selected_track.borrow_mut() = Some(selected_track.clone());
    bindings.rebuild();

    let scroller = gtk::ScrolledWindow::new();
    scroller.set_policy(gtk::PolicyType::Automatic, gtk::PolicyType::Automatic);
    scroller.set_vexpand(true);
    scroller.set_child(Some(&strips));
    strip_area.append(&scroller);
    strip_area.append(&gtk::Separator::new(gtk::Orientation::Horizontal));
    strip_area.append(&build_send_bar());
    deck.append(&strip_area);
    deck.upcast()
}

fn build_selected_meter(bindings: &MixerBindings) -> gtk::Widget {
    let dock = gtk::Box::new(gtk::Orientation::Vertical, 3);
    dock.set_margin_top(6);
    dock.set_margin_bottom(6);
    dock.set_margin_start(6);
    dock.set_margin_end(6);
    let title = gtk::Label::new(Some("OUT"));
    title.add_css_class("caption");
    dock.append(&title);
    let meters = gtk::Box::new(gtk::Orientation::Horizontal, 2);
    bindings.master_meters.borrow_mut().clear();
    for _ in 0..2 {
        let meter = gtk::LevelBar::for_interval(0.0, 1.0);
        meter.set_orientation(gtk::Orientation::Vertical);
        meter.set_inverted(true);
        meter.set_value(0.0);
        meter.set_size_request(8, 110);
        meter.set_vexpand(true);
        meters.append(&meter);
        bindings.master_meters.borrow_mut().push(meter);
    }
    dock.append(&meters);
    let peak = gtk::Label::new(Some("-3.1"));
    peak.add_css_class("caption");
    peak.set_tooltip_text(Some("Selected track peak level in dBFS"));
    dock.append(&peak);
    dock.upcast()
}

#[allow(clippy::too_many_lines)]
fn build_mixer_strip(
    spec: &MixerStripSpec,
    selection_group: &mut Option<gtk::ToggleButton>,
    selected_track: &gtk::Label,
    controller: &Rc<RefCell<AppController>>,
    bindings: &MixerBindings,
) -> gtk::Widget {
    let strip = gtk::Box::new(gtk::Orientation::Vertical, 4);
    strip.set_margin_top(3);
    strip.set_margin_bottom(3);
    strip.set_margin_start(3);
    strip.set_margin_end(3);
    strip.set_size_request(62, -1);

    let select = gtk::ToggleButton::with_label(&spec.number);
    select.set_tooltip_text(Some(&format!("Select {}", spec.name)));
    if let Some(group) = selection_group.as_ref() {
        select.set_group(Some(group));
    } else {
        *selection_group = Some(select.clone());
    }
    select.set_active(spec.selected);
    select.connect_toggled({
        let selected_track = selected_track.clone();
        let name = spec.name.clone();
        let controller = controller.clone();
        let mixer_insert_id = spec.mixer_insert_id;
        move |button| {
            if button.is_active() {
                selected_track.set_label(&name);
                controller.borrow_mut().selection_mut().mixer_insert = Some(mixer_insert_id);
            }
        }
    });
    strip.append(&select);

    let label = gtk::Label::new(Some(&spec.name));
    label.set_ellipsize(gtk::pango::EllipsizeMode::End);
    label.set_tooltip_text(Some(&spec.name));
    label.set_width_chars(7);
    strip.append(&label);

    let level = gtk::Box::new(gtk::Orientation::Horizontal, 2);
    let meter = gtk::LevelBar::for_interval(0.0, 1.0);
    meter.set_orientation(gtk::Orientation::Vertical);
    meter.set_inverted(true);
    meter.set_value(spec.level);
    meter.set_vexpand(true);
    meter.set_size_request(8, 90);
    level.append(&meter);
    let fader = gtk::Scale::with_range(gtk::Orientation::Vertical, -60.0, 6.0, 1.0);
    fader.set_value(spec.fader);
    fader.set_inverted(true);
    fader.set_draw_value(false);
    fader.set_vexpand(true);
    fader.set_tooltip_text(Some(&format!("{} level", spec.name)));
    install_mixer_fader_commit(&fader, spec, controller, bindings);
    level.append(&fader);
    strip.append(&level);

    let mute = gtk::ToggleButton::with_label("M");
    mute.set_tooltip_text(Some(&format!("Mute {}", spec.name)));
    mute.set_active(spec.muted);
    let pan = gtk::ScaleButton::new(
        -1.0,
        1.0,
        0.01,
        &["audio-volume-low-symbolic", "audio-volume-high-symbolic"],
    );
    pan.set_value(spec.pan);
    pan.set_tooltip_text(Some(&format!("{} panning", spec.name)));
    install_mixer_pan_mute(&pan, &mute, spec.mixer_insert_id, controller, bindings);
    let primary_controls = gtk::Box::new(gtk::Orientation::Horizontal, 2);
    let solo = gtk::ToggleButton::with_label("S");
    solo.set_tooltip_text(Some(&format!("Solo {}", spec.name)));
    solo.set_active(spec.solo);
    solo.connect_toggled({
        let controller = controller.clone();
        let mixer_insert_id = spec.mixer_insert_id;
        move |button| {
            let edit = Transaction::one(
                "Toggle mixer solo",
                Command::SetMixerInsertSolo {
                    mixer_insert_id,
                    solo: button.is_active(),
                },
            );
            let _ = controller.borrow_mut().execute(&edit);
        }
    });
    primary_controls.append(&solo);
    primary_controls.append(&mute);
    primary_controls.append(&pan);
    strip.append(&primary_controls);

    let utilities = build_mixer_utilities();
    for (column, label, tooltip) in [
        (0, "Ø", "Invert phase"),
        (1, "↔", "Swap stereo channels"),
        (2, "FX", "Disable all effects"),
    ] {
        let control = gtk::ToggleButton::with_label(label);
        control.set_tooltip_text(Some(tooltip));
        utilities.attach(&control, column, 0, 1, 1);
    }
    let latency = gtk::ToggleButton::with_label("L");
    latency.set_active(true);
    latency.set_tooltip_text(Some("Plugin delay compensation"));
    utilities.attach(&latency, 0, 1, 1, 1);
    let record = gtk::ToggleButton::with_label("R");
    record.set_tooltip_text(Some("Record arm"));
    utilities.attach(&record, 1, 1, 1, 1);
    let send = gtk::Button::with_label("▲");
    send.set_tooltip_text(Some("Route or sidechain selected track here"));
    utilities.attach(&send, 2, 1, 1, 1);
    strip.append(&utilities);

    let frame = gtk::Frame::new(None);
    if spec.master {
        frame.set_tooltip_text(Some("Master output strip"));
    }
    frame.set_child(Some(&strip));
    bindings.controls.borrow_mut().push(MixerControlBinding {
        mixer_insert_id: spec.mixer_insert_id,
        fader,
        pan,
        mute,
    });
    frame.upcast()
}

fn install_mixer_pan_mute(
    pan: &gtk::ScaleButton,
    mute: &gtk::ToggleButton,
    mixer_insert_id: MixerInsertId,
    controller: &Rc<RefCell<AppController>>,
    bindings: &MixerBindings,
) {
    pan.connect_value_changed({
        let controller = controller.clone();
        let syncing = bindings.syncing.clone();
        move |_, value| {
            if syncing.get() {
                return;
            }
            update_mixer_controls(
                &controller,
                mixer_insert_id,
                None,
                Some(f64_to_f32(value)),
                None,
            );
        }
    });
    mute.connect_toggled({
        let controller = controller.clone();
        let syncing = bindings.syncing.clone();
        move |button| {
            if syncing.get() {
                return;
            }
            update_mixer_controls(
                &controller,
                mixer_insert_id,
                None,
                None,
                Some(button.is_active()),
            );
        }
    });
}

fn build_mixer_utilities() -> gtk::Grid {
    gtk::Grid::builder()
        .column_spacing(2)
        .row_spacing(2)
        .build()
}

fn install_mixer_fader_commit(
    fader: &gtk::Scale,
    spec: &MixerStripSpec,
    controller: &Rc<RefCell<AppController>>,
    bindings: &MixerBindings,
) {
    let drag_start = Rc::new(Cell::new(None::<f64>));
    let gesture = gtk::GestureClick::new();
    gesture.set_button(gdk::BUTTON_PRIMARY);
    gesture.connect_pressed({
        let drag_start = drag_start.clone();
        let fader = fader.clone();
        move |_, _, _, _| drag_start.set(Some(fader.value()))
    });
    gesture.connect_released({
        let drag_start = drag_start.clone();
        let fader = fader.clone();
        let controller = controller.clone();
        let bindings = bindings.clone();
        let mixer_insert_id = spec.mixer_insert_id;
        move |_, _, _, _| {
            let previous = drag_start.take();
            if previous.is_some_and(|value| (value - fader.value()).abs() > f64::EPSILON) {
                update_mixer_controls(
                    &controller,
                    mixer_insert_id,
                    Some(f64_to_f32(fader.value())),
                    None,
                    None,
                );
                bindings.sync(&controller.borrow());
            }
        }
    });
    fader.add_controller(gesture);
}

fn update_mixer_controls(
    controller: &Rc<RefCell<AppController>>,
    mixer_insert_id: MixerInsertId,
    gain_db: Option<f32>,
    pan: Option<f32>,
    muted: Option<bool>,
) {
    let controls = controller
        .borrow()
        .session()
        .mixer_inserts
        .iter()
        .find(|insert| insert.id == mixer_insert_id)
        .map(|insert| mixer::merged_controls(insert, gain_db, pan, muted));
    if let Some((gain_db, pan, muted)) = controls {
        let edit = Transaction::one(
            "Adjust Mixer insert",
            Command::SetMixerInsertControls {
                mixer_insert_id,
                gain_db,
                pan,
                muted,
            },
        );
        if gain_db.is_finite() && pan.is_finite() {
            let coalesce_pan = {
                let controller = controller.borrow();
                controller
                    .session()
                    .mixer_inserts
                    .iter()
                    .find(|insert| insert.id == mixer_insert_id)
                    .is_some_and(|insert| {
                        (insert.gain_db - gain_db).abs() <= f32::EPSILON
                            && (insert.pan - pan).abs() > f32::EPSILON
                            && insert.muted == muted
                    })
            };
            if coalesce_pan {
                if let Ok(revision) = controller
                    .borrow_mut()
                    .execute_coalesced(format!("mixer-pan-{mixer_insert_id:?}"), &edit)
                {
                    schedule_end_coalescing(controller, revision);
                }
            } else {
                let _ = controller.borrow_mut().execute(&edit);
            }
        }
    }
}

fn build_send_bar() -> gtk::Widget {
    let routing = gtk::Box::new(gtk::Orientation::Horizontal, 4);
    routing.set_margin_top(4);
    routing.set_margin_bottom(4);
    routing.set_margin_start(6);
    routing.set_margin_end(6);
    let label = gtk::Label::new(Some("Route selected to"));
    label.add_css_class("dim-label");
    routing.append(&label);
    let destination = gtk::DropDown::from_strings(&["Master", "Send A", "Send B", "Insert 7"]);
    destination.set_tooltip_text(Some("Send destination"));
    routing.append(&destination);
    let amount = gtk::Scale::with_range(gtk::Orientation::Horizontal, 0.0, 1.0, 0.01);
    amount.set_value(0.75);
    amount.set_draw_value(false);
    amount.set_hexpand(true);
    amount.set_tooltip_text(Some("Audio send level"));
    routing.append(&amount);
    let sidechain = gtk::ToggleButton::with_label("Sidechain");
    sidechain.set_tooltip_text(Some("Create a sidechain-only route"));
    routing.append(&sidechain);
    let send = gtk::Button::with_label("Send");
    routing.append(&send);
    let scroller = gtk::ScrolledWindow::new();
    scroller.set_policy(gtk::PolicyType::Automatic, gtk::PolicyType::Never);
    scroller.set_propagate_natural_height(true);
    scroller.set_child(Some(&routing));
    scroller.upcast()
}

fn build_mixer_inspector(inspector_split: &adw::OverlaySplitView) -> (gtk::Widget, gtk::Label) {
    let inspector = gtk::Box::new(gtk::Orientation::Vertical, 0);
    let heading = gtk::Box::new(gtk::Orientation::Horizontal, 4);
    heading.set_margin_top(6);
    heading.set_margin_bottom(6);
    heading.set_margin_start(8);
    heading.set_margin_end(6);
    let selected_track = gtk::Label::new(Some("Drums"));
    selected_track.add_css_class("heading");
    selected_track.set_xalign(0.0);
    selected_track.set_hexpand(true);
    heading.append(&selected_track);
    let close = gtk::Button::builder()
        .icon_name("window-close-symbolic")
        .tooltip_text("Hide track inspector")
        .build();
    close.add_css_class("flat");
    close.connect_clicked({
        let inspector_split = inspector_split.clone();
        move |_| inspector_split.set_show_sidebar(false)
    });
    heading.append(&close);
    inspector.append(&heading);
    inspector.append(&gtk::Separator::new(gtk::Orientation::Horizontal));

    let content = gtk::Box::new(gtk::Orientation::Vertical, 8);
    content.set_margin_top(8);
    content.set_margin_bottom(8);
    content.set_margin_start(8);
    content.set_margin_end(8);
    content.append(&build_inspector_io_row(
        "Audio Input",
        &["None", "Input 1", "Input 2", "Input 1 + 2"],
        true,
    ));
    content.append(&build_fx_chain());
    content.append(&build_track_properties());
    content.append(&build_inspector_io_row(
        "Audio Output",
        &["Master", "Out 1 + 2", "Out 3 + 4", "None"],
        false,
    ));

    let scroller = gtk::ScrolledWindow::new();
    scroller.set_policy(gtk::PolicyType::Never, gtk::PolicyType::Automatic);
    scroller.set_vexpand(true);
    scroller.set_child(Some(&content));
    inspector.append(&scroller);
    (inspector.upcast(), selected_track)
}

fn build_inspector_io_row(label: &str, choices: &[&str], recording_input: bool) -> gtk::Widget {
    let row = gtk::Box::new(gtk::Orientation::Horizontal, 4);
    let dropdown = gtk::DropDown::from_strings(choices);
    dropdown.set_hexpand(true);
    dropdown.set_tooltip_text(Some(label));
    row.append(&dropdown);
    if recording_input {
        let position = gtk::DropDown::from_strings(&["Pre FX", "Post FX"]);
        position.set_tooltip_text(Some("Recording position in the effects chain"));
        row.append(&position);
        let latency = gtk::Button::builder()
            .icon_name("preferences-system-time-symbolic")
            .tooltip_text("Input latency control")
            .build();
        row.append(&latency);
    }
    row.upcast()
}

fn build_fx_chain() -> gtk::Widget {
    let section = gtk::Box::new(gtk::Orientation::Vertical, 4);
    let title = gtk::Label::new(Some("Effects"));
    title.add_css_class("heading");
    title.set_xalign(0.0);
    section.append(&title);
    let effects = gtk::ListBox::new();
    effects.set_selection_mode(gtk::SelectionMode::None);
    effects.add_css_class("boxed-list");
    for (name, enabled, wet) in [
        ("Slot 1", false, 1.0),
        ("Compressor", true, 0.72),
        ("Reverb", true, 0.38),
        ("Slot 4", false, 1.0),
        ("Limiter", true, 0.86),
    ] {
        let row = gtk::Box::new(gtk::Orientation::Horizontal, 4);
        row.set_margin_top(4);
        row.set_margin_bottom(4);
        row.set_margin_start(6);
        row.set_margin_end(6);
        let bypass = gtk::CheckButton::new();
        bypass.set_active(enabled);
        bypass.set_tooltip_text(Some("Enable or bypass this effect"));
        row.append(&bypass);
        let plugin = gtk::Button::with_label(name);
        plugin.add_css_class("flat");
        plugin.set_hexpand(true);
        plugin.set_halign(gtk::Align::Fill);
        row.append(&plugin);
        let wet_mix = gtk::ScaleButton::new(
            0.0,
            1.0,
            0.01,
            &["audio-volume-low-symbolic", "audio-volume-high-symbolic"],
        );
        wet_mix.set_value(wet);
        wet_mix.set_tooltip_text(Some("Effect wet mix"));
        row.append(&wet_mix);
        effects.append(&row);
    }
    section.append(&effects);
    section.upcast()
}

fn build_track_properties() -> gtk::Widget {
    let section = gtk::Box::new(gtk::Orientation::Vertical, 4);
    let title = gtk::Label::new(Some("Track Properties"));
    title.add_css_class("heading");
    title.set_xalign(0.0);
    section.append(&title);

    let equalizer = gtk::DrawingArea::new();
    equalizer.set_content_height(92);
    equalizer.set_hexpand(true);
    equalizer.set_draw_func(|widget, context, width, height| {
        let foreground = widget.color();
        let style = adw::StyleManager::for_display(&widget.display());
        let accent = style.accent_color_rgba();
        set_cairo_color(context, &foreground, 0.12);
        context.paint().expect("paint equalizer background");
        set_cairo_color(context, &foreground, 0.24);
        context.set_line_width(1.0);
        for column in 1..4 {
            let x = f64::from(width * column / 4) + 0.5;
            context.move_to(x, 0.0);
            context.line_to(x, f64::from(height));
        }
        context.stroke().expect("draw equalizer grid");
        set_cairo_color(context, &accent, 0.9);
        context.set_line_width(2.0);
        context.move_to(0.0, f64::from(height) * 0.48);
        context.curve_to(
            f64::from(width) * 0.28,
            f64::from(height) * 0.48,
            f64::from(width) * 0.35,
            f64::from(height) * 0.82,
            f64::from(width) * 0.5,
            f64::from(height) * 0.82,
        );
        context.curve_to(
            f64::from(width) * 0.65,
            f64::from(height) * 0.82,
            f64::from(width) * 0.72,
            f64::from(height) * 0.48,
            f64::from(width),
            f64::from(height) * 0.48,
        );
        context.stroke().expect("draw equalizer response");
    });
    let frame = gtk::Frame::new(None);
    frame.set_child(Some(&equalizer));
    section.append(&frame);

    let pan = inspector_property_scale("Pan", -1.0, 1.0, 0.0);
    section.append(&pan);
    let width = inspector_property_scale("Stereo Width", 0.0, 2.0, 1.0);
    section.append(&width);
    let pdc = gtk::Box::new(gtk::Orientation::Horizontal, 4);
    pdc.append(&gtk::Label::new(Some("PDC")));
    let delay = gtk::SpinButton::with_range(0.0, 10_000.0, 1.0);
    delay.set_value(0.0);
    delay.set_tooltip_text(Some("Manual plugin delay compensation in samples"));
    delay.set_hexpand(true);
    pdc.append(&delay);
    section.append(&pdc);
    section.upcast()
}

fn inspector_property_scale(label: &str, min: f64, max: f64, value: f64) -> gtk::Widget {
    let row = gtk::Box::new(gtk::Orientation::Horizontal, 4);
    let title = gtk::Label::new(Some(label));
    title.set_width_chars(11);
    title.set_xalign(0.0);
    row.append(&title);
    let scale = gtk::Scale::with_range(gtk::Orientation::Horizontal, min, max, 0.01);
    scale.set_value(value);
    scale.set_draw_value(false);
    scale.set_hexpand(true);
    row.append(&scale);
    row.upcast()
}

fn open_piano_roll(application: &adw::Application, controller: &Rc<RefCell<AppController>>) {
    let window = adw::ApplicationWindow::builder()
        .application(application)
        .title("Piano Roll — Pattern 1")
        .default_width(1_180)
        .default_height(720)
        .resizable(true)
        .build();
    let toolbar = adw::ToolbarView::new();
    let header = adw::HeaderBar::new();
    header.set_title_widget(Some(&adw::WindowTitle::new("Piano Roll", "Pattern 1")));
    toolbar.add_top_bar(&header);
    let piano_roll = build_piano_roll(controller);
    toolbar.add_top_bar(&piano_roll.toolbar);
    toolbar.set_content(Some(&piano_roll.editor));
    window.set_content(Some(&toolbar));
    add_piano_history_actions(application, &window, controller, &piano_roll.bindings);
    window.present();
}

fn add_piano_history_actions(
    application: &adw::Application,
    window: &adw::ApplicationWindow,
    controller: &Rc<RefCell<AppController>>,
    bindings: &PianoRollBindings,
) {
    let undo = gio::SimpleAction::new("undo", None);
    let undo_controller = controller.clone();
    let undo_bindings = bindings.clone();
    undo.connect_activate(move |_, _| {
        if matches!(undo_controller.borrow_mut().undo(), Ok(true)) {
            undo_bindings.sync(&undo_controller.borrow());
        }
    });
    window.add_action(&undo);
    application.set_accels_for_action("win.undo", &["<Control>z"]);

    let redo = gio::SimpleAction::new("redo", None);
    let redo_controller = controller.clone();
    let redo_bindings = bindings.clone();
    redo.connect_activate(move |_, _| {
        if matches!(redo_controller.borrow_mut().redo(), Ok(true)) {
            redo_bindings.sync(&redo_controller.borrow());
        }
    });
    window.add_action(&redo);
    application.set_accels_for_action("win.redo", &["<Control><Shift>z", "<Control>y"]);
}

struct PianoRollUi {
    toolbar: gtk::ScrolledWindow,
    editor: gtk::Widget,
    bindings: PianoRollBindings,
}

#[derive(Clone, Copy, PartialEq)]
enum PianoNoteKind {
    Normal,
    Slide,
    Portamento,
}

#[derive(Clone, Copy)]
struct PianoNote {
    note_id: NoteId,
    pattern_id: PatternId,
    channel_id: ChannelId,
    pitch: i32,
    start: f64,
    length: f64,
    velocity: f64,
    kind: PianoNoteKind,
    selected: bool,
}

#[derive(Clone, Copy)]
struct PianoRollView {
    beat_width: f64,
    key_height: f64,
}

#[derive(Clone, Copy)]
struct NoteResize {
    note_id: NoteId,
    original_length: Tick,
}

#[derive(Clone)]
struct PianoRollBindings {
    notes: Rc<RefCell<Vec<PianoNote>>>,
    grid: gtk::DrawingArea,
    velocity_editor: gtk::DrawingArea,
}

impl PianoRollBindings {
    fn sync(&self, controller: &AppController) {
        *self.notes.borrow_mut() = piano_note_views(controller);
        self.grid.queue_draw();
        self.velocity_editor.queue_draw();
    }
}

fn piano_target(controller: &AppController) -> Option<(PatternId, ChannelId)> {
    let pattern = controller
        .selection()
        .pattern
        .and_then(|id| {
            controller
                .session()
                .patterns
                .iter()
                .find(|pattern| pattern.id == id)
        })
        .or_else(|| controller.session().patterns.first())?;
    let lane = controller
        .selection()
        .channel
        .and_then(|id| pattern.lanes.iter().find(|lane| lane.channel_id == id))
        .or_else(|| pattern.lanes.iter().find(|lane| !lane.notes.is_empty()))
        .or_else(|| pattern.lanes.first())?;
    Some((pattern.id, lane.channel_id))
}

fn piano_note_views(controller: &AppController) -> Vec<PianoNote> {
    let Some((pattern_id, channel_id)) = piano_target(controller) else {
        return Vec::new();
    };
    controller
        .session()
        .patterns
        .iter()
        .find(|pattern| pattern.id == pattern_id)
        .and_then(|pattern| {
            pattern
                .lanes
                .iter()
                .find(|lane| lane.channel_id == channel_id)
        })
        .map_or_else(Vec::new, |lane| {
            lane.notes
                .iter()
                .map(|note| PianoNote {
                    note_id: note.id,
                    pattern_id,
                    channel_id,
                    pitch: i32::from(note.pitch),
                    start: ticks_to_beats(note.start),
                    length: ticks_to_beats(note.length),
                    velocity: f64::from(note.velocity) / 127.0,
                    kind: PianoNoteKind::Normal,
                    selected: controller.selection().note == Some(note.id),
                })
                .collect()
        })
}

#[allow(clippy::too_many_lines)]
fn build_piano_roll(controller: &Rc<RefCell<AppController>>) -> PianoRollUi {
    let notes = Rc::new(RefCell::new(piano_note_views(&controller.borrow())));
    let view = Rc::new(RefCell::new(PianoRollView {
        beat_width: controller
            .borrow()
            .presentation()
            .piano_roll_horizontal_zoom,
        key_height: controller.borrow().presentation().piano_roll_vertical_zoom,
    }));
    let note_kind = Rc::new(RefCell::new(PianoNoteKind::Normal));
    let note_resize = Rc::new(RefCell::new(None::<NoteResize>));

    let grid = gtk::DrawingArea::new();
    grid.set_content_width(6_144);
    grid.set_content_height(1_612);
    grid.set_focusable(true);
    grid.set_accessible_role(gtk::AccessibleRole::Grid);
    grid.set_tooltip_text(Some(
        "Piano Roll. Use arrow keys to move selected notes; drag note edges to resize.",
    ));
    grid.set_draw_func({
        let notes = notes.clone();
        let view = view.clone();
        move |widget, context, width, height| {
            draw_piano_roll_grid(
                widget,
                context,
                width,
                height,
                &notes.borrow(),
                *view.borrow(),
            );
        }
    });

    let velocity_editor = gtk::DrawingArea::new();
    velocity_editor.set_content_width(6_144);
    velocity_editor.set_content_height(170);
    velocity_editor.set_accessible_role(gtk::AccessibleRole::Slider);
    velocity_editor.set_tooltip_text(Some(
        "Note velocity editor. Drag points vertically to change velocity.",
    ));
    velocity_editor.set_draw_func({
        let notes = notes.clone();
        let view = view.clone();
        move |widget, context, width, height| {
            draw_velocity_editor(
                widget,
                context,
                width,
                height,
                &notes.borrow(),
                *view.borrow(),
            );
        }
    });
    let bindings = PianoRollBindings {
        notes: notes.clone(),
        grid: grid.clone(),
        velocity_editor: velocity_editor.clone(),
    };
    let velocity_click = gtk::GestureClick::new();
    velocity_click.set_button(gdk::BUTTON_PRIMARY);
    velocity_click.connect_pressed({
        let notes = notes.clone();
        let view = view.clone();
        let grid = grid.clone();
        let velocity_editor = velocity_editor.clone();
        let controller = controller.clone();
        let bindings = bindings.clone();
        move |_, _, x, y| {
            let target_start = x / view.borrow().beat_width;
            let new_velocity = (1.0 - y / f64::from(velocity_editor.height())).clamp(0.0, 1.0);
            let closest = notes
                .borrow()
                .iter()
                .enumerate()
                .min_by(|(_, left), (_, right)| {
                    (left.start - target_start)
                        .abs()
                        .total_cmp(&(right.start - target_start).abs())
                })
                .map(|(index, _)| index);
            if let Some(index) = closest {
                let mut notes = notes.borrow_mut();
                for note in notes.iter_mut() {
                    note.selected = false;
                }
                notes[index].selected = true;
                notes[index].velocity = new_velocity;
                let changed = notes[index];
                drop(notes);
                let edit = Transaction::one(
                    "Change note velocity",
                    Command::SetPatternNote {
                        pattern_id: changed.pattern_id,
                        channel_id: changed.channel_id,
                        note_id: changed.note_id,
                        note: Some(piano_domain_note(changed)),
                    },
                );
                let _ = controller.borrow_mut().execute(&edit);
                controller.borrow_mut().selection_mut().note = Some(changed.note_id);
                bindings.sync(&controller.borrow());
                grid.queue_draw();
                velocity_editor.queue_draw();
            }
        }
    });
    velocity_editor.add_controller(velocity_click);

    let click = gtk::GestureClick::new();
    click.set_button(0);
    click.connect_pressed({
        let notes = notes.clone();
        let view = view.clone();
        let grid = grid.clone();
        let velocity_editor = velocity_editor.clone();
        let controller = controller.clone();
        let bindings = bindings.clone();
        move |gesture, _, x, y| {
            let view = *view.borrow();
            if y < 28.0 {
                return;
            }
            let pitch = 96 - f64_to_i32_floor((y - 28.0) / view.key_height);
            let start = ((x / view.beat_width) * 4.0).floor() / 4.0;
            if gesture.current_button() == gdk::BUTTON_SECONDARY {
                if let Some(note) = notes
                    .borrow()
                    .iter()
                    .find(|note| {
                        note.pitch == pitch
                            && start >= note.start
                            && start < note.start + note.length
                    })
                    .copied()
                {
                    let edit = Transaction::one(
                        "Delete Piano Roll note",
                        Command::SetPatternNote {
                            pattern_id: note.pattern_id,
                            channel_id: note.channel_id,
                            note_id: note.note_id,
                            note: None,
                        },
                    );
                    let _ = controller.borrow_mut().execute(&edit);
                    controller.borrow_mut().selection_mut().note = None;
                }
            } else if piano_note_resize_target(&notes.borrow(), view, x, y).is_some() {
                return;
            } else if let Some(index) = piano_note_at(&notes.borrow(), view, x, y) {
                let selected = notes.borrow()[index];
                let mut controller = controller.borrow_mut();
                controller.selection_mut().pattern = Some(selected.pattern_id);
                controller.selection_mut().channel = Some(selected.channel_id);
                controller.selection_mut().note = Some(selected.note_id);
            } else {
                let target = {
                    let controller = controller.borrow();
                    piano_target(&controller)
                };
                if let Some((pattern_id, channel_id)) = target {
                    let note = Note {
                        id: NoteId::new(),
                        start: beats_to_ticks(start),
                        length: Tick::beats(1),
                        pitch: u8::try_from(pitch.clamp(0, 127)).expect("clamped MIDI pitch"),
                        velocity: 102,
                        channel: 0,
                    };
                    let edit = Transaction::one(
                        "Add Piano Roll note",
                        Command::SetPatternNote {
                            pattern_id,
                            channel_id,
                            note_id: note.id,
                            note: Some(note),
                        },
                    );
                    let _ = controller.borrow_mut().execute(&edit);
                    let mut controller = controller.borrow_mut();
                    controller.selection_mut().pattern = Some(pattern_id);
                    controller.selection_mut().channel = Some(channel_id);
                    controller.selection_mut().note = Some(note.id);
                }
            }
            bindings.sync(&controller.borrow());
            grid.grab_focus();
            grid.queue_draw();
            velocity_editor.queue_draw();
        }
    });
    grid.add_controller(click);

    let note_shortcuts = gtk::EventControllerKey::new();
    note_shortcuts.connect_key_pressed({
        let controller = controller.clone();
        let bindings = bindings.clone();
        move |_, key, _, modifiers| {
            let target = {
                let controller = controller.borrow();
                let Some((pattern_id, channel_id)) = piano_target(&controller) else {
                    return gtk::glib::Propagation::Proceed;
                };
                let Some(note_id) = controller.selection().note else {
                    return gtk::glib::Propagation::Proceed;
                };
                controller
                    .session()
                    .patterns
                    .iter()
                    .find(|pattern| pattern.id == pattern_id)
                    .and_then(|pattern| {
                        pattern
                            .lanes
                            .iter()
                            .find(|lane| lane.channel_id == channel_id)
                    })
                    .and_then(|lane| lane.notes.iter().find(|note| note.id == note_id).copied())
                    .map(|note| (pattern_id, channel_id, note))
            };
            let Some((pattern_id, channel_id, note)) = target else {
                return gtk::glib::Propagation::Proceed;
            };
            let command = if key == gdk::Key::Left
                || key == gdk::Key::Right
                || key == gdk::Key::Up
                || key == gdk::Key::Down
            {
                let tick_delta = if key == gdk::Key::Left {
                    -240
                } else if key == gdk::Key::Right {
                    240
                } else {
                    0
                };
                let pitch_delta = if key == gdk::Key::Up {
                    1
                } else if key == gdk::Key::Down {
                    -1
                } else {
                    0
                };
                let moved = Note {
                    start: Tick((note.start.0 + tick_delta).max(0)),
                    pitch: u8::try_from((i32::from(note.pitch) + pitch_delta).clamp(0, 127))
                        .expect("clamped MIDI pitch"),
                    ..note
                };
                Command::SetPatternNote {
                    pattern_id,
                    channel_id,
                    note_id: note.id,
                    note: Some(moved),
                }
            } else if key == gdk::Key::Delete || key == gdk::Key::BackSpace {
                Command::SetPatternNote {
                    pattern_id,
                    channel_id,
                    note_id: note.id,
                    note: None,
                }
            } else if key == gdk::Key::d && modifiers.contains(gdk::ModifierType::CONTROL_MASK) {
                let copy = Note {
                    id: NoteId::new(),
                    start: Tick(note.start.0 + note.length.0),
                    ..note
                };
                Command::SetPatternNote {
                    pattern_id,
                    channel_id,
                    note_id: copy.id,
                    note: Some(copy),
                }
            } else {
                return gtk::glib::Propagation::Proceed;
            };
            if controller
                .borrow_mut()
                .execute(&Transaction::one("Edit Piano Roll note", command))
                .is_ok()
            {
                if key == gdk::Key::Delete || key == gdk::Key::BackSpace {
                    controller.borrow_mut().selection_mut().note = None;
                }
                bindings.sync(&controller.borrow());
            }
            gtk::glib::Propagation::Stop
        }
    });
    grid.add_controller(note_shortcuts);

    let resize_drag = gtk::GestureDrag::new();
    resize_drag.set_button(gdk::BUTTON_PRIMARY);
    resize_drag.connect_drag_begin({
        let notes = notes.clone();
        let view = view.clone();
        let note_resize = note_resize.clone();
        let controller = controller.clone();
        move |gesture, x, y| {
            let target =
                piano_note_resize_target(&notes.borrow(), *view.borrow(), x, y).map(|index| {
                    let note = notes.borrow()[index];
                    NoteResize {
                        note_id: note.note_id,
                        original_length: beats_to_ticks(note.length),
                    }
                });
            *note_resize.borrow_mut() = target;
            if let Some(target) = target {
                controller.borrow_mut().selection_mut().note = Some(target.note_id);
                gesture.set_state(gtk::EventSequenceState::Claimed);
            }
        }
    });
    resize_drag.connect_drag_update({
        let notes = notes.clone();
        let view = view.clone();
        let note_resize = note_resize.clone();
        let grid = grid.clone();
        let velocity_editor = velocity_editor.clone();
        move |gesture, offset_x, _| {
            let Some(resize) = *note_resize.borrow() else {
                return;
            };
            let view = *view.borrow();
            let mut notes = notes.borrow_mut();
            let Some(note) = notes.iter_mut().find(|note| note.note_id == resize.note_id) else {
                return;
            };
            let original_length = ticks_to_beats(resize.original_length);
            let raw_end = note.start + original_length + offset_x / view.beat_width;
            let bypass_snap = gesture
                .current_event_state()
                .contains(gdk::ModifierType::ALT_MASK);
            let snapped_end = snap_timeline_value(raw_end, 0.25, bypass_snap);
            note.length = (snapped_end - note.start).max(if bypass_snap { 0.02 } else { 0.25 });
            note.selected = true;
            drop(notes);
            grid.queue_draw();
            velocity_editor.queue_draw();
        }
    });
    resize_drag.connect_drag_end({
        let note_resize = note_resize.clone();
        let notes = notes.clone();
        let controller = controller.clone();
        let bindings = bindings.clone();
        move |_, _, _| {
            let Some(resize) = note_resize.borrow_mut().take() else {
                return;
            };
            let changed = notes
                .borrow()
                .iter()
                .find(|note| note.note_id == resize.note_id)
                .copied();
            if let Some(changed) = changed {
                let new_length = beats_to_ticks(changed.length);
                if new_length != resize.original_length {
                    let edit = Transaction::one(
                        "Resize Piano Roll note",
                        Command::SetPatternNote {
                            pattern_id: changed.pattern_id,
                            channel_id: changed.channel_id,
                            note_id: changed.note_id,
                            note: Some(piano_domain_note(changed)),
                        },
                    );
                    let _ = controller.borrow_mut().execute(&edit);
                }
            }
            bindings.sync(&controller.borrow());
        }
    });
    grid.add_controller(resize_drag);

    let resize_cursor = gtk::EventControllerMotion::new();
    resize_cursor.connect_motion({
        let notes = notes.clone();
        let view = view.clone();
        move |controller, x, y| {
            let cursor =
                if piano_note_resize_target(&notes.borrow(), *view.borrow(), x, y).is_some() {
                    Some("ew-resize")
                } else {
                    None
                };
            if let Some(widget) = controller.widget() {
                widget.set_cursor_from_name(cursor);
            }
        }
    });
    resize_cursor.connect_leave(|controller| {
        if let Some(widget) = controller.widget() {
            widget.set_cursor_from_name(None);
        }
    });
    grid.add_controller(resize_cursor);

    let keyboard = build_piano_keyboard(&view);
    let note_scroller = gtk::ScrolledWindow::new();
    note_scroller.set_policy(gtk::PolicyType::Automatic, gtk::PolicyType::Automatic);
    note_scroller.set_hexpand(true);
    note_scroller.set_vexpand(true);
    note_scroller.set_child(Some(&grid));
    let keyboard_scroller = gtk::ScrolledWindow::new();
    keyboard_scroller.set_policy(gtk::PolicyType::Never, gtk::PolicyType::External);
    keyboard_scroller.set_child(Some(&keyboard));
    keyboard_scroller.set_vadjustment(Some(&note_scroller.vadjustment()));

    let upper = gtk::Box::new(gtk::Orientation::Horizontal, 0);
    upper.append(&keyboard_scroller);
    upper.append(&gtk::Separator::new(gtk::Orientation::Vertical));
    upper.append(&note_scroller);

    let event_target = gtk::Box::new(gtk::Orientation::Vertical, 4);
    event_target.set_size_request(90, -1);
    event_target.set_margin_top(6);
    event_target.set_margin_start(6);
    event_target.set_margin_end(6);
    let control_label = gtk::Label::new(Some("Control"));
    control_label.add_css_class("heading");
    control_label.set_xalign(0.0);
    event_target.append(&control_label);
    let target = gtk::DropDown::from_strings(&[
        "Velocity",
        "Pan",
        "Release",
        "Pitch",
        "Mod X",
        "Mod Y",
        "Automation",
    ]);
    target.set_tooltip_text(Some("Event editor target"));
    event_target.append(&target);

    let velocity_scroller = gtk::ScrolledWindow::new();
    velocity_scroller.set_policy(gtk::PolicyType::External, gtk::PolicyType::Never);
    velocity_scroller.set_hexpand(true);
    velocity_scroller.set_child(Some(&velocity_editor));
    velocity_scroller.set_hadjustment(Some(&note_scroller.hadjustment()));
    let lower = gtk::Box::new(gtk::Orientation::Horizontal, 0);
    lower.append(&event_target);
    lower.append(&gtk::Separator::new(gtk::Orientation::Vertical));
    lower.append(&velocity_scroller);

    let editor = gtk::Paned::new(gtk::Orientation::Vertical);
    editor.set_start_child(Some(&upper));
    editor.set_end_child(Some(&lower));
    editor.set_resize_start_child(true);
    editor.set_resize_end_child(false);
    editor.set_shrink_start_child(false);
    editor.set_shrink_end_child(true);
    editor.set_position(470);

    let toolbar = build_piano_roll_toolbar(
        &grid,
        &keyboard,
        &velocity_editor,
        &view,
        &note_kind,
        controller,
    );
    PianoRollUi {
        toolbar,
        editor: editor.upcast(),
        bindings,
    }
}

#[allow(clippy::too_many_lines)]
fn build_piano_roll_toolbar(
    grid: &gtk::DrawingArea,
    keyboard: &gtk::DrawingArea,
    velocity_editor: &gtk::DrawingArea,
    view: &Rc<RefCell<PianoRollView>>,
    note_kind: &Rc<RefCell<PianoNoteKind>>,
    controller: &Rc<RefCell<AppController>>,
) -> gtk::ScrolledWindow {
    let controls = gtk::Box::new(gtk::Orientation::Horizontal, 4);
    controls.set_margin_top(5);
    controls.set_margin_bottom(5);
    controls.set_margin_start(8);
    controls.set_margin_end(8);
    controls.add_css_class("toolbar");

    let main_menu = gtk::MenuButton::builder()
        .icon_name("open-menu-symbolic")
        .tooltip_text("Piano Roll menu")
        .popover(&build_piano_roll_menu())
        .build();
    controls.append(&main_menu);

    let tools_menu = gtk::MenuButton::builder()
        .icon_name("applications-engineering-symbolic")
        .tooltip_text("Piano Roll tools")
        .popover(&build_piano_tools_menu())
        .build();
    controls.append(&tools_menu);

    let draw = piano_tool_button("D", "Draw notes");
    draw.set_active(true);
    controls.append(&draw);
    for (label, tooltip) in [
        ("P", "Paint repeated notes"),
        ("✂", "Slice notes"),
        ("E", "Erase notes"),
        ("S", "Select notes"),
        ("Z", "Zoom tool"),
        ("▶", "Playback and scrub tool"),
    ] {
        let tool = piano_tool_button(label, tooltip);
        tool.set_group(Some(&draw));
        controls.append(&tool);
    }
    controls.append(&gtk::Separator::new(gtk::Orientation::Vertical));

    let snap = gtk::DropDown::from_strings(&[
        "Main", "Line", "Beat", "1/2 beat", "1/4 beat", "1/6 beat", "Step", "None",
    ]);
    snap.set_selected(4);
    snap.set_tooltip_text(Some("Snap notes and events to the grid"));
    controls.append(&snap);

    let color = gtk::DropDown::from_strings(&[
        "Color 1", "Color 2", "Color 3", "Color 4", "Color 5", "Color 6", "Color 7", "Color 8",
    ]);
    color.set_tooltip_text(Some("Note color and MIDI channel group"));
    controls.append(&color);

    let normal = gtk::ToggleButton::with_label("N");
    normal.set_tooltip_text(Some("Create normal notes"));
    normal.set_active(true);
    let slide = gtk::ToggleButton::with_label("Slide");
    slide.set_tooltip_text(Some("Create slide notes"));
    slide.set_group(Some(&normal));
    let portamento = gtk::ToggleButton::with_label("Porta");
    portamento.set_tooltip_text(Some("Create portamento notes"));
    portamento.set_group(Some(&normal));
    normal.connect_toggled({
        let note_kind = note_kind.clone();
        move |button| {
            if button.is_active() {
                *note_kind.borrow_mut() = PianoNoteKind::Normal;
            }
        }
    });
    slide.connect_toggled({
        let note_kind = note_kind.clone();
        move |button| {
            if button.is_active() {
                *note_kind.borrow_mut() = PianoNoteKind::Slide;
            }
        }
    });
    portamento.connect_toggled({
        let note_kind = note_kind.clone();
        move |button| {
            if button.is_active() {
                *note_kind.borrow_mut() = PianoNoteKind::Portamento;
            }
        }
    });
    controls.append(&normal);
    controls.append(&slide);
    controls.append(&portamento);

    let play = gtk::Button::builder()
        .icon_name("media-playback-start-symbolic")
        .tooltip_text("Play or pause the current pattern")
        .build();
    controls.append(&play);
    controls.append(&gtk::Separator::new(gtk::Orientation::Vertical));

    let channel = gtk::DropDown::from_strings(&["FLEX", "Drums", "Bass", "Synth", "Sampler"]);
    channel.set_tooltip_text(Some("Target channel"));
    controls.append(&channel);
    let ghosts = gtk::MenuButton::builder()
        .icon_name("view-reveal-symbolic")
        .tooltip_text("Ghost-note channels")
        .popover(&build_ghost_note_menu())
        .build();
    controls.append(&ghosts);

    controls.append(&gtk::Separator::new(gtk::Orientation::Vertical));
    let horizontal_zoom = gtk::Scale::with_range(gtk::Orientation::Horizontal, 48.0, 192.0, 8.0);
    horizontal_zoom.set_value(view.borrow().beat_width);
    horizontal_zoom.set_draw_value(false);
    horizontal_zoom.set_size_request(110, -1);
    horizontal_zoom.set_tooltip_text(Some("Horizontal zoom"));
    horizontal_zoom.connect_value_changed({
        let view = view.clone();
        let grid = grid.clone();
        let velocity_editor = velocity_editor.clone();
        let controller = controller.clone();
        move |scale| {
            view.borrow_mut().beat_width = scale.value();
            controller
                .borrow_mut()
                .presentation_mut()
                .piano_roll_horizontal_zoom = scale.value();
            let width = f64_to_i32_round(64.0 * scale.value());
            grid.set_content_width(width);
            velocity_editor.set_content_width(width);
            grid.queue_draw();
            velocity_editor.queue_draw();
        }
    });
    controls.append(&horizontal_zoom);

    let vertical_zoom =
        gtk::ScaleButton::new(14.0, 36.0, 1.0, &["zoom-out-symbolic", "zoom-in-symbolic"]);
    vertical_zoom.set_value(view.borrow().key_height);
    vertical_zoom.set_tooltip_text(Some("Vertical note zoom"));
    vertical_zoom.connect_value_changed({
        let view = view.clone();
        let grid = grid.clone();
        let keyboard = keyboard.clone();
        let controller = controller.clone();
        move |_, value| {
            view.borrow_mut().key_height = value;
            controller
                .borrow_mut()
                .presentation_mut()
                .piano_roll_vertical_zoom = value;
            let height = f64_to_i32_round(28.0 + 72.0 * value);
            grid.set_content_height(height);
            keyboard.set_content_height(height);
            grid.queue_draw();
            keyboard.queue_draw();
        }
    });
    controls.append(&vertical_zoom);

    let scroller = gtk::ScrolledWindow::new();
    scroller.set_policy(gtk::PolicyType::Automatic, gtk::PolicyType::Never);
    scroller.set_propagate_natural_height(true);
    scroller.set_child(Some(&controls));
    scroller
}

fn piano_tool_button(label: &str, tooltip: &str) -> gtk::ToggleButton {
    let button = gtk::ToggleButton::with_label(label);
    button.set_tooltip_text(Some(tooltip));
    button
}

fn build_piano_roll_menu() -> gtk::Popover {
    let content = popover_content("Piano Roll");
    for (label, icon) in [
        ("Import MIDI", "document-open-symbolic"),
        ("Export MIDI", "document-save-symbolic"),
        ("Edit", "edit-select-all-symbolic"),
        ("View", "view-grid-symbolic"),
        ("Select", "edit-select-all-symbolic"),
        ("Add Time Marker", "bookmark-new-symbolic"),
    ] {
        content.append(&popover_button(label, icon));
    }
    popover_with_child(&content)
}

fn build_piano_tools_menu() -> gtk::Popover {
    let content = popover_content("Tools");
    for (label, icon) in [
        ("Quick Legato", "format-text-italic-symbolic"),
        ("Articulate", "audio-volume-high-symbolic"),
        ("Arpeggiate", "go-up-symbolic"),
        ("Chop", "edit-cut-symbolic"),
        ("Flam", "format-justify-fill-symbolic"),
        ("Randomize", "media-playlist-shuffle-symbolic"),
        ("Strum", "view-sort-ascending-symbolic"),
        ("Scale Levels", "view-more-symbolic"),
    ] {
        content.append(&popover_button(label, icon));
    }
    popover_with_child(&content)
}

fn build_ghost_note_menu() -> gtk::Popover {
    let content = popover_content("Ghost Notes");
    let show = gtk::CheckButton::with_label("Show ghost notes");
    show.set_active(true);
    content.append(&show);
    let editable = gtk::CheckButton::with_label("Editable ghosts");
    content.append(&editable);
    content.append(&gtk::Separator::new(gtk::Orientation::Horizontal));
    for (name, active) in [
        ("Drums", true),
        ("Bass", true),
        ("Synth", false),
        ("Sampler", false),
    ] {
        let channel = gtk::CheckButton::with_label(name);
        channel.set_active(active);
        content.append(&channel);
    }
    popover_with_child(&content)
}

fn build_piano_keyboard(view: &Rc<RefCell<PianoRollView>>) -> gtk::DrawingArea {
    let keyboard = gtk::DrawingArea::new();
    keyboard.set_content_width(82);
    keyboard.set_content_height(1_612);
    keyboard.set_draw_func({
        let view = view.clone();
        move |widget, context, width, height| {
            draw_piano_keyboard(widget, context, width, height, *view.borrow());
        }
    });
    keyboard.set_tooltip_text(Some("Click keys to preview the target channel"));
    keyboard
}

fn draw_piano_keyboard(
    widget: &gtk::DrawingArea,
    context: &cairo::Context,
    width: i32,
    height: i32,
    view: PianoRollView,
) {
    let foreground = widget.color();
    let style = adw::StyleManager::for_display(&widget.display());
    let background = if style.is_dark() {
        gdk::RGBA::new(0.12, 0.12, 0.12, 1.0)
    } else {
        gdk::RGBA::new(0.96, 0.96, 0.96, 1.0)
    };
    set_cairo_color(context, &background, 1.0);
    context.paint().expect("paint piano keyboard");
    set_cairo_color(context, &foreground, 0.08);
    context.rectangle(0.0, 0.0, f64::from(width), 28.0);
    context.fill().expect("paint keyboard header");

    for row in 0..72 {
        let pitch = 96 - row;
        let y = 28.0 + f64::from(row) * view.key_height;
        let black = is_black_key(pitch);
        set_cairo_color(context, &foreground, if black { 0.72 } else { 0.04 });
        let key_width = if black {
            f64::from(width) * 0.66
        } else {
            f64::from(width)
        };
        context.rectangle(0.0, y, key_width, view.key_height - 1.0);
        context.fill().expect("draw piano key");
        if pitch.rem_euclid(12) == 0 {
            set_cairo_color(context, &foreground, if black { 0.15 } else { 0.72 });
            context.set_font_size(10.0);
            context.move_to(f64::from(width) - 27.0, y + view.key_height * 0.68);
            context
                .show_text(&format!("C{}", pitch / 12 - 1))
                .expect("draw octave label");
        }
    }
    let _ = height;
}

fn draw_piano_roll_grid(
    widget: &gtk::DrawingArea,
    context: &cairo::Context,
    width: i32,
    height: i32,
    notes: &[PianoNote],
    view: PianoRollView,
) {
    let style = adw::StyleManager::for_display(&widget.display());
    let foreground = widget.color();
    let background = if style.is_dark() {
        gdk::RGBA::new(0.10, 0.10, 0.10, 1.0)
    } else {
        gdk::RGBA::new(0.97, 0.97, 0.97, 1.0)
    };
    let accent = style.accent_color_rgba();
    set_cairo_color(context, &background, 1.0);
    context.paint().expect("paint Piano Roll background");

    for row in 0..72 {
        let pitch = 96 - row;
        let y = 28.0 + f64::from(row) * view.key_height;
        if is_black_key(pitch) {
            set_cairo_color(context, &foreground, 0.055);
            context.rectangle(0.0, y, f64::from(width), view.key_height);
            context.fill().expect("shade accidental row");
        }
        set_cairo_color(context, &foreground, 0.11);
        context.move_to(0.0, y + view.key_height + 0.5);
        context.line_to(f64::from(width), y + view.key_height + 0.5);
    }
    context.stroke().expect("draw pitch grid");

    let quarter_width = view.beat_width / 4.0;
    let columns = f64_to_i32_ceil(f64::from(width) / quarter_width);
    for column in 0..=columns {
        let x = f64::from(column) * quarter_width + 0.5;
        let alpha = if column % 16 == 0 {
            0.36
        } else if column % 4 == 0 {
            0.21
        } else {
            0.09
        };
        set_cairo_color(context, &foreground, alpha);
        context.move_to(x, 28.0);
        context.line_to(x, f64::from(height));
    }
    context.stroke().expect("draw time grid");

    set_cairo_color(context, &foreground, 0.08);
    context.rectangle(0.0, 0.0, f64::from(width), 28.0);
    context.fill().expect("paint timeline ruler");
    set_cairo_color(context, &foreground, 0.72);
    context.set_font_size(10.0);
    for bar in 0..16 {
        context.move_to(f64::from(bar) * view.beat_width * 4.0 + 7.0, 18.0);
        context
            .show_text(&(bar + 1).to_string())
            .expect("draw bar number");
    }

    draw_ghost_notes(context, &foreground, view);
    for note in notes {
        let x = note.start * view.beat_width;
        let y = 28.0 + f64::from(96 - note.pitch) * view.key_height + 2.0;
        let note_width = (note.length * view.beat_width - 2.0).max(5.0);
        set_cairo_color(context, &accent, if note.selected { 1.0 } else { 0.78 });
        context.rectangle(x + 1.0, y, note_width, view.key_height - 4.0);
        context.fill().expect("draw note");
        if note.selected {
            set_cairo_color(context, &foreground, 0.58);
            context.rectangle(x + note_width - 3.0, y + 2.0, 3.0, view.key_height - 8.0);
            context.fill().expect("draw note resize handle");
        }
        set_cairo_color(context, &foreground, 0.82);
        context.set_font_size(9.0);
        context.move_to(x + 4.0, y + view.key_height * 0.62);
        context
            .show_text(&midi_note_name(note.pitch))
            .expect("draw note label");
        if note.kind != PianoNoteKind::Normal {
            context.move_to(x + note_width - 9.0, y + 4.0);
            context.line_to(x + note_width - 3.0, y + 4.0);
            context.line_to(x + note_width - 3.0, y + 10.0);
            context.close_path();
            context.fill().expect("draw note transition flag");
        }
    }

    let marker_x = view.beat_width * 8.0;
    set_cairo_color(context, &accent, 0.85);
    context.rectangle(marker_x, 0.0, 2.0, f64::from(height));
    context.fill().expect("draw time marker");
    context.move_to(marker_x + 5.0, 18.0);
    context.show_text("Verse").expect("draw time marker label");
}

fn draw_ghost_notes(context: &cairo::Context, foreground: &gdk::RGBA, view: PianoRollView) {
    for (pitch, start, length) in [
        (72, 6.0, 0.5),
        (69, 6.5, 0.5),
        (72, 7.0, 0.5),
        (69, 7.5, 0.5),
        (76, 10.0, 1.0),
    ] {
        let x = start * view.beat_width;
        let y = 28.0 + f64::from(96 - pitch) * view.key_height + 3.0;
        set_cairo_color(context, foreground, 0.18);
        context.rectangle(
            x + 1.0,
            y,
            length * view.beat_width - 2.0,
            view.key_height - 6.0,
        );
        context.fill().expect("draw ghost note");
    }
}

fn draw_velocity_editor(
    widget: &gtk::DrawingArea,
    context: &cairo::Context,
    width: i32,
    height: i32,
    notes: &[PianoNote],
    view: PianoRollView,
) {
    let style = adw::StyleManager::for_display(&widget.display());
    let foreground = widget.color();
    let accent = style.accent_color_rgba();
    set_cairo_color(context, &foreground, 0.045);
    context.paint().expect("paint velocity editor");
    for beat in 0..=f64_to_i32_ceil(f64::from(width) / view.beat_width) {
        let x = f64::from(beat) * view.beat_width + 0.5;
        set_cairo_color(
            context,
            &foreground,
            if beat % 4 == 0 { 0.28 } else { 0.12 },
        );
        context.move_to(x, 0.0);
        context.line_to(x, f64::from(height));
    }
    context.stroke().expect("draw velocity grid");
    for note in notes {
        let x = note.start * view.beat_width + 3.0;
        let top = f64::from(height) * (1.0 - note.velocity).clamp(0.04, 0.94);
        set_cairo_color(context, &accent, if note.selected { 1.0 } else { 0.62 });
        context.set_line_width(if note.selected { 3.0 } else { 2.0 });
        context.move_to(x, f64::from(height));
        context.line_to(x, top);
        context.stroke().expect("draw velocity stem");
        context.arc(x, top, 4.0, 0.0, std::f64::consts::TAU);
        context.fill().expect("draw velocity handle");
        set_cairo_color(context, &accent, 0.34);
        context.set_line_width(1.0);
        context.move_to(x, top);
        context.line_to(x + note.length * view.beat_width, top);
        context.stroke().expect("draw note duration tail");
    }
}

fn is_black_key(pitch: i32) -> bool {
    matches!(pitch.rem_euclid(12), 1 | 3 | 6 | 8 | 10)
}

fn midi_note_name(pitch: i32) -> String {
    const NAMES: [&str; 12] = [
        "C", "C♯", "D", "D♯", "E", "F", "F♯", "G", "G♯", "A", "A♯", "B",
    ];
    format!("{}{}", NAMES[pitch.rem_euclid(12) as usize], pitch / 12 - 1)
}

fn piano_note_at(notes: &[PianoNote], view: PianoRollView, x: f64, y: f64) -> Option<usize> {
    if y < 28.0 {
        return None;
    }
    let pitch = 96 - f64_to_i32_floor((y - 28.0) / view.key_height);
    notes.iter().rposition(|note| {
        let left = note.start * view.beat_width;
        let right = (note.start + note.length) * view.beat_width;
        note.pitch == pitch && x >= left && x <= right
    })
}

fn piano_note_resize_target(
    notes: &[PianoNote],
    view: PianoRollView,
    x: f64,
    y: f64,
) -> Option<usize> {
    let index = piano_note_at(notes, view, x, y)?;
    let right = (notes[index].start + notes[index].length) * view.beat_width;
    (x >= right - 7.0 && x <= right + 3.0).then_some(index)
}

fn snap_timeline_value(value: f64, step: f64, bypass: bool) -> f64 {
    if bypass {
        value
    } else {
        (value / step).round() * step
    }
}

fn piano_domain_note(note: PianoNote) -> Note {
    Note {
        id: note.note_id,
        start: beats_to_ticks(note.start),
        length: beats_to_ticks(note.length),
        pitch: u8::try_from(note.pitch.clamp(0, 127)).expect("clamped MIDI pitch"),
        velocity: normalized_velocity_to_midi(note.velocity),
        channel: 0,
    }
}

#[allow(clippy::cast_possible_truncation)]
fn f64_to_i32_floor(value: f64) -> i32 {
    value
        .floor()
        .clamp(f64::from(i32::MIN), f64::from(i32::MAX)) as i32
}

#[allow(clippy::cast_possible_truncation)]
fn f64_to_i32_round(value: f64) -> i32 {
    value
        .round()
        .clamp(f64::from(i32::MIN), f64::from(i32::MAX)) as i32
}

#[allow(clippy::cast_possible_truncation)]
fn f64_to_i32_ceil(value: f64) -> i32 {
    value.ceil().clamp(f64::from(i32::MIN), f64::from(i32::MAX)) as i32
}

#[allow(clippy::cast_possible_truncation)]
fn f64_to_f32(value: f64) -> f32 {
    value.clamp(f64::from(f32::MIN), f64::from(f32::MAX)) as f32
}

fn show_shortcuts(parent: &adw::ApplicationWindow) {
    let dialog = adw::AlertDialog::builder()
        .heading("Keyboard Shortcuts")
        .body("Ctrl+Z  Undo\nCtrl+Shift+Z  Redo\nF5  Playlist\nF6  Channel Rack\nF7  Piano Roll\nF9  Mixer\nF11  Maximize or restore")
        .build();
    dialog.add_response("close", "Close");
    dialog.set_default_response(Some("close"));
    dialog.present(Some(parent));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn history_action_enablement_tracks_controller_history() {
        let mut controller = AppController::new(Session::new("Actions")).expect("controller");
        assert_eq!(history_action_state(&controller), (false, false));
        controller
            .execute(&Transaction::one(
                "Rename",
                Command::RenameSession {
                    name: "Changed".into(),
                },
            ))
            .expect("execute");
        assert_eq!(history_action_state(&controller), (true, false));
        controller.undo().expect("undo");
        assert_eq!(history_action_state(&controller), (false, true));
    }

    #[test]
    fn project_directory_names_are_local_and_stable() {
        assert_eq!(
            project_directory_name("My Great Song"),
            "my_great_song.radiant"
        );
        assert_eq!(project_directory_name("Beat-01"), "beat-01.radiant");
    }
}
pub mod plugin_ui;
