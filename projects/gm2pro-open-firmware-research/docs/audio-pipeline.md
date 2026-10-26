# Audio Pipeline

## Areas To Map

- A2DP codec negotiation and decode path.
- SBC/AAC support and bitpool/codec parameters.
- Game or low-latency mode changes.
- DSP/EQ tables and gain stages.
- DAC/PWM/I2S speaker output path.
- Microphone ADC/PDM path.
- HFP CVSD/mSBC call path.
- TWS synchronization between earbuds.

## Test Matrix

| Test | Capture | Expected Documentation |
| --- | --- | --- |
| Music playback | HCI log + audio observation | Codec and transport behavior |
| AVRCP controls | HCI log | Play/pause/next/volume mappings |
| Phone call | HCI log | HFP codec and mic path |
| Game mode | HCI log + latency observation | Profile or vendor command changes |

