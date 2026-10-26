package com.jni.CallBack;

import android.content.Context;
import android.net.Network;
import android.os.SystemClock;
import com.jni.WifiCameraApi;
import java.io.FileDescriptor;
import java.lang.reflect.Field;

public class WifiCallBackFuc {
    public boolean breakbindflag;
    public Context gcontext;
    public Network gnetwork;
    public String name;
    public String name2;
    public int start;

    public static class conConfigHolder {
        public static WifiCallBackFuc gWifiCallBackFuc = new WifiCallBackFuc();
    }

    public void fdtowifi(int i) {
    }

    private WifiCallBackFuc() {
        this.start = 0;
        this.breakbindflag = false;
        this.name = "allpacket";
        this.name2 = "allproduct";
    }

    public void breakbind() {
        this.breakbindflag = true;
    }

    public int bindSocketToNetwork(int i) {
        if (this.breakbindflag) {
            return 0;
        }
        if (this.gnetwork != null) {
            FileDescriptor fileDescriptor = new FileDescriptor();
            try {
                Field declaredField = FileDescriptor.class.getDeclaredField("descriptor");
                declaredField.setAccessible(true);
                declaredField.setInt(fileDescriptor, i);
                this.gnetwork.bindSocket(fileDescriptor);
                return 0;
            } catch (Exception e) {
                e.printStackTrace();
            }
        }
        return 1;
    }

    public int bindSocketToNetworkOut(int i) {
        // SecureeEar is local-only. The original app exposed a second bind path
        // for non-Wi-Fi networks; keep the JNI method available but refuse it.
        return 1;
    }

    public synchronized String getPackageName(Context context) {
        return context == null ? null : context.getPackageName();
    }

    public String getString() {
        return this.name;
    }

    public String getString2() {
        return this.name2;
    }

    public String getString3() {
        return getPackageName(this.gcontext);
    }

    public void setContext(Context context) {
        if (this.gcontext == null) {
            this.gcontext = context;
        }
        loop();
    }

    public void setLocalNetwork(Network network) {
        this.gnetwork = network;
    }

    /* JADX WARN: Type inference failed for: r0v2, types: [com.jni.CallBack.WifiCallBackFuc$3] */
    public void loop() {
        if (this.start == 0) {
            this.start = 1;
            new Thread() {
                @Override
                public void run() {
                    WifiCameraApi.getInstance().gWifiCamera.CallBackFucStart(WifiCallBackFuc.getInstance());
                }
            }.start();
            SystemClock.sleep(100L);
        }
    }

    public static WifiCallBackFuc getInstance() {
        return conConfigHolder.gWifiCallBackFuc;
    }
}
