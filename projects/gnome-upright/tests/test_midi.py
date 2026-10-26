import sys
from pathlib import Path
sys.path.insert(0,str(Path(__file__).parents[1]/'src'))
from gnome_upright.constants import START_NOTE, KEY_COUNT
def test_piano_range_is_25_keys(): assert START_NOTE + KEY_COUNT - 1 == 72
