package p001b;

import android.content.Context;
import android.graphics.drawable.Drawable;
import android.text.SpannableStringBuilder;
import android.view.LayoutInflater;
import android.view.View;
import android.view.ViewGroup;
import android.widget.PopupWindow;
import android.widget.TextView;
import com.minn.android.hanziime.R;

/* JADX INFO: renamed from: b.i */
/* JADX INFO: loaded from: classes.dex */
public final class C0037i {

    /* JADX INFO: renamed from: a */
    public Drawable f245a;

    /* JADX INFO: renamed from: b */
    public int f246b;

    /* JADX INFO: renamed from: c */
    public int f247c;

    /* JADX INFO: renamed from: d */
    public int f248d;

    /* JADX INFO: renamed from: e */
    public int f249e;

    /* JADX INFO: renamed from: f */
    public SpannableStringBuilder f250f;

    /* JADX INFO: renamed from: g */
    public PopupWindow f251g;

    /* JADX INFO: renamed from: h */
    public TextView f252h;

    /* JADX INFO: renamed from: i */
    public View f253i;

    /* JADX INFO: renamed from: j */
    public final /* synthetic */ ViewOnTouchListenerC0038j f254j;

    public C0037i(ViewOnTouchListenerC0038j viewOnTouchListenerC0038j, Context context, View view, int i) {
        this.f254j = viewOnTouchListenerC0038j;
        m101a(context, view, i, context.getResources().getText(R.string.tip_en_end_of_tutorial), R.string.touch_to_finish);
    }

    /* JADX INFO: renamed from: a */
    public final void m101a(Context context, View view, int i, CharSequence charSequence, int i2) {
        this.f245a = context.getResources().getDrawable(R.drawable.dialog_bubble);
        this.f246b = i;
        this.f247c = 0;
        double width = view.getWidth();
        Double.isNaN(width);
        this.f248d = (int) (width * 0.9d);
        this.f249e = 51;
        this.f250f = new SpannableStringBuilder().append(charSequence).append((CharSequence) "\n").append(context.getResources().getText(i2));
        this.f253i = view;
        PopupWindow popupWindow = new PopupWindow(context);
        this.f251g = popupWindow;
        popupWindow.setBackgroundDrawable(null);
        TextView textView = (TextView) ((LayoutInflater) context.getSystemService("layout_inflater")).inflate(R.layout.bubble_text, (ViewGroup) null);
        this.f252h = textView;
        textView.setBackgroundDrawable(this.f245a);
        this.f252h.setText(this.f250f);
        this.f251g.setContentView(this.f252h);
        this.f251g.setFocusable(false);
        this.f251g.setTouchable(true);
        this.f251g.setOutsideTouchable(false);
    }

    public C0037i(ViewOnTouchListenerC0038j viewOnTouchListenerC0038j, Context context, View view, int i, SpannableStringBuilder spannableStringBuilder) {
        this.f254j = viewOnTouchListenerC0038j;
        m101a(context, view, i, spannableStringBuilder, R.string.touch_to_continue);
    }
}
