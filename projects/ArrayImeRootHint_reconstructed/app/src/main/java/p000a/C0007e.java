package p000a;

import android.content.Context;
import android.content.res.Resources;
import android.content.res.XmlResourceParser;
import com.minn.android.hanziime.R;

/* JADX INFO: renamed from: a.e */
/* JADX INFO: loaded from: classes.dex */
public final class C0007e extends C0015m {

    /* JADX INFO: renamed from: A */
    public C0006d f88A;

    /* JADX INFO: renamed from: B */
    public C0006d f89B;

    /* JADX INFO: renamed from: C */
    public C0006d f90C;

    /* JADX INFO: renamed from: D */
    public final AbstractC0010h f91D;

    /* JADX INFO: renamed from: r */
    public C0006d f92r;

    /* JADX INFO: renamed from: s */
    public C0006d f93s;

    /* JADX INFO: renamed from: t */
    public C0006d f94t;

    /* JADX INFO: renamed from: u */
    public C0006d f95u;

    /* JADX INFO: renamed from: v */
    public C0006d f96v;

    /* JADX INFO: renamed from: w */
    public C0006d f97w;

    /* JADX INFO: renamed from: x */
    public C0006d f98x;

    /* JADX INFO: renamed from: y */
    public C0006d f99y;

    /* JADX INFO: renamed from: z */
    public C0006d f100z;

    public C0007e(Context context, int i) {
        super(context, i);
        this.f91D = (AbstractC0010h) context;
    }

    @Override // p000a.C0015m
    /* JADX INFO: renamed from: a */
    public final C0013k mo50a(Resources resources, C0014l c0014l, int i, int i2, XmlResourceParser xmlResourceParser) {
        C0006d c0006d = new C0006d(resources, c0014l, i, i2, xmlResourceParser);
        int i3 = c0006d.f122a[0];
        if (i3 == -101) {
            this.f92r = c0006d;
        } else if (i3 == 32) {
            this.f93s = c0006d;
        } else if (i3 == 48) {
            this.f94t = c0006d;
        } else if (i3 == 49) {
            this.f95u = c0006d;
        } else if (i3 == 50) {
            this.f96v = c0006d;
        } else if (i3 == 51) {
            this.f97w = c0006d;
        } else if (i3 == 52) {
            this.f98x = c0006d;
        } else if (i3 == 53) {
            this.f99y = c0006d;
        } else if (i3 == 54) {
            this.f100z = c0006d;
        } else if (i3 == 55) {
            this.f88A = c0006d;
        } else if (i3 == 56) {
            this.f89B = c0006d;
        } else if (i3 == 57) {
            this.f90C = c0006d;
        }
        return c0006d;
    }

    /* JADX INFO: renamed from: d */
    public final void m51d(Resources resources, int i) {
        int[] iArr = {27};
        int[] iArr2 = {-101};
        if (i == 1) {
            C0006d c0006d = this.f92r;
            c0006d.f125d = null;
            c0006d.f124c = null;
            c0006d.f123b = "Esc";
            c0006d.f122a = iArr;
            return;
        }
        if (i == 0) {
            this.f92r.f124c = resources.getDrawable(R.drawable.sym_keyboard_return);
            C0006d c0006d2 = this.f92r;
            c0006d2.f123b = null;
            c0006d2.f122a = iArr2;
        }
    }
}
