"""Persistence helpers used by the window-level session coordinator."""
import json
from pathlib import Path

def canonical_midi_path(path: str) -> str:
    value = Path(path).expanduser().resolve()
    if value.suffix.lower() not in {'.mid', '.midi'}: raise ValueError('Expected a MIDI file')
    return str(value)

def serialise_session(tempo: int, notes: list[dict]) -> str:
    return json.dumps({'tempo': tempo, 'notes': notes}, separators=(',', ':'))
