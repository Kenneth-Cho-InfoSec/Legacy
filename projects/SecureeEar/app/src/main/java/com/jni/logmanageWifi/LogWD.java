package com.jni.logmanageWifi;

import java.io.PrintWriter;
import java.io.StringWriter;

public class LogWD {
    public static void writeMsg(Object obj, int i, String str) {
        LogManagerWD.getInstance().Log(obj, i, str);
    }

    public static void writeMsg(Throwable th) {
        StringWriter stringWriter = new StringWriter();
        PrintWriter printWriter = new PrintWriter(stringWriter);
        th.printStackTrace(printWriter);
        for (Throwable cause = th.getCause(); cause != null; cause = cause.getCause()) {
            cause.printStackTrace(printWriter);
        }
        printWriter.close();
        LogManagerWD.getInstance().Log(stringWriter.toString());
    }
}
