package p000a;

import android.os.Handler;
import android.os.Message;
import android.text.Layout;
import android.text.SpannableStringBuilder;
import android.text.StaticLayout;
import android.view.View;
import android.widget.PopupWindow;
import android.widget.TextView;
import com.minn.android.hanziime.HzimMainTWService;
import com.minn.android.hanziime.KeyboardView;
import java.util.ArrayList;
import p001b.C0030b;
import p001b.C0032d;
import p001b.C0033e;
import p001b.C0037i;
import p001b.ViewOnTouchListenerC0038j;

/* JADX INFO: renamed from: a.g */
/* JADX INFO: loaded from: classes.dex */
public final class HandlerC0009g extends Handler {

    /* JADX INFO: renamed from: a */
    public final /* synthetic */ int f101a;

    /* JADX INFO: renamed from: b */
    public final /* synthetic */ Object f102b;

    public /* synthetic */ HandlerC0009g(Object obj, int i) {
        this.f101a = i;
        this.f102b = obj;
    }

    /* JADX WARN: Removed duplicated region for block: B:102:0x01f1  */
    /* JADX WARN: Removed duplicated region for block: B:114:0x0219  */
    /* JADX WARN: Removed duplicated region for block: B:117:0x0224  */
    /* JADX WARN: Removed duplicated region for block: B:91:0x01ca  */
    @Override // android.os.Handler
    /*
        Code decompiled incorrectly, please refer to instructions dump.
    */
    public final void handleMessage(Message message) {
        int length;
        C0022t c0022t;
        ArrayList arrayList;
        C0030b c0030b;
        int i;
        int iMo45a = 0;
        iMo45a = 0;
        Object obj = this.f102b;
        int i2 = 1;
        switch (this.f101a) {
            case 0:
                int i3 = message.what;
                HzimMainTWService hzimMainTWService = (HzimMainTWService) obj;
                if (i3 == 0) {
                    char[] cArr = HzimMainTWService.f263J;
                    if (hzimMainTWService.f106c != null) {
                        C0033e c0033eM89b = hzimMainTWService.f281o.m89b();
                        hzimMainTWService.f106c = c0033eM89b;
                        if (!(hzimMainTWService.f281o.f240c == 2)) {
                            iMo45a = c0033eM89b.mo45a(hzimMainTWService.f107d);
                        } else if ((c0033eM89b instanceof C0033e) && c0033eM89b.f230b != null) {
                            String strM43j = hzimMainTWService.f107d.m43j(0);
                            C0032d c0032d = hzimMainTWService.f281o.f238a;
                            if (!c0032d.m81a() || (c0030b = c0032d.f228d[0]) == null) {
                                length = -1;
                                if (length < 0) {
                                    C0002b c0002b = hzimMainTWService.f107d;
                                    int size = length - 1;
                                    c0002b.getClass();
                                    try {
                                        arrayList = c0002b.f80a[0];
                                        if (size < 0) {
                                            size = arrayList.size() - 1;
                                        }
                                    } catch (Exception unused) {
                                    }
                                    c0022t = (size < arrayList.size() && size >= 0) ? (C0022t) arrayList.get(size) : null;
                                    hzimMainTWService.f107d.m35b();
                                } else {
                                    c0022t = null;
                                }
                                int iMo47c = hzimMainTWService.f106c.mo47c(hzimMainTWService.f107d);
                                if (c0022t != null) {
                                    C0002b c0002b2 = hzimMainTWService.f107d;
                                    int i4 = c0002b2.f81b[0];
                                    c0002b2.f80a[0].add(i4, c0022t);
                                    c0002b2.m40g(0, i4, 1, 0);
                                    c0002b2.m41h(0, i4 + 1);
                                }
                                iMo45a = iMo47c;
                            } else {
                                int length2 = strM43j.length();
                                int i5 = c0030b.f207c;
                                int i6 = length2 % i5;
                                int i7 = length2 / i5;
                                if (i6 == 1 && ((i7 == 1 || i7 == 2) && !strM43j.contains("*") && !strM43j.contains("?"))) {
                                    length = strM43j.length();
                                }
                                if (length < 0) {
                                }
                                int iMo47c2 = hzimMainTWService.f106c.mo47c(hzimMainTWService.f107d);
                                if (c0022t != null) {
                                }
                                iMo45a = iMo47c2;
                            }
                        }
                        int i8 = hzimMainTWService.f272D ? hzimMainTWService.f104a.f24W : -1;
                        if (iMo45a > 0) {
                            hzimMainTWService.f104a.m17j(hzimMainTWService.f106c);
                        } else if (hzimMainTWService.f276H.booleanValue() && hzimMainTWService.isInputViewShown()) {
                            hzimMainTWService.f285s.m66f("n");
                            hzimMainTWService.f285s.mo47c(hzimMainTWService.f107d);
                            hzimMainTWService.f104a.m17j(hzimMainTWService.f285s);
                            hzimMainTWService.f285s.m66f(null);
                        } else if (!hzimMainTWService.f276H.booleanValue() && hzimMainTWService.isInputViewShown()) {
                            hzimMainTWService.f104a.m9b();
                            hzimMainTWService.f104a.m6F(2);
                            hzimMainTWService.hideWindow();
                        }
                        if (hzimMainTWService.f272D) {
                            hzimMainTWService.f104a.f24W = i8;
                        }
                        GestureDetectorOnGestureListenerC0001a0 gestureDetectorOnGestureListenerC0001a0 = hzimMainTWService.f104a;
                        if (gestureDetectorOnGestureListenerC0001a0 instanceof GestureDetectorOnGestureListenerC0001a0) {
                            gestureDetectorOnGestureListenerC0001a0.m5E();
                        }
                        break;
                    } else if (hzimMainTWService.f276H.booleanValue() && hzimMainTWService.isInputViewShown()) {
                        hzimMainTWService.f285s.m66f("n");
                        hzimMainTWService.f285s.mo47c(hzimMainTWService.f107d);
                        hzimMainTWService.f104a.m17j(hzimMainTWService.f285s);
                        break;
                    } else if (!hzimMainTWService.f276H.booleanValue() && hzimMainTWService.isInputViewShown()) {
                        hzimMainTWService.f104a.m9b();
                        hzimMainTWService.f104a.m6F(2);
                        hzimMainTWService.hideWindow();
                        break;
                    }
                } else if (i3 == 1) {
                    if (hzimMainTWService.f273E == null) {
                        if (hzimMainTWService.isInputViewShown()) {
                            ViewOnTouchListenerC0038j viewOnTouchListenerC0038j = new ViewOnTouchListenerC0038j(hzimMainTWService, hzimMainTWService.f105b.f188b);
                            hzimMainTWService.f273E = viewOnTouchListenerC0038j;
                            int[] iArr = viewOnTouchListenerC0038j.f258d;
                            View view = viewOnTouchListenerC0038j.f256b;
                            view.getLocationInWindow(iArr);
                            viewOnTouchListenerC0038j.f259e = -1;
                            view.setOnTouchListener(viewOnTouchListenerC0038j);
                            viewOnTouchListenerC0038j.m102a();
                        } else {
                            sendMessageDelayed(obtainMessage(1), 100L);
                        }
                    }
                    break;
                } else if (i3 == 2) {
                    C0033e c0033e = hzimMainTWService.f283q;
                    break;
                }
                break;
            case 1:
                int i9 = message.what;
                KeyboardView keyboardView = (KeyboardView) obj;
                if (i9 == 1) {
                    int i10 = message.arg1;
                    int[] iArr2 = KeyboardView.f293E0;
                    keyboardView.m131o(i10);
                    break;
                } else if (i9 == 2) {
                    keyboardView.f344k.setVisibility(4);
                    break;
                } else if (i9 == 3) {
                    int[] iArr3 = KeyboardView.f293E0;
                    C0013k c0013k = keyboardView.f374z[keyboardView.f333e0];
                    keyboardView.m120c(keyboardView.f320T, c0013k.f130i, c0013k.f131j, keyboardView.f367v0);
                    sendMessageDelayed(Message.obtain(this, 3), 50L);
                    break;
                } else if (i9 == 4) {
                    if (keyboardView.f335f0 != 0 && (i = keyboardView.f320T) >= 0) {
                        C0013k[] c0013kArr = keyboardView.f374z;
                        if (i < c0013kArr.length && keyboardView.mo103j(c0013kArr[i])) {
                            keyboardView.f337g0 = true;
                            keyboardView.m132p(-1);
                        }
                    }
                    break;
                }
                break;
            case 2:
                int i11 = message.what;
                GestureDetectorOnGestureListenerC0001a0 gestureDetectorOnGestureListenerC0001a02 = (GestureDetectorOnGestureListenerC0001a0) obj;
                if (i11 == 0) {
                    gestureDetectorOnGestureListenerC0001a02.m23p(message.arg1, message.arg2 == 1);
                    break;
                } else if (i11 == 1) {
                    if (gestureDetectorOnGestureListenerC0001a02.f54n == 1) {
                        gestureDetectorOnGestureListenerC0001a02.getClass();
                    }
                    break;
                } else if (i11 == 2) {
                    while (true) {
                        C0005c0 c0005c0 = (C0005c0) gestureDetectorOnGestureListenerC0001a02.f8G.poll();
                        if (c0005c0 != null) {
                            gestureDetectorOnGestureListenerC0001a02.m28u(c0005c0);
                        }
                        break;
                    }
                }
                break;
            default:
                if (message.what == 0) {
                    C0037i c0037i = (C0037i) message.obj;
                    int[] iArr4 = ((ViewOnTouchListenerC0038j) obj).f258d;
                    int width = iArr4[0];
                    int i12 = iArr4[1];
                    PopupWindow popupWindow = c0037i.f251g;
                    SpannableStringBuilder spannableStringBuilder = c0037i.f250f;
                    TextView textView = c0037i.f252h;
                    int paddingRight = textView.getPaddingRight() + textView.getPaddingLeft();
                    int paddingBottom = textView.getPaddingBottom() + textView.getPaddingTop();
                    StaticLayout staticLayout = new StaticLayout(spannableStringBuilder, textView.getPaint(), c0037i.f248d - paddingRight, Layout.Alignment.ALIGN_NORMAL, 1.0f, 0.0f, true);
                    float fMax = 0.0f;
                    for (int i13 = 0; i13 < staticLayout.getLineCount(); i13++) {
                        fMax = Math.max(fMax, staticLayout.getLineWidth(i13));
                    }
                    popupWindow.setWidth(c0037i.f248d);
                    popupWindow.setHeight(staticLayout.getHeight() + paddingBottom);
                    int paddingTop = i12 - (c0037i.f252h.getPaddingTop() + staticLayout.getHeight());
                    if (c0037i.f253i.getVisibility() == 0 && c0037i.f253i.getWindowVisibility() == 0) {
                        try {
                            if ((c0037i.f249e & 80) == 80) {
                                paddingTop -= c0037i.f251g.getHeight();
                            }
                            if ((c0037i.f249e & 5) == 5) {
                                width -= c0037i.f251g.getWidth();
                            }
                            c0037i.f252h.setOnTouchListener(new ViewOnTouchListenerC0028z(c0037i, i2));
                            c0037i.f251g.showAtLocation(c0037i.f253i, 0, c0037i.f246b + width, c0037i.f247c + paddingTop);
                        } catch (Exception unused2) {
                            return;
                        }
                        break;
                    }
                }
                break;
        }
    }
}
