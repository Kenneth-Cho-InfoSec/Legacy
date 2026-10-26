package p000a;

import android.R;
import android.content.res.Resources;
import android.content.res.TypedArray;
import android.content.res.XmlResourceParser;
import android.graphics.drawable.Drawable;
import android.text.TextUtils;
import android.util.Log;
import android.util.TypedValue;
import android.util.Xml;
import java.util.StringTokenizer;

/* JADX INFO: renamed from: a.k */
/* JADX INFO: loaded from: classes.dex */
public class C0013k {

    /* JADX INFO: renamed from: t */
    public static final int[] f116t = {R.attr.state_checkable, R.attr.state_checked};

    /* JADX INFO: renamed from: u */
    public static final int[] f117u = {R.attr.state_pressed, R.attr.state_checkable, R.attr.state_checked};

    /* JADX INFO: renamed from: v */
    public static final int[] f118v = {R.attr.state_checkable};

    /* JADX INFO: renamed from: w */
    public static final int[] f119w = {R.attr.state_pressed, R.attr.state_checkable};

    /* JADX INFO: renamed from: x */
    public static final int[] f120x = new int[0];

    /* JADX INFO: renamed from: y */
    public static final int[] f121y = {R.attr.state_pressed};

    /* JADX INFO: renamed from: a */
    public int[] f122a;

    /* JADX INFO: renamed from: b */
    public CharSequence f123b;

    /* JADX INFO: renamed from: c */
    public Drawable f124c;

    /* JADX INFO: renamed from: d */
    public Drawable f125d;

    /* JADX INFO: renamed from: e */
    public final int f126e;

    /* JADX INFO: renamed from: f */
    public final int f127f;

    /* JADX INFO: renamed from: g */
    public final int f128g;

    /* JADX INFO: renamed from: h */
    public final boolean f129h;

    /* JADX INFO: renamed from: i */
    public int f130i;

    /* JADX INFO: renamed from: j */
    public int f131j;

    /* JADX INFO: renamed from: k */
    public boolean f132k;

    /* JADX INFO: renamed from: l */
    public boolean f133l;

    /* JADX INFO: renamed from: m */
    public final CharSequence f134m;

    /* JADX INFO: renamed from: n */
    public final CharSequence f135n;

    /* JADX INFO: renamed from: o */
    public final int f136o;

    /* JADX INFO: renamed from: p */
    public final C0015m f137p;

    /* JADX INFO: renamed from: q */
    public final int f138q;

    /* JADX INFO: renamed from: r */
    public final boolean f139r;

    /* JADX INFO: renamed from: s */
    public final boolean f140s;

    public C0013k(C0014l c0014l) {
        this.f137p = c0014l.f147g;
        this.f127f = c0014l.f142b;
        this.f126e = c0014l.f141a;
        this.f128g = c0014l.f143c;
        this.f136o = c0014l.f145e;
    }

    /* JADX INFO: renamed from: a */
    public boolean mo49a(int i, int i2) {
        int i3 = this.f136o;
        boolean z = (i3 & 1) > 0;
        boolean z2 = (i3 & 2) > 0;
        boolean z3 = (i3 & 4) > 0;
        boolean z4 = (i3 & 8) > 0;
        int i4 = this.f130i;
        int i5 = this.f126e;
        if ((i >= i4 || (z && i <= i4 + i5)) && (i < i5 + i4 || (z2 && i >= i4))) {
            int i6 = this.f131j;
            int i7 = this.f127f;
            if ((i2 >= i6 || (z3 && i2 <= i6 + i7)) && (i2 < i7 + i6 || (z4 && i2 >= i6))) {
                return true;
            }
        }
        return false;
    }

    /* JADX INFO: renamed from: b */
    public final int m56b(int i, int i2) {
        int i3 = ((this.f126e / 2) + this.f130i) - i;
        int i4 = ((this.f127f / 2) + this.f131j) - i2;
        return (i4 * i4) + (i3 * i3);
    }

    public C0013k(Resources resources, C0014l c0014l, int i, int i2, XmlResourceParser xmlResourceParser) {
        int i3;
        this(c0014l);
        this.f130i = i;
        this.f131j = i2;
        TypedArray typedArrayObtainAttributes = resources.obtainAttributes(Xml.asAttributeSet(xmlResourceParser), AbstractC0021s.Keyboard);
        this.f126e = C0015m.m57b(typedArrayObtainAttributes, 0, this.f137p.f158k, c0014l.f141a);
        this.f127f = C0015m.m57b(typedArrayObtainAttributes, 1, this.f137p.f159l, c0014l.f142b);
        int iM57b = C0015m.m57b(typedArrayObtainAttributes, 2, this.f137p.f158k, c0014l.f143c);
        this.f128g = iM57b;
        typedArrayObtainAttributes.recycle();
        TypedArray typedArrayObtainAttributes2 = resources.obtainAttributes(Xml.asAttributeSet(xmlResourceParser), AbstractC0021s.Keyboard_Key);
        this.f130i += iM57b;
        TypedValue typedValue = new TypedValue();
        typedArrayObtainAttributes2.getValue(0, typedValue);
        int i4 = typedValue.type;
        if (i4 == 16 || i4 == 17) {
            this.f122a = new int[]{typedValue.data};
        } else if (i4 == 3) {
            String string = typedValue.string.toString();
            if (string.length() > 0) {
                int iIndexOf = 0;
                i3 = 1;
                while (true) {
                    iIndexOf = string.indexOf(",", iIndexOf + 1);
                    if (iIndexOf <= 0) {
                        break;
                    } else {
                        i3++;
                    }
                }
            } else {
                i3 = 0;
            }
            int[] iArr = new int[i3];
            StringTokenizer stringTokenizer = new StringTokenizer(string, ",");
            int i5 = 0;
            while (stringTokenizer.hasMoreTokens()) {
                int i6 = i5 + 1;
                try {
                    iArr[i5] = Integer.parseInt(stringTokenizer.nextToken());
                } catch (NumberFormatException unused) {
                    Log.e("Keyboard", "Error parsing keycodes ".concat(string));
                }
                i5 = i6;
            }
            this.f122a = iArr;
        }
        Drawable drawable = typedArrayObtainAttributes2.getDrawable(7);
        this.f125d = drawable;
        if (drawable != null) {
            drawable.setBounds(0, 0, drawable.getIntrinsicWidth(), this.f125d.getIntrinsicHeight());
        }
        this.f135n = typedArrayObtainAttributes2.getText(2);
        this.f138q = typedArrayObtainAttributes2.getResourceId(1, 0);
        this.f139r = typedArrayObtainAttributes2.getBoolean(6, false);
        typedArrayObtainAttributes2.getBoolean(4, false);
        this.f129h = typedArrayObtainAttributes2.getBoolean(5, false);
        this.f136o = c0014l.f145e | typedArrayObtainAttributes2.getInt(3, 0);
        Drawable drawable2 = typedArrayObtainAttributes2.getDrawable(10);
        this.f124c = drawable2;
        if (drawable2 != null) {
            drawable2.setBounds(0, 0, drawable2.getIntrinsicWidth(), this.f124c.getIntrinsicHeight());
        }
        this.f123b = typedArrayObtainAttributes2.getText(9);
        this.f134m = typedArrayObtainAttributes2.getText(8);
        if (this.f122a == null && !TextUtils.isEmpty(this.f123b)) {
            this.f122a = new int[]{this.f123b.charAt(0)};
        }
        typedArrayObtainAttributes2.recycle();
        TypedArray typedArrayObtainAttributes3 = resources.obtainAttributes(Xml.asAttributeSet(xmlResourceParser), AbstractC0021s.WnnKeyboard_Key);
        this.f140s = typedArrayObtainAttributes3.getBoolean(0, false);
        typedArrayObtainAttributes3.recycle();
    }
}
