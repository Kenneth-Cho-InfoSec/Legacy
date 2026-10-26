package p001b;

import android.content.Context;
import android.content.res.Resources;
import android.content.res.TypedArray;
import android.content.res.XmlResourceParser;
import android.util.Xml;
import p000a.AbstractC0021s;

/* JADX INFO: renamed from: b.c */
/* JADX INFO: loaded from: classes.dex */
public final class C0031c {

    /* JADX INFO: renamed from: a */
    public final int f209a;

    /* JADX INFO: renamed from: b */
    public final int f210b;

    /* JADX INFO: renamed from: i */
    public String f217i;

    /* JADX INFO: renamed from: k */
    public final String f219k;

    /* JADX INFO: renamed from: l */
    public final Context f220l;

    /* JADX INFO: renamed from: c */
    public final String[] f211c = new String[5];

    /* JADX INFO: renamed from: d */
    public final boolean[] f212d = new boolean[12];

    /* JADX INFO: renamed from: e */
    public final String f213e = new String("\b\u001d");

    /* JADX INFO: renamed from: f */
    public final String f214f = new String("1234567890");

    /* JADX INFO: renamed from: g */
    public final String f215g = new String(" ");

    /* JADX INFO: renamed from: h */
    public final String f216h = new String("\u001b");

    /* JADX INFO: renamed from: j */
    public final String f218j = new String("*?");

    public C0031c(Resources resources, Context context, XmlResourceParser xmlResourceParser) {
        this.f217i = new String("");
        this.f220l = context;
        TypedArray typedArrayObtainAttributes = resources.obtainAttributes(Xml.asAttributeSet(xmlResourceParser), AbstractC0021s.HzimConfig_InputMethod);
        typedArrayObtainAttributes.getString(14);
        this.f209a = typedArrayObtainAttributes.getResourceId(15, 0);
        this.f210b = typedArrayObtainAttributes.getResourceId(13, 0);
        for (int i = 0; i < 5; i++) {
            this.f211c[i] = null;
        }
        this.f211c[0] = typedArrayObtainAttributes.getString(19);
        this.f211c[1] = typedArrayObtainAttributes.getString(20);
        this.f211c[2] = typedArrayObtainAttributes.getString(18);
        this.f211c[3] = typedArrayObtainAttributes.getString(12);
        this.f211c[4] = typedArrayObtainAttributes.getString(17);
        this.f212d[0] = typedArrayObtainAttributes.getBoolean(1, false);
        this.f212d[1] = typedArrayObtainAttributes.getBoolean(5, false);
        this.f212d[2] = typedArrayObtainAttributes.getBoolean(2, false);
        this.f212d[3] = typedArrayObtainAttributes.getBoolean(5, false);
        this.f212d[4] = typedArrayObtainAttributes.getBoolean(21, false);
        this.f212d[5] = typedArrayObtainAttributes.getBoolean(24, false);
        this.f212d[6] = typedArrayObtainAttributes.getBoolean(26, false);
        this.f212d[7] = typedArrayObtainAttributes.getBoolean(10, false);
        this.f212d[8] = typedArrayObtainAttributes.getBoolean(8, false);
        this.f212d[9] = typedArrayObtainAttributes.getBoolean(0, false);
        this.f212d[10] = typedArrayObtainAttributes.getBoolean(25, false);
        this.f212d[11] = typedArrayObtainAttributes.getBoolean(16, false);
        this.f219k = typedArrayObtainAttributes.getString(9);
        this.f217i = typedArrayObtainAttributes.getString(11);
        typedArrayObtainAttributes.recycle();
    }

    /* JADX INFO: renamed from: a */
    public final C0030b m80a(int i) {
        if (i < 0 || i >= 5) {
            return null;
        }
        String[] strArr = this.f211c;
        if (strArr[i] != null) {
            return new C0030b(strArr[i], this.f220l);
        }
        return null;
    }
}
