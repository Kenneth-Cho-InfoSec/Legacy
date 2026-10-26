package com.jni;

import com.jni.CallBack.WifiCallBack;
import com.jni.CallBack.WifiCallBackFuc;
import com.jni.WifiCameraInfo.WifiCameraCmd;
import com.jni.WifiCameraInfo.WifiCameraExposure;
import com.jni.WifiCameraInfo.WifiCameraFirmInfo;
import com.jni.WifiCameraInfo.WifiCameraLedStatus;
import com.jni.WifiCameraInfo.WifiCameraLicInfo;
import com.jni.WifiCameraInfo.WifiCameraParameter;
import com.jni.WifiCameraInfo.WifiCameraPic;
import com.jni.WifiCameraInfo.WifiCameraResolution;
import com.jni.WifiCameraInfo.WifiCameraStatusInfo;
import com.jni.WifiCameraInfo.WifiScreenParameters;
import java.util.ArrayList;

public class WifiCamera {
    public static final int LED_CAMERA = 1;

    public native int CallBackFucStart(WifiCallBackFuc wifiCallBackFuc);

    public native int CallBackStart(WifiCallBack wifiCallBack);

    public native int caAllStreamGet();

    public native int caInit();

    public native int caStart();

    public native int caStop();

    public native int cameraAcceptFileList(String str, ArrayList<cameraFileInfoBean> arrayList);

    public native int cameraChangeAngleDiff(int i);

    public native int cameraCheckOnline();

    public native int cameraCmd(WifiCameraCmd wifiCameraCmd);

    public native int cameraDelShake(int i);

    public native int cameraExposureGet(WifiCameraExposure wifiCameraExposure);

    public native int cameraExposureSet(WifiCameraExposure wifiCameraExposure);

    public native int cameraFirmInfoGet(WifiCameraFirmInfo wifiCameraFirmInfo);

    public native int cameraLedStatusGet(WifiCameraLedStatus wifiCameraLedStatus, int i);

    public native int cameraLedStatusSet(WifiCameraLedStatus wifiCameraLedStatus, int i);

    public native int cameraLicInfoGet(WifiCameraLicInfo wifiCameraLicInfo);

    public native int cameraMaxVangle(int i);

    public native int cameraParameterGet(WifiCameraParameter wifiCameraParameter);

    public native int cameraParameterSet(WifiCameraParameter wifiCameraParameter);

    public native int cameraRecordStatusSync(int i);

    public native int cameraSetLic(String str, byte[] bArr, int i);

    public native int cameraSetLowPowerMode(int i);

    public native int cameraStatusInfoGet(WifiCameraStatusInfo wifiCameraStatusInfo);

    public native int cameraWifiConfGet(WifiCameraResolution wifiCameraResolution);

    public native int cameraWifiConfSet(WifiCameraResolution wifiCameraResolution);

    public native int cameraWifiResolutionGet(ArrayList<WifiCameraResolution> arrayList);

    public native int camerawifiAllowUpFile(String str, int i);

    public native int camerawifiUpDir(String str);

    public native int getAviTime(String str);

    public native int getCameraTimeout();

    public native int getaudio(WifiCameraPic wifiCameraPic);

    public native int getisfilter();

    public native int humidityGet();

    public native int openLog(String str, int i);

    public native int openVideoForceApi();

    public native int screenParametersGet(WifiScreenParameters wifiScreenParameters);

    public native int screenParametersGetFromBuffer(byte[] bArr, int i, WifiScreenParameters wifiScreenParameters);

    public native int screenParametersSet(WifiScreenParameters wifiScreenParameters);

    public native int screenParametersSetTobuffer(byte[] bArr, int i, WifiScreenParameters wifiScreenParameters);

    public native int setCameraTimeout(int i);

    public native int setisfilter(int i);

    public native int startAviVideoRecord(String str, int i, int i2, int i3);

    public native int stopAviVideoRecord();

    public native int temperatureGet();

    public native int tewlGet(int i, int i2, int i3, int i4, int i5);

    public native int updateFirmware(String str);

    public native int updatemcuFirmware(String str);

    static {
        System.loadLibrary("WifiCamera");
    }
}
