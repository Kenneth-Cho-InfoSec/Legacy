package p000a;

import android.app.Dialog;
import android.content.Context;
import android.graphics.drawable.Drawable;
import android.widget.TextView;

/* JADX INFO: renamed from: a.a */
/* JADX INFO: loaded from: classes.dex */
public final class C0000a extends TextView {

    /* JADX INFO: renamed from: a */
    public Dialog f0a;

    public C0000a(Context context) {
        super(context);
        this.f0a = null;
        setSoundEffectsEnabled(false);
    }

    @Override // android.view.View
    public final void onWindowVisibilityChanged(int i) {
        Dialog dialog;
        super.onWindowVisibilityChanged(i);
        if (i == 0 || (dialog = this.f0a) == null) {
            return;
        }
        dialog.dismiss();
    }

    @Override // android.view.View
    public void setBackgroundDrawable(Drawable drawable) {
        super.setBackgroundDrawable(drawable);
        setPadding(20, 0, 20, 0);
    }
}
