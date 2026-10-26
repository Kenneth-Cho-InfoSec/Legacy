package com.jni;

import android.content.Context;
import android.os.Environment;
import android.os.SystemClock;
import com.jni.CallBack.WifiCallBack;
import com.jni.CallBack.WifiCallBackFuc;
import com.jni.WifiCameraInfo.WifiCameraPic;
import com.jni.logmanageWifi.LogWD;

public class WifiCameraApi {
    public static String APP_SDCARD = Environment.getExternalStorageDirectory().getPath();
    public WifiCamera gWifiCamera;
    private boolean isInit;
    public boolean log_switch;

    public static class conConfigHolder {
        public static WifiCameraApi gWifiCameraApi = new WifiCameraApi();
    }

    private WifiCameraApi() {
        this.gWifiCamera = null;
        this.isInit = false;
        this.log_switch = true;
        this.gWifiCamera = new WifiCamera();
    }

    /* JADX WARN: Type inference failed for: r0v0, types: [com.jni.WifiCameraApi$1] */
    public void audiostart() {
        new Thread() {
            @Override
            public void run() {
                WifiCameraPic wifiCameraPic = new WifiCameraPic();
                while (WifiCameraApi.this.isInit) {
                    if (WifiCameraApi.this.gWifiCamera.getaudio(wifiCameraPic) < 0) {
                        SystemClock.sleep(10L);
                    } else {
                        WifiCallBack.getInstance().sendaudio(wifiCameraPic, 3);
                    }
                }
            }
        }.start();
    }

    public void init(Context context) {
        if (this.isInit) {
            return;
        }
        LogWD.writeMsg(this, 2, "begin2");
        this.isInit = true;
        if (this.log_switch) {
            this.gWifiCamera.openLog(APP_SDCARD, 1);
        }
        String str = APP_SDCARD + "/test.jpeg";
        WifiCallBackFuc.getInstance().setContext(context);
        WifiCallBack.getInstance().loop();
        this.gWifiCamera.caInit();
        LogWD.writeMsg(this, 2, "end");
    }

    public void setLog_switch(boolean z) {
        this.log_switch = z;
    }

    public static WifiCameraApi getInstance() {
        return conConfigHolder.gWifiCameraApi;
    }
}
