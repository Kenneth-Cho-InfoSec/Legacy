package com.jni.WifiCameraInfo;

public class WifiCameraParameter {
    public int auto_white_balance;
    public int backlight_campensation;
    public int brightness;
    public int contrast;
    public int gain;
    public int gamma;
    public int hue;
    public int saturation;
    public int sharpness;
    public int white_balance_temperature;

    public void SetParameterbrightness(int i) {
        this.brightness = i;
    }

    public void SetParametercontrast(int i) {
        this.contrast = i;
    }

    public void SetParameterhue(int i) {
        this.hue = i;
    }

    public void SetParametersaturation(int i) {
        this.saturation = i;
    }

    public void SetParametersharpness(int i) {
        this.sharpness = i;
    }

    public void SetParametergamma(int i) {
        this.gamma = i;
    }

    public void SetParameterauto_white_balance(int i) {
        this.auto_white_balance = i;
    }

    public void SetParameterwhite_balance_temperature(int i) {
        this.white_balance_temperature = i;
    }

    public void SetParameterbacklight_campensation(int i) {
        this.backlight_campensation = i;
    }

    public void SetParametergain(int i) {
        this.gain = i;
    }

    public int GetParameterbrightness() {
        return this.brightness;
    }

    public int GetParametercontrast() {
        return this.contrast;
    }

    public int GetParameterhue() {
        return this.hue;
    }

    public int GetParametersaturation() {
        return this.saturation;
    }

    public int GetParametersharpness() {
        return this.sharpness;
    }

    public int GetParametergamma() {
        return this.gamma;
    }

    public int GetParameterauto_white_balance() {
        return this.auto_white_balance;
    }

    public int GetParameterwhite_balance_temperature() {
        return this.white_balance_temperature;
    }

    public int GetParameterbacklight_campensation() {
        return this.backlight_campensation;
    }

    public int GetParametergain() {
        return this.gain;
    }
}
