package p001b;

import p000a.AbstractC0010h;
import p000a.C0007e;

/* JADX INFO: renamed from: b.g */
/* JADX INFO: loaded from: classes.dex */
public final class C0035g {

    /* JADX INFO: renamed from: f */
    public static C0035g f236f;

    /* JADX INFO: renamed from: g */
    public static int f237g;

    /* JADX INFO: renamed from: a */
    public final C0032d f238a;

    /* JADX INFO: renamed from: b */
    public final AbstractC0010h f239b;

    /* JADX INFO: renamed from: c */
    public int f240c;

    /* JADX INFO: renamed from: d */
    public char f241d;

    /* JADX INFO: renamed from: e */
    public boolean f242e = false;

    public C0035g(AbstractC0010h abstractC0010h) {
        this.f239b = abstractC0010h;
        this.f238a = new C0032d(abstractC0010h);
        f237g = 1;
    }

    /* JADX INFO: renamed from: a */
    public final String m88a(String str) {
        C0030b c0030b;
        C0032d c0032d = this.f238a;
        if (!c0032d.m81a() || (c0030b = c0032d.f228d[0]) == null) {
            return "";
        }
        int length = str.length();
        int i = c0030b.f207c;
        int i2 = length % i;
        int i3 = length / i;
        if (i2 == 1) {
            return str.substring(((i3 - 1) * 2) + i2 + 1);
        }
        return "";
    }

    /* JADX INFO: renamed from: b */
    public final C0033e m89b() {
        C0032d c0032d = this.f238a;
        c0032d.getClass();
        C0033e c0033e = new C0033e(c0032d);
        if (c0033e.f231c == null) {
            return null;
        }
        return c0033e;
    }

    /* JADX INFO: renamed from: c */
    public final int m90c() {
        return ((C0031c) this.f238a.f225a.get(C0032d.f221e)).f210b;
    }

    /* JADX INFO: renamed from: d */
    public final C0007e m91d() {
        return new C0007e(this.f239b, ((C0031c) this.f238a.f225a.get(C0032d.f221e)).f209a);
    }

    /* JADX INFO: renamed from: e */
    public final void m92e() {
        this.f240c = 1;
        this.f242e = false;
    }

    /* JADX INFO: renamed from: f */
    public final void m93f() {
        C0032d c0032d = this.f238a;
        c0032d.getClass();
        int i = C0032d.f223g;
        C0032d.f221e = i;
        c0032d.m83c(i);
    }

    /* JADX INFO: renamed from: g */
    public final void m94g(int i) {
        C0032d c0032d = this.f238a;
        if (i < 0) {
            c0032d.getClass();
            int i2 = C0032d.f223g;
            C0032d.f221e = i2;
            c0032d.m83c(i2);
            return;
        }
        c0032d.getClass();
        int i3 = i + 4;
        C0032d.f221e = i3;
        C0032d.f223g = i3;
        c0032d.m83c(i3);
    }

    /* JADX INFO: renamed from: h */
    public final void m95h() {
        C0032d c0032d = this.f238a;
        c0032d.getClass();
        int i = C0032d.f221e + 1;
        C0032d.f221e = i;
        if (i > c0032d.f225a.size() - 1) {
            C0032d.f221e = 0;
        } else {
            int i2 = C0032d.f221e;
            if (i2 > 0 && i2 < 4) {
                C0032d.f221e = 4;
            }
        }
        int i3 = C0032d.f221e;
        if (i3 >= 4) {
            c0032d.m83c(i3);
            C0032d.f223g = C0032d.f221e;
        }
    }

    /* JADX INFO: renamed from: i */
    public final void m96i() {
        int i;
        C0032d c0032d = this.f238a;
        c0032d.getClass();
        int i2 = C0032d.f221e;
        if (i2 < 4) {
            i = C0032d.f223g;
            C0032d.f221e = i;
        } else {
            int i3 = i2 + 1;
            C0032d.f221e = i3;
            if (i3 > c0032d.f225a.size() - 1) {
                C0032d.f221e = 4;
            }
            i = C0032d.f221e;
            C0032d.f223g = i;
        }
        c0032d.m83c(i);
    }

    /* JADX INFO: renamed from: j */
    public final void m97j() {
        int i;
        this.f238a.getClass();
        int i2 = C0032d.f221e;
        if (i2 >= 4 || i2 < 0) {
            i = C0032d.f224h;
        } else {
            if (i2 != 0) {
                if (i2 > 0 && i2 < 4) {
                    i = 0;
                }
                C0032d.f224h = C0032d.f221e;
            }
            i = C0032d.f222f;
        }
        C0032d.f221e = i;
        C0032d.f224h = C0032d.f221e;
    }

    /* JADX INFO: renamed from: k */
    public final void m98k() {
        int i;
        this.f238a.getClass();
        int i2 = C0032d.f221e;
        if (i2 >= 4 || i2 <= 0) {
            i = C0032d.f222f;
        } else {
            if (i2 <= 0 || i2 >= 3) {
                C0032d.f221e = 1;
                C0032d.f222f = C0032d.f221e;
            }
            i = i2 + 1;
        }
        C0032d.f221e = i;
        C0032d.f222f = C0032d.f221e;
    }

    /* JADX WARN: Removed duplicated region for block: B:11:0x0022  */
    /* JADX WARN: Removed duplicated region for block: B:13:? A[RETURN, SYNTHETIC] */
    /* JADX INFO: renamed from: l */
    /*
        Code decompiled incorrectly, please refer to instructions dump.
    */
    public final void m99l() {
        int size;
        int i;
        C0032d c0032d = this.f238a;
        c0032d.getClass();
        int i2 = C0032d.f221e - 1;
        C0032d.f221e = i2;
        if (i2 >= 0) {
            if (i2 > 1 && i2 < 4) {
                size = 0;
            }
            i = C0032d.f221e;
            if (i < 4) {
                c0032d.m83c(i);
                C0032d.f223g = C0032d.f221e;
                return;
            }
            return;
        }
        size = c0032d.f225a.size() - 1;
        C0032d.f221e = size;
        i = C0032d.f221e;
        if (i < 4) {
        }
    }

    /* JADX INFO: renamed from: m */
    public final void m100m() {
        C0032d c0032d = this.f238a;
        c0032d.getClass();
        int i = C0032d.f224h;
        C0032d.f221e = i;
        c0032d.m83c(i);
    }
}
