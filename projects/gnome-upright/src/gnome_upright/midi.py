from __future__ import annotations
from gi.repository import GLib, GObject

class MidiManager(GObject.Object):
    __gsignals__ = {'note-on': (GObject.SignalFlags.RUN_FIRST,None,(int,int,str,int)), 'note-off': (GObject.SignalFlags.RUN_FIRST,None,(str,)), 'sustain': (GObject.SignalFlags.RUN_FIRST,None,(bool,int)), 'devices-changed': (GObject.SignalFlags.RUN_FIRST,None,(object,))}
    def __init__(self): super().__init__(); self._ports=[]; self._names=[]; self.channel=0; self.velocity_sensitivity=1.0
    def refresh(self):
        try:
            import mido; names=mido.get_input_names()
        except Exception: names=[]
        self.emit('devices-changed', names); return names
    def connect_port(self, name):
        self.close()
        try:
            import mido
            self._ports.append(mido.open_input(name, callback=self._receive)); self._names.append(name); return True
        except Exception: return False
    def _receive(self, msg):
        def deliver():
            source=f'midi-{getattr(msg,"channel",0)}-{getattr(msg,"note",0)}'
            if self.channel and getattr(msg,'channel',0) != self.channel-1: return False
            if msg.type == 'note_on' and msg.velocity: self.emit('note-on',msg.note,max(1,min(127,round(msg.velocity*self.velocity_sensitivity))),source,msg.channel)
            elif msg.type in ('note_off','note_on'): self.emit('note-off',source)
            elif msg.type == 'control_change' and msg.control == 64: self.emit('sustain',msg.value >= 64,msg.channel)
            return False
        GLib.idle_add(deliver)
    def close(self):
        for port in self._ports: port.close()
        self._ports.clear(); self._names.clear()
