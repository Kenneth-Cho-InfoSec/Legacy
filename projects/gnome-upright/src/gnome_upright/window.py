from __future__ import annotations
import json
from pathlib import Path
from gi.repository import Adw, Gio, GLib, Gtk
from .audio import AudioEngine
from .constants import APP_ID
from .formats import export_midi, import_midi
from .midi import MidiManager
from .piano import PianoKeyboard
from .preferences import present_preferences
from .recording import Recorder
from .shortcuts import install as install_shortcuts

class UprightWindow(Adw.ApplicationWindow):
    def __init__(self, app):
        self.settings=Gio.Settings.new(APP_ID)
        if not self.settings.get_boolean('window-size-v2'):
            self.settings.set_int('window-width',920); self.settings.set_int('window-height',560); self.settings.set_boolean('window-size-v2',True)
        super().__init__(application=app, title='GNOME Upright', default_width=self.settings.get_int('window-width'), default_height=self.settings.get_int('window-height'))
        self.audio=AudioEngine(); self.recorder=Recorder(self.settings.get_int('tempo')); self.midi=MidiManager(); self._sustain=False
        overlay=Adw.ToastOverlay(); self.set_content(overlay); toolbar=Adw.ToolbarView(); overlay.set_child(toolbar)
        header=Adw.HeaderBar(); toolbar.add_top_bar(header)
        self.record_button=self._button('media-record-symbolic','Record',self.record); header.pack_start(self.record_button)
        header.pack_start(self._button('media-playback-stop-symbolic','Stop',self.stop)); header.pack_start(self._button('media-playback-start-symbolic','Play',self.play))
        header.pack_end(self._button('document-save-symbolic','Export MIDI',self.export)); header.pack_end(self._button('document-open-symbolic','Import MIDI',self.import_dialog)); header.pack_end(self._button('preferences-system-symbolic','Preferences',lambda *_:present_preferences(self,self.settings,self.midi.refresh())))
        main=Gtk.Box(orientation=Gtk.Orientation.VERTICAL,spacing=12, margin_top=12,margin_bottom=12,margin_start=18,margin_end=18); toolbar.set_content(main)
        self.status=Gtk.Label(xalign=0, css_classes=['dim-label']); main.append(self.status)
        self.piano=PianoKeyboard(); self.piano.set_size_request(-1,260); main.append(self.piano)
        transport=Gtk.Box(spacing=8,halign=Gtk.Align.CENTER); main.append(transport)
        self.tempo=Gtk.SpinButton(adjustment=Gtk.Adjustment(value=self.settings.get_int('tempo'),lower=30,upper=300,step_increment=1), numeric=True); self.tempo.set_tooltip_text('Tempo (BPM)'); self.tempo.connect('value-changed',self._tempo_changed); transport.append(Gtk.Label(label='Tempo')); transport.append(self.tempo)
        self.quantize=Gtk.DropDown.new_from_strings(['No quantization','¼ note','⅛ note','¹⁄₁₆ note']); transport.append(self.quantize)
        self.sustain=Gtk.ToggleButton(label='Sustain'); self.sustain.connect('toggled',lambda b:self.set_sustain(b.get_active())); transport.append(self.sustain)
        self.info=Gtk.Label(xalign=.5,css_classes=['caption']); main.append(self.info)
        self.piano.connect('note-on',self._note_on); self.piano.connect('note-off',self._note_off); self.midi.connect('note-on',self._note_on); self.midi.connect('note-off',self._note_off); self.midi.connect('sustain',lambda _,on,ch:self.set_sustain(on,ch)); self.audio.connect('availability-changed',lambda _,ok,message:self._toast(message)); self.recorder.connect('changed',lambda *_:self._refresh()); self.recorder.connect('state-changed',lambda _,state:self._refresh())
        self.settings.connect('changed',self._settings_changed); self.piano.apply_preferences(self.settings); self.audio.configure(self.settings.get_string('audio-backend'),self.settings.get_int('polyphony')); self.audio.set_volume(self.settings.get_double('volume')); self.midi.channel=self.settings.get_int('midi-channel'); self.midi.velocity_sensitivity=self.settings.get_double('velocity-sensitivity')
        self.audio.start(); self.midi.refresh(); self._restore(); self._refresh()
        self._install_actions(); self._apply_shortcuts()
    def _install_actions(self):
        for name, callback in [('record',self.record),('stop',self.stop),('play',self.play),('export',self.export)]:
            action=Gio.SimpleAction.new(name,None); action.connect('activate',lambda _a,_p,fn=callback:fn()); self.add_action(action)
    def _apply_shortcuts(self):
        install_shortcuts(self.get_application(),self.settings.get_value('key-bindings').unpack())
    def _button(self,icon,label,callback):
        button=Gtk.Button(icon_name=icon,tooltip_text=label); button.connect('clicked',callback); return button
    def _toast(self,message):
        root=self.get_content()
        if isinstance(root,Adw.ToastOverlay): root.add_toast(Adw.Toast.new(message))
    def _tempo_changed(self,widget): self.recorder.session.tempo=round(widget.get_value()); self.settings.set_int('tempo',self.recorder.session.tempo)
    def _settings_changed(self, _settings, key):
        if key in {'pressed-key-color','midi-input-color','playback-color','white-key-color','black-key-color','show-note-names','show-keyboard-labels','show-velocity'}: self.piano.apply_preferences(self.settings)
        elif key == 'volume': self.audio.set_volume(self.settings.get_double('volume'))
        elif key in {'audio-backend','polyphony'}: self.audio.configure(self.settings.get_string('audio-backend'),self.settings.get_int('polyphony'))
        elif key == 'midi-channel': self.midi.channel=self.settings.get_int('midi-channel')
        elif key == 'velocity-sensitivity': self.midi.velocity_sensitivity=self.settings.get_double('velocity-sensitivity')
        elif key == 'key-bindings': self._apply_shortcuts()
        elif key == 'tempo' and not self.recorder.recording and not self.recorder.playing:
            self.recorder.session.tempo=self.settings.get_int('tempo'); self.tempo.set_value(self.recorder.session.tempo)
    def _note_on(self,source,pitch,velocity,ident,channel=0):
        self.audio.note_on(pitch,velocity,ident,channel); self.recorder.note_on(pitch,velocity,ident,channel)
    def _note_off(self,source,ident): self.audio.note_off(ident); self.recorder.note_off(ident)
    def set_sustain(self,on,channel=0):
        self._sustain=on; self.sustain.set_active(on); self.audio.set_sustain(on,channel)
    def record(self,*_): self.recorder.start(); self._toast('Recording started')
    def stop(self,*_): self.recorder.stop(); self.recorder.stop_playback(); self.audio.all_notes_off(); self._save()
    def play(self,*_): self.recorder.play(lambda *a:self._note_on(None,*a),lambda ident:self._note_off(None,ident),lambda:self._save())
    def clear(self,*_): self.recorder.clear(); self._save()
    def _refresh(self):
        state='Recording' if self.recorder.recording else 'Playing' if self.recorder.playing else 'Ready'; self.status.set_label(state); self.info.set_label(f'{len(self.recorder.session.notes)} notes · {self.recorder.duration():.1f} seconds · {self.recorder.session.tempo} BPM'); self.record_button.set_sensitive(not self.recorder.playing)
    def _save(self):
        if not self.settings.get_boolean('autosave'): return
        payload={'tempo':self.recorder.session.tempo,'notes':[note.to_dict() for note in self.recorder.session.notes]}; self.settings.set_string('last-session',json.dumps(payload))
    def _restore(self):
        try:
            data=json.loads(self.settings.get_string('last-session')); from .models import Note
            self.recorder.session.tempo=data['tempo']; self.tempo.set_value(data['tempo']); self.recorder.session.notes=[Note(**n) for n in data['notes']]
        except (ValueError,KeyError,TypeError): pass
    def export(self,*_):
        if not self.recorder.session.notes: self._toast('Record or import notes before exporting'); return
        dialog=Gtk.FileDialog(title='Export MIDI',initial_name='upright-recording.mid'); dialog.save(self,None,self._export_done)
    def _export_done(self,dialog,result):
        try:
            file=dialog.save_finish(result); export_midi(file.get_path(),self.recorder.session.notes,self.recorder.session.tempo); self._toast('MIDI exported')
        except Exception as error: self._toast(f'Export failed: {error}')
    def import_dialog(self,*_):
        dialog=Gtk.FileDialog(title='Import MIDI'); dialog.open(self,None,self._import_done)
    def _import_done(self,dialog,result):
        try: self.open_file(dialog.open_finish(result))
        except Exception: pass
    def open_file(self,file):
        try:
            notes,tempo=import_midi(file.get_path()); self.stop(); self.recorder.session.notes=notes; self.recorder.session.tempo=tempo; self.tempo.set_value(tempo); self._save(); self._refresh(); self._toast(f'Imported {len(notes)} notes')
        except Exception as error: self._toast(f'Import failed: {error}')
    def do_close_request(self):
        self.stop(); width,height=self.get_default_size(); self.settings.set_int('window-width',width); self.settings.set_int('window-height',height); self.midi.close(); self.audio.close(); return False
