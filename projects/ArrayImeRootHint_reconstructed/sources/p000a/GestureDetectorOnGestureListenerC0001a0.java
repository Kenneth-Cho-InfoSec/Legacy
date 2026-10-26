package p000a;

import android.app.Dialog;
import android.content.res.Resources;
import android.graphics.Rect;
import android.graphics.drawable.Drawable;
import android.media.AudioManager;
import android.media.ToneGenerator;
import android.os.Build;
import android.os.Vibrator;
import android.text.SpannableString;
import android.text.TextUtils;
import android.text.style.ForegroundColorSpan;
import android.text.style.ImageSpan;
import android.text.style.RelativeSizeSpan;
import android.text.style.SubscriptSpan;
import android.text.style.SuperscriptSpan;
import android.text.style.TypefaceSpan;
import android.util.Log;
import android.view.GestureDetector;
import android.view.MotionEvent;
import android.view.View;
import android.view.ViewGroup;
import android.view.ViewTreeObserver;
import android.view.Window;
import android.view.WindowManager;
import android.widget.AbsoluteLayout;
import android.widget.ImageView;
import android.widget.LinearLayout;
import android.widget.PopupWindow;
import android.widget.ScrollView;
import android.widget.TextView;
import com.minn.android.hanziime.R;
import java.util.ArrayList;
import java.util.LinkedList;

/* JADX INFO: renamed from: a.a0 */
/* JADX INFO: loaded from: classes.dex */
public final class GestureDetectorOnGestureListenerC0001a0 implements GestureDetector.OnGestureListener {

    /* JADX INFO: renamed from: C */
    public int f4C;

    /* JADX INFO: renamed from: D */
    public int f5D;

    /* JADX INFO: renamed from: H */
    public GestureDetector f9H;

    /* JADX INFO: renamed from: N */
    public C0000a f15N;

    /* JADX INFO: renamed from: O */
    public int f16O;

    /* JADX INFO: renamed from: P */
    public int f17P;

    /* JADX INFO: renamed from: Q */
    public TextView f18Q;

    /* JADX INFO: renamed from: R */
    public AbsoluteLayout.LayoutParams f19R;

    /* JADX INFO: renamed from: S */
    public boolean f20S;

    /* JADX INFO: renamed from: T */
    public int f21T;

    /* JADX INFO: renamed from: c */
    public C0005c0 f32c;

    /* JADX INFO: renamed from: e */
    public TextView f36e;

    /* JADX INFO: renamed from: f */
    public TextView f38f;

    /* JADX INFO: renamed from: f0 */
    public float f39f0;

    /* JADX INFO: renamed from: g */
    public ScrollView f40g;

    /* JADX INFO: renamed from: g0 */
    public float f41g0;

    /* JADX INFO: renamed from: h */
    public ViewGroup f42h;

    /* JADX INFO: renamed from: i */
    public ImageView f44i;

    /* JADX INFO: renamed from: j */
    public LinearLayout f46j;

    /* JADX INFO: renamed from: k */
    public AbsoluteLayout f48k;

    /* JADX INFO: renamed from: l */
    public LinearLayout f50l;

    /* JADX INFO: renamed from: m */
    public AbstractC0010h f52m;

    /* JADX INFO: renamed from: m0 */
    public int f53m0;

    /* JADX INFO: renamed from: n */
    public int f54n;

    /* JADX INFO: renamed from: n0 */
    public LinearLayout f55n0;

    /* JADX INFO: renamed from: o */
    public boolean f56o;

    /* JADX INFO: renamed from: o0 */
    public PopupWindow f57o0;

    /* JADX INFO: renamed from: p */
    public int f58p;

    /* JADX INFO: renamed from: p0 */
    public TextView f59p0;

    /* JADX INFO: renamed from: q */
    public int f60q;

    /* JADX INFO: renamed from: q0 */
    public int f61q0;

    /* JADX INFO: renamed from: r */
    public int f62r;

    /* JADX INFO: renamed from: r0 */
    public LinearLayout f63r0;

    /* JADX INFO: renamed from: s */
    public int f64s;

    /* JADX INFO: renamed from: s0 */
    public PopupWindow f65s0;

    /* JADX INFO: renamed from: t */
    public int f66t;

    /* JADX INFO: renamed from: t0 */
    public int f67t0;

    /* JADX INFO: renamed from: u */
    public int f68u;

    /* JADX INFO: renamed from: u0 */
    public int f69u0;

    /* JADX INFO: renamed from: v */
    public int f70v;

    /* JADX INFO: renamed from: x */
    public InterfaceC0003b0 f74x;

    /* JADX INFO: renamed from: a */
    public View f28a = null;

    /* JADX INFO: renamed from: b */
    public Dialog f30b = null;

    /* JADX INFO: renamed from: d */
    public ViewGroup f34d = null;

    /* JADX INFO: renamed from: w */
    public boolean f72w = true;

    /* JADX INFO: renamed from: z */
    public Vibrator f78z = null;

    /* JADX INFO: renamed from: A */
    public AudioManager f1A = null;

    /* JADX INFO: renamed from: B */
    public ToneGenerator f3B = null;

    /* JADX INFO: renamed from: E */
    public final ArrayList f6E = new ArrayList();

    /* JADX INFO: renamed from: F */
    public final ArrayList f7F = new ArrayList();

    /* JADX INFO: renamed from: G */
    public final LinkedList f8G = new LinkedList();

    /* JADX INFO: renamed from: I */
    public int f10I = 0;

    /* JADX INFO: renamed from: J */
    public int f11J = 1;

    /* JADX INFO: renamed from: K */
    public boolean f12K = false;

    /* JADX INFO: renamed from: L */
    public boolean f13L = false;

    /* JADX INFO: renamed from: M */
    public int f14M = 0;

    /* JADX INFO: renamed from: U */
    public final ArrayList f22U = new ArrayList();

    /* JADX INFO: renamed from: V */
    public final ArrayList f23V = new ArrayList();

    /* JADX INFO: renamed from: W */
    public int f24W = -1;

    /* JADX INFO: renamed from: X */
    public TextView f25X = null;

    /* JADX INFO: renamed from: Y */
    public Drawable f26Y = null;

    /* JADX INFO: renamed from: Z */
    public int f27Z = 0;

    /* JADX INFO: renamed from: a0 */
    public boolean f29a0 = true;

    /* JADX INFO: renamed from: b0 */
    public int f31b0 = 2;

    /* JADX INFO: renamed from: c0 */
    public int f33c0 = 1;

    /* JADX INFO: renamed from: d0 */
    public int f35d0 = 0;

    /* JADX INFO: renamed from: e0 */
    public final int f37e0 = 1041;

    /* JADX INFO: renamed from: h0 */
    public final boolean f43h0 = true;

    /* JADX INFO: renamed from: i0 */
    public final boolean f45i0 = true;

    /* JADX INFO: renamed from: j0 */
    public final boolean f47j0 = true;

    /* JADX INFO: renamed from: k0 */
    public boolean f49k0 = true;

    /* JADX INFO: renamed from: l0 */
    public TextView f51l0 = null;

    /* JADX INFO: renamed from: v0 */
    public final HandlerC0009g f71v0 = new HandlerC0009g(this, 2);

    /* JADX INFO: renamed from: w0 */
    public final ViewOnClickListenerC0025w f73w0 = new ViewOnClickListenerC0025w(this, 1);

    /* JADX INFO: renamed from: x0 */
    public final ViewOnClickListenerC0025w f75x0 = new ViewOnClickListenerC0025w(this, 2);

    /* JADX INFO: renamed from: y0 */
    public final ViewOnLongClickListenerC0027y f77y0 = new ViewOnLongClickListenerC0027y(this, 0);

    /* JADX INFO: renamed from: z0 */
    public final ViewOnLongClickListenerC0027y f79z0 = new ViewOnLongClickListenerC0027y(this, 1);

    /* JADX INFO: renamed from: A0 */
    public final ViewOnClickListenerC0025w f2A0 = new ViewOnClickListenerC0025w(this, 3);

    /* JADX INFO: renamed from: y */
    public final int f76y = -1;

    /* JADX INFO: renamed from: a */
    public static void m0a(GestureDetectorOnGestureListenerC0001a0 gestureDetectorOnGestureListenerC0001a0, View view, ArrayList arrayList) {
        gestureDetectorOnGestureListenerC0001a0.getClass();
        if (view.isShown()) {
            gestureDetectorOnGestureListenerC0001a0.m26s();
            if (view instanceof TextView) {
                int id = ((TextView) view).getId();
                C0005c0 c0005c0 = (C0005c0) arrayList.get(id);
                gestureDetectorOnGestureListenerC0001a0.f53m0 = id;
                if (gestureDetectorOnGestureListenerC0001a0.f71v0.hasMessages(1)) {
                    gestureDetectorOnGestureListenerC0001a0.f8G.add(c0005c0);
                } else {
                    gestureDetectorOnGestureListenerC0001a0.m10c();
                    gestureDetectorOnGestureListenerC0001a0.m28u(c0005c0);
                }
            }
        }
    }

    /* JADX INFO: renamed from: A */
    public final void m1A() {
        if (this.f40g.fullScroll(130)) {
            return;
        }
        this.f40g.scrollTo(0, 0);
    }

    /* JADX INFO: renamed from: B */
    public final void m2B() {
        if (this.f40g.fullScroll(33)) {
            return;
        }
        ScrollView scrollView = this.f40g;
        scrollView.scrollTo(0, scrollView.getChildAt(0).getHeight());
    }

    /* JADX INFO: renamed from: C */
    public final void m3C() {
        if (this.f40g.pageScroll(33)) {
            return;
        }
        ScrollView scrollView = this.f40g;
        scrollView.scrollTo(0, scrollView.getChildAt(0).getHeight());
    }

    /* JADX INFO: renamed from: D */
    public final boolean m4D(int i) {
        int i2 = this.f56o ? this.f31b0 : this.f33c0;
        if (this.f54n == 1 && i == 0) {
            m10c();
        }
        this.f54n = i;
        if (i == 0) {
            this.f40g.setLayoutParams(new LinearLayout.LayoutParams(-1, this.f62r * i2));
            this.f40g.scrollTo(0, 0);
            this.f50l.setVisibility(8);
            this.f46j.setVisibility(0);
            this.f48k.setVisibility(8);
            this.f42h.setMinimumHeight(-1);
            return false;
        }
        if (i == 2) {
            this.f40g.setLayoutParams(new LinearLayout.LayoutParams(-1, this.f62r * i2));
            this.f50l.setVisibility(8);
            this.f42h.setMinimumHeight(-1);
            this.f71v0.removeMessages(1);
            return false;
        }
        AbstractC0010h abstractC0010h = AbstractC0010h.f103k;
        int i3 = this.f56o ? this.f31b0 : this.f33c0;
        Drawable drawable = this.f52m.getResources().getDrawable(R.drawable.keyboard_background);
        Rect rect = new Rect(0, 0, 0, 0);
        drawable.getPadding(rect);
        this.f40g.setLayoutParams(new LinearLayout.LayoutParams(-1, !this.f43h0 ? ((i3 * this.f62r) + this.f68u) - this.f70v : (i3 * this.f62r) + this.f68u + rect.top + rect.bottom));
        this.f50l.setVisibility(8);
        this.f48k.setVisibility(0);
        this.f42h.setMinimumHeight(-1);
        return true;
    }

    /* JADX INFO: renamed from: E */
    public final void m5E() {
        TextView textView;
        TextView textView2 = this.f25X;
        if (textView2 != null) {
            textView2.setBackgroundDrawable(this.f26Y);
            textView2.setPadding(0, 0, 0, 0);
        }
        int i = this.f24W;
        if (i == -1) {
            textView = null;
        } else {
            textView = (TextView) (this.f29a0 ? this.f22U : this.f23V).get(i);
        }
        this.f25X = textView;
        if (textView != null) {
            this.f26Y = textView.getBackground();
            textView.setBackgroundResource(R.drawable.cand_back_focuse);
            int[] iArr = new int[2];
            this.f40g.getLocationOnScreen(iArr);
            int i2 = iArr[1];
            int height = this.f40g.getHeight() + i2;
            int[] iArr2 = new int[2];
            textView.getLocationOnScreen(iArr2);
            int i3 = iArr2[1];
            int height2 = textView.getHeight() + i3;
            if (height2 > height) {
                this.f40g.scrollBy(0, height2 - height);
            } else if (i3 < i2) {
                this.f40g.scrollBy(0, i3 - i2);
            }
        }
    }

    /* JADX INFO: renamed from: F */
    public final void m6F(int i) {
        if (m4D(i)) {
            m18k(this.f74x, false, -1);
            return;
        }
        if (i == 0) {
            this.f12K = false;
            m32y();
        } else if (this.f34d.isShown()) {
            this.f52m.setCandidatesViewShown(false);
        }
    }

    /* JADX INFO: renamed from: G */
    public final void m7G() {
        PopupWindow popupWindow = this.f65s0;
        if (popupWindow == null) {
            return;
        }
        if (Build.VERSION.SDK_INT >= 28) {
            popupWindow.setWindowLayoutType(2038);
            ViewTreeObserver viewTreeObserver = this.f34d.getViewTreeObserver();
            viewTreeObserver.addOnGlobalLayoutListener(new ViewTreeObserverOnGlobalLayoutListenerC0026x(this, viewTreeObserver));
        } else {
            int i = (this.f67t0 + 2) * (-1);
            if (popupWindow.isShowing()) {
                PopupWindow popupWindow2 = this.f65s0;
                popupWindow2.update(0, i, popupWindow2.getWidth(), this.f65s0.getHeight());
            } else {
                this.f65s0.showAtLocation(this.f34d, 53, 0, i);
            }
            m8H();
        }
    }

    /* JADX INFO: renamed from: H */
    public final void m8H() {
        int i;
        PopupWindow popupWindow = this.f57o0;
        if (popupWindow == null) {
            return;
        }
        if (Build.VERSION.SDK_INT >= 28) {
            popupWindow.setWindowLayoutType(2038);
            int[] iArr = new int[2];
            this.f34d.getLocationOnScreen(iArr);
            i = (iArr[1] - this.f69u0) - (this.f61q0 - 10);
        } else {
            i = (this.f61q0 - 10) * (-1);
        }
        if (!this.f57o0.isShowing()) {
            this.f57o0.showAtLocation(this.f34d, 51, 0, i);
        } else {
            PopupWindow popupWindow2 = this.f57o0;
            popupWindow2.update(0, i, popupWindow2.getWidth(), this.f57o0.getHeight());
        }
    }

    /* JADX INFO: renamed from: b */
    public final void m9b() {
        m12e();
        m10c();
        m11d();
        AbsoluteLayout absoluteLayout = this.f48k;
        int childCount = absoluteLayout.getChildCount();
        for (int i = 0; i < childCount; i++) {
            absoluteLayout.getChildAt(i).setVisibility(8);
        }
        this.f11J = 1;
        this.f4C = 0;
        this.f5D = 0;
        this.f6E.clear();
        this.f7F.clear();
        this.f22U.clear();
        this.f23V.clear();
        this.f10I = 0;
        this.f35d0 = 0;
        this.f12K = false;
        this.f52m.mo55a(new C0004c(-268435453));
        if (this.f72w) {
            m4D(2);
        }
        if (this.f72w && this.f34d.isShown()) {
            this.f52m.setCandidatesViewShown(false);
        }
        this.f13L = false;
        m32y();
    }

    /* JADX INFO: renamed from: c */
    public final void m10c() {
        TextView textView = this.f25X;
        if (textView != null) {
            textView.setBackgroundDrawable(this.f26Y);
            textView.setPadding(0, 0, 0, 0);
            this.f25X = null;
        }
        this.f27Z = 0;
        this.f29a0 = true;
        this.f24W = -1;
        this.f71v0.removeMessages(0);
        this.f52m.mo55a(new C0004c(-268427263));
    }

    /* JADX INFO: renamed from: d */
    public final void m11d() {
        LinearLayout linearLayout = this.f46j;
        int childCount = linearLayout.getChildCount();
        for (int i = 0; i < childCount; i++) {
            LinearLayout linearLayout2 = (LinearLayout) linearLayout.getChildAt(i);
            int childCount2 = linearLayout2.getChildCount();
            for (int i2 = 0; i2 < childCount2; i2++) {
                linearLayout2.getChildAt(i2).setVisibility(8);
            }
        }
    }

    /* JADX INFO: renamed from: e */
    public final void m12e() {
        ViewGroup viewGroup;
        Dialog dialog = this.f30b;
        if (dialog != null) {
            dialog.dismiss();
            this.f30b = null;
            View view = this.f28a;
            if (view == null || (viewGroup = (ViewGroup) view.getParent()) == null) {
                return;
            }
            viewGroup.removeView(this.f28a);
        }
    }

    /* JADX INFO: renamed from: f */
    public final C0000a m13f() {
        C0000a c0000a = new C0000a(this.f40g.getContext());
        c0000a.setTextSize(0, this.f39f0);
        c0000a.setBackgroundResource(R.drawable.cand_back);
        c0000a.setCompoundDrawablePadding(0);
        c0000a.setGravity(17);
        c0000a.setSingleLine();
        c0000a.setPadding(0, 0, 0, 0);
        c0000a.setLayoutParams(new LinearLayout.LayoutParams(-2, -2, 1.0f));
        c0000a.setMinHeight(this.f62r);
        c0000a.setMinimumWidth(this.f60q);
        c0000a.setSoundEffectsEnabled(false);
        return c0000a;
    }

    /* JADX INFO: renamed from: g */
    public final void m14g(boolean z) {
        int i;
        int i2;
        boolean z2 = this.f12K;
        int i3 = this.f11J;
        if (!z2) {
            if ((this.f56o ? this.f31b0 : this.f33c0) >= i3) {
                m15h();
                return;
            }
        }
        this.f17P = 0;
        TextView textView = this.f18Q;
        if (textView != null) {
            textView.getId();
        }
        if (z) {
            i = this.f35d0;
            i2 = this.f64s;
        } else {
            i = this.f35d0;
            i2 = this.f62r;
        }
        this.f35d0 = i + i2;
        this.f11J++;
    }

    /* JADX INFO: renamed from: h */
    public final void m15h() {
        float f;
        LinearLayout linearLayout = (LinearLayout) this.f46j.getChildAt(this.f11J - 1);
        if (this.f10I < this.f66t) {
            if (this.f11J == 1) {
                this.f15N.setVisibility(8);
            }
            f = 0.0f;
        } else {
            this.f15N.setVisibility(8);
            f = 1.0f;
        }
        LinearLayout.LayoutParams layoutParams = new LinearLayout.LayoutParams(-2, -2, f);
        int childCount = linearLayout.getChildCount();
        for (int i = 0; i < childCount; i++) {
            View childAt = linearLayout.getChildAt(i);
            if (childAt != this.f15N) {
                childAt.setLayoutParams(layoutParams);
                childAt.setPadding(0, 0, 0, 0);
            }
        }
        this.f10I = 0;
        this.f21T = 0;
        this.f11J++;
    }

    /* JADX INFO: renamed from: i */
    public final void m16i() {
        PopupWindow popupWindow = this.f57o0;
        if (popupWindow != null && popupWindow.isShowing()) {
            this.f57o0.dismiss();
        }
    }

    /* JADX INFO: renamed from: j */
    public final void m17j(InterfaceC0003b0 interfaceC0003b0) {
        this.f71v0.removeMessages(1);
        this.f13L = false;
        this.f12K = false;
        this.f16O = 0;
        this.f17P = 0;
        this.f18Q = null;
        this.f20S = false;
        this.f21T = 0;
        m9b();
        this.f74x = interfaceC0003b0;
        this.f52m.mo55a(new C0004c(-268435453));
        this.f15N.setVisibility(0);
        this.f15N.setBackgroundResource(R.drawable.cand_back);
        m18k(interfaceC0003b0, true, this.f56o ? this.f31b0 : this.f33c0);
    }

    /* JADX WARN: Code restructure failed: missing block: B:40:0x005e, code lost:
    
        if (r12.f20S != false) goto L66;
     */
    /* JADX WARN: Code restructure failed: missing block: B:41:0x0060, code lost:
    
        m14g(false);
        r12.f20S = true;
        r1.removeMessages(1);
        r1.sendMessage(r1.obtainMessage(2));
     */
    /* JADX WARN: Removed duplicated region for block: B:16:0x002a  */
    /* JADX INFO: renamed from: k */
    /*
        Code decompiled incorrectly, please refer to instructions dump.
    */
    public final void m18k(InterfaceC0003b0 interfaceC0003b0, boolean z, int i) {
        boolean z2;
        String str;
        int i2;
        if (interfaceC0003b0 == null) {
            return;
        }
        boolean z3 = interfaceC0003b0 instanceof C0023u;
        HandlerC0009g handlerC0009g = this.f71v0;
        int i3 = this.f76y;
        if (!z3) {
            z2 = false;
        } else if (z) {
            z2 = false;
            if (this.f37e0 != 1042) {
                i3 = -1;
            }
        } else {
            if (i == -1) {
                i = this.f11J + 7;
            } else {
                if (i == 1) {
                    i = this.f11J + 1;
                }
                z2 = false;
                if (this.f37e0 != 1042) {
                }
            }
            handlerC0009g.sendEmptyMessageDelayed(1, 50L);
            z2 = true;
            if (this.f37e0 != 1042) {
            }
        }
        String str2 = null;
        C0005c0 c0005c0Mo46b = null;
        while (true) {
            if (i3 != -1 && this.f4C + this.f5D >= i3) {
                break;
            }
            for (int i4 = 0; i4 < 1000 && (c0005c0Mo46b = interfaceC0003b0.mo46b()) != null && !z3 && str2 != null && str2.equals(c0005c0Mo46b.f87a) && !this.f47j0; i4++) {
            }
            if (c0005c0Mo46b == null) {
                break;
            }
            str2 = c0005c0Mo46b.f87a;
            if (z3 && str2.length() > 3 && c0005c0Mo46b.f87a.charAt(0) == '[') {
                if (this.f4C + this.f5D != 0) {
                    m14g(false);
                }
                String str3 = c0005c0Mo46b.f87a;
                Resources resources = this.f52m.getResources();
                if (str3.equals(resources.getString(R.string.half_symbol_categories_txt))) {
                    i2 = R.string.half_symbol_txt;
                } else if (str3.equals(resources.getString(R.string.full_symbol_categories_txt))) {
                    i2 = R.string.full_symbol_txt;
                } else {
                    str = new String("");
                    c0005c0Mo46b.f87a = str;
                    m29v(true, c0005c0Mo46b);
                    m14g(true);
                }
                str = resources.getString(i2);
                c0005c0Mo46b.f87a = str;
                m29v(true, c0005c0Mo46b);
                m14g(true);
            } else {
                m29v(false, c0005c0Mo46b);
                if (z || z2) {
                    if (i < this.f11J) {
                        this.f13L = true;
                        break;
                    }
                }
            }
        }
        if (this.f4C + this.f5D < 1) {
            if (this.f72w) {
                this.f52m.setCandidatesViewShown(false);
                return;
            } else {
                this.f13L = false;
                this.f12K = false;
                m4D(0);
            }
        }
        m32y();
        if (this.f34d.isShown()) {
            return;
        }
        this.f52m.setCandidatesViewShown(true);
    }

    /* JADX INFO: renamed from: l */
    public final void m19l(View view, C0005c0 c0005c0) {
        if (!(view instanceof C0000a) || this.f28a == null) {
            return;
        }
        m12e();
        this.f30b = new Dialog(view.getContext(), R.style.Dialog);
        ((TextView) this.f28a.findViewById(R.id.candidate_longpress_dialog_text)).setText(c0005c0.f87a);
        this.f30b.setContentView(this.f28a);
        C0000a c0000a = (C0000a) view;
        Dialog dialog = this.f30b;
        Dialog dialog2 = c0000a.f0a;
        if (dialog2 != null) {
            dialog2.dismiss();
            c0000a.f0a = null;
        }
        c0000a.f0a = dialog;
        Window window = dialog.getWindow();
        WindowManager.LayoutParams attributes = window.getAttributes();
        attributes.token = c0000a.getWindowToken();
        attributes.type = 1003;
        window.setAttributes(attributes);
        window.addFlags(131072);
        c0000a.f0a.show();
    }

    /* JADX INFO: renamed from: m */
    public final int m20m(boolean z) {
        Resources resources = this.f52m.getResources();
        if (!this.f56o) {
            return z ? resources.getDimensionPixelSize(R.dimen.cand_space_width_emoji_symbol) : resources.getDimensionPixelSize(R.dimen.cand_space_width);
        }
        if (z) {
            return 0;
        }
        return resources.getDimensionPixelSize(R.dimen.cand_space_width);
    }

    /* JADX INFO: renamed from: n */
    public final int m21n() {
        return ((!this.f47j0 || this.f49k0) && (this.f56o || !this.f49k0)) ? 5 : 10;
    }

    /* JADX INFO: renamed from: o */
    public final int m22o() {
        return this.f53m0;
    }

    @Override // android.view.GestureDetector.OnGestureListener
    public final boolean onDown(MotionEvent motionEvent) {
        return false;
    }

    @Override // android.view.GestureDetector.OnGestureListener
    public final boolean onFling(MotionEvent motionEvent, MotionEvent motionEvent2, float f, float f2) {
        AbstractC0010h abstractC0010h;
        C0004c c0004c;
        if (motionEvent2 == null || motionEvent == null || motionEvent2.getY() >= motionEvent.getY()) {
            if (this.f40g.getScrollY() == 0) {
                Vibrator vibrator = this.f78z;
                if (vibrator != null) {
                    try {
                        vibrator.vibrate(5L);
                    } catch (Exception e) {
                        Log.e("iwnn", "TextCandidatesViewManager::onFling Sound " + e.toString());
                    }
                }
                this.f12K = false;
                abstractC0010h = this.f52m;
                c0004c = new C0004c(-268435453);
                abstractC0010h.mo55a(c0004c);
                return true;
            }
            return false;
        }
        if (this.f54n == 0 && this.f13L) {
            Vibrator vibrator2 = this.f78z;
            if (vibrator2 != null) {
                try {
                    vibrator2.vibrate(5L);
                } catch (Exception e2) {
                    Log.e("iwnn", "TextCandidatesViewManager::onFling Vibrator " + e2.toString());
                }
            }
            this.f12K = true;
            abstractC0010h = this.f52m;
            c0004c = new C0004c(-268435452);
            abstractC0010h.mo55a(c0004c);
            return true;
        }
        return false;
    }

    @Override // android.view.GestureDetector.OnGestureListener
    public final boolean onScroll(MotionEvent motionEvent, MotionEvent motionEvent2, float f, float f2) {
        return false;
    }

    @Override // android.view.GestureDetector.OnGestureListener
    public final boolean onSingleTapUp(MotionEvent motionEvent) {
        return false;
    }

    /* JADX WARN: Multi-variable type inference failed */
    /* JADX WARN: Type inference failed for: r15v1 */
    /* JADX WARN: Type inference failed for: r15v11 */
    /* JADX WARN: Type inference failed for: r15v12 */
    /* JADX WARN: Type inference failed for: r15v13 */
    /* JADX WARN: Type inference failed for: r15v14 */
    /* JADX WARN: Type inference failed for: r15v15 */
    /* JADX WARN: Type inference failed for: r15v2, types: [int] */
    /* JADX WARN: Type inference failed for: r15v3 */
    /* JADX WARN: Type inference failed for: r2v1, types: [a.g, android.os.Handler] */
    /* JADX WARN: Type inference fix 'apply assigned field type' failed
    java.lang.UnsupportedOperationException: ArgType.getObject(), call class: class jadx.core.dex.instructions.args.ArgType$PrimitiveArg
    	at jadx.core.dex.instructions.args.ArgType.getObject(ArgType.java:593)
    	at jadx.core.dex.attributes.nodes.ClassTypeVarsAttr.getTypeVarsMapFor(ClassTypeVarsAttr.java:35)
    	at jadx.core.dex.nodes.utils.TypeUtils.replaceClassGenerics(TypeUtils.java:177)
    	at jadx.core.dex.visitors.typeinference.FixTypesVisitor.insertExplicitUseCast(FixTypesVisitor.java:397)
    	at jadx.core.dex.visitors.typeinference.FixTypesVisitor.tryFieldTypeWithNewCasts(FixTypesVisitor.java:359)
    	at jadx.core.dex.visitors.typeinference.FixTypesVisitor.applyFieldType(FixTypesVisitor.java:309)
    	at jadx.core.dex.visitors.typeinference.FixTypesVisitor.visit(FixTypesVisitor.java:94)
     */
    /* JADX INFO: renamed from: p */
    public final void m23p(int i, boolean z) {
        TextView textView;
        int left;
        int i2;
        boolean z2 = this.f24W == -1;
        if (i == 0) {
            m5E();
        }
        ArrayList arrayList = this.f22U;
        int size = arrayList.size();
        if (this.f29a0 && size == 0) {
            this.f29a0 = false;
        }
        boolean z3 = this.f29a0;
        ArrayList arrayList2 = this.f23V;
        if (!z3) {
            arrayList = arrayList2;
        }
        int size2 = arrayList.size();
        int i3 = this.f24W;
        int size3 = i3 == -1 ? 0 : i3 + i;
        boolean z4 = false;
        while (size3 >= 0 && size3 < size2) {
            TextView textView2 = (TextView) arrayList.get(size3);
            if (!textView2.isShown()) {
                break;
            }
            if (!z || ((left = textView2.getLeft()) <= (i2 = this.f27Z) && i2 < textView2.getRight())) {
                break;
            }
            if (left == 0) {
                z4 = true;
            }
            size3 += i;
        }
        size3 = -1;
        if (size3 < 0 && z4 && !this.f29a0 && i > 0) {
            size3 = arrayList2.size() - 1;
        }
        if (size3 >= 0) {
            this.f24W = size3;
            m5E();
            if (!z) {
                int i4 = this.f24W;
                if (i4 == -1) {
                    textView = null;
                } else {
                    textView = (TextView) (this.f29a0 ? this.f22U : this.f23V).get(i4);
                }
                this.f27Z = textView.getLeft();
            }
        } else {
            ?? r15 = z;
            r15 = z;
            if (this.f13L && size > 0) {
                boolean z5 = this.f29a0;
                if ((z5 && i < 0) || (!z5 && i > 0)) {
                    z = false;
                }
                boolean z6 = !z5;
                this.f29a0 = z6;
                r15 = z;
                if (!z6) {
                    r15 = z;
                    if (!this.f12K) {
                        this.f12K = true;
                        this.f52m.mo55a(new C0004c(-268435452));
                        r15 = z;
                    }
                }
            }
            if (size == 0) {
                r15 = 0;
            }
            if (i > 0) {
                this.f24W = -1;
            } else {
                this.f24W = this.f29a0 ? size - 1 : arrayList2.size() - 1;
            }
            ?? r2 = this.f71v0;
            r2.sendMessage(r2.obtainMessage(0, i, r15));
        }
        if (z2) {
            this.f52m.mo55a(new C0004c(-268427264));
        }
    }

    /* JADX INFO: renamed from: q */
    public final boolean m24q(View view, ArrayList arrayList) {
        if (this.f28a == null || !this.f45i0) {
            return false;
        }
        if (!view.isShown()) {
            return true;
        }
        Drawable background = view.getBackground();
        if (background != null && background.getState().length == 0) {
            return true;
        }
        this.f32c = (C0005c0) arrayList.get(((TextView) view).getId());
        m10c();
        m19l(view, this.f32c);
        return true;
    }

    /* JADX INFO: renamed from: r */
    public final boolean m25r() {
        return this.f9H.onTouchEvent(null);
    }

    /* JADX INFO: renamed from: s */
    public final void m26s() {
        Vibrator vibrator = this.f78z;
        if (vibrator != null) {
            try {
                vibrator.vibrate(5L);
            } catch (Exception e) {
                Log.e("HzimService", "TextCandidatesViewManager::selectCandidate Vibrator " + e.toString());
            }
        }
        AudioManager audioManager = this.f1A;
        if (audioManager != null) {
            try {
                audioManager.playSoundEffect(5, -1.0f);
            } catch (Exception e2) {
                Log.e("HzimService", "TextCandidatesViewManager::selectCandidate Sound " + e2.toString());
            }
        }
    }

    /* JADX INFO: renamed from: t */
    public final void m27t(int i) {
        AbstractC0010h abstractC0010h;
        C0004c c0004c;
        if (this.f34d.isShown()) {
            if (i == 62) {
                if (this.f12K) {
                    this.f12K = false;
                    abstractC0010h = this.f52m;
                    c0004c = new C0004c(-268435453);
                } else {
                    this.f12K = true;
                    abstractC0010h = this.f52m;
                    c0004c = new C0004c(-268435452);
                }
                abstractC0010h.mo55a(c0004c);
                return;
            }
            switch (i) {
                case AbstractC0021s.HzimConfig_InputMethod_rootCodeDictionaryPath /* 19 */:
                    m23p(-1, true);
                    break;
                case AbstractC0021s.HzimConfig_InputMethod_rootDictionaryPath /* 20 */:
                    m23p(1, true);
                    break;
                case AbstractC0021s.HzimConfig_InputMethod_selectKeyShift /* 21 */:
                    m23p(-1, false);
                    break;
                case AbstractC0021s.HzimConfig_InputMethod_spaceAutoUp /* 22 */:
                    m23p(1, false);
                    break;
            }
        }
    }

    /* JADX INFO: renamed from: u */
    public final void m28u(C0005c0 c0005c0) {
        this.f12K = false;
        this.f52m.mo55a(new C0004c(-268435453));
        this.f52m.mo55a(new C0004c(c0005c0));
    }

    /* JADX WARN: Removed duplicated region for block: B:15:0x003b  */
    /* JADX INFO: renamed from: v */
    /*
        Code decompiled incorrectly, please refer to instructions dump.
    */
    public final void m29v(boolean z, C0005c0 c0005c0) {
        boolean z2;
        TextView textViewM13f;
        ViewOnLongClickListenerC0027y viewOnLongClickListenerC0027y;
        ArrayList arrayList;
        String str = c0005c0.f87a;
        int length = str.length();
        int iMeasureText = length < 3 ? this.f60q : (int) this.f15N.getPaint().measureText((CharSequence) str, 0, length);
        C0000a c0000a = this.f15N;
        int paddingRight = c0000a.getPaddingRight() + c0000a.getPaddingLeft() + iMeasureText;
        int i = this.f58p;
        boolean z3 = this.f12K;
        int i2 = this.f11J;
        if (z3) {
            z2 = true;
        } else {
            if ((this.f56o ? this.f31b0 : this.f33c0) >= i2) {
                z2 = false;
            }
        }
        this.f49k0 = z2;
        boolean z4 = this.f47j0;
        ImageSpan imageSpan = null;
        if (z2) {
            int iM21n = m21n();
            int i3 = this.f58p / iM21n;
            int iMin = Math.min((m20m(z4) + (paddingRight + i3)) / i3, iM21n);
            if (z) {
                iMin = iM21n;
            }
            int i4 = this.f17P;
            if (iM21n < i4 + iMin) {
                if (iM21n != i4) {
                    AbsoluteLayout.LayoutParams layoutParams = this.f19R;
                    ((ViewGroup.LayoutParams) layoutParams).width = ((iM21n - i4) * i3) + ((ViewGroup.LayoutParams) layoutParams).width;
                    TextView textView = this.f18Q;
                    if (textView != null) {
                        this.f48k.updateViewLayout(textView, layoutParams);
                    }
                }
                this.f17P = 0;
                TextView textView2 = this.f18Q;
                if (textView2 != null) {
                    textView2.getId();
                }
                this.f11J++;
                this.f35d0 += z ? this.f64s : this.f62r;
            }
            if (this.f16O == 0) {
                this.f35d0 = 0;
            }
            AbsoluteLayout absoluteLayout = this.f48k;
            int i5 = i3 * iMin;
            int i6 = this.f62r;
            if (z) {
                i6 = this.f64s;
            }
            AbsoluteLayout.LayoutParams layoutParams2 = new AbsoluteLayout.LayoutParams(i5, i6, (this.f58p / m21n()) * this.f17P, this.f35d0);
            textViewM13f = (TextView) absoluteLayout.getChildAt(this.f16O);
            if (textViewM13f == null) {
                textViewM13f = m13f();
                textViewM13f.setLayoutParams(layoutParams2);
                this.f48k.addView(textViewM13f);
            } else {
                this.f48k.updateViewLayout(textViewM13f, layoutParams2);
            }
            this.f17P += iMin;
            this.f16O++;
            this.f18Q = textViewM13f;
            this.f19R = layoutParams2;
        } else {
            paddingRight = Math.max(paddingRight, this.f58p / m21n());
            int iM20m = this.f10I + paddingRight;
            if (i < iM20m && this.f4C + this.f5D != 0) {
                m15h();
                if ((this.f56o ? this.f31b0 : this.f33c0) < this.f11J) {
                    this.f10I = 0;
                    m29v(z, c0005c0);
                    return;
                } else {
                    this.f10I = paddingRight;
                    iM20m = m20m(z4) + paddingRight;
                }
            }
            this.f10I = iM20m;
            textViewM13f = (TextView) ((LinearLayout) this.f46j.getChildAt(this.f11J - 1)).getChildAt(this.f21T);
            if (z) {
                if (this.f11J == 1) {
                    this.f15N.setBackgroundDrawable(null);
                }
                this.f10I += this.f66t;
            } else {
                m20m(z4);
            }
            this.f21T++;
        }
        int i7 = this.f49k0 ? this.f5D : this.f4C;
        int i8 = i7 % 10;
        SpannableString spannableString = new SpannableString(String.valueOf(i8 == 9 ? 0 : i8 + 1) + c0005c0.f87a);
        spannableString.setSpan(new SuperscriptSpan(), 0, 1, 33);
        spannableString.setSpan(new ForegroundColorSpan(-256), 0, 1, 33);
        spannableString.setSpan(new TypefaceSpan("monospace"), 0, 1, 33);
        spannableString.setSpan(new RelativeSizeSpan(0.5f), 0, 1, 33);
        spannableString.setSpan(new SubscriptSpan(), 1, 2, 33);
        textViewM13f.setText(spannableString);
        textViewM13f.setId(i7);
        textViewM13f.setVisibility(0);
        textViewM13f.setPressed(false);
        textViewM13f.setFocusable(false);
        if (z) {
            textViewM13f.setText("      " + c0005c0.f87a);
            textViewM13f.setTextSize(0, this.f41g0);
            textViewM13f.setBackgroundDrawable(null);
            textViewM13f.setGravity(16);
            textViewM13f.setMinHeight(this.f64s);
            textViewM13f.setHeight(this.f64s);
            textViewM13f.setOnClickListener(null);
            textViewM13f.setOnLongClickListener(null);
        } else {
            textViewM13f.setTextSize(0, this.f39f0);
            textViewM13f.setGravity(1);
            textViewM13f.setMinHeight(this.f62r);
            textViewM13f.setHeight(this.f62r);
            if (this.f49k0) {
                textViewM13f.setOnClickListener(this.f75x0);
                viewOnLongClickListenerC0027y = this.f79z0;
            } else {
                textViewM13f.setOnClickListener(this.f73w0);
                viewOnLongClickListenerC0027y = this.f77y0;
            }
            textViewM13f.setOnLongClickListener(viewOnLongClickListenerC0027y);
            textViewM13f.setBackgroundResource(R.drawable.cand_back);
        }
        textViewM13f.setTextColor(this.f14M);
        if (i < paddingRight) {
            textViewM13f.setEllipsize(TextUtils.TruncateAt.END);
        } else {
            textViewM13f.setEllipsize(null);
        }
        if (c0005c0.f87a.equals(" ")) {
            imageSpan = new ImageSpan(this.f52m, R.drawable.word_half_space, 1);
        } else if (c0005c0.f87a.equals("\u3000")) {
            imageSpan = new ImageSpan(this.f52m, R.drawable.word_full_space, 1);
        }
        if (imageSpan != null) {
            SpannableString spannableString2 = new SpannableString("   ");
            spannableString2.setSpan(imageSpan, 1, 2, 33);
            textViewM13f.setText(spannableString2);
        }
        textViewM13f.setPadding(0, 0, 0, 0);
        if (this.f49k0) {
            this.f7F.add(this.f5D, c0005c0);
            this.f5D++;
            arrayList = this.f23V;
        } else {
            this.f6E.add(this.f4C, c0005c0);
            this.f4C++;
            arrayList = this.f22U;
        }
        arrayList.add(textViewM13f);
    }

    /* JADX INFO: renamed from: w */
    public final void m30w(String str) {
        TextView textView = this.f59p0;
        if (textView == null) {
            return;
        }
        textView.setText(str);
    }

    /* JADX INFO: renamed from: x */
    public final void m31x(String str, boolean z, boolean z2) {
        TextView textView = this.f51l0;
        if (textView != null) {
            textView.setText(str);
        }
        Vibrator vibrator = this.f78z;
        if (vibrator != null && z2) {
            try {
                vibrator.vibrate(150L);
            } catch (Exception e) {
                Log.e("HzimService", "TextCandidates1LineViewManager::setInfoArea Vibrator " + e.toString());
            }
        }
        ToneGenerator toneGenerator = this.f3B;
        if (toneGenerator == null || !z) {
            return;
        }
        try {
            toneGenerator.startTone(24);
        } catch (Exception e2) {
            Log.e("HzimService", "TextCandidates1LineViewManager::setInfoArea Tone " + e2.toString());
        }
    }

    /* JADX INFO: renamed from: y */
    public final void m32y() {
        int i = 0;
        if (this.f12K) {
            this.f44i.setVisibility(0);
            i = R.drawable.cand_up;
        } else if (this.f13L) {
            this.f44i.setVisibility(0);
            i = R.drawable.cand_down;
        } else {
            this.f44i.setVisibility(8);
            this.f15N.setVisibility(8);
        }
        if (i != 0) {
            this.f44i.setImageResource(i);
            this.f44i.setBackgroundResource(R.drawable.cand_back);
        }
    }

    /* JADX INFO: renamed from: z */
    public final void m33z() {
        if (this.f40g.pageScroll(130)) {
            return;
        }
        this.f40g.scrollTo(0, 0);
    }

    @Override // android.view.GestureDetector.OnGestureListener
    public final void onLongPress(MotionEvent motionEvent) {
    }

    @Override // android.view.GestureDetector.OnGestureListener
    public final void onShowPress(MotionEvent motionEvent) {
    }
}
