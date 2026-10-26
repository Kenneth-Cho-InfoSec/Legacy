package com.jni.WifiCameraInfo;

public class WifiCameraStatusInfo {
    public int battery;
    public int camerastatus;
    public int isCharge;
    public int isLowPowerOff;
    public int isSensorOk;
    public int isusedbyother;
    public int motorOnoff;
    public int picchangeid;
    public int workMode;

    public int GetisCharge() {
        return this.isCharge;
    }

    public int Getbattery() {
        return this.battery;
    }

    public int GetisSensorOk() {
        return this.isSensorOk;
    }

    public int GetisLowPowerOff() {
        return this.isLowPowerOff;
    }

    public void SetisCharge(int i) {
        this.isCharge = i;
    }

    public void Setbattery(int i) {
        this.battery = i;
    }

    public void SetisSensorOk(int i) {
        this.isSensorOk = i;
    }

    public void SetisLowPowerOff(int i) {
        this.isLowPowerOff = i;
    }

    public void SetworkMode(int i) {
        this.workMode = i;
    }

    public void SetmotorLevel(int i) {
        this.motorOnoff = i;
    }

    public void Setcamerastatus(int i) {
        this.camerastatus = i;
    }

    public void Setpicchangeid(int i) {
        this.picchangeid = i;
    }

    public void Setisusedbyother(int i) {
        this.isusedbyother = i;
    }
}
