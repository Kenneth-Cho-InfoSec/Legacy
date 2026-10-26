package com.jni.WifiCameraInfo;

public class WifiCameraLicInfo {

    private String f80sn = null;
    private byte[] license = null;

    public String getSn() {
        return this.f80sn;
    }

    public byte[] getLicense() {
        return this.license;
    }

    public void Setsn(String str) {
        this.f80sn = str;
    }

    public void Setlicense(byte[] bArr) {
        this.license = bArr;
    }
}
