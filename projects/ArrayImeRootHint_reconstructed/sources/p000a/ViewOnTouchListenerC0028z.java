package p000a;

import android.view.MotionEvent;
import android.view.View;
import com.minn.android.hanziime.R;
import p001b.C0037i;
import p001b.ViewOnTouchListenerC0038j;

/* JADX INFO: renamed from: a.z */
/* JADX INFO: loaded from: classes.dex */
public final class ViewOnTouchListenerC0028z implements View.OnTouchListener {

    /* JADX INFO: renamed from: a */
    public final /* synthetic */ int f185a;

    /* JADX INFO: renamed from: b */
    public final /* synthetic */ Object f186b;

    public /* synthetic */ ViewOnTouchListenerC0028z(Object obj, int i) {
        this.f185a = i;
        this.f186b = obj;
    }

    @Override // android.view.View.OnTouchListener
    public final boolean onTouch(View view, MotionEvent motionEvent) {
        switch (this.f185a) {
            case 0:
                int action = motionEvent.getAction();
                GestureDetectorOnGestureListenerC0001a0 gestureDetectorOnGestureListenerC0001a0 = (GestureDetectorOnGestureListenerC0001a0) this.f186b;
                int i = action != 0 ? action != 1 ? 0 : gestureDetectorOnGestureListenerC0001a0.f12K ? R.drawable.cand_up : R.drawable.cand_down : gestureDetectorOnGestureListenerC0001a0.f12K ? R.drawable.cand_up_press : R.drawable.cand_down_press;
                if (i != 0) {
                    gestureDetectorOnGestureListenerC0001a0.f44i.setImageResource(i);
                }
                return false;
            default:
                C0037i c0037i = (C0037i) this.f186b;
                boolean z = !c0037i.f254j.f260f;
                if (motionEvent.getAction() == 1) {
                    ViewOnTouchListenerC0038j viewOnTouchListenerC0038j = c0037i.f254j;
                    if (viewOnTouchListenerC0038j.f259e >= viewOnTouchListenerC0038j.f255a.size()) {
                        viewOnTouchListenerC0038j.f256b.setOnTouchListener(null);
                    } else {
                        viewOnTouchListenerC0038j.m102a();
                    }
                }
                return z;
        }
    }
}
