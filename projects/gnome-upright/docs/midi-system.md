# MIDI system

The MIDI manager enumerates input ports through Mido/python-rtmidi. The callback uses `GLib.idle_add`, which keeps GTK and recording state on the main thread. It handles note on/off, velocity, MIDI channels, and sustain pedal CC64. MIDI backends expose ALSA and PipeWire-connected ports through RtMidi where supported by the system.
