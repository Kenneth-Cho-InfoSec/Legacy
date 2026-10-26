package com.minn.android.hanziime;

import android.content.Context;
import android.content.res.Resources;
import android.content.res.TypedArray;
import android.graphics.drawable.Drawable;
import android.util.AttributeSet;
import android.view.inputmethod.InputMethodSubtype;
import p000a.AbstractC0010h;
import p000a.AbstractC0020r;
import p000a.C0006d;
import p000a.C0007e;
import p000a.C0013k;

/* JADX INFO: loaded from: classes.dex */
public class HzimKeyboardView extends KeyboardView {

    /* JADX INFO: renamed from: I0 */
    public TypedArray f262I0;

    public HzimKeyboardView(Context context, AttributeSet attributeSet) {
        super(context, attributeSet);
        this.f262I0 = null;
    }

    @Override // com.minn.android.hanziime.KeyboardView
    /* JADX INFO: renamed from: j */
    public final boolean mo103j(C0013k c0013k) {
        if (c0013k.f122a[0] != -3) {
            return super.mo103j(c0013k);
        }
        getOnKeyboardActionListener().mo59a(-100, null);
        return true;
    }

    public void setEnterKeyAs(int i) {
        Resources resources;
        C0007e c0007e = (C0007e) getKeyboard();
        int i2 = 1;
        if (i != 1) {
            if (i == 0) {
                resources = getResources();
                i2 = 0;
            }
            m123f();
        }
        resources = getResources();
        c0007e.m51d(resources, i2);
        m123f();
    }

    public void setKeyPreviewAs(String str) {
        C0007e c0007e = (C0007e) getKeyboard();
        if (str == "") {
            this.f262I0 = getResources().obtainTypedArray(R.array.ar30_12key_preview_array_nx);
        } else {
            try {
                this.f262I0 = getResources().obtainTypedArray(AbstractC0020r.class.getField("ar30_12key_preview_array_" + str + "x").getInt(null));
            } catch (Exception e) {
                e.printStackTrace();
            }
        }
        TypedArray typedArray = this.f262I0;
        C0006d c0006d = c0007e.f94t;
        AbstractC0010h abstractC0010h = c0007e.f91D;
        c0006d.f125d = abstractC0010h.getResources().getDrawable(typedArray.getResourceId(0, -1));
        Drawable drawable = c0007e.f94t.f125d;
        drawable.setBounds(0, 0, drawable.getIntrinsicWidth(), c0007e.f94t.f125d.getIntrinsicHeight());
        c0007e.f94t.f123b = null;
        c0007e.f95u.f125d = abstractC0010h.getResources().getDrawable(typedArray.getResourceId(1, -1));
        Drawable drawable2 = c0007e.f95u.f125d;
        drawable2.setBounds(0, 0, drawable2.getIntrinsicWidth(), c0007e.f95u.f125d.getIntrinsicHeight());
        c0007e.f95u.f123b = null;
        c0007e.f96v.f125d = abstractC0010h.getResources().getDrawable(typedArray.getResourceId(2, -1));
        Drawable drawable3 = c0007e.f96v.f125d;
        drawable3.setBounds(0, 0, drawable3.getIntrinsicWidth(), c0007e.f96v.f125d.getIntrinsicHeight());
        c0007e.f96v.f123b = null;
        c0007e.f97w.f125d = abstractC0010h.getResources().getDrawable(typedArray.getResourceId(3, -1));
        Drawable drawable4 = c0007e.f97w.f125d;
        drawable4.setBounds(0, 0, drawable4.getIntrinsicWidth(), c0007e.f97w.f125d.getIntrinsicHeight());
        c0007e.f97w.f123b = null;
        c0007e.f98x.f125d = abstractC0010h.getResources().getDrawable(typedArray.getResourceId(4, -1));
        Drawable drawable5 = c0007e.f98x.f125d;
        drawable5.setBounds(0, 0, drawable5.getIntrinsicWidth(), c0007e.f98x.f125d.getIntrinsicHeight());
        c0007e.f98x.f123b = null;
        c0007e.f99y.f125d = abstractC0010h.getResources().getDrawable(typedArray.getResourceId(5, -1));
        Drawable drawable6 = c0007e.f99y.f125d;
        drawable6.setBounds(0, 0, drawable6.getIntrinsicWidth(), c0007e.f99y.f125d.getIntrinsicHeight());
        c0007e.f99y.f123b = null;
        c0007e.f100z.f125d = abstractC0010h.getResources().getDrawable(typedArray.getResourceId(6, -1));
        Drawable drawable7 = c0007e.f100z.f125d;
        drawable7.setBounds(0, 0, drawable7.getIntrinsicWidth(), c0007e.f100z.f125d.getIntrinsicHeight());
        c0007e.f100z.f123b = null;
        c0007e.f88A.f125d = abstractC0010h.getResources().getDrawable(typedArray.getResourceId(7, -1));
        Drawable drawable8 = c0007e.f88A.f125d;
        drawable8.setBounds(0, 0, drawable8.getIntrinsicWidth(), c0007e.f88A.f125d.getIntrinsicHeight());
        c0007e.f88A.f123b = null;
        c0007e.f89B.f125d = abstractC0010h.getResources().getDrawable(typedArray.getResourceId(8, -1));
        Drawable drawable9 = c0007e.f89B.f125d;
        drawable9.setBounds(0, 0, drawable9.getIntrinsicWidth(), c0007e.f89B.f125d.getIntrinsicHeight());
        c0007e.f89B.f123b = null;
        c0007e.f90C.f125d = abstractC0010h.getResources().getDrawable(typedArray.getResourceId(9, -1));
        Drawable drawable10 = c0007e.f90C.f125d;
        drawable10.setBounds(0, 0, drawable10.getIntrinsicWidth(), c0007e.f90C.f125d.getIntrinsicHeight());
        c0007e.f90C.f123b = null;
        m123f();
        this.f262I0.recycle();
    }

    public void setSubtypeOnSpaceKey(InputMethodSubtype inputMethodSubtype) {
        C0007e c0007e = (C0007e) getKeyboard();
        Drawable drawable = getResources().getDrawable(inputMethodSubtype.getIconResId());
        C0006d c0006d = c0007e.f93s;
        if (c0006d != null) {
            c0006d.f124c = drawable;
        }
        m123f();
    }
}
