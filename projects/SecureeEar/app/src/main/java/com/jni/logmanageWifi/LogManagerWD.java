package com.jni.logmanageWifi;

import android.os.Environment;
import android.os.Handler;
import android.os.HandlerThread;
import android.text.format.DateFormat;
import android.util.Log;
import java.io.File;
import java.io.FileNotFoundException;
import java.io.IOException;
import java.io.RandomAccessFile;
import java.text.SimpleDateFormat;
import java.util.Calendar;
import java.util.Date;
import java.nio.charset.StandardCharsets;
import java.util.concurrent.locks.Lock;
import java.util.concurrent.locks.ReentrantLock;

public class LogManagerWD {
    public static final String APP_LOG_SAVE = "/Awifi_log1d/";
    public static final String APP_SDCARD_OLD = "/sdcard";
    public static final int MODE_ALL = 16777215;
    public static final int MODE_AOA_TEST = 1;
    public static final int MODE_CLOSE = 0;
    public static final int MODE_LOGIN = 2;
    static LogManagerWD logManager = null;
    private static String logname = "";
    private Handler handler;
    private HandlerThread handlerThread = new HandlerThread("writelog");
    public static String APP_SDCARD = Environment.getExternalStorageDirectory().getPath();
    public static String APP_PACKAGE_NAME = "com.i4seasons.aoademo";
    public static int LOG_SWITCH = 16777215;
    private static Lock reentantLock = new ReentrantLock();

    public LogManagerWD() {
        this.handlerThread.start();
        this.handler = new Handler(this.handlerThread.getLooper());
        logname = new SimpleDateFormat("HH_mm_ss").format(Calendar.getInstance().getTime());
    }

    public static LogManagerWD getInstance() {
        if (logManager == null) {
            try {
                reentantLock.lock();
                if (logManager == null) {
                    logManager = new LogManagerWD();
                }
            } finally {
                reentantLock.unlock();
            }
        }
        return logManager;
    }

    public void Log(Object obj, int i, String str) {
        String className;
        if (checkPermit(i)) {
            StackTraceElement[] stackTrace = Thread.currentThread().getStackTrace();
            int length = stackTrace.length;
            int lineNumber = 0;
            int i2 = 0;
            while (true) {
                if (i2 >= length) {
                    className = "";
                    break;
                }
                StackTraceElement stackTraceElement = stackTrace[i2];
                if (stackTraceElement.getClassName().equals(obj.getClass().getName())) {
                    className = stackTraceElement.getClassName();
                    lineNumber = stackTraceElement.getLineNumber();
                    break;
                }
                i2++;
            }
            String str2 = new SimpleDateFormat("yyyy-MM-dd HH:mm:ss SSS").format(Calendar.getInstance().getTime());
            Log.d("WiFiDisk", "[" + str2 + "]*[ " + className + " + line " + lineNumber + "]\n*[" + str + "]");
            writeMsg("[" + str2 + "]*[ " + className + " + line " + lineNumber + "]\n*[" + str + "]");
        }
    }

    public void Log(Object obj, String str) {
        String className;
        if (checkPermit(16777215)) {
            StackTraceElement[] stackTrace = Thread.currentThread().getStackTrace();
            int length = stackTrace.length;
            int lineNumber = 0;
            int i = 0;
            while (true) {
                if (i >= length) {
                    className = "";
                    break;
                }
                StackTraceElement stackTraceElement = stackTrace[i];
                if (stackTraceElement.getClassName().equals(obj.getClass().getName())) {
                    className = stackTraceElement.getClassName();
                    lineNumber = stackTraceElement.getLineNumber();
                    break;
                }
                i++;
            }
            String str2 = new SimpleDateFormat("yyyy-MM-dd HH:mm:ss SSS").format(Calendar.getInstance().getTime());
            Log.d("WiFiDisk", "[" + str2 + "]*[ " + className + " + line " + lineNumber + "]\n*[" + str + "]");
            writeMsg("[" + str2 + "]*[ " + className + " + line " + lineNumber + "]\n*[" + str + "]");
        }
    }

    public void Log(String str) {
        Log.d("WiFiDisk", str);
        writeMsg(str);
    }

    private boolean checkPermit(int i) {
        return (i & LOG_SWITCH) != 0;
    }

    private void writeMsg(String str) {
        this.handler.post(new WriteRunnable(str));
    }

    private void writeError(String str) {
        RandomAccessFile randomAccessFileMakeErrorFile = makeErrorFile();
        if (randomAccessFileMakeErrorFile != null) {
            try {
                randomAccessFileMakeErrorFile.seek(randomAccessFileMakeErrorFile.length());
                randomAccessFileMakeErrorFile.write((str + "\n\n").getBytes("gbk"));
                randomAccessFileMakeErrorFile.close();
            } catch (IOException e) {
                e.printStackTrace();
            }
        }
    }

    class WriteRunnable implements Runnable {
        private String msg;

        public WriteRunnable(String str) {
            this.msg = str;
        }

        @Override
        public void run() {
            RandomAccessFile randomAccessFileMakeRandomFile = LogManagerWD.this.makeRandomFile();
            if (randomAccessFileMakeRandomFile != null) {
                try {
                    randomAccessFileMakeRandomFile.seek(randomAccessFileMakeRandomFile.length());
                    randomAccessFileMakeRandomFile.write((this.msg + "\n\n").getBytes(StandardCharsets.UTF_8));
                    randomAccessFileMakeRandomFile.close();
                } catch (IOException e) {
                    e.printStackTrace();
                }
            }
        }
    }

    public RandomAccessFile makeRandomFile() {
        if (Environment.getExternalStorageState().equals("mounted")) {
            File file = new File(getLogPath());
            if (!file.exists()) {
                file.mkdir();
            }
            Date time = Calendar.getInstance().getTime();
            new DateFormat();
            File file2 = new File(file.getPath() + File.separator + DateFormat.format("yyyy-MM-dd", time).toString());
            if (!file2.exists()) {
                file2.mkdir();
            }
            try {
                return new RandomAccessFile(new File(file2.getPath() + File.separator + logname + ".txt"), "rw");
            } catch (FileNotFoundException e) {
                e.printStackTrace();
            }
        }
        return null;
    }

    private RandomAccessFile makeErrorFile() {
        if (Environment.getExternalStorageState().equals("mounted")) {
            File file = new File(getLogPath());
            if (!file.exists()) {
                file.mkdir();
            }
            File file2 = new File(file.getPath() + File.separator + DateFormat.format("yyyy-MM-dd", Calendar.getInstance().getTime()).toString());
            if (!file2.exists()) {
                file2.mkdir();
            }
            try {
                return new RandomAccessFile(new File(file2.getPath() + File.separator + logname + "Error.txt"), "rw");
            } catch (FileNotFoundException e) {
                e.printStackTrace();
            }
        }
        return null;
    }

    public static String getLogPath() {
        return APP_SDCARD + APP_LOG_SAVE;
    }
}
