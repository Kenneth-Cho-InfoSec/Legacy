package com.kennethchoinfosec.secureeear.camera;

import android.content.Context;
import android.graphics.Bitmap;
import android.graphics.BitmapFactory;
import android.os.Handler;
import android.os.Looper;

import com.jni.CallBack.WifiCallBack;
import com.jni.WifiCameraApi;
import com.jni.WifiCameraInfo.WifiCameraPic;
import com.jni.WifiCameraInfo.WifiCameraStatusInfo;
import com.jni.logmanageWifi.LogManagerWD;

import java.io.File;
import java.util.Arrays;
import java.util.Locale;
import java.util.concurrent.ExecutorService;
import java.util.concurrent.Executors;
import java.util.concurrent.atomic.AtomicBoolean;

public final class NativeCameraController implements WifiCallBack.DeviceStatusInterface {
    public interface Listener {
        void onStatus(String value);

        void onDevice(String value);

        void onFrame(Bitmap bitmap, String metadata);

        void onMetrics(long frames, long audioPackets);
    }

    private final Context appContext;
    private final Handler mainHandler = new Handler(Looper.getMainLooper());
    private final ExecutorService cameraExecutor = Executors.newSingleThreadExecutor();
    private final ExecutorService frameExecutor = Executors.newSingleThreadExecutor();
    private final AtomicBoolean started = new AtomicBoolean(false);
    private final AtomicBoolean decodingFrame = new AtomicBoolean(false);
    private final Listener listener;
    private long frameCount;
    private long audioCount;
    private long lastPreviewMs;

    public NativeCameraController(Context context, Listener listener) {
        this.appContext = context.getApplicationContext();
        this.listener = listener;
    }

    public boolean isStarted() {
        return started.get();
    }

    public void start() {
        if (!started.compareAndSet(false, true)) {
            postStatus("Camera already running");
            return;
        }
        postStatus("Starting secure local camera session");
        cameraExecutor.execute(new Runnable() {
            @Override
            public void run() {
                try {
                    File logDir = new File(appContext.getExternalFilesDir(null), "local-camera-log");
                    if (!logDir.exists() && !logDir.mkdirs()) {
                        postStatus("Local log directory unavailable");
                    }
                    WifiCameraApi.APP_SDCARD = logDir.getAbsolutePath();
                    LogManagerWD.APP_SDCARD = logDir.getAbsolutePath();
                    WifiCallBack.getInstance().setDeviceStatusInterface(NativeCameraController.this);

                    WifiCameraApi api = WifiCameraApi.getInstance();
                    api.setLog_switch(false);
                    api.init(appContext);
                    int startResult = api.gWifiCamera.caStart();
                    postStatus("Camera start result: " + startResult);
                    postDevice("Waiting for camera stream");
                } catch (Throwable throwable) {
                    started.set(false);
                    postStatus("Start failed: " + throwable.getClass().getSimpleName());
                }
            }
        });
    }

    public void stop() {
        if (!started.getAndSet(false)) {
            postStatus("Camera already stopped");
            return;
        }
        cameraExecutor.execute(new Runnable() {
            @Override
            public void run() {
                try {
                    int stopResult = WifiCameraApi.getInstance().gWifiCamera.caStop();
                    postStatus("Camera stopped: " + stopResult);
                } catch (Throwable throwable) {
                    postStatus("Stop failed: " + throwable.getClass().getSimpleName());
                }
            }
        });
    }

    public void shutdown() {
        if (started.get()) {
            stop();
        }
        cameraExecutor.shutdown();
        frameExecutor.shutdown();
    }

    @Override
    public void getStatus(int dtype, int status) {
        String label;
        switch (status) {
            case 1:
                label = "device found";
                break;
            case 2:
                label = "online";
                postDevice("Online; receiving local camera callbacks");
                break;
            case 3:
                label = "online failed";
                break;
            case 4:
                label = "offline";
                break;
            case 5:
                label = "socket creation failed";
                break;
            default:
                label = "unknown";
                break;
        }
        postStatus("Camera " + label + " (" + status + ")");
    }

    @Override
    public void getData(int dtype, WifiCameraPic wifiCameraPic) {
        try {
            if (wifiCameraPic == null || wifiCameraPic.data == null || wifiCameraPic.data.length == 0) {
                return;
            }
            frameCount++;
            long now = System.currentTimeMillis();
            if (now - lastPreviewMs <= 100 || !decodingFrame.compareAndSet(false, true)) {
                postMetrics();
                return;
            }
            lastPreviewMs = now;
            final byte[] frameBytes = Arrays.copyOf(wifiCameraPic.data, wifiCameraPic.data.length);
            final int width = wifiCameraPic.width;
            final int height = wifiCameraPic.height;
            final float angle = wifiCameraPic.angle;
            final int type = wifiCameraPic.type;
            frameExecutor.execute(new Runnable() {
                @Override
                public void run() {
                    try {
                        if (!looksLikeJpeg(frameBytes)) {
                            postStatus("Stream active; waiting for JPEG preview frame");
                            return;
                        }
                        BitmapFactory.Options options = new BitmapFactory.Options();
                        options.inPreferredConfig = Bitmap.Config.RGB_565;
                        Bitmap bitmap = BitmapFactory.decodeByteArray(frameBytes, 0, frameBytes.length, options);
                        if (bitmap != null) {
                            String metadata = String.format(Locale.US,
                                    "%dx%d angle %.1f type %d", width, height, angle, type);
                            postFrame(bitmap, metadata);
                        }
                    } catch (Throwable throwable) {
                        postStatus("Frame skipped: " + throwable.getClass().getSimpleName());
                    } finally {
                        decodingFrame.set(false);
                    }
                }
            });
            postMetrics();
        } catch (Throwable throwable) {
            decodingFrame.set(false);
            postStatus("Frame callback skipped: " + throwable.getClass().getSimpleName());
        }
    }

    @Override
    public void getaudioData(int dtype, WifiCameraPic wifiCameraPic) {
        try {
            audioCount++;
            postMetrics();
        } catch (Throwable throwable) {
            postStatus("Audio callback skipped: " + throwable.getClass().getSimpleName());
        }
    }

    @Override
    public void getCameraInfo(int dtype, WifiCameraStatusInfo info) {
        try {
            if (info != null) {
                postDevice("Battery " + info.battery + "% | charge " + info.isCharge + " | shared " + info.isusedbyother);
            }
        } catch (Throwable throwable) {
            postStatus("Status callback skipped: " + throwable.getClass().getSimpleName());
        }
    }

    @Override
    public int sendnotifyfile(int i, int i2, int i3, String path) {
        postStatus("Local file event received");
        return 0;
    }

    private void postStatus(final String value) {
        mainHandler.post(new Runnable() {
            @Override
            public void run() {
                listener.onStatus(value);
            }
        });
    }

    private void postDevice(final String value) {
        mainHandler.post(new Runnable() {
            @Override
            public void run() {
                listener.onDevice(value);
            }
        });
    }

    private void postFrame(final Bitmap bitmap, final String metadata) {
        mainHandler.post(new Runnable() {
            @Override
            public void run() {
                listener.onFrame(bitmap, metadata);
            }
        });
    }

    private void postMetrics() {
        mainHandler.post(new Runnable() {
            @Override
            public void run() {
                listener.onMetrics(frameCount, audioCount);
            }
        });
    }

    private boolean looksLikeJpeg(byte[] data) {
        return data.length > 4
                && (data[0] & 0xff) == 0xff
                && (data[1] & 0xff) == 0xd8;
    }
}
