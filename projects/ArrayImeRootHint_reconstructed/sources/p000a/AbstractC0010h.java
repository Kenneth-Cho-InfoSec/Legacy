package p000a;

import android.content.SharedPreferences;
import android.content.res.Resources;
import android.inputmethodservice.InputMethodService;
import android.os.Build;
import android.preference.PreferenceManager;
import android.util.Log;
import android.view.GestureDetector;
import android.view.KeyEvent;
import android.view.LayoutInflater;
import android.view.View;
import android.view.ViewGroup;
import android.view.WindowManager;
import android.view.inputmethod.EditorInfo;
import android.view.inputmethod.InputConnection;
import android.widget.AbsoluteLayout;
import android.widget.Button;
import android.widget.ImageView;
import android.widget.LinearLayout;
import android.widget.PopupWindow;
import android.widget.ScrollView;
import android.widget.TextView;
import com.minn.android.hanziime.BaseInputView;
import com.minn.android.hanziime.HzimMainTWService;
import com.minn.android.hanziime.KeyboardView;
import com.minn.android.hanziime.R;
import java.util.ArrayList;
import p001b.C0029a;
import p001b.C0035g;

/* JADX INFO: renamed from: a.h */
/* JADX INFO: loaded from: classes.dex */
public abstract class AbstractC0010h extends InputMethodService {

    /* JADX INFO: renamed from: k */
    public static AbstractC0010h f103k;

    /* JADX INFO: renamed from: h */
    public boolean f111h;

    /* JADX INFO: renamed from: a */
    public GestureDetectorOnGestureListenerC0001a0 f104a = null;

    /* JADX INFO: renamed from: b */
    public C0029a f105b = null;

    /* JADX INFO: renamed from: c */
    public InterfaceC0003b0 f106c = null;

    /* JADX INFO: renamed from: d */
    public C0002b f107d = null;

    /* JADX INFO: renamed from: e */
    public InputConnection f108e = null;

    /* JADX INFO: renamed from: f */
    public boolean f109f = true;

    /* JADX INFO: renamed from: g */
    public boolean f110g = true;

    /* JADX INFO: renamed from: i */
    public GestureDetectorOnGestureListenerC0001a0 f112i = null;

    /* JADX INFO: renamed from: j */
    public final ArrayList f113j = new ArrayList();

    /* JADX INFO: renamed from: a */
    public abstract boolean mo55a(C0004c c0004c);

    @Override // android.inputmethodservice.InputMethodService
    public void hideWindow() {
        super.hideWindow();
        this.f110g = true;
        hideStatusIcon();
    }

    @Override // android.inputmethodservice.InputMethodService
    public final void onComputeInsets(InputMethodService.Insets insets) {
        super.onComputeInsets(insets);
        insets.contentTopInsets = insets.visibleTopInsets;
    }

    @Override // android.inputmethodservice.InputMethodService, android.app.Service
    public void onCreate() {
        int i = getResources().getConfiguration().screenLayout;
        super.onCreate();
        PreferenceManager.getDefaultSharedPreferences(this);
        f103k = this;
        GestureDetectorOnGestureListenerC0001a0 gestureDetectorOnGestureListenerC0001a0 = new GestureDetectorOnGestureListenerC0001a0();
        this.f112i = gestureDetectorOnGestureListenerC0001a0;
        this.f104a = gestureDetectorOnGestureListenerC0001a0;
        InterfaceC0003b0 interfaceC0003b0 = this.f106c;
        if (interfaceC0003b0 != null) {
            interfaceC0003b0.mo48d();
        }
        C0002b c0002b = this.f107d;
        if (c0002b != null) {
            c0002b.m34a();
        }
    }

    @Override // android.inputmethodservice.InputMethodService
    public final View onCreateCandidatesView() {
        if (this.f104a == null) {
            return super.onCreateCandidatesView();
        }
        WindowManager windowManager = (WindowManager) getSystemService("window");
        GestureDetectorOnGestureListenerC0001a0 gestureDetectorOnGestureListenerC0001a0 = this.f112i;
        this.f104a = gestureDetectorOnGestureListenerC0001a0;
        int width = windowManager.getDefaultDisplay().getWidth();
        windowManager.getDefaultDisplay().getHeight();
        gestureDetectorOnGestureListenerC0001a0.f52m = this;
        gestureDetectorOnGestureListenerC0001a0.f58p = width;
        Resources resources = getResources();
        gestureDetectorOnGestureListenerC0001a0.f60q = resources.getDimensionPixelSize(R.dimen.cand_minimum_width);
        gestureDetectorOnGestureListenerC0001a0.f62r = resources.getDimensionPixelSize(R.dimen.cand_minimum_height);
        gestureDetectorOnGestureListenerC0001a0.f64s = resources.getDimensionPixelSize(R.dimen.cand_category_minimum_height);
        gestureDetectorOnGestureListenerC0001a0.f66t = resources.getDimensionPixelSize(R.dimen.cand_left_align_threshold);
        gestureDetectorOnGestureListenerC0001a0.f68u = resources.getDimensionPixelSize(R.dimen.keyboard_height);
        gestureDetectorOnGestureListenerC0001a0.f70v = resources.getDimensionPixelSize(R.dimen.symbol_keyboard_height);
        resources.getDrawable(R.drawable.tab_no_select).getMinimumHeight();
        gestureDetectorOnGestureListenerC0001a0.f56o = resources.getConfiguration().orientation != 2;
        gestureDetectorOnGestureListenerC0001a0.f39f0 = resources.getDimensionPixelSize(R.dimen.cand_normal_text_size);
        gestureDetectorOnGestureListenerC0001a0.f41g0 = resources.getDimensionPixelSize(R.dimen.cand_category_text_size);
        LayoutInflater layoutInflater = getLayoutInflater();
        ViewGroup viewGroup = (ViewGroup) layoutInflater.inflate(R.layout.candidates, (ViewGroup) null);
        gestureDetectorOnGestureListenerC0001a0.f34d = viewGroup;
        gestureDetectorOnGestureListenerC0001a0.f36e = (TextView) viewGroup.findViewById(R.id.candview_symbol);
        gestureDetectorOnGestureListenerC0001a0.f38f = (TextView) gestureDetectorOnGestureListenerC0001a0.f34d.findViewById(R.id.candview_emoticon);
        gestureDetectorOnGestureListenerC0001a0.f40g = (ScrollView) gestureDetectorOnGestureListenerC0001a0.f34d.findViewById(R.id.candview_scroll);
        gestureDetectorOnGestureListenerC0001a0.f42h = (ViewGroup) gestureDetectorOnGestureListenerC0001a0.f34d.findViewById(R.id.candview_base);
        SharedPreferences defaultSharedPreferences = PreferenceManager.getDefaultSharedPreferences(gestureDetectorOnGestureListenerC0001a0.f52m);
        gestureDetectorOnGestureListenerC0001a0.f31b0 = Integer.parseInt(defaultSharedPreferences.getString("setting_portrait", "1"));
        gestureDetectorOnGestureListenerC0001a0.f33c0 = Integer.parseInt(defaultSharedPreferences.getString("setting_landscape", "1"));
        LinearLayout linearLayout = (LinearLayout) gestureDetectorOnGestureListenerC0001a0.f34d.findViewById(R.id.candidates_1st_view);
        gestureDetectorOnGestureListenerC0001a0.f46j = linearLayout;
        linearLayout.setOnClickListener(gestureDetectorOnGestureListenerC0001a0.f73w0);
        gestureDetectorOnGestureListenerC0001a0.f50l = (LinearLayout) gestureDetectorOnGestureListenerC0001a0.f34d.findViewById(R.id.candview_tab);
        TextView textView = gestureDetectorOnGestureListenerC0001a0.f36e;
        ViewOnClickListenerC0025w viewOnClickListenerC0025w = gestureDetectorOnGestureListenerC0001a0.f2A0;
        textView.setOnClickListener(viewOnClickListenerC0025w);
        gestureDetectorOnGestureListenerC0001a0.f38f.setOnClickListener(viewOnClickListenerC0025w);
        int i = gestureDetectorOnGestureListenerC0001a0.f58p;
        for (int i2 = 0; i2 < 5; i2++) {
            LinearLayout linearLayout2 = new LinearLayout(gestureDetectorOnGestureListenerC0001a0.f40g.getContext());
            linearLayout2.setOrientation(0);
            linearLayout2.setLayoutParams(new LinearLayout.LayoutParams(-1, -2));
            for (int i3 = 0; i3 < i / gestureDetectorOnGestureListenerC0001a0.f60q; i3++) {
                linearLayout2.addView(gestureDetectorOnGestureListenerC0001a0.m13f());
            }
            if (i2 == 0) {
                C0000a c0000aM13f = gestureDetectorOnGestureListenerC0001a0.m13f();
                LinearLayout.LayoutParams layoutParams = new LinearLayout.LayoutParams(-2, -2);
                layoutParams.weight = 0.0f;
                layoutParams.gravity = 5;
                c0000aM13f.setLayoutParams(layoutParams);
                linearLayout2.addView(c0000aM13f);
                gestureDetectorOnGestureListenerC0001a0.f15N = c0000aM13f;
            }
            gestureDetectorOnGestureListenerC0001a0.f46j.addView(linearLayout2);
        }
        gestureDetectorOnGestureListenerC0001a0.f48k = (AbsoluteLayout) gestureDetectorOnGestureListenerC0001a0.f34d.findViewById(R.id.candidates_2nd_view);
        gestureDetectorOnGestureListenerC0001a0.f14M = resources.getColor(R.color.candidate_text);
        LinearLayout linearLayout3 = (LinearLayout) layoutInflater.inflate(R.layout.root_words, (ViewGroup) null);
        gestureDetectorOnGestureListenerC0001a0.f55n0 = linearLayout3;
        TextView textView2 = (TextView) linearLayout3.findViewById(R.id.textView);
        gestureDetectorOnGestureListenerC0001a0.f59p0 = textView2;
        gestureDetectorOnGestureListenerC0001a0.f61q0 = textView2.getLineHeight() * 2;
        PopupWindow popupWindow = gestureDetectorOnGestureListenerC0001a0.f57o0;
        if (popupWindow != null && popupWindow.isShowing()) {
            gestureDetectorOnGestureListenerC0001a0.f57o0.dismiss();
        }
        PopupWindow popupWindow2 = new PopupWindow(gestureDetectorOnGestureListenerC0001a0.f52m);
        gestureDetectorOnGestureListenerC0001a0.f57o0 = popupWindow2;
        popupWindow2.setClippingEnabled(false);
        gestureDetectorOnGestureListenerC0001a0.f57o0.setBackgroundDrawable(null);
        gestureDetectorOnGestureListenerC0001a0.f57o0.setInputMethodMode(2);
        gestureDetectorOnGestureListenerC0001a0.f57o0.setContentView(gestureDetectorOnGestureListenerC0001a0.f55n0);
        int i4 = Build.VERSION.SDK_INT;
        if (i4 >= 23) {
            gestureDetectorOnGestureListenerC0001a0.f57o0.setElevation(5.0f);
            gestureDetectorOnGestureListenerC0001a0.f57o0.setWindowLayoutType(1999);
        }
        gestureDetectorOnGestureListenerC0001a0.f63r0 = (LinearLayout) layoutInflater.inflate(R.layout.root_words2, (ViewGroup) null);
        PopupWindow popupWindow3 = gestureDetectorOnGestureListenerC0001a0.f65s0;
        if (popupWindow3 != null && popupWindow3.isShowing()) {
            gestureDetectorOnGestureListenerC0001a0.f65s0.dismiss();
        }
        PopupWindow popupWindow4 = new PopupWindow(gestureDetectorOnGestureListenerC0001a0.f52m);
        gestureDetectorOnGestureListenerC0001a0.f65s0 = popupWindow4;
        popupWindow4.setClippingEnabled(false);
        gestureDetectorOnGestureListenerC0001a0.f65s0.setBackgroundDrawable(null);
        gestureDetectorOnGestureListenerC0001a0.f65s0.setInputMethodMode(2);
        gestureDetectorOnGestureListenerC0001a0.f65s0.setContentView(gestureDetectorOnGestureListenerC0001a0.f63r0);
        if (i4 >= 23) {
            gestureDetectorOnGestureListenerC0001a0.f65s0.setElevation(5.0f);
            gestureDetectorOnGestureListenerC0001a0.f65s0.setWindowLayoutType(1999);
        }
        gestureDetectorOnGestureListenerC0001a0.f44i = (ImageView) (i4 >= 23 ? gestureDetectorOnGestureListenerC0001a0.f63r0 : gestureDetectorOnGestureListenerC0001a0.f34d).findViewById(R.id.read_more_button);
        double d = gestureDetectorOnGestureListenerC0001a0.f61q0;
        Double.isNaN(d);
        gestureDetectorOnGestureListenerC0001a0.f67t0 = (int) (d * 1.5d);
        gestureDetectorOnGestureListenerC0001a0.f44i.setOnTouchListener(new ViewOnTouchListenerC0028z(gestureDetectorOnGestureListenerC0001a0, 0));
        gestureDetectorOnGestureListenerC0001a0.f44i.setOnClickListener(new ViewOnClickListenerC0025w(gestureDetectorOnGestureListenerC0001a0, 4));
        gestureDetectorOnGestureListenerC0001a0.m6F(2);
        gestureDetectorOnGestureListenerC0001a0.f9H = new GestureDetector(gestureDetectorOnGestureListenerC0001a0);
        View viewInflate = layoutInflater.inflate(R.layout.candidate_longpress_dialog, (ViewGroup) null);
        gestureDetectorOnGestureListenerC0001a0.f28a = viewInflate;
        ((Button) viewInflate.findViewById(R.id.candidate_longpress_dialog_select)).setOnClickListener(new ViewOnClickListenerC0025w(gestureDetectorOnGestureListenerC0001a0, 5));
        ((Button) gestureDetectorOnGestureListenerC0001a0.f28a.findViewById(R.id.candidate_longpress_dialog_cancel)).setOnClickListener(new ViewOnClickListenerC0025w(gestureDetectorOnGestureListenerC0001a0, 0));
        LinearLayout linearLayout4 = (LinearLayout) gestureDetectorOnGestureListenerC0001a0.f34d.findViewById(R.id.infoLine);
        if (i4 >= 23) {
            linearLayout4.setVisibility(8);
            gestureDetectorOnGestureListenerC0001a0.f51l0 = null;
        } else {
            linearLayout4.setVisibility(0);
            gestureDetectorOnGestureListenerC0001a0.f51l0 = (TextView) gestureDetectorOnGestureListenerC0001a0.f34d.findViewById(R.id.textViewInfoArea);
        }
        int identifier = gestureDetectorOnGestureListenerC0001a0.f52m.getResources().getIdentifier("status_bar_height", "dimen", "android");
        if (identifier > 0) {
            gestureDetectorOnGestureListenerC0001a0.f69u0 = gestureDetectorOnGestureListenerC0001a0.f52m.getResources().getDimensionPixelSize(identifier);
        }
        ViewGroup viewGroup2 = gestureDetectorOnGestureListenerC0001a0.f34d;
        this.f104a.m6F(0);
        if (i4 >= 33) {
            setCandidatesViewShown(true);
            setExtractViewShown(onEvaluateFullscreenMode());
        }
        return viewGroup2;
    }

    @Override // android.inputmethodservice.InputMethodService
    public View onCreateInputView() {
        KeyboardView keyboardView;
        PreferenceManager.getDefaultSharedPreferences(this);
        if (this.f105b == null) {
            return super.onCreateInputView();
        }
        WindowManager windowManager = (WindowManager) getSystemService("window");
        C0029a c0029a = this.f105b;
        windowManager.getDefaultDisplay().getWidth();
        windowManager.getDefaultDisplay().getHeight();
        c0029a.f187a = this;
        int i = getResources().getConfiguration().orientation;
        c0029a.f200n = this;
        C0035g c0035g = C0035g.f236f;
        if (c0035g == null) {
            c0035g = null;
        }
        c0029a.f199m = c0035g;
        if (c0029a.f194h) {
            c0035g.getClass();
            c0029a.f199m.m91d();
        }
        new C0015m(this, R.xml.keyboard_4key);
        PreferenceManager.getDefaultSharedPreferences(this);
        KeyboardView keyboardView2 = (KeyboardView) c0029a.f187a.getLayoutInflater().inflate(R.layout.keyboard_android_default, (ViewGroup) null);
        c0029a.f188b = keyboardView2;
        keyboardView2.setOnKeyboardActionListener(c0029a);
        c0029a.f190d = null;
        c0029a.f189c = (BaseInputView) getLayoutInflater().inflate(R.layout.keyboard_default_main, (ViewGroup) null);
        if (c0029a.f194h && (keyboardView = c0029a.f188b) != null) {
            c0029a.f189c.addView(keyboardView);
        }
        BaseInputView baseInputView = c0029a.f189c;
        c0029a.f192f = 0;
        c0029a.f193g = 0;
        c0029a.f199m.getClass();
        c0029a.m68f(c0029a.f199m.m91d());
        return baseInputView;
    }

    @Override // android.inputmethodservice.InputMethodService, android.inputmethodservice.AbstractInputMethodService, android.app.Service
    public final void onDestroy() {
        super.onDestroy();
        f103k = null;
        HandlerC0009g handlerC0009g = ((HzimMainTWService) this).f277I;
        handlerC0009g.sendMessageDelayed(handlerC0009g.obtainMessage(2), 0L);
    }

    @Override // android.inputmethodservice.InputMethodService, android.view.KeyEvent.Callback
    public final boolean onKeyDown(int i, KeyEvent keyEvent) {
        boolean zMo55a = mo55a(new C0004c(keyEvent));
        this.f111h = zMo55a;
        C0012j c0012j = new C0012j();
        c0012j.f114a = zMo55a;
        c0012j.f115b = i;
        ArrayList arrayList = this.f113j;
        int size = arrayList.size();
        if (size != 0) {
            int i2 = 0;
            while (true) {
                if (i2 >= size) {
                    break;
                }
                if (((C0012j) arrayList.get(i2)).f115b == i) {
                    arrayList.remove(i2);
                    break;
                }
                i2++;
            }
        }
        arrayList.add(c0012j);
        boolean z = this.f111h;
        return !z ? super.onKeyDown(i, keyEvent) : z;
    }

    @Override // android.inputmethodservice.InputMethodService, android.view.KeyEvent.Callback
    public final boolean onKeyLongPress(int i, KeyEvent keyEvent) {
        if (f103k == null) {
            Log.e("h", "HzimService::onKeyLongPress()  Unprocessing onCreate() ");
            return super.onKeyLongPress(i, keyEvent);
        }
        C0004c c0004c = new C0004c(keyEvent);
        c0004c.f82a = -268435420;
        return mo55a(c0004c);
    }

    @Override // android.inputmethodservice.InputMethodService, android.view.KeyEvent.Callback
    public final boolean onKeyUp(int i, KeyEvent keyEvent) {
        boolean z = this.f111h;
        ArrayList arrayList = this.f113j;
        int size = arrayList.size();
        int i2 = 0;
        while (true) {
            if (i2 >= size) {
                break;
            }
            C0012j c0012j = (C0012j) arrayList.get(i2);
            if (c0012j.f115b == i) {
                z = c0012j.f114a;
                arrayList.remove(i2);
                break;
            }
            i2++;
        }
        return !z ? super.onKeyUp(i, keyEvent) : mo55a(new C0004c(keyEvent));
    }

    @Override // android.inputmethodservice.InputMethodService
    public final void onStartInput(EditorInfo editorInfo, boolean z) {
        C0002b c0002b;
        super.onStartInput(editorInfo, z);
        this.f108e = getCurrentInputConnection();
        if (!z && (c0002b = this.f107d) != null) {
            c0002b.m34a();
        }
        if (Build.VERSION.SDK_INT == 33) {
            setCandidatesViewShown(true);
        }
    }

    /* JADX WARN: Removed duplicated region for block: B:44:0x00e9  */
    /* JADX WARN: Type inference failed for: r3v6, types: [boolean, java.lang.String] */
    /*  JADX ERROR: JadxRuntimeException in pass: RegionMakerVisitor
        jadx.core.utils.exceptions.JadxRuntimeException: Can't find top splitter block for handler:B:29:0x00a3
        	at jadx.core.utils.BlockUtils.getTopSplitterForHandler(BlockUtils.java:1182)
        	at jadx.core.dex.visitors.regions.maker.ExcHandlersRegionMaker.collectHandlerRegions(ExcHandlersRegionMaker.java:53)
        	at jadx.core.dex.visitors.regions.maker.ExcHandlersRegionMaker.process(ExcHandlersRegionMaker.java:38)
        	at jadx.core.dex.visitors.regions.RegionMakerVisitor.visit(RegionMakerVisitor.java:27)
        */
    @Override // android.inputmethodservice.InputMethodService
    /*
        Code decompiled incorrectly, please refer to instructions dump.
    */
    public void onStartInputView(android.view.inputmethod.EditorInfo r13, boolean r14) {
        /*
            Method dump skipped, instruction units count: 270
            To view this dump add '--comments-level debug' option
        */
        throw new UnsupportedOperationException("Method not decompiled: p000a.AbstractC0010h.onStartInputView(android.view.inputmethod.EditorInfo, boolean):void");
    }

    @Override // android.inputmethodservice.InputMethodService
    public final void requestHideSelf(int i) {
        super.requestHideSelf(i);
        if (this.f105b == null) {
            hideWindow();
        }
    }

    @Override // android.inputmethodservice.InputMethodService
    public void setCandidatesViewShown(boolean z) {
        super.setCandidatesViewShown(z);
        if (z) {
            showWindow(true);
        } else if (this.f109f && this.f105b == null) {
            hideWindow();
        }
    }
}
