package p000a;

import android.view.View;

/* JADX INFO: renamed from: a.y */
/* JADX INFO: loaded from: classes.dex */
public final class ViewOnLongClickListenerC0027y implements View.OnLongClickListener {

    /* JADX INFO: renamed from: a */
    public final /* synthetic */ int f183a;

    /* JADX INFO: renamed from: b */
    public final /* synthetic */ GestureDetectorOnGestureListenerC0001a0 f184b;

    public /* synthetic */ ViewOnLongClickListenerC0027y(GestureDetectorOnGestureListenerC0001a0 gestureDetectorOnGestureListenerC0001a0, int i) {
        this.f183a = i;
        this.f184b = gestureDetectorOnGestureListenerC0001a0;
    }

    @Override // android.view.View.OnLongClickListener
    public final boolean onLongClick(View view) {
        switch (this.f183a) {
            case 0:
                GestureDetectorOnGestureListenerC0001a0 gestureDetectorOnGestureListenerC0001a0 = this.f184b;
                return gestureDetectorOnGestureListenerC0001a0.m24q(view, gestureDetectorOnGestureListenerC0001a0.f6E);
            default:
                GestureDetectorOnGestureListenerC0001a0 gestureDetectorOnGestureListenerC0001a02 = this.f184b;
                return gestureDetectorOnGestureListenerC0001a02.m24q(view, gestureDetectorOnGestureListenerC0001a02.f7F);
        }
    }
}
