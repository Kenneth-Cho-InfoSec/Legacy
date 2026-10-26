package com.minn.android.hanziime;

import android.content.Intent;
import android.content.SharedPreferences;
import android.content.res.Configuration;
import android.net.Uri;
import android.os.Build;
import android.preference.PreferenceManager;
import android.provider.Settings;
import android.text.SpannableStringBuilder;
import android.text.style.BackgroundColorSpan;
import android.text.style.ForegroundColorSpan;
import android.text.style.UnderlineSpan;
import android.view.KeyEvent;
import android.view.View;
import android.view.inputmethod.EditorInfo;
import android.view.inputmethod.InputConnection;
import android.widget.PopupWindow;
import java.util.ArrayList;
import p000a.AbstractC0010h;
import p000a.C0002b;
import p000a.C0004c;
import p000a.C0005c0;
import p000a.C0022t;
import p000a.C0023u;
import p000a.GestureDetectorOnGestureListenerC0001a0;
import p000a.HandlerC0009g;
import p000a.InterfaceC0003b0;
import p001b.AbstractC0034f;
import p001b.C0029a;
import p001b.C0030b;
import p001b.C0032d;
import p001b.C0033e;
import p001b.C0035g;
import p001b.C0036h;
import p001b.ViewOnTouchListenerC0038j;

/* JADX INFO: loaded from: classes.dex */
public class HzimMainTWService extends AbstractC0010h {

    /* JADX INFO: renamed from: J */
    public static final char[] f263J = {' '};

    /* JADX INFO: renamed from: K */
    public static final char[] f264K = {'\b'};

    /* JADX INFO: renamed from: L */
    public static final BackgroundColorSpan f265L;

    /* JADX INFO: renamed from: M */
    public static final BackgroundColorSpan f266M;

    /* JADX INFO: renamed from: N */
    public static final ForegroundColorSpan f267N;

    /* JADX INFO: renamed from: O */
    public static final UnderlineSpan f268O;

    /* JADX INFO: renamed from: E */
    public ViewOnTouchListenerC0038j f273E;

    /* JADX INFO: renamed from: G */
    public Boolean f275G;

    /* JADX INFO: renamed from: H */
    public Boolean f276H;

    /* JADX INFO: renamed from: I */
    public final HandlerC0009g f277I;

    /* JADX INFO: renamed from: l */
    public final SpannableStringBuilder f278l;

    /* JADX INFO: renamed from: m */
    public String f279m;

    /* JADX INFO: renamed from: n */
    public int f280n;

    /* JADX INFO: renamed from: q */
    public C0033e f283q;

    /* JADX INFO: renamed from: s */
    public C0023u f285s;

    /* JADX INFO: renamed from: t */
    public boolean f286t;

    /* JADX INFO: renamed from: u */
    public boolean f287u;

    /* JADX INFO: renamed from: v */
    public boolean f288v;

    /* JADX INFO: renamed from: w */
    public int f289w;

    /* JADX INFO: renamed from: x */
    public boolean f290x;

    /* JADX INFO: renamed from: y */
    public int f291y;

    /* JADX INFO: renamed from: z */
    public boolean f292z;

    /* JADX INFO: renamed from: o */
    public C0035g f281o = null;

    /* JADX INFO: renamed from: p */
    public C0005c0[] f282p = null;

    /* JADX INFO: renamed from: r */
    public C0036h f284r = null;

    /* JADX INFO: renamed from: A */
    public boolean f269A = false;

    /* JADX INFO: renamed from: B */
    public boolean f270B = true;

    /* JADX INFO: renamed from: C */
    public boolean f271C = true;

    /* JADX INFO: renamed from: D */
    public boolean f272D = false;

    /* JADX INFO: renamed from: F */
    public Configuration f274F = new Configuration();

    static {
        new BackgroundColorSpan(-7829249);
        f265L = new BackgroundColorSpan(-10039894);
        new BackgroundColorSpan(-6310195);
        f266M = new BackgroundColorSpan(-983041);
        f267N = new ForegroundColorSpan(-16777216);
        f268O = new UnderlineSpan();
    }

    public HzimMainTWService() {
        this.f283q = null;
        this.f285s = null;
        Boolean bool = Boolean.FALSE;
        this.f275G = bool;
        this.f276H = bool;
        this.f277I = new HandlerC0009g(this, 0);
        this.f107d = new C0002b();
        this.f104a = new GestureDetectorOnGestureListenerC0001a0();
        this.f105b = new C0029a();
        this.f283q = null;
        this.f285s = null;
        this.f106c = null;
        this.f278l = new SpannableStringBuilder();
        this.f109f = false;
        this.f286t = false;
        this.f287u = true;
        this.f288v = true;
        this.f276H = Boolean.TRUE;
    }

    /* JADX WARN: Can't fix incorrect switch cases order, some code will duplicate */
    @Override // p000a.AbstractC0010h
    /* JADX INFO: renamed from: a */
    public final synchronized boolean mo55a(C0004c c0004c) {
        boolean zM114l;
        C0033e c0033e;
        int i;
        C0005c0 c0005c0;
        int i2 = c0004c.f82a;
        boolean zM112j = true;
        if (i2 == -268435425) {
            m111i(c0004c.f85d);
            return true;
        }
        switch (i2) {
            case -268435455:
                if (m110h()) {
                    m106d("");
                } else {
                    m104b(1);
                }
                this.f107d.m34a();
                m116n();
                this.f281o.m92e();
                if (this.f271C) {
                    KeyboardView keyboardView = this.f105b.f188b;
                    if (keyboardView != null) {
                        keyboardView.m119b();
                    }
                    requestHideSelf(0);
                }
                break;
            case -268435454:
                this.f106c.mo45a(this.f107d);
                break;
            default:
                switch (i2) {
                    case -268435437:
                        this.f283q.getClass();
                        return true;
                    case -268435436:
                        this.f283q.getClass();
                        return true;
                    case -268435435:
                        this.f282p = this.f283q.m86g();
                        return true;
                    case -268435434:
                        this.f283q.getClass();
                        return true;
                    case -268435433:
                        this.f283q.getClass();
                        return true;
                    case -268435432:
                        C0005c0[] c0005c0Arr = this.f282p;
                        if (c0005c0Arr != null) {
                            c0004c.f86e = c0005c0Arr[0];
                            int i3 = 0;
                            while (true) {
                                C0005c0[] c0005c0Arr2 = this.f282p;
                                if (i3 >= c0005c0Arr2.length - 1) {
                                    c0005c0Arr2[c0005c0Arr2.length - 1] = null;
                                    if (c0005c0Arr2[0] == null) {
                                        this.f282p = null;
                                    }
                                    return true;
                                }
                                int i4 = i3 + 1;
                                c0005c0Arr2[i3] = c0005c0Arr2[i4];
                                i3 = i4;
                            }
                        }
                        break;
                    case -268435431:
                        m116n();
                        return true;
                    default:
                        switch (i2) {
                            case -268435428:
                                setInputView(onCreateInputView());
                                return true;
                            case -268435427:
                                return this.f104a.m25r();
                            default:
                                switch (i2) {
                                    case -268435408:
                                        this.f281o.m94g(c0004c.f83b);
                                        m117o();
                                        return true;
                                    case -268435407:
                                        this.f281o.m100m();
                                        m117o();
                                        return true;
                                    case -268435406:
                                        this.f281o.m98k();
                                        m117o();
                                        return true;
                                    case -268435405:
                                        this.f281o.getClass();
                                        m117o();
                                        return true;
                                    case -268435404:
                                        this.f281o.m96i();
                                        m117o();
                                        return true;
                                    case -268435403:
                                        this.f281o.m97j();
                                        m117o();
                                        return true;
                                    default:
                                        switch (i2) {
                                            case -268431360:
                                                GestureDetectorOnGestureListenerC0001a0 gestureDetectorOnGestureListenerC0001a0 = this.f104a;
                                                if (gestureDetectorOnGestureListenerC0001a0 instanceof GestureDetectorOnGestureListenerC0001a0) {
                                                    gestureDetectorOnGestureListenerC0001a0.m3C();
                                                }
                                                return true;
                                            case -268431359:
                                                GestureDetectorOnGestureListenerC0001a0 gestureDetectorOnGestureListenerC0001a02 = this.f104a;
                                                if (gestureDetectorOnGestureListenerC0001a02 instanceof GestureDetectorOnGestureListenerC0001a0) {
                                                    gestureDetectorOnGestureListenerC0001a02.m33z();
                                                }
                                                return true;
                                            case -268431358:
                                                GestureDetectorOnGestureListenerC0001a0 gestureDetectorOnGestureListenerC0001a03 = this.f104a;
                                                if (gestureDetectorOnGestureListenerC0001a03 instanceof GestureDetectorOnGestureListenerC0001a0) {
                                                    gestureDetectorOnGestureListenerC0001a03.m2B();
                                                }
                                                return true;
                                            case -268431357:
                                                GestureDetectorOnGestureListenerC0001a0 gestureDetectorOnGestureListenerC0001a04 = this.f104a;
                                                if (gestureDetectorOnGestureListenerC0001a04 instanceof GestureDetectorOnGestureListenerC0001a0) {
                                                    gestureDetectorOnGestureListenerC0001a04.m1A();
                                                }
                                                return true;
                                            default:
                                                switch (i2) {
                                                    case -268427264:
                                                        return true;
                                                    case -268427263:
                                                        this.f105b.m70h(this);
                                                        return true;
                                                }
                                        }
                                }
                        }
                }
                break;
        }
        KeyEvent keyEvent = c0004c.f85d;
        int keyCode = keyEvent != null ? keyEvent.getKeyCode() : 0;
        if (this.f110g) {
            InputConnection inputConnection = this.f108e;
            if (inputConnection != null) {
                switch (c0004c.f82a) {
                    case -268435450:
                        sendKeyChar(c0004c.f84c[0]);
                        break;
                    case -268435449:
                        int keyCode2 = c0004c.f85d.getKeyCode();
                        if (keyCode2 != 59) {
                            if (keyCode2 != 60) {
                                if (keyCode2 != 62) {
                                    if (c0004c.f85d.isCapsLockOn() && c0004c.f85d.isShiftPressed() && keyCode2 >= 29 && keyCode2 <= 54) {
                                        c0004c.f85d.getMetaState();
                                        this.f108e.sendKeyEvent(new KeyEvent(c0004c.f85d.getDownTime(), c0004c.f85d.getEventTime(), c0004c.f85d.getAction(), c0004c.f85d.getKeyCode(), c0004c.f85d.getRepeatCount(), 0));
                                        return true;
                                    }
                                } else if (c0004c.f85d.isCtrlPressed()) {
                                    this.f281o.m93f();
                                    setCandidatesViewShown(true);
                                    m117o();
                                    return true;
                                }
                            } else if (c0004c.f85d.isCtrlPressed() && c0004c.f85d.isShiftPressed()) {
                                this.f281o.m99l();
                                this.f281o.f238a.getClass();
                                if (C0032d.f221e >= 4) {
                                    setCandidatesViewShown(true);
                                    this.f281o.m93f();
                                    m117o();
                                } else {
                                    setCandidatesViewShown(false);
                                    this.f281o.m100m();
                                    this.f106c = null;
                                    this.f284r = null;
                                    this.f287u = false;
                                    showStatusIcon(this.f281o.m90c());
                                }
                                return true;
                            }
                        } else if (c0004c.f85d.isCtrlPressed() && c0004c.f85d.isShiftPressed()) {
                            this.f281o.m95h();
                            this.f281o.f238a.getClass();
                            if (C0032d.f221e >= 4) {
                                setCandidatesViewShown(true);
                                this.f281o.m93f();
                                m117o();
                            } else {
                                setCandidatesViewShown(false);
                                this.f281o.m100m();
                                this.f106c = null;
                                this.f284r = null;
                                this.f287u = false;
                                showStatusIcon(this.f281o.m90c());
                            }
                            return true;
                        }
                        break;
                    case -268435445:
                        m105c(c0004c.f86e);
                        this.f280n = c0004c.f82a;
                        break;
                    case -268435442:
                        if (keyCode != 66) {
                            inputConnection.sendKeyEvent(keyEvent);
                            this.f108e.sendKeyEvent(new KeyEvent(1, keyEvent.getKeyCode()));
                        } else {
                            sendKeyChar('\n');
                        }
                        break;
                }
            }
            return false;
        }
        int i5 = c0004c.f82a;
        if (i5 == -268435452) {
            this.f104a.m6F(1);
            this.f105b.m69g();
            return true;
        }
        if (i5 == -268435453) {
            this.f104a.m6F(0);
            this.f105b.m73k();
            return true;
        }
        switch (i5) {
            case -268435450:
                if (this.f106c == null || (!m109g() && ((c0033e = this.f283q) == null || c0033e.f230b == null))) {
                    m106d(new String(c0004c.f84c));
                } else {
                    C0033e c0033e2 = this.f283q;
                    if (c0033e2 == null || c0033e2.f230b == null) {
                        zM114l = m114l(c0004c.f84c);
                        C0029a c0029a = this.f105b;
                        if (c0029a instanceof C0029a) {
                            if (this.f281o.f240c == 1) {
                                c0029a.m71i(0);
                            } else {
                                c0029a.m71i(1);
                            }
                        }
                    } else {
                        zM114l = m113k(c0004c.f84c);
                        C0029a c0029a2 = this.f105b;
                        if (c0029a2 instanceof C0029a) {
                            if (this.f281o.f240c == 1) {
                                c0029a2.m71i(0);
                                if (this.f281o.f238a.m81a()) {
                                    this.f105b.m72j("");
                                }
                            } else {
                                c0029a2.m71i(1);
                            }
                            C0035g c0035g = this.f281o;
                            if (c0035g.f240c == 2 && c0035g.f238a.m81a()) {
                                this.f105b.m72j(this.f281o.m88a(this.f107d.m43j(0)));
                            }
                        }
                    }
                    zM112j = zM114l;
                }
                return zM112j;
            case -268435449:
                int keyCode3 = c0004c.f85d.getKeyCode();
                if (keyCode3 != 4) {
                    if (keyCode3 != 62) {
                        if (keyCode3 == 111) {
                            this.f281o.m92e();
                            this.f107d.m34a();
                            m115m(null);
                            m116n();
                            this.f280n = c0004c.f82a;
                            return true;
                        }
                        if (keyCode3 != 115) {
                            switch (keyCode3) {
                                case 57:
                                case 58:
                                    if (getResources().getConfiguration().keyboard == 1) {
                                        if (c0004c.f85d.getRepeatCount() == 0) {
                                            int i6 = this.f291y + 1;
                                            this.f291y = i6;
                                            if (i6 > 2) {
                                                this.f291y = 0;
                                            }
                                        }
                                        this.f292z = true;
                                        return true;
                                    }
                                    break;
                                case 59:
                                    this.f281o.f238a.getClass();
                                    if (C0032d.f221e == 0 && getResources().getConfiguration().keyboard == 1) {
                                        if (c0004c.f85d.getRepeatCount() == 0) {
                                            int i7 = this.f289w + 1;
                                            this.f289w = i7;
                                            if (i7 > 2) {
                                                this.f289w = 0;
                                            }
                                        }
                                        this.f290x = true;
                                        return true;
                                    }
                                    if (c0004c.f85d.isCtrlPressed() && c0004c.f85d.isShiftPressed()) {
                                        this.f281o.m95h();
                                        this.f281o.f238a.getClass();
                                        if (C0032d.f221e >= 4) {
                                            setCandidatesViewShown(true);
                                            this.f281o.m93f();
                                            m117o();
                                        } else {
                                            setCandidatesViewShown(false);
                                            this.f281o.m100m();
                                            this.f106c = null;
                                            this.f284r = null;
                                            this.f287u = false;
                                            showStatusIcon(this.f281o.m90c());
                                        }
                                        return true;
                                    }
                                    break;
                                case 60:
                                    this.f281o.f238a.getClass();
                                    if (C0032d.f221e == 0 && getResources().getConfiguration().keyboard == 1) {
                                        if (c0004c.f85d.getRepeatCount() == 0) {
                                            int i8 = this.f289w + 1;
                                            this.f289w = i8;
                                            if (i8 > 2) {
                                                this.f289w = 0;
                                            }
                                        }
                                        this.f290x = true;
                                        return true;
                                    }
                                    if (c0004c.f85d.isCtrlPressed() && c0004c.f85d.isShiftPressed()) {
                                        this.f281o.m99l();
                                        this.f281o.f238a.getClass();
                                        if (C0032d.f221e >= 4) {
                                            setCandidatesViewShown(true);
                                            this.f281o.m93f();
                                            m117o();
                                        } else {
                                            setCandidatesViewShown(false);
                                            this.f281o.m100m();
                                            this.f106c = null;
                                            this.f284r = null;
                                            this.f287u = false;
                                            showStatusIcon(this.f281o.m90c());
                                        }
                                        return true;
                                    }
                                    break;
                                default:
                                    if (c0004c.f85d.isCapsLockOn() && c0004c.f85d.isShiftPressed() && keyCode3 >= 29 && keyCode3 <= 54) {
                                        c0004c.f85d.getMetaState();
                                        this.f108e.sendKeyEvent(new KeyEvent(c0004c.f85d.getDownTime(), c0004c.f85d.getEventTime(), c0004c.f85d.getAction(), c0004c.f85d.getKeyCode(), c0004c.f85d.getRepeatCount(), 0));
                                        return true;
                                    }
                                    break;
                            }
                        } else {
                            c0004c.f85d.isCapsLockOn();
                        }
                    } else if (c0004c.f85d.isCtrlPressed()) {
                        this.f281o.f238a.getClass();
                        if (C0032d.f221e >= 4) {
                            setCandidatesViewShown(false);
                            this.f281o.m100m();
                            this.f106c = null;
                            this.f284r = null;
                            this.f287u = false;
                            showStatusIcon(this.f281o.m90c());
                        } else {
                            setCandidatesViewShown(true);
                            this.f281o.m93f();
                            m117o();
                        }
                        return true;
                    }
                } else if (this.f107d.m42i(0) > 0) {
                    this.f281o.m92e();
                    this.f107d.m34a();
                    m115m(null);
                    m116n();
                    this.f280n = c0004c.f82a;
                    return true;
                }
                m115m(null);
                zM112j = m112j(c0004c.f85d);
                i = c0004c.f82a;
                this.f280n = i;
                return zM112j;
            case -268435445:
                if (this.f286t) {
                    c0005c0 = c0004c.f86e;
                } else {
                    if (this.f279m.contains(c0004c.f86e.f87a) && this.f280n == -268435445) {
                        this.f108e.deleteSurroundingText(1, 0);
                    }
                    if (m110h()) {
                        if (this.f107d.m43j(0).equalsIgnoreCase("W")) {
                            int iM22o = this.f104a.m22o();
                            char[] cArr = {0};
                            cArr[0] = (char) ((iM22o >= 9 || iM22o < 0) ? iM22o == 9 ? 48 : 0 : iM22o + 49);
                            m113k(cArr);
                        } else {
                            m105c(c0004c.f86e);
                            this.f107d.m34a();
                            this.f281o.m92e();
                            C0029a c0029a3 = this.f105b;
                            if (c0029a3 instanceof C0029a) {
                                c0029a3.m71i(0);
                            }
                            m116n();
                        }
                        i = c0004c.f82a;
                        this.f280n = i;
                        return zM112j;
                    }
                    c0005c0 = c0004c.f86e;
                }
                m105c(c0005c0);
                i = c0004c.f82a;
                this.f280n = i;
                return zM112j;
            case -268435442:
                m115m(null);
                boolean zM112j2 = m112j(c0004c.f85d);
                if (zM112j2) {
                    zM112j = zM112j2;
                } else {
                    int keyCode4 = keyEvent.getKeyCode();
                    if (keyCode4 == 66 || keyCode == 160) {
                        sendKeyChar('\n');
                    } else {
                        this.f108e.sendKeyEvent(keyEvent);
                        this.f108e.sendKeyEvent(new KeyEvent(1, keyCode4));
                    }
                }
                i = c0004c.f82a;
                this.f280n = i;
                return zM112j;
            case -268435439:
                m104b(0);
                this.f107d.m34a();
                m115m("e");
                m116n();
                zM112j = false;
                return zM112j;
            case -268435402:
                throw null;
            default:
                zM112j = false;
                return zM112j;
        }
    }

    /* JADX INFO: renamed from: b */
    public final void m104b(int i) {
        InterfaceC0003b0 interfaceC0003b0;
        String strM43j = this.f107d.m43j(i);
        if (strM43j.length() > 0) {
            if (this.f288v && (interfaceC0003b0 = this.f106c) != null) {
                interfaceC0003b0.getClass();
            }
            this.f108e.commitText(strM43j, 1);
        }
        this.f104a.m9b();
    }

    /* JADX INFO: renamed from: c */
    public final void m105c(C0005c0 c0005c0) {
        this.f108e.commitText(c0005c0.f87a, 1);
    }

    /* JADX INFO: renamed from: d */
    public final void m106d(String str) {
        this.f108e.commitText(str, 1);
    }

    /* JADX INFO: renamed from: e */
    public final void m107e(int i) {
        if (i == 0) {
            return;
        }
        m108f(Character.toChars(i));
    }

    /* JADX INFO: renamed from: f */
    public final void m108f(char[] cArr) {
        C0022t c0022t = new C0022t(cArr);
        char c = cArr[0];
        if (c != f263J[0] && c != '\t') {
            if (!this.f279m.contains(c0022t.f172a) || m110h()) {
                this.f107d.m39f(c0022t);
                m116n();
                return;
            } else if (this.f280n == -268435445) {
                this.f108e.deleteSurroundingText(1, 0);
            }
        }
        m104b(1);
        m106d(c0022t.f172a);
        this.f107d.m34a();
    }

    /* JADX INFO: renamed from: g */
    public final boolean m109g() {
        C0036h c0036h = this.f284r;
        if (c0036h == null) {
            return false;
        }
        c0036h.getClass();
        return C0036h.f243a != null;
    }

    /* JADX INFO: renamed from: h */
    public final boolean m110h() {
        this.f281o.f238a.getClass();
        return C0032d.f221e >= 4;
    }

    @Override // p000a.AbstractC0010h, android.inputmethodservice.InputMethodService
    public final void hideWindow() {
        BaseInputView baseInputView = this.f105b.f189c;
        m106d("");
        this.f107d.m34a();
        this.f105b.m70h(this);
        this.f277I.removeMessages(1);
        KeyboardView keyboardView = this.f105b.f188b;
        if (keyboardView != null) {
            keyboardView.m119b();
        }
        this.f104a.m16i();
        GestureDetectorOnGestureListenerC0001a0 gestureDetectorOnGestureListenerC0001a0 = this.f104a;
        PopupWindow popupWindow = gestureDetectorOnGestureListenerC0001a0.f65s0;
        if (popupWindow != null && popupWindow.isShowing()) {
            gestureDetectorOnGestureListenerC0001a0.f65s0.dismiss();
        }
        super.hideWindow();
    }

    /* JADX INFO: renamed from: i */
    public final void m111i(KeyEvent keyEvent) {
        int keyCode = keyEvent.getKeyCode();
        if (!this.f290x && (keyCode == 59 || keyCode == 60)) {
            this.f289w = 0;
            this.f290x = true;
        }
        if (this.f292z) {
            return;
        }
        if (keyCode == 57 || keyCode == 58) {
            this.f291y = 0;
            this.f292z = true;
        }
    }

    /* JADX WARN: Multi-variable type inference failed */
    /* JADX WARN: Removed duplicated region for block: B:140:0x01b8 A[FALL_THROUGH] */
    /*  JADX ERROR: UnsupportedOperationException in pass: RegionMakerVisitor
        java.lang.UnsupportedOperationException
        	at java.base/java.util.Collections$UnmodifiableCollection.add(Collections.java:1068)
        	at jadx.core.dex.visitors.regions.maker.SwitchRegionMaker$1.leaveRegion(SwitchRegionMaker.java:390)
        	at jadx.core.dex.visitors.regions.DepthRegionTraversal.traverseInternal(DepthRegionTraversal.java:70)
        	at jadx.core.dex.visitors.regions.DepthRegionTraversal.lambda$traverseInternal$0(DepthRegionTraversal.java:68)
        	at java.base/java.util.ArrayList.forEach(ArrayList.java:1511)
        	at jadx.core.dex.visitors.regions.DepthRegionTraversal.traverseInternal(DepthRegionTraversal.java:68)
        	at jadx.core.dex.visitors.regions.DepthRegionTraversal.traverse(DepthRegionTraversal.java:23)
        	at jadx.core.dex.visitors.regions.maker.SwitchRegionMaker.insertBreaksForCase(SwitchRegionMaker.java:370)
        	at jadx.core.dex.visitors.regions.maker.SwitchRegionMaker.insertBreaks(SwitchRegionMaker.java:85)
        	at jadx.core.dex.visitors.regions.PostProcessRegions.leaveRegion(PostProcessRegions.java:33)
        	at jadx.core.dex.visitors.regions.DepthRegionTraversal.traverseInternal(DepthRegionTraversal.java:70)
        	at jadx.core.dex.visitors.regions.DepthRegionTraversal.lambda$traverseInternal$0(DepthRegionTraversal.java:68)
        	at java.base/java.util.ArrayList.forEach(ArrayList.java:1511)
        	at jadx.core.dex.visitors.regions.DepthRegionTraversal.traverseInternal(DepthRegionTraversal.java:68)
        	at jadx.core.dex.visitors.regions.DepthRegionTraversal.lambda$traverseInternal$0(DepthRegionTraversal.java:68)
        	at java.base/java.util.ArrayList.forEach(ArrayList.java:1511)
        	at java.base/java.util.Collections$UnmodifiableCollection.forEach(Collections.java:1093)
        	at jadx.core.dex.visitors.regions.DepthRegionTraversal.traverseInternal(DepthRegionTraversal.java:68)
        	at jadx.core.dex.visitors.regions.DepthRegionTraversal.lambda$traverseInternal$0(DepthRegionTraversal.java:68)
        	at java.base/java.util.ArrayList.forEach(ArrayList.java:1511)
        	at jadx.core.dex.visitors.regions.DepthRegionTraversal.traverseInternal(DepthRegionTraversal.java:68)
        	at jadx.core.dex.visitors.regions.DepthRegionTraversal.lambda$traverseInternal$0(DepthRegionTraversal.java:68)
        	at java.base/java.util.ArrayList.forEach(ArrayList.java:1511)
        	at java.base/java.util.Collections$UnmodifiableCollection.forEach(Collections.java:1093)
        	at jadx.core.dex.visitors.regions.DepthRegionTraversal.traverseInternal(DepthRegionTraversal.java:68)
        	at jadx.core.dex.visitors.regions.DepthRegionTraversal.lambda$traverseInternal$0(DepthRegionTraversal.java:68)
        	at java.base/java.util.ArrayList.forEach(ArrayList.java:1511)
        	at jadx.core.dex.visitors.regions.DepthRegionTraversal.traverseInternal(DepthRegionTraversal.java:68)
        	at jadx.core.dex.visitors.regions.DepthRegionTraversal.lambda$traverseInternal$0(DepthRegionTraversal.java:68)
        	at java.base/java.util.ArrayList.forEach(ArrayList.java:1511)
        	at java.base/java.util.Collections$UnmodifiableCollection.forEach(Collections.java:1093)
        	at jadx.core.dex.visitors.regions.DepthRegionTraversal.traverseInternal(DepthRegionTraversal.java:68)
        	at jadx.core.dex.visitors.regions.DepthRegionTraversal.lambda$traverseInternal$0(DepthRegionTraversal.java:68)
        	at java.base/java.util.ArrayList.forEach(ArrayList.java:1511)
        	at jadx.core.dex.visitors.regions.DepthRegionTraversal.traverseInternal(DepthRegionTraversal.java:68)
        	at jadx.core.dex.visitors.regions.DepthRegionTraversal.lambda$traverseInternal$0(DepthRegionTraversal.java:68)
        	at java.base/java.util.ArrayList.forEach(ArrayList.java:1511)
        	at java.base/java.util.Collections$UnmodifiableCollection.forEach(Collections.java:1093)
        	at jadx.core.dex.visitors.regions.DepthRegionTraversal.traverseInternal(DepthRegionTraversal.java:68)
        	at jadx.core.dex.visitors.regions.DepthRegionTraversal.lambda$traverseInternal$0(DepthRegionTraversal.java:68)
        	at java.base/java.util.ArrayList.forEach(ArrayList.java:1511)
        	at jadx.core.dex.visitors.regions.DepthRegionTraversal.traverseInternal(DepthRegionTraversal.java:68)
        	at jadx.core.dex.visitors.regions.DepthRegionTraversal.lambda$traverseInternal$0(DepthRegionTraversal.java:68)
        	at java.base/java.util.ArrayList.forEach(ArrayList.java:1511)
        	at java.base/java.util.Collections$UnmodifiableCollection.forEach(Collections.java:1093)
        	at jadx.core.dex.visitors.regions.DepthRegionTraversal.traverseInternal(DepthRegionTraversal.java:68)
        	at jadx.core.dex.visitors.regions.DepthRegionTraversal.lambda$traverseInternal$0(DepthRegionTraversal.java:68)
        	at java.base/java.util.ArrayList.forEach(ArrayList.java:1511)
        	at jadx.core.dex.visitors.regions.DepthRegionTraversal.traverseInternal(DepthRegionTraversal.java:68)
        	at jadx.core.dex.visitors.regions.DepthRegionTraversal.lambda$traverseInternal$0(DepthRegionTraversal.java:68)
        	at java.base/java.util.ArrayList.forEach(ArrayList.java:1511)
        	at java.base/java.util.Collections$UnmodifiableCollection.forEach(Collections.java:1093)
        	at jadx.core.dex.visitors.regions.DepthRegionTraversal.traverseInternal(DepthRegionTraversal.java:68)
        	at jadx.core.dex.visitors.regions.DepthRegionTraversal.lambda$traverseInternal$0(DepthRegionTraversal.java:68)
        	at java.base/java.util.ArrayList.forEach(ArrayList.java:1511)
        	at jadx.core.dex.visitors.regions.DepthRegionTraversal.traverseInternal(DepthRegionTraversal.java:68)
        	at jadx.core.dex.visitors.regions.DepthRegionTraversal.traverse(DepthRegionTraversal.java:19)
        	at jadx.core.dex.visitors.regions.PostProcessRegions.process(PostProcessRegions.java:23)
        	at jadx.core.dex.visitors.regions.RegionMakerVisitor.visit(RegionMakerVisitor.java:31)
        */
    /* JADX INFO: renamed from: j */
    /*
        Code decompiled incorrectly, please refer to instructions dump.
    */
    public final boolean m112j(android.view.KeyEvent r11) {
        /*
            Method dump skipped, instruction units count: 758
            To view this dump add '--comments-level debug' option
        */
        throw new UnsupportedOperationException("Method not decompiled: com.minn.android.hanziime.HzimMainTWService.m112j(android.view.KeyEvent):boolean");
    }

    /* JADX WARN: Removed duplicated region for block: B:115:0x019c  */
    /* JADX WARN: Removed duplicated region for block: B:250:0x03e0  */
    /* JADX WARN: Removed duplicated region for block: B:251:0x03f7 A[PHI: r8
  0x03f7: PHI (r8v25 a.c0) = (r8v23 a.c0), (r8v34 a.c0) binds: [B:263:0x042a, B:249:0x03de] A[DONT_GENERATE, DONT_INLINE]] */
    /* JADX WARN: Removed duplicated region for block: B:280:0x04a9  */
    /* JADX WARN: Removed duplicated region for block: B:283:0x04af  */
    /* JADX WARN: Removed duplicated region for block: B:296:0x04e0  */
    /* JADX WARN: Removed duplicated region for block: B:311:0x0511  */
    /* JADX WARN: Removed duplicated region for block: B:34:0x0084  */
    /* JADX INFO: renamed from: k */
    /*
        Code decompiled incorrectly, please refer to instructions dump.
    */
    public final boolean m113k(char[] cArr) {
        int i;
        int i2;
        String str;
        String str2;
        String str3;
        String str4;
        int i3;
        char c;
        GestureDetectorOnGestureListenerC0001a0 gestureDetectorOnGestureListenerC0001a0;
        int i4;
        int size;
        C0005c0 c0005c0M84e;
        C0022t c0022t;
        if (cArr == null) {
            return false;
        }
        C0035g c0035g = this.f281o;
        c0035g.getClass();
        C0035g.f237g = 1;
        char upperCase = Character.toUpperCase(cArr[0]);
        c0035g.f241d = upperCase;
        C0032d c0032d = c0035g.f238a;
        String str5 = c0032d.f226b.f216h;
        if (str5 == null ? false : str5.contains(String.valueOf(upperCase))) {
            C0035g.f237g = 4;
            i = 4;
        } else {
            int i5 = c0035g.f240c;
            if (c0032d.m81a() && !AbstractC0034f.m87a(i5, 3) && !AbstractC0034f.m87a(i5, 4)) {
                String strValueOf = String.valueOf(c0035g.f241d);
                C0030b c0030b = c0032d.f228d[0];
                if (!((c0030b == null || (str4 = c0030b.f205a) == null) ? false : str4.contains(strValueOf)) || c0032d.m82b(strValueOf)) {
                    String str6 = c0032d.f226b.f213e;
                    if (str6 == null ? false : str6.contains(strValueOf)) {
                        C0035g.f237g = 3;
                    } else if (c0032d.m82b(strValueOf) && AbstractC0034f.m87a(i5, 1)) {
                        c0035g.f242e = false;
                        C0035g.f237g = 1;
                    } else if (c0032d.m82b(strValueOf) && AbstractC0034f.m87a(i5, 2)) {
                        c0035g.f242e = false;
                        C0035g.f237g = 12;
                    } else {
                        String str7 = c0032d.f226b.f217i;
                        if (str7 == null ? false : str7.contains(strValueOf)) {
                            c0035g.f242e = false;
                            C0035g.f237g = 11;
                        } else {
                            String str8 = c0032d.f226b.f214f;
                            if (str8 == null ? false : str8.contains(strValueOf)) {
                                c0035g.f242e = false;
                                C0035g.f237g = 5;
                            } else if (c0032d.f226b.f218j.contains(strValueOf)) {
                                c0035g.f242e = true;
                                c0035g.f240c = 2;
                                C0035g.f237g = 14;
                            } else if (strValueOf.equalsIgnoreCase("I")) {
                                c0035g.f242e = false;
                                C0035g.f237g = 13;
                            }
                        }
                    }
                } else {
                    String str9 = c0032d.f226b.f217i;
                    if (!(str9 == null ? false : str9.contains(strValueOf))) {
                        c0035g.f240c = 2;
                        C0035g.f237g = 2;
                    }
                }
            } else if (c0032d.m81a() && AbstractC0034f.m87a(i5, 3) && String.valueOf(c0035g.f241d).equalsIgnoreCase("I")) {
                c0035g.f242e = false;
                C0035g.f237g = 6;
                i = 6;
            } else if (AbstractC0034f.m87a(i5, 1)) {
                char c2 = c0035g.f241d;
                C0035g.f237g = 1;
                String strValueOf2 = String.valueOf(c2);
                C0030b c0030b2 = c0032d.f228d[1];
                if (((c0030b2 == null || (str3 = c0030b2.f205a) == null) ? false : str3.contains(strValueOf2)) || c0032d.f226b.f218j.contains(strValueOf2)) {
                    c0035g.f240c = 2;
                    C0035g.f237g = 2;
                } else {
                    String str10 = c0032d.f226b.f213e;
                    if (str10 == null ? false : str10.contains(strValueOf2)) {
                        C0035g.f237g = 3;
                    }
                }
                i = C0035g.f237g;
            } else if (AbstractC0034f.m87a(i5, 2)) {
                char c3 = c0035g.f241d;
                C0035g.f237g = 1;
                String strValueOf3 = String.valueOf(c3);
                C0030b c0030b3 = c0032d.f228d[1];
                if (!((c0030b3 == null || (str2 = c0030b3.f205a) == null) ? false : str2.contains(strValueOf3)) || c0032d.m82b(strValueOf3)) {
                    String str11 = c0032d.f226b.f213e;
                    if (str11 == null ? false : str11.contains(strValueOf3)) {
                        C0035g.f237g = 3;
                    } else if (c0032d.m82b(strValueOf3)) {
                        c0035g.f242e = false;
                        C0035g.f237g = 12;
                    } else {
                        String str12 = c0032d.f226b.f217i;
                        if (str12 == null ? false : str12.contains(strValueOf3)) {
                            c0035g.f242e = false;
                            C0035g.f237g = 11;
                        } else {
                            String str13 = c0032d.f226b.f214f;
                            if (str13 == null ? false : str13.contains(strValueOf3)) {
                                c0035g.f242e = false;
                                C0035g.f237g = 5;
                            } else if (c0032d.f226b.f218j.contains(strValueOf3)) {
                                c0035g.f242e = true;
                                C0035g.f237g = 2;
                            }
                        }
                    }
                    i = C0035g.f237g;
                } else {
                    String str14 = c0032d.f226b.f217i;
                    if (!(str14 == null ? false : str14.contains(strValueOf3))) {
                        C0035g.f237g = 2;
                        i = C0035g.f237g;
                    }
                }
            } else if (AbstractC0034f.m87a(i5, 3)) {
                char c4 = c0035g.f241d;
                C0035g.f237g = 1;
                String strValueOf4 = String.valueOf(c4);
                String str15 = c0032d.f226b.f214f;
                if (str15 == null ? false : str15.contains(strValueOf4)) {
                    c0035g.f242e = false;
                    i2 = 6;
                } else {
                    i2 = 6;
                    if (c0032d.m82b(strValueOf4)) {
                        c0035g.f242e = false;
                    } else {
                        C0030b c0030b4 = c0032d.f228d[1];
                        if (((c0030b4 == null || (str = c0030b4.f205a) == null) ? false : str.contains(strValueOf4)) && !c0032d.m82b(strValueOf4)) {
                            C0035g.f237g = 7;
                        }
                        i = C0035g.f237g;
                    }
                }
                C0035g.f237g = i2;
                i = C0035g.f237g;
            } else if (AbstractC0034f.m87a(i5, 4)) {
                C0035g.f237g = 1;
                i = 1;
            } else {
                c0032d.m81a();
            }
            i = C0035g.f237g;
        }
        int iM42i = this.f107d.m42i(0);
        if (i == 2 || i == 11) {
            cArr[0] = Character.toUpperCase(cArr[0]);
            this.f107d.m39f(new C0022t(cArr));
        } else if (i == 14) {
            if (iM42i % 2 == 1) {
                this.f107d.m35b();
            }
            C0022t c0022t2 = new C0022t(cArr);
            this.f107d.m39f(c0022t2);
            this.f107d.m39f(c0022t2);
        } else if (i == 13) {
            if (iM42i > 0) {
                int i6 = iM42i % 2;
                if (i6 == 0) {
                    cArr[0] = '8';
                    this.f107d.m39f(new C0022t(cArr));
                    cArr[0] = '1';
                    c0022t = new C0022t(cArr);
                } else if (i6 == 1) {
                    this.f107d.m34a();
                    this.f281o.m92e();
                    cArr[0] = Character.toUpperCase(cArr[0]);
                    c0022t = new C0022t(cArr);
                }
            } else {
                cArr[0] = Character.toUpperCase(cArr[0]);
                c0022t = new C0022t(cArr);
            }
            this.f107d.m39f(c0022t);
        } else if (i == 3) {
            String strM43j = this.f107d.m43j(1);
            int i7 = this.f107d.f81b[0];
            if (i7 > 0) {
                if (i7 >= 2 && this.f281o.f238a.m81a()) {
                    String strSubstring = strM43j.substring(i7 - 2, i7);
                    if (strSubstring.equals("**") || strSubstring.equals("??")) {
                        this.f107d.m35b();
                    }
                }
                this.f107d.m35b();
                iM42i = this.f107d.m42i(0);
                if (iM42i == 0) {
                    this.f281o.m92e();
                }
            } else {
                iM42i = -1;
            }
        } else if (i == 4) {
            this.f107d.m34a();
        } else if (i == 5) {
            String upperCase2 = this.f281o.f238a.f226b.f219k.toUpperCase();
            if (iM42i == 1 && upperCase2.equals(this.f107d.m43j(0))) {
                C0035g.f237g = 12;
            }
            cArr[0] = Character.toUpperCase(cArr[0]);
            c0022t = new C0022t(cArr);
            this.f107d.m39f(c0022t);
        } else if (i == 6) {
            String strM43j2 = this.f107d.m43j(1);
            if (this.f281o.f238a.m81a() && strM43j2.length() == 3 && strM43j2.substring(0, 2).equals("22") && strM43j2.charAt(2) >= '0' && strM43j2.charAt(2) <= '9') {
                this.f104a.m27t(62);
                return true;
            }
            if (!this.f281o.f238a.f226b.f212d[10] || strM43j2.contains("*") || strM43j2.contains("?") || (strM43j2.substring(0, 1).equalsIgnoreCase("w") && strM43j2.charAt(1) >= '0' && strM43j2.charAt(1) <= '9')) {
                char c5 = cArr[0];
                if (c5 < '0' || c5 > '9') {
                    this.f104a.m27t(62);
                    return true;
                }
                char upperCase3 = Character.toUpperCase(c5);
                cArr[0] = upperCase3;
                c0005c0M84e = this.f283q.m84e(upperCase3 == '0' ? 9 : (upperCase3 < '1' || upperCase3 > '9') ? 0 : upperCase3 - '1');
                if (c0005c0M84e == null) {
                    this.f104a.m31x(getString(R.string.no_found_word), true, true);
                    this.f104a.m30w(getString(R.string.no_found_word));
                } else {
                    this.f108e.commitText(c0005c0M84e.f87a, 1);
                    this.f107d.m34a();
                    this.f281o.m92e();
                }
            } else {
                int i8 = this.f104a.f24W;
                if (i8 < 0) {
                    char upperCase4 = Character.toUpperCase(cArr[0]);
                    cArr[0] = upperCase4;
                    i8 = upperCase4 == '0' ? 9 : (upperCase4 < '1' || upperCase4 > '9') ? 0 : upperCase4 - '1';
                }
                c0005c0M84e = this.f283q.m84e(i8);
                if (c0005c0M84e == null) {
                }
            }
        } else if (i == 7) {
            m105c(this.f283q.m84e(0));
            this.f107d.m34a();
            cArr[0] = Character.toUpperCase(cArr[0]);
            this.f107d.m39f(new C0022t(cArr));
            this.f281o.f240c = 2;
        } else if (i == 12) {
            String strM43j3 = this.f107d.m43j(0);
            if (this.f281o.f238a.m81a() && strM43j3.length() == 1) {
                this.f108e.commitText(strM43j3, 1);
                this.f107d.m34a();
                this.f281o.m92e();
                m116n();
            }
        }
        C0035g c0035g2 = this.f281o;
        String strM43j4 = this.f107d.m43j(0);
        c0035g2.getClass();
        int i9 = C0035g.f237g;
        if (i9 != 1) {
            if (i9 != 5) {
                int length = strM43j4.length();
                if (length > 0) {
                    int i10 = length - 1;
                    String strSubstring2 = strM43j4.substring(0, i10);
                    String strSubstring3 = strM43j4.substring(i10);
                    if (strSubstring3.equals("?") && strSubstring2.contains("*")) {
                        c = 5;
                    } else if (strSubstring3.equals("*") && strSubstring2.contains("?")) {
                        c = 6;
                    } else if (c0035g2.f242e) {
                        c = 7;
                    } else {
                        int length2 = strM43j4.length();
                        C0032d c0032d2 = c0035g2.f238a;
                        if (c0032d2.m81a()) {
                            C0030b c0030b5 = c0032d2.f228d[0];
                            if (c0030b5 == null) {
                                c = 1;
                            } else {
                                int i11 = c0030b5.f207c;
                                int i12 = length2 % i11;
                                length2 /= i11;
                                if (strM43j4.equalsIgnoreCase("I")) {
                                    c = 11;
                                } else {
                                    i3 = i12;
                                }
                            }
                        } else {
                            i3 = 0;
                        }
                        int i13 = c0032d2.f227c;
                        if (iM42i != 0) {
                            if (iM42i != -1) {
                                if (length2 != 0) {
                                    if (length2 < i13) {
                                        int i14 = C0035g.f237g;
                                        if (i14 == 12 || i14 == 11 || i14 == 13) {
                                            c0035g2.f240c = 3;
                                            if (c0032d2.m81a() && i3 == 1 && ((length2 == 1 || length2 == 2) && !strM43j4.contains("*") && !strM43j4.contains("?"))) {
                                                boolean z = strM43j4.charAt(0) - '0' == 2 && strM43j4.charAt(1) - '0' == 2;
                                                if (i3 != 1 || length2 != 1 || !z) {
                                                    c = '\n';
                                                }
                                            }
                                        } else if (i14 == 5) {
                                        }
                                    } else {
                                        if (length2 == i13) {
                                            c0035g2.f240c = 3;
                                        }
                                        c = 1;
                                    }
                                    c = 3;
                                } else if (c0032d2.m81a() && i3 == 1) {
                                    c = 2;
                                } else {
                                    c0035g2.m92e();
                                    c = '\b';
                                }
                            }
                        }
                    }
                }
            }
        }
        if (c == 2 || c == '\b' || c == 7) {
            m116n();
        } else {
            if (c == '\n') {
                C0002b c0002b = this.f107d;
                int i15 = c0002b.f81b[0] - 1;
                String strM44k = c0002b.m44k(0, i15, i15);
                this.f107d.m35b();
                int iIntValue = Integer.valueOf(strM44k).intValue();
                if (this.f283q.mo47c(this.f107d) > 0) {
                    C0005c0 c0005c0Mo46b = this.f283q.mo46b();
                    for (int i16 = iIntValue == 0 ? 9 : iIntValue - 1; i16 > 0; i16--) {
                        c0005c0Mo46b = this.f283q.mo46b();
                    }
                    m105c(c0005c0Mo46b);
                }
                this.f107d.m34a();
                this.f281o.m92e();
                this.f104a.f24W = -1;
                this.f272D = false;
            } else if (c == 3) {
                if (this.f283q.mo45a(this.f107d) >= 0) {
                    C0033e c0033e = this.f283q;
                    if (c0033e.f234f == null) {
                        c0033e.f234f = new ArrayList();
                    }
                    c0033e.f234f.clear();
                    while (c0033e.f233e.hasNext()) {
                        c0033e.f234f.add(c0033e.f233e.next());
                    }
                    size = c0033e.f234f.size();
                } else {
                    size = 0;
                }
                if (size == 0) {
                    this.f104a.m31x(getString(R.string.no_found_word), true, true);
                    this.f104a.m30w(getString(R.string.no_found_word));
                    this.f107d.m34a();
                    this.f281o.m92e();
                    m106d("");
                } else if (size == 1) {
                    m105c(this.f283q.m84e(0));
                    this.f107d.m34a();
                    this.f281o.m92e();
                } else if (size > 1) {
                    this.f104a.m31x(getString(R.string.repeat_words), true, true);
                    this.f104a.m30w(getString(R.string.repeat_words));
                    this.f281o.f240c = 3;
                }
            } else if (c == '\t') {
                this.f108e.commitText(this.f107d.m43j(0), 1);
                this.f107d.m34a();
                this.f281o.m92e();
            } else if (c == 11) {
                this.f107d.m34a();
                this.f281o.m92e();
                this.f104a.m31x(getString(R.string.no_found_word), true, true);
                this.f104a.m30w(getString(R.string.no_found_word));
                m116n();
                this.f104a.f24W = -1;
            } else {
                if (c == 1) {
                    if (cArr == f264K) {
                        return false;
                    }
                    m106d(String.valueOf(cArr));
                    this.f104a.m16i();
                    return true;
                }
                if (c == 5) {
                    this.f107d.m35b();
                    if (i == 14) {
                        this.f107d.m35b();
                    }
                    gestureDetectorOnGestureListenerC0001a0 = this.f104a;
                    i4 = R.string.have_asterisk;
                } else if (c == 6) {
                    this.f107d.m35b();
                    if (i == 14) {
                        this.f107d.m35b();
                    }
                    gestureDetectorOnGestureListenerC0001a0 = this.f104a;
                    i4 = R.string.have_question;
                }
                gestureDetectorOnGestureListenerC0001a0.m31x(getString(i4), true, true);
                this.f104a.m30w(getString(i4));
            }
            m116n();
        }
        return true;
    }

    /* JADX INFO: renamed from: l */
    public final boolean m114l(char[] cArr) {
        if (cArr == null) {
            return false;
        }
        return m113k(cArr);
    }

    /* JADX INFO: renamed from: m */
    public final void m115m(String str) {
        HandlerC0009g handlerC0009g = this.f277I;
        if (str != null) {
            handlerC0009g.removeMessages(0);
            this.f286t = true;
            this.f285s.m66f(str);
            this.f106c = this.f285s;
            return;
        }
        if (this.f286t) {
            handlerC0009g.removeMessages(0);
            this.f286t = false;
            C0033e c0033e = this.f283q;
            if (c0033e == null || c0033e.f231c == null) {
                c0033e = null;
            }
            this.f106c = c0033e;
        }
    }

    /* JADX WARN: Removed duplicated region for block: B:17:0x0054  */
    /* JADX INFO: renamed from: n */
    /*
        Code decompiled incorrectly, please refer to instructions dump.
    */
    public final void m116n() {
        boolean z = this.f287u;
        HandlerC0009g handlerC0009g = this.f277I;
        if (!z) {
            m104b(0);
            this.f107d.m34a();
            if (this.f286t) {
                handlerC0009g.removeMessages(0);
                handlerC0009g.sendMessageDelayed(handlerC0009g.obtainMessage(0), 0L);
                return;
            }
            return;
        }
        String strM43j = this.f107d.m43j(0);
        int i = -1;
        if (strM43j.length() != 0) {
            C0032d c0032d = this.f281o.f238a;
            if (c0032d.m81a()) {
                int length = strM43j.length();
                if (length == 3) {
                    int iCharAt = strM43j.charAt(0) - '0';
                    int iCharAt2 = strM43j.charAt(1) - '0';
                    if (iCharAt != 2 || iCharAt2 != 2) {
                        int i2 = c0032d.f228d[0].f207c;
                        int i3 = length % i2;
                        int i4 = length / i2;
                        if (i3 == 1 && ((i4 == 1 || i4 == 2) && !strM43j.contains("*") && !strM43j.contains("?"))) {
                            int iIntValue = Integer.valueOf(strM43j.substring(strM43j.length() - 1)).intValue();
                            i = iIntValue == 0 ? 9 : iIntValue - 1;
                        }
                    }
                }
            }
            this.f104a.f24W = i;
            if (i < 0) {
                this.f272D = false;
            } else {
                this.f272D = true;
            }
            handlerC0009g.removeMessages(0);
            this.f104a.f34d.isShown();
            handlerC0009g.sendMessageDelayed(handlerC0009g.obtainMessage(0), 0L);
        } else {
            handlerC0009g.removeMessages(0);
            handlerC0009g.sendMessageDelayed(handlerC0009g.obtainMessage(0), 0L);
            this.f104a.f24W = -1;
        }
        this.f105b.m70h(this);
        if (this.f281o.f240c == 3) {
            return;
        }
        SpannableStringBuilder spannableStringBuilder = this.f278l;
        spannableStringBuilder.clear();
        String strM43j2 = this.f107d.m43j(0);
        C0032d c0032d2 = this.f281o.f238a;
        boolean zM81a = c0032d2.m81a();
        C0030b[] c0030bArr = c0032d2.f228d;
        C0030b c0030b = zM81a ? c0030bArr[0] : c0030bArr[1];
        String str = "";
        String string = "";
        if (c0030b != null) {
            int i5 = 0;
            while (true) {
                if (i5 >= strM43j2.length()) {
                    break;
                }
                int i6 = c0030b.f207c + i5;
                if (i6 > strM43j2.length()) {
                    string = string + strM43j2.substring(i5, i5 + 1);
                    break;
                }
                StringBuilder sb = new StringBuilder();
                sb.append(string);
                String strSubstring = strM43j2.substring(i5, i6);
                C0030b c0030b2 = c0030bArr[0];
                C0030b c0030b3 = c0030bArr[1];
                if (c0032d2.m81a()) {
                    if (c0030b2 != null && strSubstring.length() == c0030b2.f207c) {
                        C0030b.f203g = false;
                        int iM74a = c0030b2.m74a(strSubstring) - 1;
                        if (iM74a >= 0) {
                            strSubstring = c0030b2.m77d(iM74a);
                        }
                    }
                    if (c0030b3 != null) {
                        C0030b.f203g = false;
                        int iM74a2 = c0030b3.m74a(strSubstring) - 1;
                        if (iM74a2 >= 0) {
                            strSubstring = c0030b3.m77d(iM74a2);
                        }
                    }
                } else if (c0030b3 != null) {
                    C0030b.f203g = false;
                    int iM74a3 = c0030b3.m74a(strSubstring) - 1;
                    if (iM74a3 >= 0) {
                        strSubstring = c0030b3.m77d(iM74a3);
                    }
                }
                sb.append(strSubstring);
                string = sb.toString();
                i5 = i6;
            }
        }
        if (this.f281o.f238a.m81a() && strM43j2.length() == 3 && strM43j2.substring(0, 2).equals("22")) {
            switch (Integer.valueOf(strM43j2.substring(2, 3)).intValue()) {
                case 0:
                    str = " (注音符號)";
                    break;
                case 1:
                    str = " (標點符號)";
                    break;
                case 2:
                    str = " (括號符號)";
                    break;
                case 3:
                    str = " (一般符號)";
                    break;
                case 4:
                    str = " (數學符號)";
                    break;
                case 5:
                    str = " (方向符號)";
                    break;
                case 6:
                    str = " (單位符號)";
                    break;
                case 7:
                    str = " (圖表符號)";
                    break;
                case 8:
                    str = " (順序符號)";
                    break;
                case 9:
                    str = " (希臘符號)";
                    break;
            }
            string = string + str;
        }
        this.f104a.m31x(string, false, false);
        this.f104a.m30w(string);
        spannableStringBuilder.insert(0, (CharSequence) string);
        int length2 = string.length();
        if (spannableStringBuilder.length() != 0) {
            if (length2 > 0 && length2 < spannableStringBuilder.length()) {
                spannableStringBuilder.setSpan(f265L, 0, length2, 33);
            }
            if (length2 < spannableStringBuilder.length()) {
                spannableStringBuilder.setSpan(f266M, length2, spannableStringBuilder.length(), 33);
                spannableStringBuilder.setSpan(f267N, 0, spannableStringBuilder.length(), 33);
            }
            spannableStringBuilder.setSpan(f268O, 0, spannableStringBuilder.length(), 33);
        }
        int i7 = length2 != 0 ? 1 : 0;
        if (spannableStringBuilder.length() >= 0) {
            this.f108e.setComposingText(spannableStringBuilder, i7);
            if (spannableStringBuilder.length() > 0) {
                this.f104a.m8H();
                this.f104a.m7G();
            } else if (spannableStringBuilder.length() == 0) {
                this.f104a.m16i();
                GestureDetectorOnGestureListenerC0001a0 gestureDetectorOnGestureListenerC0001a0 = this.f104a;
                PopupWindow popupWindow = gestureDetectorOnGestureListenerC0001a0.f65s0;
                if (popupWindow != null && popupWindow.isShowing()) {
                    gestureDetectorOnGestureListenerC0001a0.f65s0.dismiss();
                }
            }
        }
    }

    /* JADX INFO: renamed from: o */
    public final void m117o() {
        C0035g c0035g = C0035g.f236f;
        C0036h c0036h = null;
        if (c0035g == null) {
            c0035g = null;
        }
        this.f281o = c0035g;
        if (c0035g == null || !m110h()) {
            this.f106c = null;
            this.f284r = null;
            this.f287u = false;
        } else {
            C0033e c0033eM89b = this.f281o.m89b();
            this.f283q = c0033eM89b;
            this.f106c = c0033eM89b;
            C0032d c0032d = this.f281o.f238a;
            c0032d.getClass();
            C0036h c0036h2 = new C0036h(c0032d);
            if (C0036h.f243a != null && C0036h.f244b != null) {
                c0036h = c0036h2;
            }
            this.f284r = c0036h;
            this.f110g = false;
            this.f287u = true;
            this.f281o.m92e();
        }
        C0029a c0029a = this.f105b;
        c0029a.m68f(c0029a.f199m.m91d());
        C0002b c0002b = this.f107d;
        if (c0002b != null && this.f108e != null) {
            c0002b.m34a();
            this.f278l.clear();
            HandlerC0009g handlerC0009g = this.f277I;
            handlerC0009g.removeMessages(0);
            handlerC0009g.sendMessageDelayed(handlerC0009g.obtainMessage(0), 0L);
            this.f108e.setComposingText("", 0);
        }
        this.f104a.m16i();
        showStatusIcon(this.f281o.m90c());
    }

    @Override // android.inputmethodservice.InputMethodService, android.inputmethodservice.AbstractInputMethodService, android.app.Service, android.content.ComponentCallbacks
    public final void onConfigurationChanged(Configuration configuration) {
        try {
            super.onConfigurationChanged(configuration);
            if (this.f108e != null) {
                int i = configuration.hardKeyboardHidden;
                if (this.f274F.orientation == configuration.orientation || this.f275G.booleanValue()) {
                    return;
                }
                C0029a c0029a = this.f105b;
                c0029a.getClass();
                c0029a.f199m.getClass();
                c0029a.m68f(c0029a.f199m.m91d());
                this.f274F = configuration;
                this.f275G = Boolean.TRUE;
            }
        } catch (Exception unused) {
        }
    }

    @Override // p000a.AbstractC0010h, android.inputmethodservice.InputMethodService, android.app.Service
    public final void onCreate() {
        super.onCreate();
        this.f279m = getResources().getString(R.string.word_separators);
        if (C0035g.f236f == null) {
            C0035g.f236f = new C0035g(this);
        }
        C0035g c0035g = C0035g.f236f;
        this.f281o = c0035g;
        c0035g.m100m();
        if (this.f283q == null) {
            this.f283q = this.f281o.m89b();
        }
        if (this.f285s == null) {
            this.f285s = new C0023u(this);
        }
    }

    @Override // p000a.AbstractC0010h, android.inputmethodservice.InputMethodService
    public final View onCreateInputView() {
        this.f105b.f194h = super.onEvaluateInputViewShown();
        return super.onCreateInputView();
    }

    @Override // android.inputmethodservice.InputMethodService
    public final boolean onEvaluateFullscreenMode() {
        return PreferenceManager.getDefaultSharedPreferences(this).getBoolean("fullscreen", false) && getResources().getConfiguration().orientation == 2;
    }

    @Override // android.inputmethodservice.InputMethodService
    public final boolean onEvaluateInputViewShown() {
        super.onEvaluateInputViewShown();
        return true;
    }

    @Override // p000a.AbstractC0010h, android.inputmethodservice.InputMethodService
    public final void onStartInputView(EditorInfo editorInfo, boolean z) {
        SharedPreferences defaultSharedPreferences = PreferenceManager.getDefaultSharedPreferences(this);
        if (Build.VERSION.SDK_INT >= 23 && !Settings.canDrawOverlays(this)) {
            Intent intent = new Intent("android.settings.action.MANAGE_OVERLAY_PERMISSION", Uri.parse("package:" + getPackageName()));
            intent.addFlags(268435456);
            startActivity(intent);
            System.exit(0);
            return;
        }
        if (z) {
            super.onStartInputView(editorInfo, z);
        } else {
            this.f270B = defaultSharedPreferences.getBoolean("auto_hide_candidate", false);
            GestureDetectorOnGestureListenerC0001a0 gestureDetectorOnGestureListenerC0001a0 = this.f104a;
            if (gestureDetectorOnGestureListenerC0001a0 != null && gestureDetectorOnGestureListenerC0001a0.f34d.getVisibility() == 0) {
                this.f104a.m9b();
                GestureDetectorOnGestureListenerC0001a0 gestureDetectorOnGestureListenerC0001a02 = this.f104a;
                if (gestureDetectorOnGestureListenerC0001a02 instanceof GestureDetectorOnGestureListenerC0001a0) {
                    gestureDetectorOnGestureListenerC0001a02.f72w = this.f270B;
                }
            }
            this.f269A = defaultSharedPreferences.getBoolean("auto_caps", false);
            this.f289w = 0;
            this.f291y = 0;
            showStatusIcon(this.f281o.m90c());
            this.f281o.m92e();
            C0002b c0002b = this.f107d;
            if (c0002b != null) {
                c0002b.m34a();
            }
            this.f104a.m16i();
        }
        if (editorInfo.inputType == 0) {
            this.f110g = true;
        } else {
            this.f271C = false;
            this.f287u = defaultSharedPreferences.getBoolean("opt_en_prediction", false);
            defaultSharedPreferences.getBoolean("opt_en_spell_correction", true);
            this.f288v = defaultSharedPreferences.getBoolean("opt_en_enable_learning", true);
            int i = editorInfo.inputType;
            int i2 = i & 15;
            if (i2 == 1) {
                int i3 = i & 4080;
                if (i3 == 128 || i3 == 144 || i3 == 192) {
                    this.f288v = false;
                    this.f287u = false;
                }
            } else if (i2 == 2 || i2 == 3 || i2 == 4) {
                this.f287u = false;
                this.f288v = false;
            }
            if (!this.f287u) {
                this.f288v = false;
            }
            if (this.f106c != null) {
                this.f287u = true;
            }
        }
        this.f104a.f72w = false;
        this.f276H = Boolean.TRUE;
        this.f281o.f238a.getClass();
        if (C0032d.f221e != 0) {
            m117o();
            return;
        }
        int i4 = getResources().getConfiguration().keyboard;
        setCandidatesViewShown(true);
        this.f106c = null;
        this.f284r = null;
        this.f287u = false;
        showStatusIcon(this.f281o.m90c());
    }

    @Override // android.inputmethodservice.InputMethodService
    public final void onUpdateSelection(int i, int i2, int i3, int i4, int i5, int i6) {
    }

    @Override // p000a.AbstractC0010h, android.inputmethodservice.InputMethodService
    public final void setCandidatesViewShown(boolean z) {
        super.setCandidatesViewShown(z);
        if (!z) {
            this.f104a.m9b();
            return;
        }
        HandlerC0009g handlerC0009g = this.f277I;
        handlerC0009g.removeMessages(0);
        handlerC0009g.sendMessageDelayed(handlerC0009g.obtainMessage(0), 0L);
    }
}
