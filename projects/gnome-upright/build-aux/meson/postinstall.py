#!/usr/bin/env python3
import os
import subprocess

prefix = os.environ.get('MESON_INSTALL_PREFIX', '/usr/local')
if not os.environ.get('DESTDIR'):
    subprocess.call(['glib-compile-schemas', os.path.join(prefix, 'share/glib-2.0/schemas')])
    subprocess.call(['gtk4-update-icon-cache', '-q', '-t', '-f', os.path.join(prefix, 'share/icons/hicolor')])
