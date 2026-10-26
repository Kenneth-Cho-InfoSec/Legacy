from __future__ import annotations
from dataclasses import asdict, dataclass

@dataclass(slots=True)
class Note:
    pitch: int
    start: float
    end: float | None = None
    velocity: int = 100
    channel: int = 0
    source: str = ''

    @property
    def duration(self) -> float:
        return max(0.001, (self.end or self.start) - self.start)

    def to_dict(self) -> dict:
        return asdict(self)

@dataclass(slots=True)
class Session:
    tempo: int = 120
    time_signature: tuple[int, int] = (4, 4)
    notes: list[Note] | None = None

    def __post_init__(self):
        if self.notes is None: self.notes = []
