from gi.repository import Adw, Gio, GLib, Gtk
from .constants import APP_ID
from .window import UprightWindow

class UprightApplication(Adw.Application):
    def __init__(self):
        super().__init__(application_id=APP_ID, flags=Gio.ApplicationFlags.HANDLES_OPEN); self.create_action('quit',lambda *_:self.quit(),['<primary>q'])
    def create_action(self,name,callback,accels=[]):
        action=Gio.SimpleAction.new(name,None); action.connect('activate',callback); self.add_action(action); self.set_accels_for_action(f'app.{name}',accels)
    def do_activate(self):
        win=self.props.active_window or UprightWindow(self); win.present()
    def do_open(self,files,n_hint):
        self.do_activate(); self.props.active_window.open_file(files[0])
