package com.kennethchoinfosec.secureeear.security;

import android.Manifest;
import android.os.Build;

import java.util.ArrayList;
import java.util.Collections;
import java.util.List;

public final class PermissionPolicy {
    private PermissionPolicy() {
    }

    public static List<String> runtimePermissions() {
        List<String> permissions = new ArrayList<>();
        if (Build.VERSION.SDK_INT >= 33) {
            permissions.add(Manifest.permission.NEARBY_WIFI_DEVICES);
        } else {
            permissions.add(Manifest.permission.ACCESS_COARSE_LOCATION);
            permissions.add(Manifest.permission.ACCESS_FINE_LOCATION);
        }
        return Collections.unmodifiableList(permissions);
    }
}
