package com.minn.android.hanziime;

import android.app.FragmentTransaction;
import android.content.Intent;
import android.net.Uri;
import android.os.Build;
import android.os.Bundle;
import android.preference.PreferenceActivity;
import android.provider.Settings;
import p000a.C0011i;

/* JADX INFO: loaded from: classes.dex */
public class HzimSettingsActivity extends PreferenceActivity {
    @Override // android.app.Activity
    public final Intent getIntent() {
        return new Intent(super.getIntent());
    }

    @Override // android.preference.PreferenceActivity, android.app.Activity
    public final void onCreate(Bundle bundle) {
        super.onCreate(bundle);
        if (Build.VERSION.SDK_INT >= 23 && !Settings.canDrawOverlays(this)) {
            Intent intent = new Intent("android.settings.action.MANAGE_OVERLAY_PERMISSION");
            intent.setData(Uri.parse("package:" + getPackageName()));
            startActivityForResult(intent, 0);
        }
        setTitle(R.string.settings_name);
        FragmentTransaction fragmentTransactionBeginTransaction = getFragmentManager().beginTransaction();
        fragmentTransactionBeginTransaction.replace(android.R.id.content, new C0011i());
        fragmentTransactionBeginTransaction.commit();
    }
}
