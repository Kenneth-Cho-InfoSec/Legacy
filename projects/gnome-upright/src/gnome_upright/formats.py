from __future__ import annotations
from pathlib import Path
from .models import Note

def export_midi(path: str, notes: list[Note], tempo: int, time_signature=(4,4)):
    import mido
    midi=mido.MidiFile(type=1, ticks_per_beat=480); track=mido.MidiTrack(); midi.tracks.append(track)
    track.append(mido.MetaMessage('track_name', name='GNOME Upright recording', time=0)); track.append(mido.MetaMessage('set_tempo', tempo=mido.bpm2tempo(tempo), time=0)); track.append(mido.MetaMessage('time_signature', numerator=time_signature[0], denominator=time_signature[1], time=0))
    events=[]
    for n in notes: events += [(n.start, 1, n), ((n.end or n.start), 0, n)]
    last=0.
    for when, on, note in sorted(events, key=lambda e:(e[0],e[1])):
        ticks=round((when-last)*tempo/60*480); last=when
        track.append(mido.Message('note_on' if on else 'note_off', note=note.pitch, velocity=note.velocity if on else 0, channel=note.channel, time=ticks))
    track.append(mido.MetaMessage('end_of_track', time=0)); midi.save(path)

def import_midi(path: str) -> tuple[list[Note], int]:
    import mido
    midi=mido.MidiFile(path); tempo=500000; active={}; result=[]; elapsed=0
    for msg in mido.merge_tracks(midi.tracks):
        elapsed += mido.tick2second(msg.time,midi.ticks_per_beat,tempo)
        if msg.type == 'set_tempo': tempo=msg.tempo
        if msg.type == 'note_on' and msg.velocity: active[(msg.channel,msg.note)] = Note(msg.note,elapsed,velocity=msg.velocity,channel=msg.channel)
        elif msg.type in ('note_off','note_on') and (msg.channel,msg.note) in active:
            note=active.pop((msg.channel,msg.note)); note.end=elapsed; result.append(note)
    return result, round(mido.tempo2bpm(tempo))
