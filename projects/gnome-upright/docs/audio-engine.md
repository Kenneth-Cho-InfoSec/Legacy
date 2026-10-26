# Audio engine

The default synthesizer is FluidSynth using the PipeWire driver and a locally installed General MIDI SoundFont. `UPRIGHT_SOUNDFONT` selects the SoundFont and `UPRIGHT_AUDIO_DRIVER` can select a FluidSynth-supported driver. Calls include velocity, MIDI channel, voice release, all-notes-off, and sustain CC64 handling.

Audio is deliberately optional at import time so opening a MIDI file remains possible on machines without FluidSynth; the application reports the exact remediation in a toast.
