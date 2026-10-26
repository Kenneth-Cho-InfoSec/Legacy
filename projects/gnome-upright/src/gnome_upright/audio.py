"""Low-latency FluidSynth backend; all calls are confined to GTK's main thread."""
from __future__ import annotations
import os
import logging
from gi.repository import GObject

LOG = logging.getLogger(__name__)

class AudioEngine(GObject.Object):
    __gsignals__ = {'availability-changed': (GObject.SignalFlags.RUN_FIRST, None, (bool, str))}
    def __init__(self):
        super().__init__(); self._synth = None; self._sfid = -1; self._active = {}; self.sustain = False; self._deferred = set(); self.backend='pipewire'; self.polyphony=64

    def start(self) -> None:
        try:
            import fluidsynth
            self._synth = fluidsynth.Synth(gain=.8, **{'synth.polyphony': self.polyphony})
            self._synth.start(driver=os.environ.get('UPRIGHT_AUDIO_DRIVER', self.backend))
            soundfont = os.environ.get('UPRIGHT_SOUNDFONT', '/usr/share/soundfonts/FluidR3_GM.sf2')
            self._sfid = self._synth.sfload(soundfont)
            if self._sfid < 0: raise RuntimeError(f'Cannot load SoundFont: {soundfont}')
            self._synth.program_select(0, self._sfid, 0, 0)
            self.emit('availability-changed', True, 'FluidSynth ready')
        except Exception as error:
            LOG.warning('Audio unavailable: %s', error); self._synth = None
            self.emit('availability-changed', False, 'Install FluidSynth and set UPRIGHT_SOUNDFONT to enable sound')

    def set_volume(self, volume: float) -> None:
        if self._synth: self._synth.cc(0, 7, max(0, min(127, round(volume * 127))))
    def configure(self, backend: str, polyphony: int) -> None:
        changed = (self.backend, self.polyphony) != (backend, polyphony)
        self.backend, self.polyphony = backend, polyphony
        if changed and self._synth:
            self.close(); self.start()
    def note_on(self, pitch: int, velocity: int, source: str, channel: int = 0) -> None:
        self.note_off(source)
        if self._synth: self._synth.noteon(channel, pitch, max(1, min(127, velocity)))
        self._active[source] = (pitch, channel)
    def note_off(self, source: str) -> None:
        event = self._active.pop(source, None)
        if not event: return
        if self.sustain: self._deferred.add((source, *event)); return
        if self._synth: self._synth.noteoff(event[1], event[0])
    def set_sustain(self, enabled: bool, channel: int = 0) -> None:
        self.sustain = enabled
        if self._synth: self._synth.cc(channel, 64, 127 if enabled else 0)
        if not enabled:
            for _, pitch, ch in self._deferred:
                if self._synth: self._synth.noteoff(ch, pitch)
            self._deferred.clear()
    def all_notes_off(self) -> None:
        if self._synth:
            for channel in range(16): self._synth.cc(channel, 123, 0)
        self._active.clear(); self._deferred.clear()
    def close(self) -> None:
        self.all_notes_off()
        if self._synth: self._synth.delete(); self._synth = None
