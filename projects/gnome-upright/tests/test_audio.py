import sys
from pathlib import Path
sys.path.insert(0,str(Path(__file__).parents[1]/'src'))
from gnome_upright.audio import AudioEngine
def test_audio_engine_constructs(): assert AudioEngine() is not None
