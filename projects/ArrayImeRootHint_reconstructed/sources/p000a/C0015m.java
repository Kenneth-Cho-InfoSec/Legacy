package p000a;

import android.content.Context;
import android.content.res.Resources;
import android.content.res.TypedArray;
import android.content.res.XmlResourceParser;
import android.util.DisplayMetrics;
import android.util.Log;
import android.util.TypedValue;
import android.util.Xml;
import java.util.ArrayList;

/* JADX INFO: renamed from: a.m */
/* JADX INFO: loaded from: classes.dex */
public class C0015m {

    /* JADX INFO: renamed from: a */
    public int f148a;

    /* JADX INFO: renamed from: b */
    public int f149b;

    /* JADX INFO: renamed from: c */
    public int f150c;

    /* JADX INFO: renamed from: d */
    public int f151d;

    /* JADX INFO: renamed from: e */
    public boolean f152e;

    /* JADX INFO: renamed from: f */
    public final C0013k f153f;

    /* JADX INFO: renamed from: g */
    public int f154g;

    /* JADX INFO: renamed from: h */
    public int f155h;

    /* JADX INFO: renamed from: i */
    public final ArrayList f156i;

    /* JADX INFO: renamed from: j */
    public final ArrayList f157j;

    /* JADX INFO: renamed from: k */
    public final int f158k;

    /* JADX INFO: renamed from: l */
    public final int f159l;

    /* JADX INFO: renamed from: m */
    public final int f160m;

    /* JADX INFO: renamed from: n */
    public int f161n;

    /* JADX INFO: renamed from: o */
    public int f162o;

    /* JADX INFO: renamed from: p */
    public int[][] f163p;

    /* JADX INFO: renamed from: q */
    public int f164q;

    /* JADX WARN: Code restructure failed: missing block: B:11:0x005e, code lost:
    
        r3 = new p000a.C0014l(r12, r18, r9);
        r4 = r3.f146f;
     */
    /* JADX WARN: Code restructure failed: missing block: B:12:0x0065, code lost:
    
        if (r4 == 0) goto L56;
     */
    /* JADX WARN: Code restructure failed: missing block: B:14:0x0069, code lost:
    
        if (r4 == r18.f160m) goto L57;
     */
    /* JADX WARN: Code restructure failed: missing block: B:15:0x006b, code lost:
    
        r4 = r9.next();
     */
    /* JADX WARN: Code restructure failed: missing block: B:16:0x006f, code lost:
    
        if (r4 == 1) goto L53;
     */
    /* JADX WARN: Code restructure failed: missing block: B:17:0x0071, code lost:
    
        if (r4 != 3) goto L78;
     */
    /* JADX WARN: Code restructure failed: missing block: B:19:0x007b, code lost:
    
        if (r9.getName().equals("Row") == false) goto L79;
     */
    /*
        Code decompiled incorrectly, please refer to instructions dump.
    */
    public C0015m(Context context, int i) {
        C0014l c0014l;
        DisplayMetrics displayMetrics = context.getResources().getDisplayMetrics();
        int i2 = displayMetrics.widthPixels;
        this.f158k = i2;
        this.f159l = displayMetrics.heightPixels;
        this.f148a = 0;
        int i3 = i2 / 10;
        this.f149b = i3;
        this.f151d = 0;
        this.f150c = i3;
        ArrayList arrayList = new ArrayList();
        this.f156i = arrayList;
        this.f157j = new ArrayList();
        this.f160m = 0;
        XmlResourceParser xml = context.getResources().getXml(i);
        Resources resources = context.getResources();
        C0013k c0013kMo50a = null;
        C0014l c0014l2 = null;
        boolean z = false;
        int i4 = 0;
        loop0: while (true) {
            boolean z2 = false;
            while (true) {
                int i5 = 0;
                while (true) {
                    try {
                        int next = xml.next();
                        if (next == 1) {
                            break loop0;
                        }
                        if (next == 2) {
                            String name = xml.getName();
                            if ("Row".equals(name)) {
                                break;
                            }
                            if ("Key".equals(name)) {
                                c0013kMo50a = mo50a(resources, c0014l2, i5, i4, xml);
                                arrayList.add(c0013kMo50a);
                                int i6 = c0013kMo50a.f122a[0];
                                ArrayList arrayList2 = this.f157j;
                                if (i6 == -1) {
                                    this.f153f = c0013kMo50a;
                                    arrayList.size();
                                } else {
                                    if (i6 == -6) {
                                    }
                                    z = true;
                                }
                                arrayList2.add(c0013kMo50a);
                                z = true;
                            } else if ("Keyboard".equals(name)) {
                                m58c(resources, xml);
                            }
                        } else if (next == 3) {
                            if (z) {
                                int i7 = c0013kMo50a.f128g + c0013kMo50a.f126e + i5;
                                if (i7 > this.f155h) {
                                    this.f155h = i7;
                                }
                                i5 = i7;
                                z = false;
                            } else if (z2) {
                                i4 = i4 + c0014l2.f144d + c0014l2.f142b;
                                z2 = false;
                            }
                        }
                    } catch (Exception e) {
                        Log.e("Keyboard", "Parse error:" + e);
                        e.printStackTrace();
                    }
                }
                c0014l2 = c0014l;
                z2 = true;
            }
            c0014l2 = c0014l;
        }
        this.f154g = i4 - this.f151d;
    }

    /* JADX INFO: renamed from: b */
    public static int m57b(TypedArray typedArray, int i, int i2, int i3) {
        TypedValue typedValuePeekValue = typedArray.peekValue(i);
        if (typedValuePeekValue == null) {
            return i3;
        }
        int i4 = typedValuePeekValue.type;
        return i4 == 5 ? typedArray.getDimensionPixelOffset(i, i3) : i4 == 6 ? Math.round(typedArray.getFraction(i, i2, i2, i3)) : i3;
    }

    /* JADX INFO: renamed from: a */
    public C0013k mo50a(Resources resources, C0014l c0014l, int i, int i2, XmlResourceParser xmlResourceParser) {
        return new C0013k(resources, c0014l, i, i2, xmlResourceParser);
    }

    /* JADX INFO: renamed from: c */
    public final void m58c(Resources resources, XmlResourceParser xmlResourceParser) {
        TypedArray typedArrayObtainAttributes = resources.obtainAttributes(Xml.asAttributeSet(xmlResourceParser), AbstractC0021s.Keyboard);
        int i = this.f158k;
        this.f149b = m57b(typedArrayObtainAttributes, 0, i, i / 10);
        int i2 = this.f159l;
        this.f150c = m57b(typedArrayObtainAttributes, 1, i2, 75);
        this.f148a = m57b(typedArrayObtainAttributes, 2, i, 0);
        this.f151d = m57b(typedArrayObtainAttributes, 3, i2, 0);
        int i3 = (int) (this.f149b * 1.8f);
        this.f164q = i3 * i3;
        typedArrayObtainAttributes.recycle();
    }
}
