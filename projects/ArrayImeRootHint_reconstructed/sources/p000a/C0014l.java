package p000a;

import android.content.res.Resources;
import android.content.res.TypedArray;
import android.content.res.XmlResourceParser;
import android.util.Xml;

/* JADX INFO: renamed from: a.l */
/* JADX INFO: loaded from: classes.dex */
public final class C0014l {

    /* JADX INFO: renamed from: a */
    public int f141a;

    /* JADX INFO: renamed from: b */
    public int f142b;

    /* JADX INFO: renamed from: c */
    public int f143c;

    /* JADX INFO: renamed from: d */
    public int f144d;

    /* JADX INFO: renamed from: e */
    public int f145e;

    /* JADX INFO: renamed from: f */
    public final int f146f;

    /* JADX INFO: renamed from: g */
    public final C0015m f147g;

    public C0014l(C0015m c0015m) {
        this.f147g = c0015m;
    }

    public C0014l(Resources resources, C0015m c0015m, XmlResourceParser xmlResourceParser) {
        this.f147g = c0015m;
        TypedArray typedArrayObtainAttributes = resources.obtainAttributes(Xml.asAttributeSet(xmlResourceParser), AbstractC0021s.Keyboard);
        this.f141a = C0015m.m57b(typedArrayObtainAttributes, 0, c0015m.f158k, c0015m.f149b);
        int i = c0015m.f150c;
        int i2 = c0015m.f159l;
        this.f142b = C0015m.m57b(typedArrayObtainAttributes, 1, i2, i);
        this.f143c = C0015m.m57b(typedArrayObtainAttributes, 2, c0015m.f158k, c0015m.f148a);
        this.f144d = C0015m.m57b(typedArrayObtainAttributes, 3, i2, c0015m.f151d);
        typedArrayObtainAttributes.recycle();
        TypedArray typedArrayObtainAttributes2 = resources.obtainAttributes(Xml.asAttributeSet(xmlResourceParser), AbstractC0021s.Keyboard_Row);
        this.f145e = typedArrayObtainAttributes2.getInt(0, 0);
        this.f146f = typedArrayObtainAttributes2.getResourceId(1, 0);
    }
}
