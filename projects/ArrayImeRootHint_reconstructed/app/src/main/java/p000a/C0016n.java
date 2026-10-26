package p000a;

import android.view.GestureDetector;
import android.view.MotionEvent;
import com.minn.android.hanziime.KeyboardView;

/* JADX INFO: renamed from: a.n */
/* JADX INFO: loaded from: classes.dex */
public final class C0016n extends GestureDetector.SimpleOnGestureListener {

    /* JADX INFO: renamed from: a */
    public final /* synthetic */ KeyboardView f165a;

    public C0016n(KeyboardView keyboardView) {
        this.f165a = keyboardView;
    }

    @Override // android.view.GestureDetector.SimpleOnGestureListener, android.view.GestureDetector.OnGestureListener
    public final boolean onFling(MotionEvent motionEvent, MotionEvent motionEvent2, float f, float f2) {
        int i;
        int i2;
        float[] fArr;
        KeyboardView keyboardView = this.f165a;
        int i3 = 0;
        if (keyboardView.f343j0) {
            return false;
        }
        float fAbs = Math.abs(f);
        float fAbs2 = Math.abs(f2);
        float x = motionEvent2.getX() - motionEvent.getX();
        float y = motionEvent2.getY() - motionEvent.getY();
        int width = keyboardView.getWidth() / 2;
        int height = keyboardView.getHeight() / 2;
        C0019q c0019q = keyboardView.f345k0;
        float[] fArr2 = c0019q.f167a;
        float f3 = fArr2[0];
        float[] fArr3 = c0019q.f168b;
        float f4 = fArr3[0];
        long[] jArr = c0019q.f169c;
        long j = jArr[0];
        while (i3 < 4 && jArr[i3] != 0) {
            i3++;
        }
        int i4 = 1;
        float f5 = 0.0f;
        float f6 = 0.0f;
        while (i4 < i3) {
            int i5 = width;
            int i6 = i3;
            int i7 = (int) (jArr[i4] - j);
            if (i7 == 0) {
                fArr = fArr2;
            } else {
                float f7 = i7;
                float f8 = (fArr2[i4] - f3) / f7;
                fArr = fArr2;
                float f9 = 1000;
                float f10 = f8 * f9;
                f5 = f5 == 0.0f ? f10 : (f5 + f10) * 0.5f;
                float f11 = ((fArr3[i4] - f4) / f7) * f9;
                f6 = f6 == 0.0f ? f11 : (f6 + f11) * 0.5f;
            }
            i4++;
            fArr2 = fArr;
            width = i5;
            i3 = i6;
        }
        int i8 = width;
        c0019q.f171e = f5 < 0.0f ? Math.max(f5, -3.4028235E38f) : Math.min(f5, Float.MAX_VALUE);
        c0019q.f170d = f6 < 0.0f ? Math.max(f6, -3.4028235E38f) : Math.min(f6, Float.MAX_VALUE);
        C0019q c0019q2 = keyboardView.f345k0;
        float f12 = c0019q2.f171e;
        float f13 = c0019q2.f170d;
        int i9 = keyboardView.f347l0;
        float f14 = i9;
        if (f > f14 && fAbs2 < fAbs) {
            i = i8;
            if (x > i) {
                if (!keyboardView.f349m0 || f12 >= f / 4.0f) {
                    keyboardView.f297A.getClass();
                    return true;
                }
            }
            keyboardView.m120c(keyboardView.f321U, keyboardView.f310J, keyboardView.f311K, motionEvent.getEventTime());
            return false;
        }
        i = i8;
        float f15 = -i9;
        if (f >= f15 || fAbs2 >= fAbs || x >= (-i)) {
            if (f2 >= f15 || fAbs >= fAbs2) {
                i2 = height;
            } else {
                i2 = height;
                if (y < (-i2)) {
                    if (!keyboardView.f349m0 || f13 <= f2 / 4.0f) {
                        keyboardView.f297A.getClass();
                        return true;
                    }
                }
            }
            if (f2 > f14 && fAbs < fAbs2 / 2.0f && y > i2) {
                if (!keyboardView.f349m0 || f13 >= f2 / 4.0f) {
                    keyboardView.f297A.getClass();
                    return true;
                }
            }
            return false;
        }
        if (!keyboardView.f349m0 || f12 <= f / 4.0f) {
            keyboardView.f297A.getClass();
            return true;
        }
        keyboardView.m120c(keyboardView.f321U, keyboardView.f310J, keyboardView.f311K, motionEvent.getEventTime());
        return false;
    }
}
