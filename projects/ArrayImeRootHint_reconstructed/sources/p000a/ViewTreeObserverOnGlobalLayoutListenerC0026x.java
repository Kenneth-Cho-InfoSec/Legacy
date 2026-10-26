package p000a;

import android.view.ViewTreeObserver;
import android.widget.PopupWindow;

/* JADX INFO: renamed from: a.x */
/* JADX INFO: loaded from: classes.dex */
public final class ViewTreeObserverOnGlobalLayoutListenerC0026x implements ViewTreeObserver.OnGlobalLayoutListener {

    /* JADX INFO: renamed from: a */
    public final /* synthetic */ ViewTreeObserver f181a;

    /* JADX INFO: renamed from: b */
    public final /* synthetic */ GestureDetectorOnGestureListenerC0001a0 f182b;

    public ViewTreeObserverOnGlobalLayoutListenerC0026x(GestureDetectorOnGestureListenerC0001a0 gestureDetectorOnGestureListenerC0001a0, ViewTreeObserver viewTreeObserver) {
        this.f182b = gestureDetectorOnGestureListenerC0001a0;
        this.f181a = viewTreeObserver;
    }

    @Override // android.view.ViewTreeObserver.OnGlobalLayoutListener
    public final void onGlobalLayout() {
        ViewTreeObserver viewTreeObserver = this.f181a;
        if (viewTreeObserver.isAlive()) {
            viewTreeObserver.removeOnGlobalLayoutListener(this);
            int[] iArr = new int[2];
            GestureDetectorOnGestureListenerC0001a0 gestureDetectorOnGestureListenerC0001a0 = this.f182b;
            gestureDetectorOnGestureListenerC0001a0.f34d.getLocationOnScreen(iArr);
            int i = (iArr[1] - gestureDetectorOnGestureListenerC0001a0.f69u0) - (gestureDetectorOnGestureListenerC0001a0.f67t0 + 2);
            if (gestureDetectorOnGestureListenerC0001a0.f65s0.isShowing()) {
                PopupWindow popupWindow = gestureDetectorOnGestureListenerC0001a0.f65s0;
                popupWindow.update(0, i, popupWindow.getWidth(), gestureDetectorOnGestureListenerC0001a0.f65s0.getHeight());
            } else {
                gestureDetectorOnGestureListenerC0001a0.f65s0.showAtLocation(gestureDetectorOnGestureListenerC0001a0.f34d, 53, 0, i);
            }
            gestureDetectorOnGestureListenerC0001a0.m8H();
        }
    }
}
