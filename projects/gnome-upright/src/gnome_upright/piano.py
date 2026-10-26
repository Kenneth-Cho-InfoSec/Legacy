from __future__ import annotations
import logging
from gi.repository import Gdk, GObject, Graphene, Gtk
from .constants import BLACK, KEY_COUNT, KEY_MAP, NOTE_NAMES, START_NOTE

class PianoKeyboard(Gtk.Widget):
    __gsignals__ = {'note-on': (GObject.SignalFlags.RUN_FIRST,None,(int,int,str)), 'note-off': (GObject.SignalFlags.RUN_FIRST,None,(str,))}
    def __init__(self):
        super().__init__(); self.set_focusable(True); self.set_hexpand(True); self.set_vexpand(True); self._pressed={}; self._pointer={}; self._white=[n for n in range(KEY_COUNT) if n % 12 not in BLACK]
        self.appearance={'pressed-key-color':'#6ca7e8','midi-input-color':'#62d5a8','playback-color':'#d6a4ff','white-key-color':'#fafafa','black-key-color':'#1e1e1e'}; self.show_note_names=True; self.show_keyboard_labels=True; self._key_map=KEY_MAP.copy()
        click=Gtk.GestureClick(); click.connect('pressed',self._press); click.connect('released',self._release); self.add_controller(click)
        drag=Gtk.GestureDrag(); drag.connect('drag-update',self._drag); self.add_controller(drag)
        keys=Gtk.EventControllerKey(); keys.connect('key-pressed',self._key_down); keys.connect('key-released',self._key_up); self.add_controller(keys)
    def do_measure(self, orientation, for_size): return (320, 800, -1, -1) if orientation==Gtk.Orientation.HORIZONTAL else (150, 260, -1, -1)
    def _geometry(self):
        width,height=self.get_width(),self.get_height(); white=width/len(self._white); return width,height,white
    def _at(self,x,y):
        width,height,white=self._geometry();
        if y < height*.62:
            for i,n in enumerate(range(KEY_COUNT)):
                if n%12 in BLACK:
                    before=sum(1 for w in self._white if w<n); left=before*white-white*.32
                    if left<=x<left+white*.64:return n
        index=min(len(self._white)-1,max(0,int(x/white))); return self._white[index]
    def _press_note(self,n,source,velocity=100):
        if source not in self._pressed: self._pressed[source]=n; self.emit('note-on',START_NOTE+n,velocity,source); self.queue_draw()
    def _release_note(self,source):
        if source in self._pressed: self._pressed.pop(source); self.emit('note-off',source); self.queue_draw()
    def _press(self,gesture,n_press,x,y): self.grab_focus(); source=f'pointer-{gesture.get_current_sequence()!s}'; self._pointer[gesture]=source; self._press_note(self._at(x,y),source)
    def _release(self,gesture,n_press,x,y): self._release_note(self._pointer.pop(gesture,f'pointer-{gesture.get_current_sequence()!s}'))
    def _drag(self,gesture,x,y):
        source=self._pointer.get(gesture)
        if source:
            note=self._at(gesture.get_start_point()[1]+x,gesture.get_start_point()[2]+y)
            if self._pressed.get(source)!=note: self._release_note(source); self._press_note(note,source)
    def _key_down(self,controller,keyval,keycode,state):
        name=Gdk.keyval_name(keyval) or ''; source=f'key-{name.lower()}'
        if name.lower() in self._key_map: self._press_note(self._key_map[name.lower()]-START_NOTE,source); return True
        return False
    def _key_up(self,controller,keyval,keycode,state):
        name=(Gdk.keyval_name(keyval) or '').lower(); source=f'key-{name}'; self._release_note(source); return name in self._key_map
    def apply_preferences(self, settings):
        for key in self.appearance: self.appearance[key]=settings.get_string(key)
        self.show_note_names=settings.get_boolean('show-note-names'); self.show_keyboard_labels=settings.get_boolean('show-keyboard-labels'); self._key_map=settings.get_value('piano-key-bindings').unpack(); self.queue_draw()
    def _rgb(self, color):
        value=Gdk.RGBA()
        if value.parse(color): return value.red, value.green, value.blue
        return .5,.5,.5
    def _key_color(self, note, fallback):
        source=next((source for source,pitch in self._pressed.items() if pitch==note),None)
        if not source: return fallback
        if source.startswith('midi-'): return self._rgb(self.appearance['midi-input-color'])
        if source.startswith('playback-'): return self._rgb(self.appearance['playback-color'])
        return self._rgb(self.appearance['pressed-key-color'])
    def do_snapshot(self,snapshot):
        """Render using GTK's snapshot API. Errors are logged rather than silently blanking the UI."""
        try:
            width,height,white=self._geometry()
            rect=Graphene.Rect(); rect.init(0, 0, max(1, width), max(1, height))
            cr=snapshot.append_cairo(rect)
            cr.set_source_rgb(.12,.12,.12); cr.paint()
            for i,n in enumerate(self._white):
                cr.set_source_rgb(*self._key_color(n,self._rgb(self.appearance['white-key-color'])))
                cr.rectangle(i*white,0,white-1,height); cr.fill()
                cr.set_source_rgb(.18,.18,.18); cr.rectangle(i*white,0,white-1,height); cr.stroke()
                if self.show_note_names:
                    pitch=START_NOTE+n; cr.set_source_rgb(.1,.1,.1); cr.select_font_face('Sans'); cr.set_font_size(12)
                    cr.move_to(i*white+5,height-14); cr.show_text(f'{NOTE_NAMES[pitch%12]}{pitch//12-1}')
                if self.show_keyboard_labels:
                    label=next((name.upper() for name,value in self._key_map.items() if value==pitch),'')
                    cr.set_source_rgb(.35,.35,.35); cr.set_font_size(10); cr.move_to(i*white+5,height-30); cr.show_text(label)
            for n in range(KEY_COUNT):
                if n%12 in BLACK:
                    before=sum(1 for w in self._white if w<n); left=before*white-white*.32
                    cr.set_source_rgb(*self._key_color(n,self._rgb(self.appearance['black-key-color'])))
                    cr.rectangle(left,0,white*.64,height*.62); cr.fill()
            # Pycairo contexts are reference-counted Python objects; GTK owns the
            # snapshot surface and no explicit destroy() call exists or is needed.
        except Exception:
            logging.getLogger(__name__).exception('Piano snapshot failed')
