package com.minn.android.hanziime;

import android.content.Context;
import android.content.res.TypedArray;
import android.graphics.Bitmap;
import android.graphics.Canvas;
import android.graphics.Paint;
import android.graphics.PorterDuff;
import android.graphics.Rect;
import android.graphics.Region;
import android.graphics.Typeface;
import android.graphics.drawable.Drawable;
import android.os.Build;
import android.util.AttributeSet;
import android.view.GestureDetector;
import android.view.LayoutInflater;
import android.view.MotionEvent;
import android.view.View;
import android.view.ViewConfiguration;
import android.view.ViewGroup;
import android.widget.PopupWindow;
import android.widget.TextView;
import java.util.ArrayList;
import java.util.Arrays;
import java.util.HashMap;
import p000a.AbstractC0010h;
import p000a.AbstractC0021s;
import p000a.C0013k;
import p000a.C0014l;
import p000a.C0015m;
import p000a.C0016n;
import p000a.C0017o;
import p000a.C0019q;
import p000a.HandlerC0009g;
import p000a.InterfaceC0018p;

/* JADX INFO: loaded from: classes.dex */
public class KeyboardView extends View implements View.OnClickListener {

    /* JADX INFO: renamed from: E0 */
    public static final int[] f293E0 = {-5};

    /* JADX INFO: renamed from: F0 */
    public static final int[] f294F0 = {android.R.attr.state_long_pressable};

    /* JADX INFO: renamed from: G0 */
    public static final int f295G0 = ViewConfiguration.getLongPressTimeout();

    /* JADX INFO: renamed from: H0 */
    public static final int f296H0 = 12;

    /* JADX INFO: renamed from: A */
    public InterfaceC0018p f297A;

    /* JADX INFO: renamed from: A0 */
    public Bitmap f298A0;

    /* JADX INFO: renamed from: B */
    public final int f299B;

    /* JADX INFO: renamed from: B0 */
    public boolean f300B0;

    /* JADX INFO: renamed from: C */
    public int f301C;

    /* JADX INFO: renamed from: C0 */
    public Canvas f302C0;

    /* JADX INFO: renamed from: D */
    public boolean f303D;

    /* JADX INFO: renamed from: D0 */
    public final HandlerC0009g f304D0;

    /* JADX INFO: renamed from: E */
    public int f305E;

    /* JADX INFO: renamed from: F */
    public int f306F;

    /* JADX INFO: renamed from: G */
    public int f307G;

    /* JADX INFO: renamed from: H */
    public int f308H;

    /* JADX INFO: renamed from: I */
    public int f309I;

    /* JADX INFO: renamed from: J */
    public int f310J;

    /* JADX INFO: renamed from: K */
    public int f311K;

    /* JADX INFO: renamed from: L */
    public boolean f312L;

    /* JADX INFO: renamed from: M */
    public final Paint f313M;

    /* JADX INFO: renamed from: N */
    public final Rect f314N;

    /* JADX INFO: renamed from: O */
    public long f315O;

    /* JADX INFO: renamed from: P */
    public long f316P;

    /* JADX INFO: renamed from: Q */
    public int f317Q;

    /* JADX INFO: renamed from: R */
    public int f318R;

    /* JADX INFO: renamed from: S */
    public int f319S;

    /* JADX INFO: renamed from: T */
    public int f320T;

    /* JADX INFO: renamed from: U */
    public int f321U;

    /* JADX INFO: renamed from: V */
    public long f322V;

    /* JADX INFO: renamed from: W */
    public long f323W;

    /* JADX INFO: renamed from: a */
    public final Context f324a;

    /* JADX INFO: renamed from: a0 */
    public final int[] f325a0;

    /* JADX INFO: renamed from: b */
    public C0015m f326b;

    /* JADX INFO: renamed from: b0 */
    public final GestureDetector f327b0;

    /* JADX INFO: renamed from: c */
    public int f328c;

    /* JADX INFO: renamed from: c0 */
    public int f329c0;

    /* JADX INFO: renamed from: d */
    public final int f330d;

    /* JADX INFO: renamed from: d0 */
    public int f331d0;

    /* JADX INFO: renamed from: e */
    public final int f332e;

    /* JADX INFO: renamed from: e0 */
    public int f333e0;

    /* JADX INFO: renamed from: f */
    public final int f334f;

    /* JADX INFO: renamed from: f0 */
    public final int f335f0;

    /* JADX INFO: renamed from: g */
    public final int f336g;

    /* JADX INFO: renamed from: g0 */
    public boolean f337g0;

    /* JADX INFO: renamed from: h */
    public final float f338h;

    /* JADX INFO: renamed from: h0 */
    public C0013k f339h0;

    /* JADX INFO: renamed from: i */
    public final int f340i;

    /* JADX INFO: renamed from: i0 */
    public final Rect f341i0;

    /* JADX INFO: renamed from: j */
    public final float f342j;

    /* JADX INFO: renamed from: j0 */
    public boolean f343j0;

    /* JADX INFO: renamed from: k */
    public final TextView f344k;

    /* JADX INFO: renamed from: k0 */
    public final C0019q f345k0;

    /* JADX INFO: renamed from: l */
    public final PopupWindow f346l;

    /* JADX INFO: renamed from: l0 */
    public final int f347l0;

    /* JADX INFO: renamed from: m */
    public final int f348m;

    /* JADX INFO: renamed from: m0 */
    public final boolean f349m0;

    /* JADX INFO: renamed from: n */
    public final int f350n;

    /* JADX INFO: renamed from: n0 */
    public int f351n0;

    /* JADX INFO: renamed from: o */
    public final int f352o;

    /* JADX INFO: renamed from: o0 */
    public float f353o0;

    /* JADX INFO: renamed from: p */
    public int[] f354p;

    /* JADX INFO: renamed from: p0 */
    public float f355p0;

    /* JADX INFO: renamed from: q */
    public final PopupWindow f356q;

    /* JADX INFO: renamed from: q0 */
    public final Drawable f357q0;

    /* JADX INFO: renamed from: r */
    public View f358r;

    /* JADX INFO: renamed from: r0 */
    public final Drawable f359r0;

    /* JADX INFO: renamed from: s */
    public KeyboardView f360s;

    /* JADX INFO: renamed from: s0 */
    public final int[] f361s0;

    /* JADX INFO: renamed from: t */
    public boolean f362t;

    /* JADX INFO: renamed from: t0 */
    public int f363t0;

    /* JADX INFO: renamed from: u */
    public View f364u;

    /* JADX INFO: renamed from: u0 */
    public int f365u0;

    /* JADX INFO: renamed from: v */
    public int f366v;

    /* JADX INFO: renamed from: v0 */
    public long f367v0;

    /* JADX INFO: renamed from: w */
    public int f368w;

    /* JADX INFO: renamed from: w0 */
    public boolean f369w0;

    /* JADX INFO: renamed from: x */
    public final HashMap f370x;

    /* JADX INFO: renamed from: x0 */
    public final StringBuilder f371x0;

    /* JADX INFO: renamed from: y */
    public int[] f372y;

    /* JADX INFO: renamed from: y0 */
    public boolean f373y0;

    /* JADX INFO: renamed from: z */
    public C0013k[] f374z;

    /* JADX INFO: renamed from: z0 */
    public final Rect f375z0;

    public KeyboardView(Context context, AttributeSet attributeSet) {
        super(context, attributeSet, 0);
        this.f328c = -1;
        this.f303D = true;
        this.f320T = -1;
        this.f321U = -1;
        this.f325a0 = new int[12];
        this.f333e0 = -1;
        this.f341i0 = new Rect(0, 0, 0, 0);
        this.f345k0 = new C0019q();
        this.f351n0 = 1;
        this.f361s0 = new int[f296H0];
        this.f371x0 = new StringBuilder(1);
        this.f375z0 = new Rect();
        this.f304D0 = new HandlerC0009g(this, 1);
        this.f324a = context;
        TypedArray typedArrayObtainStyledAttributes = context.obtainStyledAttributes(attributeSet, AbstractC0021s.KeyboardView, 0, R.style.WnnKeyboardView);
        LayoutInflater layoutInflater = (LayoutInflater) context.getSystemService("layout_inflater");
        int indexCount = typedArrayObtainStyledAttributes.getIndexCount();
        int resourceId = 0;
        for (int i = 0; i < indexCount; i++) {
            int index = typedArrayObtainStyledAttributes.getIndex(i);
            switch (index) {
                case 0:
                    this.f340i = typedArrayObtainStyledAttributes.getColor(index, 0);
                    break;
                case 1:
                    this.f338h = typedArrayObtainStyledAttributes.getFloat(index, 0.0f);
                    break;
                case 2:
                    this.f357q0 = typedArrayObtainStyledAttributes.getDrawable(index);
                    break;
                case 3:
                    this.f332e = typedArrayObtainStyledAttributes.getDimensionPixelSize(index, 18);
                    break;
                case 4:
                    this.f330d = typedArrayObtainStyledAttributes.getDimensionPixelSize(index, 14);
                    break;
                case 5:
                    this.f334f = typedArrayObtainStyledAttributes.getColor(index, -16777216);
                    break;
                case 6:
                    resourceId = typedArrayObtainStyledAttributes.getResourceId(index, 0);
                    break;
                case 7:
                    this.f350n = typedArrayObtainStyledAttributes.getDimensionPixelOffset(index, 0);
                    break;
                case 8:
                    this.f352o = typedArrayObtainStyledAttributes.getDimensionPixelSize(index, 80);
                    break;
                case 9:
                    this.f299B = typedArrayObtainStyledAttributes.getDimensionPixelOffset(index, 0);
                    break;
                case 10:
                    this.f335f0 = typedArrayObtainStyledAttributes.getResourceId(index, 0);
                    break;
            }
        }
        typedArrayObtainStyledAttributes.recycle();
        TypedArray typedArrayObtainStyledAttributes2 = context.obtainStyledAttributes(attributeSet, AbstractC0021s.WnnKeyboardView, 0, 0);
        this.f359r0 = typedArrayObtainStyledAttributes2.getDrawable(0);
        this.f336g = typedArrayObtainStyledAttributes2.getColor(1, -16777216);
        typedArrayObtainStyledAttributes2.recycle();
        this.f342j = this.f324a.obtainStyledAttributes(AbstractC0021s.Theme).getFloat(0, 0.5f);
        PopupWindow popupWindow = new PopupWindow(context);
        this.f346l = popupWindow;
        if (resourceId != 0) {
            TextView textView = (TextView) layoutInflater.inflate(resourceId, (ViewGroup) null);
            this.f344k = textView;
            this.f348m = (int) textView.getTextSize();
            popupWindow.setContentView(textView);
            popupWindow.setBackgroundDrawable(null);
            popupWindow.setClippingEnabled(false);
            popupWindow.setOutsideTouchable(true);
            popupWindow.setFocusable(false);
            if (Build.VERSION.SDK_INT >= 28) {
                popupWindow.setWindowLayoutType(2038);
            }
        } else {
            this.f303D = false;
        }
        popupWindow.setTouchable(false);
        PopupWindow popupWindow2 = new PopupWindow(context);
        this.f356q = popupWindow2;
        if (Build.VERSION.SDK_INT >= 28) {
            popupWindow2.setWindowLayoutType(2038);
        }
        popupWindow2.setBackgroundDrawable(null);
        this.f364u = this;
        Paint paint = new Paint();
        this.f313M = paint;
        paint.setAntiAlias(true);
        paint.setTextSize(0);
        paint.setTextAlign(Paint.Align.CENTER);
        paint.setAlpha(255);
        Rect rect = new Rect(0, 0, 0, 0);
        this.f314N = rect;
        this.f370x = new HashMap();
        this.f357q0.getPadding(rect);
        this.f347l0 = (int) (getResources().getDisplayMetrics().density * 500.0f);
        this.f349m0 = true;
        m129m();
        GestureDetector gestureDetector = new GestureDetector(getContext(), new C0016n(this));
        this.f327b0 = gestureDetector;
        gestureDetector.setIsLongpressEnabled(false);
    }

    /* JADX INFO: renamed from: a */
    public final CharSequence m118a(CharSequence charSequence) {
        return (!this.f326b.f152e || charSequence == null || charSequence.length() >= 3 || !Character.isLowerCase(charSequence.charAt(0))) ? charSequence : charSequence.toString().toUpperCase();
    }

    /* JADX INFO: renamed from: b */
    public final void m119b() {
        PopupWindow popupWindow = this.f346l;
        if (popupWindow.isShowing()) {
            popupWindow.dismiss();
        }
        m128l();
        m121d();
        this.f298A0 = null;
        this.f302C0 = null;
        this.f370x.clear();
    }

    /* JADX INFO: renamed from: c */
    public final void m120c(int i, int i2, int i3, long j) {
        if (i != -1) {
            C0013k[] c0013kArr = this.f374z;
            if (i < c0013kArr.length) {
                C0013k c0013k = c0013kArr[i];
                CharSequence charSequence = c0013k.f134m;
                if (charSequence != null) {
                    this.f297A.mo63e(charSequence);
                    this.f297A.mo62d(-1);
                } else {
                    int i4 = c0013k.f122a[0];
                    int[] iArr = new int[f296H0];
                    Arrays.fill(iArr, -1);
                    m122e(i2, i3, iArr);
                    if (this.f369w0) {
                        if (this.f365u0 != -1) {
                            this.f297A.mo59a(-5, f293E0);
                        } else {
                            this.f365u0 = 0;
                        }
                        i4 = c0013k.f122a[this.f365u0];
                    }
                    this.f297A.mo59a(i4, iArr);
                    this.f297A.mo62d(i4);
                }
                this.f363t0 = i;
                this.f367v0 = j;
            }
        }
    }

    /* JADX INFO: renamed from: d */
    public final void m121d() {
        PopupWindow popupWindow = this.f356q;
        if (popupWindow.isShowing()) {
            popupWindow.dismiss();
            this.f362t = false;
            m123f();
        }
    }

    /* JADX WARN: Removed duplicated region for block: B:44:0x0112  */
    /* JADX WARN: Removed duplicated region for block: B:60:0x0145  */
    /* JADX WARN: Removed duplicated region for block: B:89:0x017f A[SYNTHETIC] */
    /* JADX INFO: renamed from: e */
    /*
        Code decompiled incorrectly, please refer to instructions dump.
    */
    public final int m122e(int i, int i2, int[] iArr) {
        int[] iArr2;
        int iM56b;
        int[] iArr3;
        int i3;
        int i4;
        int i5;
        int i6;
        KeyboardView keyboardView = this;
        int i7 = i;
        C0013k[] c0013kArr = keyboardView.f374z;
        int i8 = keyboardView.f301C + 1;
        int[] iArr4 = keyboardView.f361s0;
        Arrays.fill(iArr4, Integer.MAX_VALUE);
        C0015m c0015m = keyboardView.f326b;
        if (c0015m.f163p == null) {
            c0015m.f161n = (c0015m.f155h + 9) / 10;
            c0015m.f162o = (c0015m.f154g + 4) / 5;
            c0015m.f163p = new int[50][];
            ArrayList arrayList = c0015m.f156i;
            int[] iArr5 = new int[arrayList.size()];
            int i9 = c0015m.f161n * 10;
            int i10 = c0015m.f162o * 5;
            int i11 = 0;
            while (i11 < i9) {
                int i12 = 0;
                while (true) {
                    i4 = i8;
                    if (i12 < i10) {
                        int i13 = 0;
                        int i14 = 0;
                        while (i14 < arrayList.size()) {
                            C0013k c0013k = (C0013k) arrayList.get(i14);
                            ArrayList arrayList2 = arrayList;
                            int i15 = i9;
                            if (c0013k.m56b(i11, i12) < c0015m.f164q || c0013k.m56b((c0015m.f161n + i11) - 1, i12) < c0015m.f164q || c0013k.m56b((c0015m.f161n + i11) - 1, (c0015m.f162o + i12) - 1) < c0015m.f164q || c0013k.m56b(i11, (c0015m.f162o + i12) - 1) < c0015m.f164q) {
                                i5 = i10;
                            } else {
                                int i16 = c0015m.f161n;
                                int i17 = c0015m.f162o;
                                i5 = i10;
                                int i18 = c0013k.f130i;
                                if (i18 > i16 + i11 || i11 > i18 + c0013k.f126e || (i6 = c0013k.f131j) > i17 + i12 || i12 > i6 + c0013k.f127f) {
                                }
                                i14++;
                                arrayList = arrayList2;
                                i9 = i15;
                                i10 = i5;
                            }
                            iArr5[i13] = i14;
                            i13++;
                            i14++;
                            arrayList = arrayList2;
                            i9 = i15;
                            i10 = i5;
                        }
                        ArrayList arrayList3 = arrayList;
                        int[] iArr6 = new int[i13];
                        System.arraycopy(iArr5, 0, iArr6, 0, i13);
                        int[][] iArr7 = c0015m.f163p;
                        int i19 = c0015m.f162o;
                        iArr7[(i11 / c0015m.f161n) + ((i12 / i19) * 10)] = iArr6;
                        i12 += i19;
                        i8 = i4;
                        arrayList = arrayList3;
                        i9 = i9;
                    }
                }
                i11 += c0015m.f161n;
                i8 = i4;
            }
        }
        int i20 = i8;
        if (i7 < 0 || i7 >= c0015m.f155h || i2 < 0 || i2 >= c0015m.f154g) {
            iArr2 = new int[0];
        } else {
            int i21 = (i7 / c0015m.f161n) + ((i2 / c0015m.f162o) * 10);
            if (i21 < 50) {
                iArr2 = c0015m.f163p[i21];
            }
        }
        int length = iArr2.length;
        int i22 = i20;
        int i23 = 0;
        int i24 = -1;
        int i25 = -1;
        while (i23 < length) {
            C0013k c0013k2 = c0013kArr[iArr2[i23]];
            boolean zMo49a = c0013k2.mo49a(i7, i2);
            if (zMo49a) {
                i24 = iArr2[i23];
            }
            if (keyboardView.f312L) {
                iM56b = c0013k2.m56b(i7, i2);
                i3 = iM56b >= keyboardView.f301C ? iM56b : 0;
                iArr3 = c0013k2.f122a;
                if (iArr3[0] <= 32) {
                    int length2 = iArr3.length;
                    if (iM56b < i22) {
                        i25 = iArr2[i23];
                        i22 = iM56b;
                    }
                    if (iArr != null) {
                        int i26 = 0;
                        while (true) {
                            if (i26 >= iArr4.length) {
                                break;
                            }
                            if (iArr4[i26] > iM56b) {
                                int i27 = i26 + length2;
                                System.arraycopy(iArr4, i26, iArr4, i27, (iArr4.length - i26) - length2);
                                System.arraycopy(iArr, i26, iArr, i27, (iArr.length - i26) - length2);
                                int i28 = 0;
                                while (i28 < length2) {
                                    int i29 = i26 + i28;
                                    iArr[i29] = c0013k2.f122a[i28];
                                    iArr4[i29] = iM56b;
                                    i28++;
                                    length2 = length2;
                                }
                            } else {
                                i26++;
                            }
                        }
                    }
                }
                i23++;
                keyboardView = this;
                i7 = i;
            }
            if (zMo49a) {
                iM56b = i3;
                iArr3 = c0013k2.f122a;
                if (iArr3[0] <= 32) {
                }
                i23++;
                keyboardView = this;
                i7 = i;
            } else {
                i23++;
                keyboardView = this;
                i7 = i;
            }
        }
        return i24 == -1 ? i25 : i24;
    }

    /* JADX INFO: renamed from: f */
    public final void m123f() {
        this.f375z0.union(0, 0, getWidth(), getHeight());
        this.f373y0 = true;
        invalidate();
    }

    /* JADX INFO: renamed from: g */
    public final void m124g(int i) {
        C0013k[] c0013kArr = this.f374z;
        if (c0013kArr != null && i >= 0 && i < c0013kArr.length) {
            C0013k c0013k = c0013kArr[i];
            this.f339h0 = c0013k;
            int i2 = c0013k.f130i;
            int i3 = c0013k.f131j;
            int i4 = c0013k.f126e;
            int i5 = c0013k.f127f;
            this.f375z0.union(i2, i3, i2 + i4, i3 + i5);
            m126i();
            int i6 = c0013k.f130i;
            int i7 = c0013k.f131j;
            invalidate(i6, i7, i4 + i6, i5 + i7);
        }
    }

    public C0015m getKeyboard() {
        return this.f326b;
    }

    public InterfaceC0018p getOnKeyboardActionListener() {
        return this.f297A;
    }

    /* JADX INFO: renamed from: h */
    public final boolean m125h() {
        View view = this.f364u;
        return (view == null || view == this || !(view instanceof KeyboardView)) ? this.f303D : ((KeyboardView) view).m125h();
    }

    /* JADX WARN: Removed duplicated region for block: B:37:0x009e  */
    /* JADX INFO: renamed from: i */
    /*
        Code decompiled incorrectly, please refer to instructions dump.
    */
    public final void m126i() {
        boolean z;
        C0013k[] c0013kArr;
        C0013k c0013k;
        Bitmap bitmap = this.f298A0;
        boolean z2 = bitmap == null;
        if (z2 || this.f300B0) {
            if (z2 || (this.f300B0 && (bitmap.getWidth() != getWidth() || this.f298A0.getHeight() != getHeight()))) {
                this.f298A0 = Bitmap.createBitmap(Math.max(1, getWidth()), Math.max(1, getHeight()), Bitmap.Config.ARGB_8888);
                this.f302C0 = new Canvas(this.f298A0);
            }
            m123f();
            this.f300B0 = false;
        }
        Canvas canvas = this.f302C0;
        int i = Build.VERSION.SDK_INT;
        Rect rect = this.f375z0;
        if (i <= 27) {
            canvas.clipRect(rect, Region.Op.REPLACE);
        }
        if (this.f326b == null) {
            return;
        }
        Paint paint = this.f313M;
        C0013k[] c0013kArr2 = this.f374z;
        C0013k c0013k2 = this.f339h0;
        int i2 = this.f334f;
        paint.setColor(i2);
        if (c0013k2 != null) {
            Rect rect2 = this.f341i0;
            if (canvas.getClipBounds(rect2)) {
                int i3 = c0013k2.f130i;
                if (i3 - 1 <= rect2.left) {
                    int i4 = c0013k2.f131j;
                    z = i4 + (-1) <= rect2.top && (i3 + c0013k2.f126e) + 1 >= rect2.right && (i4 + c0013k2.f127f) + 1 >= rect2.bottom;
                }
            }
        }
        canvas.drawColor(0, PorterDuff.Mode.CLEAR);
        int length = c0013kArr2.length;
        int i5 = 0;
        while (i5 < length) {
            C0013k c0013k3 = c0013kArr2[i5];
            if (!z || c0013k2 == c0013k3) {
                paint.setColor(c0013k3.f140s ? this.f336g : i2);
                Drawable drawable = c0013k3.f140s ? this.f359r0 : this.f357q0;
                int[] iArr = C0013k.f120x;
                if (c0013k3.f133l) {
                    iArr = c0013k3.f132k ? C0013k.f117u : C0013k.f116t;
                } else if (c0013k3.f129h) {
                    iArr = c0013k3.f132k ? C0013k.f119w : C0013k.f118v;
                } else if (c0013k3.f132k) {
                    iArr = C0013k.f121y;
                }
                drawable.setState(iArr);
                CharSequence charSequence = c0013k3.f123b;
                String string = charSequence == null ? null : m118a(charSequence).toString();
                Rect bounds = drawable.getBounds();
                int i6 = bounds.right;
                int i7 = c0013k3.f126e;
                c0013kArr = c0013kArr2;
                int i8 = c0013k3.f127f;
                if (i7 != i6 || i8 != bounds.bottom) {
                    drawable.setBounds(0, 0, i7, i8);
                }
                canvas.translate(c0013k3.f130i, c0013k3.f131j);
                drawable.draw(canvas);
                Rect rect3 = this.f314N;
                if (string != null) {
                    AbstractC0010h abstractC0010h = AbstractC0010h.f103k;
                    paint.setTextSize((string.length() <= 1 || c0013k3.f122a.length >= 2) ? this.f332e : this.f330d);
                    paint.setTypeface(Typeface.DEFAULT_BOLD);
                    c0013k = c0013k2;
                    paint.setShadowLayer(this.f338h, 0.0f, 0.0f, this.f340i);
                    int i9 = rect3.left;
                    canvas.drawText(string, (((i7 - i9) - rect3.right) / 2) + i9, ((paint.getTextSize() - paint.descent()) / 2.0f) + (((i8 - rect3.top) - rect3.bottom) / 2) + rect3.top, paint);
                    paint.setShadowLayer(0.0f, 0.0f, 0.0f, 0);
                } else {
                    c0013k = c0013k2;
                    Drawable drawable2 = c0013k3.f124c;
                    if (drawable2 != null) {
                        AbstractC0010h abstractC0010h2 = AbstractC0010h.f103k;
                        canvas.translate(((((i7 - rect3.left) - rect3.right) - drawable2.getIntrinsicWidth()) / 2) + rect3.left, ((((i8 - rect3.top) - rect3.bottom) - c0013k3.f124c.getIntrinsicHeight()) / 2) + rect3.top);
                        Drawable drawable3 = c0013k3.f124c;
                        drawable3.setBounds(0, 0, drawable3.getIntrinsicWidth(), c0013k3.f124c.getIntrinsicHeight());
                        c0013k3.f124c.draw(canvas);
                        canvas.translate(-r3, -r1);
                    }
                }
                canvas.translate(-c0013k3.f130i, -c0013k3.f131j);
            } else {
                c0013kArr = c0013kArr2;
                c0013k = c0013k2;
            }
            i5++;
            c0013kArr2 = c0013kArr;
            c0013k2 = c0013k;
        }
        this.f339h0 = null;
        if (this.f362t) {
            paint.setColor(((int) (this.f342j * 255.0f)) << 24);
            canvas.drawRect(0.0f, 0.0f, getWidth(), getHeight(), paint);
        }
        this.f373y0 = false;
        rect.setEmpty();
    }

    /* JADX WARN: Multi-variable type inference failed */
    /* JADX INFO: renamed from: j */
    public boolean mo103j(C0013k c0013k) {
        C0015m c0015m;
        this.f297A.mo60b(c0013k);
        int i = c0013k.f138q;
        if (i == 0) {
            return false;
        }
        HashMap map = this.f370x;
        View view = (View) map.get(c0013k);
        this.f358r = view;
        if (view == null) {
            View viewInflate = ((LayoutInflater) getContext().getSystemService("layout_inflater")).inflate(this.f335f0, (ViewGroup) null);
            this.f358r = viewInflate;
            this.f360s = (KeyboardView) viewInflate.findViewById(R.id.keyboardView);
            View viewFindViewById = this.f358r.findViewById(R.id.closeButton);
            if (viewFindViewById != null) {
                viewFindViewById.setOnClickListener(this);
            }
            this.f360s.setOnKeyboardActionListener(new C0017o(this));
            CharSequence charSequence = c0013k.f135n;
            if (charSequence != null) {
                Context context = getContext();
                int paddingRight = getPaddingRight() + getPaddingLeft();
                c0015m = new C0015m(context, i);
                c0015m.f155h = 0;
                C0014l c0014l = new C0014l(c0015m);
                c0014l.f142b = c0015m.f150c;
                c0014l.f141a = c0015m.f149b;
                c0014l.f143c = c0015m.f148a;
                c0014l.f144d = c0015m.f151d;
                c0014l.f145e = 12;
                int i2 = 0;
                int i3 = 0;
                int i4 = 0;
                for (int i5 = 0; i5 < charSequence.length(); i5++) {
                    int iCharAt = charSequence.charAt(i5);
                    if (i3 >= Integer.MAX_VALUE || c0015m.f149b + i4 + paddingRight > c0015m.f158k) {
                        i2 += c0015m.f151d + c0015m.f150c;
                        i3 = 0;
                        i4 = 0;
                    }
                    C0013k c0013k2 = new C0013k(c0014l);
                    c0013k2.f130i = i4;
                    c0013k2.f131j = i2;
                    c0013k2.f123b = String.valueOf((char) iCharAt);
                    c0013k2.f122a = new int[]{iCharAt};
                    i3++;
                    i4 += c0013k2.f126e + c0013k2.f128g;
                    c0015m.f156i.add(c0013k2);
                    if (i4 > c0015m.f155h) {
                        c0015m.f155h = i4;
                    }
                }
                c0015m.f154g = i2 + c0015m.f150c;
            } else {
                c0015m = new C0015m(getContext(), i);
            }
            this.f360s.setKeyboard(c0015m);
            this.f360s.setPopupParent(this);
            this.f358r.measure(View.MeasureSpec.makeMeasureSpec(getWidth(), Integer.MIN_VALUE), View.MeasureSpec.makeMeasureSpec(getHeight(), Integer.MIN_VALUE));
            map.put(c0013k, this.f358r);
        } else {
            this.f360s = (KeyboardView) view.findViewById(R.id.keyboardView);
        }
        if (this.f372y == null) {
            int[] iArr = new int[2];
            this.f372y = iArr;
            getLocationInWindow(iArr);
        }
        int i6 = c0013k.f130i;
        this.f329c0 = i6;
        this.f331d0 = c0013k.f131j;
        this.f329c0 = (i6 + c0013k.f126e) - this.f358r.getMeasuredWidth();
        this.f331d0 -= this.f358r.getMeasuredHeight();
        int paddingRight2 = this.f358r.getPaddingRight() + this.f329c0 + this.f372y[0];
        int paddingBottom = this.f358r.getPaddingBottom() + this.f331d0 + this.f372y[1];
        KeyboardView keyboardView = this.f360s;
        keyboardView.f366v = paddingRight2 < 0 ? 0 : paddingRight2;
        keyboardView.f368w = paddingBottom;
        PopupWindow popupWindow = keyboardView.f346l;
        if (popupWindow.isShowing()) {
            popupWindow.dismiss();
        }
        KeyboardView keyboardView2 = this.f360s;
        C0015m c0015m2 = this.f326b;
        keyboardView2.m130n(c0015m2 != null ? c0015m2.f152e : false);
        PopupWindow popupWindow2 = this.f356q;
        popupWindow2.setContentView(this.f358r);
        popupWindow2.setWidth(this.f358r.getMeasuredWidth());
        popupWindow2.setHeight(this.f358r.getMeasuredHeight());
        popupWindow2.showAtLocation(this, 0, paddingRight2, paddingBottom);
        this.f362t = true;
        m123f();
        return true;
    }

    /* JADX WARN: Removed duplicated region for block: B:41:0x00f2  */
    /* JADX WARN: Removed duplicated region for block: B:90:0x020e  */
    /* JADX INFO: renamed from: k */
    /*
        Code decompiled incorrectly, please refer to instructions dump.
    */
    public final void m127k(MotionEvent motionEvent, boolean z) {
        int i;
        int i2;
        int i3;
        int i4;
        int i5;
        int i6;
        long j;
        int x = (int) motionEvent.getX();
        int y = ((int) motionEvent.getY()) + this.f299B;
        int action = motionEvent.getAction();
        long eventTime = motionEvent.getEventTime();
        int iM122e = m122e(x, y, null);
        this.f343j0 = z;
        C0019q c0019q = this.f345k0;
        if (action == 0) {
            c0019q.f169c[0] = 0;
        }
        c0019q.getClass();
        long eventTime2 = motionEvent.getEventTime();
        int historySize = motionEvent.getHistorySize();
        int i7 = 0;
        while (i7 < historySize) {
            c0019q.m64a(motionEvent.getHistoricalX(i7), motionEvent.getHistoricalY(i7), motionEvent.getHistoricalEventTime(i7));
            i7++;
            x = x;
            y = y;
        }
        int i8 = x;
        int i9 = y;
        c0019q.m64a(motionEvent.getX(), motionEvent.getY(), eventTime2);
        if (!this.f337g0 || action == 0 || action == 3) {
            boolean zOnTouchEvent = this.f327b0.onTouchEvent(motionEvent);
            HandlerC0009g handlerC0009g = this.f304D0;
            if (zOnTouchEvent) {
                m132p(-1);
                handlerC0009g.removeMessages(3);
                handlerC0009g.removeMessages(4);
                return;
            }
            if (!this.f362t || action == 3) {
                int i10 = f295G0;
                if (action == 0) {
                    this.f337g0 = false;
                    i = i8;
                    this.f310J = i;
                    i2 = i9;
                    this.f311K = i2;
                    this.f318R = i;
                    this.f319S = i2;
                    this.f322V = 0L;
                    this.f323W = 0L;
                    this.f317Q = -1;
                    this.f320T = iM122e;
                    this.f321U = iM122e;
                    long eventTime3 = motionEvent.getEventTime();
                    this.f315O = eventTime3;
                    this.f316P = eventTime3;
                    if (iM122e == -1) {
                        i3 = i10;
                    } else {
                        int[] iArr = this.f374z[iM122e].f122a;
                        if (iArr.length > 1) {
                            this.f369w0 = true;
                            i3 = i10;
                            if (eventTime >= this.f367v0 + 800 || iM122e != this.f363t0) {
                                this.f365u0 = -1;
                            } else {
                                this.f365u0 = (this.f365u0 + 1) % iArr.length;
                            }
                        } else {
                            i3 = i10;
                            if (eventTime > this.f367v0 + 800 || iM122e != this.f363t0) {
                                m129m();
                            }
                        }
                    }
                    this.f297A.mo61c(iM122e != -1 ? this.f374z[iM122e].f122a[0] : 0);
                    int i11 = this.f320T;
                    if (i11 < 0 || !this.f374z[i11].f139r) {
                        if (this.f320T != -1) {
                            handlerC0009g.sendMessageDelayed(handlerC0009g.obtainMessage(4, motionEvent), i3);
                        }
                        m132p(iM122e);
                    } else {
                        this.f333e0 = i11;
                        handlerC0009g.sendMessageDelayed(handlerC0009g.obtainMessage(3), 400L);
                        C0013k c0013k = this.f374z[this.f333e0];
                        m120c(this.f320T, c0013k.f130i, c0013k.f131j, this.f367v0);
                        if (this.f337g0) {
                            this.f333e0 = -1;
                        }
                    }
                    this.f308H = i5;
                    this.f309I = i4;
                }
                if (action == 1) {
                    m128l();
                    if (iM122e == this.f320T) {
                        this.f323W = (eventTime - this.f316P) + this.f323W;
                    } else {
                        m129m();
                        this.f317Q = this.f320T;
                        this.f322V = (this.f323W + eventTime) - this.f316P;
                        this.f320T = iM122e;
                        this.f323W = 0L;
                    }
                    long j2 = this.f323W;
                    if (j2 >= this.f322V || j2 >= 70 || (i6 = this.f317Q) == -1) {
                        i5 = i8;
                        i4 = i9;
                    } else {
                        this.f320T = i6;
                        i5 = this.f318R;
                        i4 = this.f319S;
                    }
                    m132p(-1);
                    Arrays.fill(this.f325a0, -1);
                    if (this.f333e0 == -1 && !this.f362t && !this.f337g0) {
                        m120c(this.f320T, i5, i4, eventTime);
                    }
                    m124g(iM122e);
                    this.f333e0 = -1;
                    this.f308H = i5;
                    this.f309I = i4;
                }
                if (action == 2) {
                    if (iM122e == -1) {
                        handlerC0009g.removeMessages(4);
                        if (iM122e != -1) {
                            handlerC0009g.sendMessageDelayed(handlerC0009g.obtainMessage(4, motionEvent), i10);
                        }
                        m132p(this.f320T);
                        this.f316P = eventTime;
                    } else {
                        int i12 = this.f320T;
                        if (i12 == -1) {
                            this.f320T = iM122e;
                            j = eventTime - this.f315O;
                        } else if (iM122e == i12) {
                            this.f323W = (eventTime - this.f316P) + this.f323W;
                            m132p(this.f320T);
                            this.f316P = eventTime;
                        } else {
                            if (this.f333e0 == -1) {
                                m129m();
                                this.f317Q = this.f320T;
                                this.f318R = this.f308H;
                                this.f319S = this.f309I;
                                this.f322V = (this.f323W + eventTime) - this.f316P;
                                this.f320T = iM122e;
                                j = 0;
                            }
                            handlerC0009g.removeMessages(4);
                            if (iM122e != -1) {
                            }
                            m132p(this.f320T);
                            this.f316P = eventTime;
                        }
                        this.f323W = j;
                        handlerC0009g.removeMessages(4);
                        if (iM122e != -1) {
                        }
                        m132p(this.f320T);
                        this.f316P = eventTime;
                    }
                    this.f308H = i5;
                    this.f309I = i4;
                }
                if (action == 3) {
                    m128l();
                    m121d();
                    this.f337g0 = true;
                    m132p(-1);
                    m124g(this.f320T);
                }
                i = i8;
                i2 = i9;
                i5 = i;
                i4 = i2;
                this.f308H = i5;
                this.f309I = i4;
            }
        }
    }

    /* JADX INFO: renamed from: l */
    public final void m128l() {
        HandlerC0009g handlerC0009g = this.f304D0;
        handlerC0009g.removeMessages(3);
        handlerC0009g.removeMessages(4);
        handlerC0009g.removeMessages(1);
    }

    /* JADX INFO: renamed from: m */
    public final void m129m() {
        this.f363t0 = -1;
        this.f365u0 = 0;
        this.f367v0 = -1L;
        this.f369w0 = false;
    }

    /* JADX INFO: renamed from: n */
    public final void m130n(boolean z) {
        C0015m c0015m = this.f326b;
        if (c0015m != null) {
            C0013k c0013k = c0015m.f153f;
            if (c0013k != null) {
                c0013k.f133l = z;
            }
            if (c0015m.f152e != z) {
                c0015m.f152e = z;
                m123f();
            }
        }
    }

    /* JADX INFO: renamed from: o */
    public final void m131o(int i) {
        CharSequence charSequence;
        Typeface typeface;
        int i2;
        PopupWindow popupWindow = this.f346l;
        C0013k[] c0013kArr = this.f374z;
        if (i < 0 || i >= c0013kArr.length) {
            return;
        }
        C0013k c0013k = c0013kArr[i];
        Drawable drawable = getContext().getResources().getDrawable(R.drawable.keyboard_key_feedback);
        TextView textView = this.f344k;
        textView.setBackgroundDrawable(drawable);
        Drawable drawable2 = c0013k.f124c;
        if (drawable2 != null) {
            Drawable drawable3 = c0013k.f125d;
            if (drawable3 != null) {
                drawable2 = drawable3;
            }
            textView.setCompoundDrawables(null, null, null, drawable2);
            textView.setText((CharSequence) null);
        } else {
            textView.setCompoundDrawables(null, null, null, null);
            if (this.f369w0) {
                StringBuilder sb = this.f371x0;
                sb.setLength(0);
                int[] iArr = c0013k.f122a;
                int i3 = this.f365u0;
                if (i3 < 0) {
                    i3 = 0;
                }
                sb.append((char) iArr[i3]);
                charSequence = sb;
            } else {
                charSequence = c0013k.f123b;
            }
            textView.setText(m118a(charSequence));
            if (c0013k.f123b.length() <= 1 || c0013k.f122a.length >= 2) {
                textView.setTextSize(0, this.f348m);
                typeface = Typeface.DEFAULT;
            } else {
                textView.setTextSize(0, this.f332e);
                typeface = Typeface.DEFAULT_BOLD;
            }
            textView.setTypeface(typeface);
        }
        textView.measure(View.MeasureSpec.makeMeasureSpec(0, 0), View.MeasureSpec.makeMeasureSpec(0, 0));
        int measuredWidth = textView.getMeasuredWidth();
        int paddingLeft = textView.getPaddingLeft();
        int i4 = c0013k.f126e;
        int iMax = Math.max(measuredWidth, textView.getPaddingRight() + paddingLeft + i4);
        ViewGroup.LayoutParams layoutParams = textView.getLayoutParams();
        int i5 = this.f352o;
        if (layoutParams != null) {
            layoutParams.width = iMax;
            layoutParams.height = i5;
        }
        this.f305E = c0013k.f130i - textView.getPaddingLeft();
        int i6 = c0013k.f131j - i5;
        int i7 = this.f350n;
        this.f306F = i6 + i7 + 20;
        this.f304D0.removeMessages(2);
        if (this.f354p == null) {
            int[] iArr2 = new int[2];
            this.f354p = iArr2;
            getLocationInWindow(iArr2);
            int[] iArr3 = this.f354p;
            iArr3[0] = iArr3[0] + this.f366v;
            iArr3[1] = iArr3[1] + this.f368w;
            int[] iArr4 = new int[2];
            getLocationOnScreen(iArr4);
            this.f307G = iArr4[1];
        }
        textView.getBackground().setState(c0013k.f138q != 0 ? f294F0 : View.EMPTY_STATE_SET);
        int i8 = this.f305E;
        int[] iArr5 = this.f354p;
        this.f305E = i8 + iArr5[0];
        int i9 = this.f306F + iArr5[1];
        this.f306F = i9;
        if (i9 + this.f307G < 0) {
            if (c0013k.f130i + i4 <= getWidth() / 2) {
                int i10 = this.f305E;
                double d = i4;
                Double.isNaN(d);
                i2 = i10 + ((int) (d * 2.5d));
            } else {
                int i11 = this.f305E;
                double d2 = i4;
                Double.isNaN(d2);
                i2 = i11 - ((int) (d2 * 2.5d));
            }
            this.f305E = i2;
            this.f306F += i5;
        }
        if (c0013k.f130i + iMax > getWidth()) {
            this.f305E = getWidth() - iMax;
        }
        if (Build.VERSION.SDK_INT >= 28) {
            this.f306F = ((getResources().getDisplayMetrics().heightPixels - getHeight()) - c0013k.f127f) + i7 + this.f306F;
        }
        if (popupWindow.isShowing()) {
            popupWindow.update(this.f305E, this.f306F, iMax, i5);
        } else {
            popupWindow.setWidth(iMax);
            popupWindow.setHeight(i5);
            popupWindow.showAtLocation(this.f364u, 0, this.f305E, this.f306F);
        }
        textView.setVisibility(0);
    }

    @Override // android.view.View.OnClickListener
    public final void onClick(View view) {
        m121d();
    }

    @Override // android.view.View
    public final void onDetachedFromWindow() {
        super.onDetachedFromWindow();
        m119b();
    }

    @Override // android.view.View
    public final void onDraw(Canvas canvas) {
        super.onDraw(canvas);
        if (this.f373y0 || this.f298A0 == null || this.f300B0) {
            m126i();
        }
        canvas.drawBitmap(this.f298A0, 0.0f, 0.0f, (Paint) null);
    }

    @Override // android.view.View
    public final void onMeasure(int i, int i2) {
        C0015m c0015m = this.f326b;
        if (c0015m == null) {
            setMeasuredDimension(0, 0);
            return;
        }
        int size = c0015m.f155h;
        if (View.MeasureSpec.getSize(i) < size + 10) {
            size = View.MeasureSpec.getSize(i);
        }
        setMeasuredDimension(size, this.f326b.f154g);
    }

    @Override // android.view.View
    public final void onSizeChanged(int i, int i2, int i3, int i4) {
        super.onSizeChanged(i, i2, i3, i4);
        this.f298A0 = null;
    }

    @Override // android.view.View
    public final boolean onTouchEvent(MotionEvent motionEvent) {
        int pointerCount = motionEvent.getPointerCount();
        int action = motionEvent.getAction();
        long eventTime = motionEvent.getEventTime();
        boolean z = pointerCount == 1;
        if (pointerCount != this.f351n0) {
            if (z) {
                MotionEvent motionEventObtain = MotionEvent.obtain(eventTime, eventTime, 0, motionEvent.getX(), motionEvent.getY(), motionEvent.getMetaState());
                m127k(motionEventObtain, false);
                motionEventObtain.recycle();
                if (action == 1) {
                    m127k(motionEvent, true);
                }
            } else {
                MotionEvent motionEventObtain2 = MotionEvent.obtain(eventTime, eventTime, 1, this.f353o0, this.f355p0, motionEvent.getMetaState());
                m127k(motionEventObtain2, true);
                motionEventObtain2.recycle();
            }
        } else if (z) {
            m127k(motionEvent, false);
            this.f353o0 = motionEvent.getX();
            this.f355p0 = motionEvent.getY();
        }
        this.f351n0 = pointerCount;
        return true;
    }

    /* JADX INFO: renamed from: p */
    public final void m132p(int i) {
        int i2 = this.f328c;
        PopupWindow popupWindow = this.f346l;
        this.f328c = i;
        C0013k[] c0013kArr = this.f374z;
        if (i2 != i) {
            if (i2 != -1 && c0013kArr.length > i2) {
                C0013k c0013k = c0013kArr[i2];
                c0013k.f132k = !c0013k.f132k;
                if (c0013k.f129h) {
                    c0013k.f133l = !c0013k.f133l;
                }
                m124g(i2);
            }
            int i3 = this.f328c;
            if (i3 != -1 && c0013kArr.length > i3) {
                c0013kArr[i3].f132k = !r2.f132k;
                m124g(i3);
            }
        }
        if (i2 != this.f328c && this.f303D && m125h()) {
            HandlerC0009g handlerC0009g = this.f304D0;
            handlerC0009g.removeMessages(1);
            if (popupWindow.isShowing() && i == -1) {
                handlerC0009g.sendMessageDelayed(handlerC0009g.obtainMessage(2), 70L);
            }
            if (i != -1) {
                if (popupWindow.isShowing() && this.f344k.getVisibility() == 0) {
                    m131o(i);
                } else {
                    handlerC0009g.sendMessageDelayed(handlerC0009g.obtainMessage(1, i, 0), 0L);
                }
            }
        }
    }

    /* JADX WARN: Removed duplicated region for block: B:12:0x0024  */
    /* JADX WARN: Removed duplicated region for block: B:32:0x0091  */
    /*
        Code decompiled incorrectly, please refer to instructions dump.
    */
    public void setKeyboard(C0015m c0015m) {
        int i;
        int iM122e;
        if (!c0015m.equals(this.f326b)) {
            this.f354p = null;
        }
        boolean z = false;
        if (this.f326b != null) {
            m132p(-1);
            int i2 = this.f333e0;
            if (i2 != -1) {
                C0013k[] c0013kArr = this.f374z;
                i = i2 < c0013kArr.length ? c0013kArr[i2].f122a[0] : -1;
            }
        }
        m128l();
        this.f326b = c0015m;
        ArrayList arrayList = c0015m.f156i;
        this.f374z = (C0013k[]) arrayList.toArray(new C0013k[arrayList.size()]);
        requestLayout();
        this.f300B0 = true;
        m123f();
        C0013k[] c0013kArr2 = this.f374z;
        if (c0013kArr2 != null) {
            int length = c0013kArr2.length;
            int iMin = 0;
            for (C0013k c0013k : c0013kArr2) {
                iMin += Math.min(c0013k.f126e, c0013k.f127f) + c0013k.f128g;
            }
            if (iMin >= 0 && length != 0) {
                int i3 = (int) ((iMin * 1.4f) / length);
                this.f301C = i3 * i3;
            }
        }
        this.f370x.clear();
        if (i == -1 || (iM122e = m122e(this.f310J, this.f311K, null)) == -1) {
            z = true;
        } else {
            C0013k[] c0013kArr3 = this.f374z;
            if (iM122e < c0013kArr3.length && i == c0013kArr3[iM122e].f122a[0]) {
                this.f333e0 = iM122e;
            }
        }
        if (z) {
            this.f304D0.removeMessages(3);
        }
        this.f337g0 = z;
    }

    public void setOnKeyboardActionListener(InterfaceC0018p interfaceC0018p) {
        this.f297A = interfaceC0018p;
    }

    public void setPopupParent(View view) {
        this.f364u = view;
    }

    public void setPreviewEnabled(boolean z) {
        this.f303D = z;
    }

    public void setProximityCorrectionEnabled(boolean z) {
        this.f312L = z;
    }

    public void setVerticalCorrection(int i) {
    }
}
