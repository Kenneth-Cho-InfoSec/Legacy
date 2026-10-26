package com.jni.WifiCameraInfo;

public class WifiCameraExposure {
    private int exposureAuto = 0;
    private int exposureAbsolute = 0;

    public void SetExposureAuto(int i) {
        this.exposureAuto = i;
    }

    public void SetExposureAbsolute(int i) {
        this.exposureAbsolute = i;
    }

    public int GetExposureAuto() {
        return this.exposureAuto;
    }

    public int GetExposureAbsolute() {
        return this.exposureAbsolute;
    }
}
