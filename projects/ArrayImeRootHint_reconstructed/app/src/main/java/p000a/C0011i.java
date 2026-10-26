package p000a;

import android.os.Bundle;
import android.preference.PreferenceFragment;
import com.minn.android.hanziime.R;

/* JADX INFO: renamed from: a.i */
/* JADX INFO: loaded from: classes.dex */
public final class C0011i extends PreferenceFragment {
    @Override // android.preference.PreferenceFragment, android.app.Fragment
    public final void onCreate(Bundle bundle) {
        super.onCreate(bundle);
        addPreferencesFromResource(R.xml.ime_preferences);
    }
}
