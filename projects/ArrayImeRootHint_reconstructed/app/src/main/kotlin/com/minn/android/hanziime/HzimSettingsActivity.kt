package com.minn.android.hanziime

import android.content.Intent
import android.net.Uri
import android.os.Build
import android.os.Bundle
import android.preference.PreferenceActivity
import android.provider.Settings
import p000a.C0011i

class HzimSettingsActivity : PreferenceActivity() {
    override fun getIntent(): Intent = Intent(super.getIntent())

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        if (Build.VERSION.SDK_INT >= 23 && !Settings.canDrawOverlays(this)) {
            val intent = Intent("android.settings.action.MANAGE_OVERLAY_PERMISSION")
            intent.data = Uri.parse("package:$packageName")
            startActivityForResult(intent, 0)
        }
        setTitle(R.string.settings_name)
        fragmentManager.beginTransaction()
            .replace(android.R.id.content, C0011i())
            .commit()
    }
}
