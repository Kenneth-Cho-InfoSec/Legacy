package com.jni;

import java.io.Serializable;

public class cameraFileInfoBean implements Serializable {
    private String mFileName;
    private long mFileSize;
    private int mFileTime;
    private boolean mIsDir;

    public boolean ismIsDir() {
        return this.mIsDir;
    }

    public void setmIsDir(boolean z) {
        this.mIsDir = z;
    }

    public String getmFileName() {
        return this.mFileName;
    }

    public void setmFileName(String str) {
        this.mFileName = str;
    }

    public long getmFileSize() {
        return this.mFileSize;
    }

    public void setmFileSize(long j) {
        this.mFileSize = j;
    }

    public int getmFileTime() {
        return this.mFileTime;
    }

    public void setmFileTime(int i) {
        this.mFileTime = i;
    }
}
