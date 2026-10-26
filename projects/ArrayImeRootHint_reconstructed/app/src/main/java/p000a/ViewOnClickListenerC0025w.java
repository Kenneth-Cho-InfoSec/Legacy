package p000a;

import android.view.View;
import android.widget.TextView;
import com.minn.android.hanziime.R;

/* JADX INFO: renamed from: a.w */
/* JADX INFO: loaded from: classes.dex */
public final class ViewOnClickListenerC0025w implements View.OnClickListener {

    /* JADX INFO: renamed from: a */
    public final /* synthetic */ int f179a;

    /* JADX INFO: renamed from: b */
    public final /* synthetic */ GestureDetectorOnGestureListenerC0001a0 f180b;

    public /* synthetic */ ViewOnClickListenerC0025w(GestureDetectorOnGestureListenerC0001a0 gestureDetectorOnGestureListenerC0001a0, int i) {
        this.f179a = i;
        this.f180b = gestureDetectorOnGestureListenerC0001a0;
    }

    @Override // android.view.View.OnClickListener
    public final void onClick(View view) {
        AbstractC0010h abstractC0010h = null;
        C0004c c0004c = null;
        AbstractC0010h abstractC0010h2;
        C0004c c0004c2;
        switch (this.f179a) {
            case 0:
                GestureDetectorOnGestureListenerC0001a0 gestureDetectorOnGestureListenerC0001a0 = this.f180b;
                gestureDetectorOnGestureListenerC0001a0.m26s();
                gestureDetectorOnGestureListenerC0001a0.f52m.mo55a(new C0004c(-268435453));
                gestureDetectorOnGestureListenerC0001a0.f52m.mo55a(new C0004c(-268435431));
                gestureDetectorOnGestureListenerC0001a0.m12e();
                break;
            case 1:
                GestureDetectorOnGestureListenerC0001a0 gestureDetectorOnGestureListenerC0001a02 = this.f180b;
                GestureDetectorOnGestureListenerC0001a0.m0a(gestureDetectorOnGestureListenerC0001a02, view, gestureDetectorOnGestureListenerC0001a02.f6E);
                break;
            case 2:
                GestureDetectorOnGestureListenerC0001a0 gestureDetectorOnGestureListenerC0001a03 = this.f180b;
                GestureDetectorOnGestureListenerC0001a0.m0a(gestureDetectorOnGestureListenerC0001a03, view, gestureDetectorOnGestureListenerC0001a03.f7F);
                break;
            case 3:
                if (view.isShown()) {
                    GestureDetectorOnGestureListenerC0001a0 gestureDetectorOnGestureListenerC0001a04 = this.f180b;
                    gestureDetectorOnGestureListenerC0001a04.m26s();
                    if (view instanceof TextView) {
                        int id = ((TextView) view).getId();
                        if (id != R.id.candview_emoticon) {
                            if (id == R.id.candview_symbol && gestureDetectorOnGestureListenerC0001a04.f37e0 != 1041) {
                                abstractC0010h = gestureDetectorOnGestureListenerC0001a04.f52m;
                                c0004c = new C0004c(-268435441, 1041);
                            }
                        } else if (gestureDetectorOnGestureListenerC0001a04.f37e0 != 1042) {
                            abstractC0010h = gestureDetectorOnGestureListenerC0001a04.f52m;
                            c0004c = new C0004c(-268435441, 1041);
                        }
                        if (abstractC0010h != null && c0004c != null) {
                            abstractC0010h.mo55a(c0004c);
                        }
                    }
                    break;
                }
                break;
            case 4:
                if (view.isShown()) {
                    GestureDetectorOnGestureListenerC0001a0 gestureDetectorOnGestureListenerC0001a05 = this.f180b;
                    gestureDetectorOnGestureListenerC0001a05.m26s();
                    gestureDetectorOnGestureListenerC0001a05.m7G();
                    if (gestureDetectorOnGestureListenerC0001a05.f12K) {
                        gestureDetectorOnGestureListenerC0001a05.f12K = false;
                        abstractC0010h2 = gestureDetectorOnGestureListenerC0001a05.f52m;
                        c0004c2 = new C0004c(-268435453);
                    } else {
                        gestureDetectorOnGestureListenerC0001a05.f12K = true;
                        abstractC0010h2 = gestureDetectorOnGestureListenerC0001a05.f52m;
                        c0004c2 = new C0004c(-268435452);
                    }
                    abstractC0010h2.mo55a(c0004c2);
                    break;
                }
                break;
            default:
                GestureDetectorOnGestureListenerC0001a0 gestureDetectorOnGestureListenerC0001a06 = this.f180b;
                gestureDetectorOnGestureListenerC0001a06.m26s();
                gestureDetectorOnGestureListenerC0001a06.m10c();
                gestureDetectorOnGestureListenerC0001a06.m28u(gestureDetectorOnGestureListenerC0001a06.f32c);
                gestureDetectorOnGestureListenerC0001a06.m12e();
                break;
        }
    }
}
