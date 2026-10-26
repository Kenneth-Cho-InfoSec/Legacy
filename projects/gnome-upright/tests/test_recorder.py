import sys
from pathlib import Path
sys.path.insert(0,str(Path(__file__).parents[1]/'src'))
from gnome_upright.recording import Recorder
def test_empty_recording_duration(): assert Recorder().duration() == 0
