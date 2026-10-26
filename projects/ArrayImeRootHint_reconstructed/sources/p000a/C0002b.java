package p000a;

import java.util.ArrayList;

/* JADX INFO: renamed from: a.b */
/* JADX INFO: loaded from: classes.dex */
public final class C0002b {

    /* JADX INFO: renamed from: a */
    public final ArrayList[] f80a = new ArrayList[3];

    /* JADX INFO: renamed from: b */
    public final int[] f81b = new int[3];

    public C0002b() {
        for (int i = 0; i < 3; i++) {
            this.f80a[i] = new ArrayList();
            this.f81b[i] = 0;
        }
    }

    /* JADX INFO: renamed from: a */
    public final void m34a() {
        for (int i = 0; i < 3; i++) {
            this.f80a[i].clear();
            this.f81b[i] = 0;
        }
    }

    /* JADX INFO: renamed from: b */
    public final void m35b() {
        int i = this.f81b[0];
        ArrayList arrayList = this.f80a[0];
        if (i > 0) {
            int i2 = i - 1;
            m36c(i2, i2);
            m41h(0, i2);
        }
        arrayList.size();
    }

    /* JADX INFO: renamed from: c */
    public final void m36c(int i, int i2) {
        int i3;
        int[] iArr = {-1, -1, -1};
        int[] iArr2 = {-1, -1, -1};
        ArrayList[] arrayListArr = this.f80a;
        ArrayList arrayList = arrayListArr[2];
        ArrayList arrayList2 = arrayListArr[1];
        iArr[0] = i;
        iArr2[0] = i2;
        int i4 = (i2 - i) + 1;
        int i5 = 0;
        for (int i6 = 3; i5 < i6; i6 = 3) {
            int i7 = iArr[i5];
            if (i7 >= 0) {
                m37d(i5, i7, iArr2[i5], i4);
            } else {
                ArrayList arrayList3 = arrayListArr[i5];
                int i8 = 0;
                int i9 = -1;
                int i10 = -1;
                while (true) {
                    if (i8 >= arrayList3.size()) {
                        break;
                    }
                    C0022t c0022t = (C0022t) arrayList3.get(i8);
                    int i11 = c0022t.f173b;
                    int i12 = i5 - 1;
                    int i13 = iArr[i12];
                    if ((i11 < i13 || i11 > iArr2[i12]) && ((i3 = c0022t.f174c) < i13 || i3 > iArr2[i12])) {
                        if (i11 <= i13 && i3 >= iArr2[i12]) {
                            iArr[i5] = i8;
                            iArr2[i5] = i8;
                            i10 = i3;
                            i9 = i11;
                            break;
                        }
                        if (i11 > iArr2[i12]) {
                            break;
                        }
                    } else {
                        if (iArr[i5] < 0) {
                            iArr[i5] = i8;
                            i9 = i11;
                        }
                        iArr2[i5] = i8;
                        i10 = c0022t.f174c;
                    }
                    i8++;
                }
                int i14 = i5 - 1;
                if (i9 != iArr[i14] || i10 != iArr2[i14]) {
                    m37d(i5, iArr[i5] + 1, iArr2[i5], i4);
                    C0022t[] c0022tArr = {new C0022t(i9, i10 - i4, m43j(i14))};
                    int size = iArr[i5];
                    ArrayList arrayList4 = arrayListArr[i5];
                    int size2 = (size < 0 || size > arrayList4.size()) ? arrayList4.size() : size;
                    if (size < 0 || size > arrayList4.size()) {
                        size = arrayList4.size();
                    }
                    for (int i15 = size2; i15 <= size; i15++) {
                        arrayList4.remove(size2);
                    }
                    for (int i16 = 0; i16 >= 0; i16--) {
                        arrayList4.add(size2, c0022tArr[i16]);
                    }
                    m40g(i5, size2, 1, (size - size2) + 1);
                    return;
                }
                m37d(i5, iArr[i5], iArr2[i5], i4);
            }
            i4 = (iArr2[i5] - iArr[i5]) + 1;
            i5++;
        }
    }

    /* JADX INFO: renamed from: d */
    public final void m37d(int i, int i2, int i3, int i4) {
        ArrayList arrayList = this.f80a[i];
        if (i4 != 0) {
            for (int i5 = i3 + 1; i5 < arrayList.size(); i5++) {
                C0022t c0022t = (C0022t) arrayList.get(i5);
                c0022t.f173b -= i4;
                c0022t.f174c -= i4;
            }
        }
        for (int i6 = i2; i6 <= i3; i6++) {
            arrayList.remove(i2);
        }
    }

    /* JADX INFO: renamed from: e */
    public final int m38e(int i, int i2) {
        int i3 = 0;
        if (i2 == 0) {
            return 0;
        }
        ArrayList arrayList = this.f80a[i + 1];
        while (i3 < arrayList.size()) {
            C0022t c0022t = (C0022t) arrayList.get(i3);
            if (c0022t.f173b <= i2 && i2 <= c0022t.f174c) {
                break;
            }
            i3++;
        }
        return i3;
    }

    /* JADX INFO: renamed from: f */
    public final void m39f(C0022t c0022t) {
        ArrayList[] arrayListArr = this.f80a;
        ArrayList arrayList = arrayListArr[0];
        int[] iArr = this.f81b;
        arrayList.add(iArr[0], c0022t);
        iArr[0] = iArr[0] + 1;
        for (int i = 1; i <= 1; i++) {
            int i2 = iArr[i - 1] - 1;
            C0022t c0022t2 = new C0022t(i2, i2, c0022t.f172a);
            ArrayList arrayList2 = arrayListArr[i];
            arrayList2.add(iArr[i], c0022t2);
            int i3 = iArr[i] + 1;
            iArr[i] = i3;
            while (i3 < arrayList2.size()) {
                C0022t c0022t3 = (C0022t) arrayList2.get(i3);
                c0022t3.f173b++;
                c0022t3.f174c++;
                i3++;
            }
        }
        int i4 = iArr[1];
        m40g(1, i4 - 1, 1, 0);
        m41h(1, i4);
    }

    /* JADX WARN: Code restructure failed: missing block: B:34:0x007d, code lost:
    
        r8 = r6;
     */
    /* JADX INFO: renamed from: g */
    /*
        Code decompiled incorrectly, please refer to instructions dump.
    */
    public final void m40g(int i, int i2, int i3, int i4) {
        if (i >= 2) {
            return;
        }
        int i5 = i + 1;
        ArrayList[] arrayListArr = this.f80a;
        ArrayList arrayList = arrayListArr[i5];
        if (arrayList.size() <= 0) {
            arrayList.add(new C0022t(0, arrayListArr[i].size() - 1, m43j(i)));
            m40g(i5, 0, 1, 0);
            return;
        }
        int i6 = (i3 == 0 ? 0 : i3 - 1) + i2;
        int i7 = (i4 == 0 ? 0 : i4 - 1) + i2;
        C0022t c0022t = (C0022t) arrayList.get(arrayList.size() - 1);
        if (c0022t.f174c < i2) {
            c0022t.f174c = i6;
            c0022t.f172a = m44k(i, c0022t.f173b, i6);
            m40g(i5, arrayList.size() - 1, 1, 1);
            return;
        }
        int i8 = 0;
        int i9 = -1;
        int i10 = -1;
        while (true) {
            if (i8 >= arrayList.size()) {
                break;
            }
            C0022t c0022t2 = (C0022t) arrayList.get(i8);
            int i11 = c0022t2.f173b;
            if (i11 <= i2) {
                if (i4 != 0 || i11 != i2) {
                    if (c0022t2.f174c >= i7) {
                        break;
                    }
                } else {
                    i8--;
                    break;
                }
            } else if (c0022t2.f174c > i7) {
                i10 = i8;
                break;
            } else {
                if (i9 < 0) {
                }
                i10 = i8;
                i8++;
            }
            i9 = i8;
            i10 = i8;
            i8++;
        }
        i8 = i9;
        int i12 = i3 - i4;
        if (i8 < 0) {
            arrayList.add(0, new C0022t(i2, i6, m44k(i, i2, i6)));
            for (int i13 = 1; i13 < arrayList.size(); i13++) {
                C0022t c0022t3 = (C0022t) arrayList.get(i13);
                c0022t3.f173b += i12;
                c0022t3.f174c += i12;
            }
            m40g(i5, 0, 1, 0);
            return;
        }
        C0022t c0022t4 = (C0022t) arrayList.get(i8);
        int i14 = c0022t4.f174c;
        int i15 = i8 + 1;
        for (int i16 = i15; i16 <= i10; i16++) {
            c0022t4 = (C0022t) arrayList.get(i15);
            int i17 = c0022t4.f174c;
            if (i14 > i17) {
                i14 = i17;
            }
            arrayList.remove(i15);
        }
        if (i14 >= i6) {
            i6 = i14 + i12;
        }
        c0022t4.f174c = i6;
        c0022t4.f172a = m44k(i, c0022t4.f173b, i6);
        while (i15 < arrayList.size()) {
            C0022t c0022t5 = (C0022t) arrayList.get(i15);
            c0022t5.f173b += i12;
            c0022t5.f174c += i12;
            i15++;
        }
        m40g(i5, i8, 1, (i10 - i8) + 1);
    }

    /* JADX INFO: renamed from: h */
    public final void m41h(int i, int i2) {
        ArrayList[] arrayListArr = this.f80a;
        if (i2 > arrayListArr[i].size()) {
            i2 = arrayListArr[i].size();
        }
        if (i2 < 0) {
            i2 = 0;
        }
        int[] iArr = this.f81b;
        if (i == 0) {
            iArr[0] = i2;
            int iM38e = m38e(0, i2);
            iArr[1] = iM38e;
            iArr[2] = m38e(1, iM38e);
            return;
        }
        if (i == 1) {
            iArr[2] = m38e(1, i2);
            iArr[1] = i2;
            iArr[0] = i2 > 0 ? ((C0022t) arrayListArr[1].get(i2 - 1)).f174c + 1 : 0;
        } else {
            iArr[2] = i2;
            int i3 = i2 > 0 ? ((C0022t) arrayListArr[2].get(i2 - 1)).f174c + 1 : 0;
            iArr[1] = i3;
            iArr[0] = i3 > 0 ? ((C0022t) arrayListArr[1].get(i3 - 1)).f174c + 1 : 0;
        }
    }

    /* JADX INFO: renamed from: i */
    public final int m42i(int i) {
        return this.f80a[i].size();
    }

    /* JADX INFO: renamed from: j */
    public final String m43j(int i) {
        return m44k(i, 0, this.f80a[i].size() - 1);
    }

    /* JADX INFO: renamed from: k */
    public final String m44k(int i, int i2, int i3) {
        try {
            StringBuffer stringBuffer = new StringBuffer();
            ArrayList arrayList = this.f80a[i];
            while (i2 <= i3) {
                stringBuffer.append(((C0022t) arrayList.get(i2)).f172a);
                i2++;
            }
            return stringBuffer.toString();
        } catch (Exception unused) {
            return null;
        }
    }
}
