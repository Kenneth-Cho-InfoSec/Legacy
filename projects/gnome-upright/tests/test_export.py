import sys
from pathlib import Path
sys.path.insert(0,str(Path(__file__).parents[1]/'src'))
from gnome_upright.models import Note
def test_note_duration_is_positive(): assert Note(60,1,1).duration == .001
