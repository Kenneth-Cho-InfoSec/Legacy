"""Native, schema-backed libadwaita preferences."""
from __future__ import annotations
import platform
from gi.repository import Adw, Gdk, Gio, GLib, Gtk

SHORTCUTS = [('record', 'Record'), ('play', 'Play'), ('stop', 'Stop'), ('export', 'Export MIDI'),
             ('octave-up', 'Raise octave'), ('octave-down', 'Lower octave'),
             ('transpose-up', 'Transpose up'), ('transpose-down', 'Transpose down')]

def _page(title, icon): return Adw.PreferencesPage(title=title, icon_name=icon)
def _group(title, description=None): return Adw.PreferencesGroup(title=title, description=description)
def _strings(values): return Gtk.StringList.new(values)

def _combo(settings, key, title, values, labels=None):
    labels = labels or values; row = Adw.ComboRow(title=title, model=_strings(labels))
    current = settings.get_string(key); row.set_selected(values.index(current) if current in values else 0)
    row.connect('notify::selected', lambda r, _p: settings.set_string(key, values[r.get_selected()]))
    return row

def _spin(settings, key, title, lower, upper, step=1, subtitle=None):
    row = Adw.SpinRow(title=title, subtitle=subtitle or '', adjustment=Gtk.Adjustment(value=settings.get_int(key), lower=lower, upper=upper, step_increment=step))
    row.connect('notify::value', lambda r, _p: settings.set_int(key, round(r.get_value())))
    return row

def _scale(settings, key, title, lower, upper, subtitle=None):
    row = Adw.ActionRow(title=title, subtitle=subtitle or ''); scale=Gtk.Scale(orientation=Gtk.Orientation.HORIZONTAL, adjustment=Gtk.Adjustment(value=settings.get_double(key), lower=lower, upper=upper, step_increment=.05)); scale.set_valign(Gtk.Align.CENTER); scale.set_size_request(160,-1)
    scale.connect('value-changed', lambda s: settings.set_double(key, s.get_value())); row.add_suffix(scale); return row

def _switch(settings, key, title, subtitle=None):
    row=Adw.SwitchRow(title=title, subtitle=subtitle or ''); settings.bind(key,row,'active',Gio.SettingsBindFlags.DEFAULT); return row

def _color(settings, key, title, parent):
    row=Adw.ActionRow(title=title); button=Gtk.Button(label=settings.get_string(key)); row.add_suffix(button)
    def choose(_button):
        initial=Gdk.RGBA(); initial.parse(settings.get_string(key)); dialog=Gtk.ColorDialog(title=title, with_alpha=False)
        dialog.choose_rgba(parent, initial, None, finished)
    def finished(dialog, result):
        try:
            color=dialog.choose_rgba_finish(result); value=color.to_string(); settings.set_string(key,value); button.set_label(value)
        except GLib.Error: pass
    button.connect('clicked',choose); return row

def _shortcut_row(settings, key, title, parent):
    row=Adw.EntryRow(title=title, text=settings.get_value('key-bindings').unpack().get(key,''))
    def changed(entry):
        value=entry.get_text().strip(); bindings=settings.get_value('key-bindings').unpack()
        if not value: entry.set_text(bindings.get(key,'')); return
        if Gtk.ShortcutTrigger.parse_string(value) is None: entry.add_css_class('error'); return
        entry.remove_css_class('error'); conflict=next((name for name,binding in bindings.items() if name!=key and binding==value),None)
        if conflict:
            dialog=Adw.MessageDialog(transient_for=parent, heading='Shortcut already in use', body=f'“{value}” is assigned to {conflict.replace("-", " ")}. Choose another shortcut.')
            dialog.add_response('close','Close'); dialog.present(); entry.set_text(bindings.get(key,'')); return
        bindings[key]=value; settings.set_value('key-bindings',GLib.Variant('a{ss}',bindings))
    row.connect('apply',changed); return row

def _piano_mapping_row(settings, pitch, title, parent):
    mappings=settings.get_value('piano-key-bindings').unpack(); current=next((key for key,value in mappings.items() if value==pitch),'')
    row=Adw.EntryRow(title=title, text=current)
    def changed(entry):
        value=entry.get_text().strip().lower(); current_map=settings.get_value('piano-key-bindings').unpack()
        if len(value)!=1: entry.add_css_class('error'); return
        conflict=current_map.get(value)
        if conflict is not None and conflict != pitch:
            dialog=Adw.MessageDialog(transient_for=parent,heading='Key already in use',body=f'“{value.upper()}” is assigned to another piano key. Choose another key.')
            dialog.add_response('close','Close'); dialog.present(); entry.set_text(next((k for k,v in current_map.items() if v==pitch),'')); return
        for name,note in list(current_map.items()):
            if note==pitch: del current_map[name]
        current_map[value]=pitch; settings.set_value('piano-key-bindings',GLib.Variant('a{si}',current_map)); entry.remove_css_class('error')
    row.connect('apply',changed); return row

def present_preferences(parent, settings: Gio.Settings, midi_devices=()):
    dialog=Adw.PreferencesDialog(); dialog.set_search_enabled(True)
    # Keyboard
    keyboard=_page('Keyboard','input-keyboard-symbolic'); shortcuts=_group('Shortcuts','Press Enter after editing a shortcut. Conflicting assignments are rejected.')
    for key,title in SHORTCUTS: shortcuts.add(_shortcut_row(settings,key,title,parent))
    piano_map=_group('Piano keys','Assign one physical keyboard key per piano note.')
    for pitch in range(48,73): piano_map.add(_piano_mapping_row(settings,pitch,f'{"C C♯ D D♯ E F F♯ G G♯ A A♯ B".split()[pitch%12]}{pitch//12-1}',parent))
    keyboard.add(shortcuts); keyboard.add(piano_map); dialog.add(keyboard)
    # Appearance
    appearance=_page('Piano','preferences-desktop-theme-symbolic'); colors=_group('Key colors')
    for key,title in [('pressed-key-color','Pressed key'),('midi-input-color','MIDI input'),('playback-color','Playback'),('white-key-color','White keys'),('black-key-color','Black keys')]: colors.add(_color(settings,key,title,parent))
    display=_group('Display');
    for key,title in [('show-note-names','Show note names'),('show-keyboard-labels','Show computer keyboard labels'),('show-velocity','Visualize MIDI velocity')]: display.add(_switch(settings,key,title))
    motion=_group('Motion');
    for key,title in [('animation-intensity','Animation intensity'),('glow-intensity','Glow intensity'),('animation-speed','Animation speed')]: motion.add(_scale(settings,key,title,0,2))
    appearance.add(colors); appearance.add(display); appearance.add(motion); dialog.add(appearance)
    # Audio and MIDI
    audio=_page('Audio & MIDI','audio-card-symbolic'); output=_group('Audio')
    output.add(_scale(settings,'volume','Master volume',0,1)); output.add(_combo(settings,'audio-backend','Audio backend',['pipewire','pulseaudio','alsa'],['PipeWire','PulseAudio','ALSA']))
    output.add(_combo(settings,'midi-latency','Latency',['low','balanced','safe'],['Low','Balanced','Safe'])); output.add(_combo(settings,'piano-sound','Piano sound',['acoustic-grand'],['Acoustic Grand Piano']))
    output.add(_spin(settings,'polyphony','Polyphony limit',8,256,8)); output.add(_switch(settings,'sustain-pedal','Enable sustain pedal support'))
    midi=_group('MIDI'); names=['None',*midi_devices]; device_values=['',*midi_devices]
    midi.add(_combo(settings,'midi-input','MIDI input device',device_values,names)); midi.add(_combo(settings,'midi-output','MIDI output device',[''],['None']))
    midi.add(_switch(settings,'midi-auto-connect','Connect MIDI devices automatically')); midi.add(_scale(settings,'velocity-sensitivity','Velocity sensitivity',.25,2)); midi.add(_spin(settings,'midi-channel','MIDI channel',0,16,1,'0 receives all channels'))
    audio.add(output); audio.add(midi); dialog.add(audio)
    # Recording
    recording=_page('Recording','media-record-symbolic'); defaults=_group('Defaults'); defaults.add(_spin(settings,'tempo','Default tempo',30,300)); defaults.add(_switch(settings,'metronome','Enable metronome')); defaults.add(_spin(settings,'count-in','Count-in bars',0,8)); defaults.add(_combo(settings,'quantization','Quantization',['none','quarter','eighth','sixteenth'],['None','¼ note','⅛ note','¹⁄₁₆ note'])); defaults.add(_switch(settings,'autosave','Autosave recordings'))
    export=_group('Export'); folder=Adw.EntryRow(title='Default export folder', text=settings.get_string('export-folder')); folder.connect('apply',lambda row:settings.set_string('export-folder',row.get_text())); export.add(folder); recording.add(defaults); recording.add(export); dialog.add(recording)
    # Standard GNOME about presentation
    about=_page('About','help-about-symbolic'); info=_group('GNOME Upright'); info.add(Adw.ActionRow(title='GNOME Upright', subtitle='A native GNOME piano and MIDI workstation built with GTK4 and libadwaita.'))
    link=Adw.ActionRow(title='Maintainer', subtitle='Kenneth Cho'); link.set_activatable(True); link.connect('activated',lambda *_:Gio.AppInfo.launch_default_for_uri('https://github.com/Kenneth-Cho-InfoSec',None)); info.add(link)
    runtime=_group('Runtime information'); runtime.add(Adw.ActionRow(title='Version',subtitle='1.0.0')); runtime.add(Adw.ActionRow(title='License',subtitle='MIT')); runtime.add(Adw.ActionRow(title='GTK',subtitle=f'{Gtk.get_major_version()}.{Gtk.get_minor_version()}.{Gtk.get_micro_version()}')); runtime.add(Adw.ActionRow(title='libadwaita',subtitle=f'{Adw.get_major_version()}.{Adw.get_minor_version()}.{Adw.get_micro_version()}')); runtime.add(Adw.ActionRow(title='Python',subtitle=platform.python_version())); runtime.add(Adw.ActionRow(title='Audio backend',subtitle=settings.get_string('audio-backend').title())); runtime.add(Adw.ActionRow(title='MIDI backend',subtitle='Mido / RtMidi'))
    about.add(info); about.add(runtime); dialog.add(about); dialog.present(parent)
