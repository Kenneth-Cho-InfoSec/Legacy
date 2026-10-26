package p001b;

import android.content.Context;
import android.content.res.Resources;
import android.text.SpannableStringBuilder;
import android.text.style.ImageSpan;
import android.view.MotionEvent;
import android.view.View;
import com.minn.android.hanziime.HzimMainTWService;
import com.minn.android.hanziime.R;
import java.util.ArrayList;
import p000a.HandlerC0009g;

/* JADX INFO: renamed from: b.j */
/* JADX INFO: loaded from: classes.dex */
public final class ViewOnTouchListenerC0038j implements View.OnTouchListener {

    /* JADX INFO: renamed from: a */
    public final ArrayList f255a;

    /* JADX INFO: renamed from: b */
    public final View f256b;

    /* JADX INFO: renamed from: c */
    public final HzimMainTWService f257c;

    /* JADX INFO: renamed from: d */
    public final int[] f258d;

    /* JADX INFO: renamed from: e */
    public int f259e;

    /* JADX INFO: renamed from: f */
    public boolean f260f;

    /* JADX INFO: renamed from: g */
    public final HandlerC0009g f261g;

    public ViewOnTouchListenerC0038j(HzimMainTWService hzimMainTWService, View view) {
        ArrayList arrayList = new ArrayList();
        this.f255a = arrayList;
        this.f258d = new int[2];
        this.f260f = false;
        this.f261g = new HandlerC0009g(this, 3);
        this.f256b = view;
        this.f257c = hzimMainTWService;
        Context context = view.getContext();
        int width = view.getWidth();
        Resources resources = view.getContext().getResources();
        int i = width / 20;
        resources.getDimensionPixelOffset(R.dimen.bubble_pointer_offset);
        SpannableStringBuilder spannableStringBuilder = new SpannableStringBuilder();
        spannableStringBuilder.clear();
        spannableStringBuilder.append(resources.getText(R.string.tip_en_to_open_keyboard));
        arrayList.add(new C0037i(this, context, view, i, spannableStringBuilder));
        spannableStringBuilder.clear();
        spannableStringBuilder.append(resources.getText(R.string.tip_en_to_close_keyboard));
        String string = spannableStringBuilder.toString();
        int iIndexOf = string.indexOf("←");
        while (iIndexOf >= 0) {
            int i2 = iIndexOf + 1;
            spannableStringBuilder.setSpan(new ImageSpan(this.f257c, R.drawable.tutorial_back, 0), iIndexOf, i2, 33);
            iIndexOf = string.indexOf("←", i2);
        }
        this.f255a.add(new C0037i(this, context, view, i, spannableStringBuilder));
        this.f255a.add(new C0037i(this, context, view, i));
    }

    /* JADX INFO: renamed from: a */
    public final void m102a() {
        int i = this.f259e;
        ArrayList arrayList = this.f255a;
        if (i >= 0) {
            if (!((C0037i) arrayList.get(i)).f251g.isShowing()) {
                return;
            }
            for (int i2 = 0; i2 <= this.f259e; i2++) {
                C0037i c0037i = (C0037i) arrayList.get(i2);
                if (c0037i.f251g.isShowing()) {
                    c0037i.f252h.setOnTouchListener(null);
                    c0037i.f251g.dismiss();
                }
            }
        }
        int i3 = this.f259e + 1;
        this.f259e = i3;
        if (i3 >= arrayList.size()) {
            this.f260f = true;
            this.f257c.sendDownUpKeyEvents(-1);
        } else {
            HandlerC0009g handlerC0009g = this.f261g;
            handlerC0009g.sendMessageDelayed(handlerC0009g.obtainMessage(0, arrayList.get(this.f259e)), 500L);
        }
    }

    @Override // android.view.View.OnTouchListener
    public final boolean onTouch(View view, MotionEvent motionEvent) {
        boolean z = !this.f260f;
        if (motionEvent.getAction() == 1 && this.f259e >= this.f255a.size()) {
            this.f256b.setOnTouchListener(null);
        }
        return z;
    }
}
