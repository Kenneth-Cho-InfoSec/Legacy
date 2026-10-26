from __future__ import annotations
import time
from gi.repository import GLib, GObject
from .models import Note, Session

class Recorder(GObject.Object):
    __gsignals__ = {'changed': (GObject.SignalFlags.RUN_FIRST, None, ()), 'state-changed': (GObject.SignalFlags.RUN_FIRST, None, (str,))}
    def __init__(self, tempo=120):
        super().__init__(); self.session = Session(tempo=tempo); self.recording = False; self.playing = False; self._started = 0.; self._active = {}; self._source_ids = []
    def start(self):
        self.stop_playback(); self.session.notes.clear(); self._active.clear(); self._started = time.monotonic(); self.recording = True; self.emit('state-changed', 'recording'); self.emit('changed')
    def note_on(self, pitch, velocity, source, channel=0):
        if self.recording: self._active[source] = Note(pitch, time.monotonic()-self._started, velocity=velocity, channel=channel, source=source)
    def note_off(self, source):
        note = self._active.pop(source, None)
        if note:
            note.end = time.monotonic()-self._started; self.session.notes.append(note); self.emit('changed')
    def stop(self):
        if not self.recording: return
        for source in list(self._active): self.note_off(source)
        self.recording = False; self.emit('state-changed', 'stopped'); self.emit('changed')
    def clear(self): self.stop(); self.stop_playback(); self.session.notes.clear(); self.emit('changed')
    def duration(self): return max((n.end or n.start for n in self.session.notes), default=0.)
    def play(self, note_on, note_off, finished):
        if self.playing or not self.session.notes: return
        self.playing = True; self.emit('state-changed', 'playing')
        scale = 120 / self.session.tempo
        for index, note in enumerate(self.session.notes):
            key = f'playback-{index}'
            self._source_ids.append(GLib.timeout_add(round(note.start*1000*scale), lambda n=note,k=key: (note_on(n.pitch,n.velocity,k,n.channel), False)[1]))
            self._source_ids.append(GLib.timeout_add(round((note.start+note.duration)*1000*scale), lambda k=key: (note_off(k), False)[1]))
        self._source_ids.append(GLib.timeout_add(round((self.duration()*scale+.05)*1000), lambda: self._finish(finished)))
    def _finish(self, callback): self._source_ids.clear(); self.playing=False; self.emit('state-changed','stopped'); callback(); return False
    def stop_playback(self):
        for ident in self._source_ids: GLib.source_remove(ident)
        self._source_ids.clear(); self.playing=False
