package com.jni.WifiCameraInfo;

public class WifiCameraFirmInfo {
    private String vendor = null;
    private String product = null;
    private String version = null;
    private String ssid = null;
    private String mac = null;
    private int capacity = 0;
    private int recordStatusSync = 0;
    public int CAMCAP_LIGHT = 1;
    public int CAMCAP_GSENSOR = 2;
    public int CAMCAP_MOTOR = 4;
    public int CAMCAP_ADEFFECT = 8;
    public int CAMCAP_MODE = 16;
    public int CAMCAP_DisableBattery = 32;
    public int CAMCAP_ResolutionSwitching = 64;
    public int CAMCAP_GSENSOR_MJPEG = 128;
    public int CAMCAP_MAX = 128;

    public String getvendor() {
        return this.vendor;
    }

    public String getproduct() {
        return this.product;
    }

    public String getversion() {
        return this.version;
    }

    public String getssid() {
        return this.ssid;
    }

    public String getmac() {
        return this.mac;
    }

    public void Setvendor(String str) {
        this.vendor = str;
    }

    public void Setproduct(String str) {
        this.product = str;
    }

    public void Setversion(String str) {
        this.version = str;
    }

    public void Setssid(String str) {
        this.ssid = str;
    }

    public void SetMac(String str) {
        this.mac = str;
    }

    public void SetCapacity(int i) {
        this.capacity = i;
    }

    public void SetrecordStatusSync(int i) {
        this.recordStatusSync = i;
    }

    public boolean suportCheck(int i) {
        int i2 = this.capacity;
        if ((i & i2) == i) {
            return true;
        }
        int i3 = this.CAMCAP_GSENSOR;
        return i == i3 && (i3 & i2) == i3;
    }
}
