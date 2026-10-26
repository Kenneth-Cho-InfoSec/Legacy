from gi.repository import Gio
from .constants import APP_ID

def settings() -> Gio.Settings:
    """Return the process-wide schema-backed preference store."""
    return Gio.Settings.new(APP_ID)
