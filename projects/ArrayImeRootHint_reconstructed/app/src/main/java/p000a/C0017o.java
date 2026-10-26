package p000a;

import com.minn.android.hanziime.KeyboardView;

/* JADX INFO: renamed from: a.o */
/* JADX INFO: loaded from: classes.dex */
public final class C0017o implements InterfaceC0018p {

    /* JADX INFO: renamed from: a */
    public final /* synthetic */ KeyboardView f166a;

    public C0017o(KeyboardView keyboardView) {
        this.f166a = keyboardView;
    }

    @Override // p000a.InterfaceC0018p
    /* JADX INFO: renamed from: a */
    public final void mo59a(int i, int[] iArr) {
        KeyboardView keyboardView = this.f166a;
        keyboardView.f297A.mo59a(i, iArr);
        keyboardView.m121d();
    }

    @Override // p000a.InterfaceC0018p
    /* JADX INFO: renamed from: c */
    public final void mo61c(int i) {
        this.f166a.f297A.mo61c(i);
    }

    @Override // p000a.InterfaceC0018p
    /* JADX INFO: renamed from: d */
    public final void mo62d(int i) {
        this.f166a.f297A.mo62d(i);
    }

    @Override // p000a.InterfaceC0018p
    /* JADX INFO: renamed from: e */
    public final void mo63e(CharSequence charSequence) {
        KeyboardView keyboardView = this.f166a;
        keyboardView.f297A.mo63e(charSequence);
        keyboardView.m121d();
    }

    @Override // p000a.InterfaceC0018p
    /* JADX INFO: renamed from: b */
    public final void mo60b(C0013k c0013k) {
    }
}
