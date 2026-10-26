package com.jni.WifiCameraInfo;

public class WifiCameraResolution {
    public int bFrameIndex;
    public int wHeight;
    public int wWidth;

    public void SetFrameIndex(int i) {
        this.bFrameIndex = i;
    }

    public void SetWidth(int i) {
        this.wWidth = i;
    }

    public void SetHeight(int i) {
        this.wHeight = i;
    }

    public int GetbFormatType() {
        return this.bFrameIndex;
    }

    public int GetwWidth() {
        return this.wWidth;
    }

    public int GetwHeight() {
        return this.wHeight;
    }
}
