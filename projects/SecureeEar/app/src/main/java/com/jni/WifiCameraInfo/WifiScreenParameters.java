package com.jni.WifiCameraInfo;

public class WifiScreenParameters {
    public int angle;
    public int high;
    public int wide;

    public void SetAngle(int i) {
        this.angle = i;
    }

    public int GetAngle() {
        return this.angle;
    }

    public void SetWide(int i) {
        this.wide = i;
    }

    public void SetHigh(int i) {
        this.high = i;
    }

    public int GetWide() {
        return this.wide;
    }

    public int GetHigh() {
        return this.high;
    }
}
