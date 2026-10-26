package com.jni.WifiCameraInfo;

public class WifiCameraCmd {
    public int action;
    public int item;
    public int type;
    public int value;
    public int CAMCAP_LIGHT = 1;
    public int CAMCAP_GSENSOR = 2;
    public int CAMCAP_MOTOR = 4;
    public int CAMCAP_ADEFFECT = 8;
    public int CAMCAP_MODE = 16;
    public int CAMCAP_MAX = 32;
    public int OP_GET = 0;
    public int OP_SET = 1;
    public int ACTION_LED_OFF = 0;
    public int ACTION_LED_ON = 1;
    public int ACTION_LED_BLINK = 2;
    public int ACTION_LED_BREATH = 3;
    public int ACTION_LED_BUTTON = 4;
    public int ACTION_MOTOR_OFF = 0;
    public int ACTION_MOTOR_ON = 1;
    public int ACTION_MOTOR_AUTO = 5;

    public int getItem() {
        return this.item;
    }

    public int getType() {
        return this.type;
    }

    public int getAction() {
        return this.action;
    }

    public int getValue() {
        return this.value;
    }

    public void SetAction(int i) {
        this.action = i;
    }

    public void SetValue(int i) {
        this.value = i;
    }
}
