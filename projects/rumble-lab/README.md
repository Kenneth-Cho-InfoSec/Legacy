# Rumble Lab

A single-page browser gamepad vibration testing tool.

## Features

- Controller detection with automatic reconnect polling
- Dual-rumble haptics via `gamepad.vibrationActuator.playEffect("dual-rumble", ...)`
- Firefox fallback via `gamepad.hapticActuators[0].pulse(...)`
- One-shot, constant, pulse, ramp-up, and ramp-down vibration modes
- Material 3 Expressive-inspired responsive UI

Open `index.html` in a supported browser and connect a compatible gamepad.
