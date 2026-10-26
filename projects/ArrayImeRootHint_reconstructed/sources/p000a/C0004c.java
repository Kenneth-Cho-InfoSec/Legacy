package p000a;

import android.view.KeyEvent;

/* JADX INFO: renamed from: a.c */
/* JADX INFO: loaded from: classes.dex */
public final class C0004c {

    /* JADX INFO: renamed from: a */
    public int f82a;

    /* JADX INFO: renamed from: b */
    public final int f83b;

    /* JADX INFO: renamed from: c */
    public final char[] f84c;

    /* JADX INFO: renamed from: d */
    public final KeyEvent f85d;

    /* JADX INFO: renamed from: e */
    public C0005c0 f86e;

    public C0004c(char c) {
        this.f83b = 0;
        this.f85d = null;
        this.f86e = null;
        this.f82a = -268435450;
        this.f84c = new char[]{c};
    }

    public C0004c(int i) {
        this.f83b = 0;
        this.f84c = null;
        this.f85d = null;
        this.f86e = null;
        this.f82a = i;
    }

    public C0004c(int i, int i2) {
        this.f84c = null;
        this.f85d = null;
        this.f86e = null;
        this.f82a = i;
        this.f83b = i2;
    }

    public C0004c(int i, KeyEvent keyEvent) {
        this.f83b = 0;
        this.f84c = null;
        this.f86e = null;
        this.f82a = -268435442;
        this.f85d = keyEvent;
    }

    public C0004c(C0005c0 c0005c0) {
        this.f83b = 0;
        this.f84c = null;
        this.f85d = null;
        this.f82a = -268435445;
        this.f86e = c0005c0;
    }

    public C0004c(KeyEvent keyEvent) {
        this.f82a = 0;
        this.f83b = 0;
        this.f84c = null;
        this.f85d = null;
        this.f86e = null;
        this.f82a = keyEvent.getAction() != 1 ? -268435449 : -268435425;
        this.f85d = keyEvent;
    }
}
