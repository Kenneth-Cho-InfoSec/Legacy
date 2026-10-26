package p001b;

import java.util.ArrayList;
import p000a.C0002b;
import p000a.C0005c0;
import p000a.InterfaceC0003b0;

/* JADX INFO: renamed from: b.e */
/* JADX INFO: loaded from: classes.dex */
public final class C0033e implements InterfaceC0003b0 {

    /* JADX INFO: renamed from: a */
    public C0030b f229a;

    /* JADX INFO: renamed from: b */
    public C0030b f230b;

    /* JADX INFO: renamed from: c */
    public C0030b f231c;

    /* JADX INFO: renamed from: d */
    public C0030b f232d;

    /* JADX INFO: renamed from: e */
    public C0030b f233e;

    /* JADX INFO: renamed from: f */
    public ArrayList f234f = null;

    /* JADX INFO: renamed from: g */
    public final boolean f235g;

    public C0033e(C0032d c0032d) {
        this.f229a = null;
        this.f230b = null;
        this.f231c = null;
        this.f232d = null;
        this.f233e = null;
        C0030b[] c0030bArr = c0032d.f228d;
        this.f229a = c0030bArr[0];
        C0030b c0030b = c0030bArr[2];
        this.f230b = c0030b;
        this.f231c = c0030bArr[3];
        this.f232d = c0030bArr[4];
        this.f233e = c0030b;
        this.f235g = c0032d.m81a();
    }

    /* JADX WARN: Removed duplicated region for block: B:55:0x00ef  */
    @Override // p000a.InterfaceC0003b0
    /* JADX INFO: renamed from: a */
    /*
        Code decompiled incorrectly, please refer to instructions dump.
    */
    public final int mo45a(C0002b c0002b) {
        char c;
        String str = null;
        C0030b c0030b = this.f231c;
        this.f233e = c0030b;
        String strM43j = c0002b.m43j(0);
        if (this.f235g) {
            int i = this.f229a.f207c;
            String str2 = "";
            int i2 = 0;
            while (true) {
                if (i2 >= strM43j.length()) {
                    strM43j = str2;
                    break;
                }
                int i3 = i2 + i;
                if (i3 > strM43j.length()) {
                    strM43j = str2 + strM43j.substring(i2, i2 + 1);
                    break;
                }
                str2 = str2 + m85f(strM43j.substring(i2, i3));
                i2 = i3;
            }
        }
        if (!strM43j.contains("*") && !strM43j.contains("?")) {
            c0030b.getClass();
            C0030b.f203g = false;
            return c0030b.m74a(strM43j);
        }
        c0030b.getClass();
        C0030b.f202f = strM43j;
        new ArrayList();
        int iIndexOf = strM43j.indexOf(42);
        int iIndexOf2 = strM43j.indexOf(63);
        if (iIndexOf >= 0 && iIndexOf2 >= 0) {
            if (iIndexOf >= iIndexOf2) {
                iIndexOf = iIndexOf2;
            }
            c = '\n';
        } else if (iIndexOf >= 0) {
            C0030b.f204h = strM43j.split("[*]+");
            c = '*';
        } else {
            C0030b.f204h = strM43j.split("[?]+");
            iIndexOf = iIndexOf2;
            c = '?';
        }
        if (iIndexOf < 0) {
            return -1;
        }
        C0030b.f203g = true;
        if ((c == '*' || c == '?') && strM43j.length() == 1) {
            return 1;
        }
        boolean zStartsWith = strM43j.startsWith("*");
        int i4 = c0030b.f207c;
        if ((zStartsWith && strM43j.length() - C0030b.f204h.length > i4) || strM43j.length() - (C0030b.f204h.length + 1) > i4) {
            return -1;
        }
        if ((strM43j.startsWith("*") || strM43j.startsWith("?")) && C0030b.f204h.length == 0) {
            return -1;
        }
        if (c != '*' || iIndexOf != 0) {
            if (c == '*' && iIndexOf > 0) {
                str = C0030b.f204h[0];
            } else if (c == '?' && iIndexOf == 0) {
                C0030b.f201e = 0;
            } else if (c == '?' && iIndexOf > 0) {
                str = C0030b.f204h[0];
            }
            if (str != null) {
                C0030b.f201e = c0030b.m75b(str, true);
            }
        }
        int i5 = C0030b.f201e;
        if (i5 >= 0) {
            return i5 + 1;
        }
        return -1;
    }

    @Override // p000a.InterfaceC0003b0
    /* JADX INFO: renamed from: b */
    public final C0005c0 mo46b() {
        if (this.f233e.hasNext()) {
            return new C0005c0(this.f233e.next(), 0);
        }
        return null;
    }

    @Override // p000a.InterfaceC0003b0
    /* JADX INFO: renamed from: c */
    public final int mo47c(C0002b c0002b) {
        C0030b c0030b = this.f230b;
        this.f233e = c0030b;
        String strM43j = c0002b.m43j(0);
        if (this.f235g) {
            int i = this.f229a.f207c;
            String str = "";
            int i2 = 0;
            while (true) {
                if (i2 >= strM43j.length()) {
                    strM43j = str;
                    break;
                }
                int i3 = i2 + i;
                if (i3 > strM43j.length()) {
                    strM43j = str + strM43j.substring(i2, i2 + 1);
                    break;
                }
                str = str + m85f(strM43j.substring(i2, i3));
                i2 = i3;
            }
        }
        if (strM43j.contains("*") || strM43j.contains("?")) {
            return -1;
        }
        c0030b.getClass();
        C0030b.f203g = false;
        return c0030b.m74a(strM43j);
    }

    @Override // p000a.InterfaceC0003b0
    /* JADX INFO: renamed from: d */
    public final void mo48d() {
        C0030b c0030b = this.f230b;
        if (c0030b != null) {
            c0030b.getClass();
            C0030b.f201e = -1;
        }
        C0030b c0030b2 = this.f231c;
        if (c0030b2 != null) {
            c0030b2.getClass();
            C0030b.f201e = -1;
        }
        C0030b c0030b3 = this.f232d;
        if (c0030b3 != null) {
            c0030b3.getClass();
            C0030b.f201e = -1;
        }
    }

    /* JADX INFO: renamed from: e */
    public final C0005c0 m84e(int i) {
        if (i < this.f234f.size()) {
            return new C0005c0((String) this.f234f.get(i), 0);
        }
        return null;
    }

    /* JADX INFO: renamed from: f */
    public final String m85f(String str) {
        if (str.contains("*")) {
            return "*";
        }
        if (str.contains("?")) {
            return "?";
        }
        C0030b c0030b = this.f229a;
        if (str.length() != c0030b.f207c) {
            return str;
        }
        C0030b.f203g = false;
        int iM74a = c0030b.m74a(str) - 1;
        return iM74a >= 0 ? c0030b.m77d(iM74a) : str;
    }

    /* JADX INFO: renamed from: g */
    public final C0005c0[] m86g() {
        ArrayList arrayList = this.f234f;
        if (arrayList == null) {
            return null;
        }
        C0005c0[] c0005c0Arr = new C0005c0[arrayList.size()];
        for (int i = 0; i < this.f234f.size(); i++) {
            c0005c0Arr[i] = new C0005c0("", 0);
            c0005c0Arr[i].f87a = (String) this.f234f.get(i);
        }
        return c0005c0Arr;
    }
}
