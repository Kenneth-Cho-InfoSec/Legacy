package com.jni.WifiCameraInfo;

public class WifiCameraPic {
    public byte[] data = null;
    public int type = 0;
    public int hasg = 0;
    public float angle = 0.0f;
    public float anglev = 0.0f;
    public float anglel = 0.0f;
    public int width = 0;
    public int height = 0;
    public int drop = 0;
    public int queuedrop = 0;
    public int okstream = 0;
    public int allstream = 0;
    public int picbutton = 0;
    public int videobutton = 0;
    public int zoomup = 0;
    public int zoomdown = 0;
    public int focusData = 0;
    public int frozen = 0;
    public int mirror = 0;
    public int audionum = 0;
    public int hasgyroscope_6axis = 0;
    public byte[] gyroscope_6axis = null;

    public byte[] getData() {
        return this.data;
    }

    public void SetData(byte[] bArr) {
        this.data = bArr;
    }

    public void SetType(int i) {
        this.type = i;
    }

    public int GetType() {
        return this.type;
    }

    public void Setangle(float f) {
        this.angle = f;
    }

    public float Getangle() {
        return this.angle;
    }

    public void Sethasg(int i) {
        this.hasg = i;
    }

    public int Gethasg() {
        return this.hasg;
    }

    public void Setwidth(int i) {
        this.width = i;
    }

    public int Getwidth() {
        return this.width;
    }

    public void Setheight(int i) {
        this.height = i;
    }

    public int Getheight() {
        return this.height;
    }

    public void Setdrop(int i) {
        this.drop = i;
    }

    public int Getdrop() {
        return this.drop;
    }

    public void Setqueuedrop(int i) {
        this.queuedrop = i;
    }

    public void Setokstream(int i) {
        this.okstream = i;
    }

    public void Setallstream(int i) {
        this.allstream = i;
    }

    public void Setanglev(float f) {
        this.anglev = f;
    }

    public void Setanglel(float f) {
        this.anglel = f;
    }

    public void Setpicbutton(int i) {
        this.picbutton = i;
    }

    public void Setvideobutton(int i) {
        this.videobutton = i;
    }

    public void Setzoomup(int i) {
        this.zoomup = i;
    }

    public void Setzoomdown(int i) {
        this.zoomdown = i;
    }

    public void SetfocusData(int i) {
        this.focusData = i;
    }

    public void Setfrozen(int i) {
        this.frozen = i;
    }

    public void Setmirror(int i) {
        this.mirror = i;
    }

    public void Setaudionum(int i) {
        this.audionum = i;
    }

    public byte[] getgyroscope_6axis() {
        return this.gyroscope_6axis;
    }

    public void Setnewg(byte[] bArr) {
        this.gyroscope_6axis = bArr;
    }

    public void Sethasnewg(int i) {
        this.hasgyroscope_6axis = i;
    }
}
