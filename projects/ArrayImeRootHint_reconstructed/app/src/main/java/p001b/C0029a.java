package p001b;

import android.media.AudioManager;
import android.os.Vibrator;
import android.view.KeyEvent;
import android.view.inputmethod.EditorInfo;
import android.view.inputmethod.InputConnection;
import com.minn.android.hanziime.BaseInputView;
import com.minn.android.hanziime.HzimKeyboardView;
import com.minn.android.hanziime.KeyboardView;
import p000a.AbstractC0010h;
import p000a.C0004c;
import p000a.C0007e;
import p000a.C0013k;
import p000a.C0015m;
import p000a.InterfaceC0018p;

/* JADX INFO: renamed from: b.a */
/* JADX INFO: loaded from: classes.dex */
public final class C0029a implements InterfaceC0018p {

    /* JADX INFO: renamed from: a */
    public AbstractC0010h f187a;

    /* JADX INFO: renamed from: b */
    public KeyboardView f188b;

    /* JADX INFO: renamed from: c */
    public BaseInputView f189c;

    /* JADX INFO: renamed from: d */
    public C0015m f190d;

    /* JADX INFO: renamed from: e */
    public boolean f191e;

    /* JADX INFO: renamed from: g */
    public int f193g;

    /* JADX INFO: renamed from: n */
    public AbstractC0010h f200n;

    /* JADX INFO: renamed from: f */
    public int f192f = 0;

    /* JADX INFO: renamed from: h */
    public boolean f194h = true;

    /* JADX INFO: renamed from: i */
    public boolean f195i = true;

    /* JADX INFO: renamed from: j */
    public Vibrator f196j = null;

    /* JADX INFO: renamed from: k */
    public AudioManager f197k = null;

    /* JADX INFO: renamed from: l */
    public boolean f198l = false;

    /* JADX INFO: renamed from: m */
    public C0035g f199m = null;

    /* JADX WARN: Can't fix incorrect switch cases order, some code will duplicate */
    /* JADX WARN: Removed duplicated region for block: B:40:0x009b  */
    /* JADX WARN: Removed duplicated region for block: B:54:0x00ec  */
    /* JADX WARN: Removed duplicated region for block: B:55:0x00fc  */
    @Override // p000a.InterfaceC0018p
    /* JADX INFO: renamed from: a */
    /*
        Code decompiled incorrectly, please refer to instructions dump.
    */
    public final void mo59a(int i, int[] iArr) {
        AbstractC0010h abstractC0010h = null;
        C0004c c0004c = null;
        AbstractC0010h abstractC0010h2;
        C0004c c0004c2;
        int i2 = 0;
        if (i == -230) {
            abstractC0010h = this.f200n;
            c0004c = new C0004c(-268435404);
            abstractC0010h.mo55a(c0004c);
        } else if (i != -214) {
            try {
                if (i == -116) {
                    throw null;
                }
                if (i == -106) {
                    abstractC0010h = this.f200n;
                    c0004c = new C0004c(-268435403);
                } else if (i != -1) {
                    if (i == -104) {
                        abstractC0010h = this.f200n;
                        c0004c = new C0004c(-268435407);
                    } else if (i == -103) {
                        int i3 = this.f193g;
                        if (i3 == 0 || i3 == 1) {
                            throw null;
                        }
                    } else if (i == -101) {
                        abstractC0010h = this.f200n;
                        c0004c = new C0004c(0, new KeyEvent(0, 66));
                    } else if (i != -100) {
                        switch (i) {
                            case -403:
                                abstractC0010h2 = this.f200n;
                                c0004c2 = new C0004c(-268435408, 0);
                                abstractC0010h2.mo55a(c0004c2);
                                break;
                            case -402:
                                abstractC0010h2 = this.f200n;
                                c0004c2 = new C0004c(-268435408, 1);
                                abstractC0010h2.mo55a(c0004c2);
                                break;
                            case -401:
                                abstractC0010h2 = this.f200n;
                                c0004c2 = new C0004c(-268435408, 2);
                                abstractC0010h2.mo55a(c0004c2);
                                break;
                            default:
                                switch (i) {
                                    case -218:
                                        abstractC0010h = this.f200n;
                                        c0004c = new C0004c(0, new KeyEvent(0, 21));
                                        break;
                                    case -217:
                                        this.f200n.mo55a(new C0004c(0, new KeyEvent(0, 22)));
                                        abstractC0010h2 = this.f200n;
                                        c0004c2 = new C0004c(-268435408, 0);
                                        abstractC0010h2.mo55a(c0004c2);
                                        break;
                                    case -216:
                                        break;
                                    default:
                                        switch (i) {
                                            case -114:
                                                break;
                                            case -113:
                                                throw null;
                                            case -112:
                                                abstractC0010h = this.f200n;
                                                c0004c = new C0004c(-268435406);
                                                break;
                                            default:
                                                if (i >= 0) {
                                                    C0015m c0015m = this.f188b.f326b;
                                                    if (c0015m != null ? c0015m.f152e : false) {
                                                        i = Character.toUpperCase(i);
                                                    }
                                                    abstractC0010h = this.f200n;
                                                    c0004c = new C0004c((char) i);
                                                }
                                                break;
                                        }
                                        break;
                                }
                                break;
                        }
                    } else {
                        abstractC0010h = this.f200n;
                        c0004c = new C0004c(0, new KeyEvent(0, 67));
                    }
                } else if (this.f192f == 0) {
                    C0007e c0007eM91d = this.f199m.m91d();
                    this.f192f = 1;
                    m68f(c0007eM91d);
                    this.f191e = true;
                } else {
                    C0007e c0007eM91d2 = this.f199m.m91d();
                    this.f192f = 0;
                    m68f(c0007eM91d2);
                    this.f191e = false;
                }
                if (abstractC0010h != null && c0004c != null) {
                    abstractC0010h.mo55a(c0004c);
                }
            } catch (Exception unused) {
            }
        }
        if (this.f191e || i == -1) {
            return;
        }
        if (this.f193g != 1) {
            if (this.f198l) {
                EditorInfo currentInputEditorInfo = this.f200n.getCurrentInputEditorInfo();
                InputConnection currentInputConnection = this.f200n.getCurrentInputConnection();
                if (currentInputConnection != null) {
                    i2 = currentInputConnection.getCursorCapsMode(currentInputEditorInfo.inputType) == 0 ? 0 : 1;
                }
            }
            if (i2 == this.f192f) {
                return;
            }
        }
        C0007e c0007eM91d3 = this.f199m.m91d();
        this.f192f = i2;
        m68f(c0007eM91d3);
    }

    /* JADX WARN: Removed duplicated region for block: B:18:0x0020  */
    /* JADX WARN: Removed duplicated region for block: B:19:0x0022  */
    /* JADX WARN: Removed duplicated region for block: B:20:0x0024  */
    @Override // p000a.InterfaceC0018p
    /* JADX INFO: renamed from: c */
    /*
        Code decompiled incorrectly, please refer to instructions dump.
    */
    public final void mo61c(int i) {
        int i2 = 5;
        Vibrator vibrator = this.f196j;
        if (vibrator != null) {
            try {
                vibrator.vibrate(5L);
            } catch (Exception unused) {
            }
        }
        AudioManager audioManager = this.f197k;
        if (audioManager != null) {
            if (i == -101) {
                i2 = 8;
            } else if (i == -100) {
                i2 = 7;
            } else if (i != 32) {
                switch (i) {
                    case -216:
                        break;
                    case -215:
                        i2 = 6;
                        break;
                    case -214:
                        break;
                    default:
                        i2 = 5;
                        break;
                }
            }
            audioManager.playSoundEffect(i2, -1.0f);
        }
    }

    @Override // p000a.InterfaceC0018p
    /* JADX INFO: renamed from: e */
    public final void mo63e(CharSequence charSequence) {
        this.f200n.getCurrentInputConnection().commitText(charSequence, 0);
    }

    /* JADX INFO: renamed from: f */
    public final void m68f(C0007e c0007e) {
        if (this.f190d == c0007e) {
            this.f188b.m130n(this.f192f != 0);
            return;
        }
        this.f188b.setKeyboard(c0007e);
        this.f188b.m130n(this.f192f != 0);
        this.f190d = c0007e;
    }

    /* JADX INFO: renamed from: g */
    public final void m69g() {
        this.f188b.setVisibility(8);
    }

    /* JADX INFO: renamed from: h */
    public final void m70h(AbstractC0010h abstractC0010h) {
        try {
            if (abstractC0010h.f107d.m42i(1) == 0) {
                if (this.f195i) {
                    return;
                }
                this.f195i = true;
                throw null;
            }
            if (this.f195i) {
                this.f195i = false;
                throw null;
            }
        } catch (Exception unused) {
        }
    }

    /* JADX INFO: renamed from: i */
    public final void m71i(int i) {
        ((HzimKeyboardView) this.f188b).setEnterKeyAs(i);
    }

    /* JADX INFO: renamed from: j */
    public final void m72j(String str) {
        if (str.length() != 1 || str.charAt(0) < '0' || str.charAt(0) > '9') {
            ((HzimKeyboardView) this.f188b).setKeyPreviewAs("");
        } else {
            ((HzimKeyboardView) this.f188b).setKeyPreviewAs(str);
        }
    }

    /* JADX INFO: renamed from: k */
    public final void m73k() {
        KeyboardView keyboardView = this.f188b;
        if (keyboardView != null) {
            keyboardView.setVisibility(0);
        }
    }

    @Override // p000a.InterfaceC0018p
    /* JADX INFO: renamed from: b */
    public final void mo60b(C0013k c0013k) {
    }

    @Override // p000a.InterfaceC0018p
    /* JADX INFO: renamed from: d */
    public final void mo62d(int i) {
    }
}
