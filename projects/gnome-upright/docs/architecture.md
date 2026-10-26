# Architecture

`UprightWindow` composes the application services and only coordinates UI state. `PianoKeyboard` is a direct GTK widget subclass which renders from current state in `do_snapshot`; it requests a redraw only when a key changes. The services communicate through typed GObject signals so hardware MIDI events are marshalled onto the GTK main loop before touching the UI or recorder.

`Recorder` stores monotonic seconds and emits playback events through GLib sources. `formats` converts that neutral representation to and from SMF ticks, keeping format logic independent of the UI. `AudioEngine` owns FluidSynth voices and implements CC64 sustain release semantics.
