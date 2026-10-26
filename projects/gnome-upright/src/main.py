#!/usr/bin/env python3
import logging
from pathlib import Path
from gnome_upright.application import UprightApplication

if __name__ == '__main__':
    log_dir = Path.home() / '.local' / 'state' / 'gnome-upright'
    log_dir.mkdir(parents=True, exist_ok=True)
    logging.basicConfig(
        level=logging.INFO,
        format='%(asctime)s %(levelname)s %(name)s: %(message)s',
        handlers=[logging.FileHandler(log_dir / 'gnome-upright.log', encoding='utf-8'), logging.StreamHandler()],
    )
    logging.info('Starting GNOME Upright')
    raise SystemExit(UprightApplication().run(None))
