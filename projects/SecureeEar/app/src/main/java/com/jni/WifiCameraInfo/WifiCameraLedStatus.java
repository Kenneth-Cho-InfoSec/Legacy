package com.jni.WifiCameraInfo;

public class WifiCameraLedStatus {
    public static final int OP_LED_BLINK = 2;
    public static final int OP_LED_BREATH = 3;
    public static final int OP_LED_OFF = 0;
    public static final int OP_LED_ON = 1;
    public int ledStatus;
    public int lightValue;

    public void SetledStatus(int i) {
        this.ledStatus = i;
    }

    public int GetledStatus() {
        return this.ledStatus;
    }

    public void SetlightValue(int i) {
        this.lightValue = i;
    }

    public int GetlightValue() {
        return this.lightValue;
    }
}
