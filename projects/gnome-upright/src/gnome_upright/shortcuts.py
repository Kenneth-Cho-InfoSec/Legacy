def install(application, bindings: dict[str, str]) -> None:
    """Apply user-configured accelerators through Gio's action accelerator system."""
    for name in ('record', 'play', 'stop', 'export'):
        application.set_accels_for_action(f'win.{name}', [bindings[name]])
