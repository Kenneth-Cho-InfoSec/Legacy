package com.jni.CallBack;

import android.os.SystemClock;
import com.jni.WifiCameraApi;
import com.jni.WifiCameraInfo.WifiCameraPic;
import com.jni.WifiCameraInfo.WifiCameraStatusInfo;
import com.jni.logmanageWifi.LogWD;

public class WifiCallBack {
    public static final int CAMERAERRINEND = 3;
    public static final int CAMERAIN = 1;
    public static final int CAMERAINEND = 2;
    public static final int CAMERAOUTEND = 4;
    public static final int CAMERA_ADD_FILE = 1;
    public static final int CAMERA_HAVEPIC = 2;
    public static final int CAMERA_HAVEPICEND = 3;
    public static final int CAMERA_NETWORKBAD = 5;
    public static final int DTYPE_I4SEASONPROGROM = 1;
    public static final int DTYPE_I4SEASONPROGROM_NEW = 3;
    public static final int DTYPE_LEXINPROGROM = 2;
    public static final int DTYPE_TIENIU = 10;
    public static final int I4SEASON_FILE_BAD = 4;
    public static final int I4SEASON_FILE_BEGIN = 1;
    public static final int I4SEASON_FILE_OK = 3;
    public static final int I4SEASON_FILE_PROCESS = 2;
    public static final int OFFLINE = 0;
    private DeviceStatusInterface deviceStatusInterface;
    private ICameraEventFileDelegate mICameraEventFileDelegate;
    private ICameraEventKeyDelegate mICameraEventKeyDelegate;
    public int loopflag = 0;
    public boolean online = false;
    public WifiCameraPic datag = null;

    public interface DeviceStatusInterface {
        void getCameraInfo(int i, WifiCameraStatusInfo wifiCameraStatusInfo);

        void getData(int i, WifiCameraPic wifiCameraPic);

        void getStatus(int i, int i2);

        void getaudioData(int i, WifiCameraPic wifiCameraPic);

        int sendnotifyfile(int i, int i2, int i3, String str);
    }

    public interface ICameraEventFileDelegate {
    }

    public interface ICameraEventKeyDelegate {
        void receiveEvent(Object obj, int i);
    }

    public static class conConfigHolder {
        public static WifiCallBack gWifiCallBack = new WifiCallBack();
    }

    public WifiCallBack(ICameraEventFileDelegate iCameraEventFileDelegate) {
        this.mICameraEventFileDelegate = iCameraEventFileDelegate;
    }

    public WifiCallBack(ICameraEventKeyDelegate iCameraEventKeyDelegate) {
        this.mICameraEventKeyDelegate = iCameraEventKeyDelegate;
    }

    public WifiCallBack(ICameraEventFileDelegate iCameraEventFileDelegate, ICameraEventKeyDelegate iCameraEventKeyDelegate) {
        this.mICameraEventKeyDelegate = iCameraEventKeyDelegate;
    }

    public WifiCallBack() {
        LogWD.writeMsg(this, 2, "begin: ");
    }

    public int sendmsg(Object obj, int i) {
        ICameraEventKeyDelegate iCameraEventKeyDelegate;
        LogWD.writeMsg(this, 2, "type: " + i);
        if (i == 1) {
            return 0;
        }
        if (i == 2) {
            ICameraEventKeyDelegate iCameraEventKeyDelegate2 = this.mICameraEventKeyDelegate;
            if (iCameraEventKeyDelegate2 == null) {
                return 0;
            }
            iCameraEventKeyDelegate2.receiveEvent(obj, i);
            return 0;
        }
        if (i != 3 || (iCameraEventKeyDelegate = this.mICameraEventKeyDelegate) == null) {
            return 0;
        }
        iCameraEventKeyDelegate.receiveEvent(obj, i);
        return 0;
    }

    public int sendpic(Object obj, int i) {
        try {
            if (!(obj instanceof WifiCameraPic)) {
                return 0;
            }
            WifiCameraPic wifiCameraPic = (WifiCameraPic) obj;
            this.datag = wifiCameraPic;
            LogWD.writeMsg(this, 2, "sendpic: type= " + i);
            DeviceStatusInterface deviceStatusInterface = this.deviceStatusInterface;
            if (deviceStatusInterface == null) {
                return 0;
            }
            deviceStatusInterface.getData(i, wifiCameraPic);
        } catch (Throwable throwable) {
            LogWD.writeMsg(this, 2, "sendpic skipped: " + throwable.getClass().getSimpleName());
        }
        return 0;
    }

    public int sendaudio(Object obj, int i) {
        try {
            if (!(obj instanceof WifiCameraPic)) {
                return 0;
            }
            WifiCameraPic wifiCameraPic = (WifiCameraPic) obj;
            this.datag = wifiCameraPic;
            LogWD.writeMsg(this, 2, "sendaudio: type= " + i);
            DeviceStatusInterface deviceStatusInterface = this.deviceStatusInterface;
            if (deviceStatusInterface == null) {
                return 0;
            }
            deviceStatusInterface.getaudioData(i, wifiCameraPic);
        } catch (Throwable throwable) {
            LogWD.writeMsg(this, 2, "sendaudio skipped: " + throwable.getClass().getSimpleName());
        }
        return 0;
    }

    public int sendstatus(Object obj, int i) {
        try {
            if (!(obj instanceof WifiCameraStatusInfo)) {
                return 0;
            }
            WifiCameraStatusInfo wifiCameraStatusInfo = (WifiCameraStatusInfo) obj;
            LogWD.writeMsg(this, 2, "sendstatus: type= " + i);
            DeviceStatusInterface deviceStatusInterface = this.deviceStatusInterface;
            if (deviceStatusInterface == null) {
                return 0;
            }
            deviceStatusInterface.getCameraInfo(i, wifiCameraStatusInfo);
        } catch (Throwable throwable) {
            LogWD.writeMsg(this, 2, "sendstatus skipped: " + throwable.getClass().getSimpleName());
        }
        return 0;
    }

    public int sendsig(int i, int i2) {
        try {
            LogWD.writeMsg(this, 2, "sendsig: dtype= " + i + " status=" + i2);
            if (i2 == 2) {
                this.online = true;
            }
            if (i2 == 4 || i2 == 3 || i2 == 1) {
                this.online = false;
            }
            DeviceStatusInterface deviceStatusInterface = this.deviceStatusInterface;
            if (deviceStatusInterface != null) {
                deviceStatusInterface.getStatus(i, i2);
            }
        } catch (Throwable throwable) {
            LogWD.writeMsg(this, 2, "sendsig skipped: " + throwable.getClass().getSimpleName());
        }
        return 0;
    }

    public int sendnotifyfile(int i, int i2, int i3, String str) {
        try {
            DeviceStatusInterface deviceStatusInterface = this.deviceStatusInterface;
            if (deviceStatusInterface != null) {
                return deviceStatusInterface.sendnotifyfile(i, i2, i3, str);
            }
        } catch (Throwable throwable) {
            LogWD.writeMsg(this, 2, "sendnotifyfile skipped: " + throwable.getClass().getSimpleName());
        }
        return 0;
    }

    /* JADX WARN: Type inference failed for: r0v2, types: [com.jni.CallBack.WifiCallBack$1] */
    public int loop() {
        if (this.loopflag != 0) {
            return 0;
        }
        this.loopflag = 1;
        new Thread() {
            @Override
            public void run() {
                WifiCameraApi.getInstance().gWifiCamera.CallBackStart(WifiCallBack.getInstance());
            }
        }.start();
        SystemClock.sleep(200L);
        return 0;
    }

    public static WifiCallBack getInstance() {
        return conConfigHolder.gWifiCallBack;
    }

    public void setDeviceStatusInterface(DeviceStatusInterface deviceStatusInterface) {
        this.deviceStatusInterface = deviceStatusInterface;
    }
}
