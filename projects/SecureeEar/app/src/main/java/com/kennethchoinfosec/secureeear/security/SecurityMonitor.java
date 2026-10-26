package com.kennethchoinfosec.secureeear.security;

import android.content.Context;
import android.net.ConnectivityManager;
import android.net.DhcpInfo;
import android.net.Network;
import android.net.NetworkCapabilities;
import android.net.wifi.WifiInfo;
import android.net.wifi.WifiManager;
import android.os.Build;
import android.text.format.Formatter;

import com.jni.CallBack.WifiCallBackFuc;

import java.util.Arrays;
import java.util.Collections;
import java.util.HashSet;
import java.util.Locale;
import java.util.Set;

public final class SecurityMonitor {
    private static final Set<String> DEVICE_PREFIXES = Collections.unmodifiableSet(
            new HashSet<>(Arrays.asList("UseeEar", "i4season", "inskam", "Yanxuan")));
    private static final Set<String> LOCAL_CAMERA_HOSTS = Collections.unmodifiableSet(
            new HashSet<>(Arrays.asList("192.168.1.254", "10.10.10.254", "192.168.1.1", "127.0.0.1")));

    private final Context appContext;

    public SecurityMonitor(Context context) {
        this.appContext = context.getApplicationContext();
    }

    public WifiSnapshot wifiSnapshot() {
        WifiManager wifiManager = (WifiManager) appContext.getApplicationContext().getSystemService(Context.WIFI_SERVICE);
        if (wifiManager == null) {
            return new WifiSnapshot("", "", false, false, false);
        }
        WifiInfo info = wifiManager.getConnectionInfo();
        String ssid = sanitizeSsid(info == null ? "" : info.getSSID());
        DhcpInfo dhcpInfo = wifiManager.getDhcpInfo();
        String gateway = dhcpInfo == null ? "" : Formatter.formatIpAddress(dhcpInfo.gateway);
        boolean expectedDevice = hasExpectedPrefix(ssid);
        boolean localGateway = LOCAL_CAMERA_HOSTS.contains(gateway);
        boolean cellularActive = isCellularActive();
        return new WifiSnapshot(ssid, gateway, expectedDevice, localGateway, cellularActive);
    }

    public void bindNativeCallbacksToCurrentWifi() {
        ConnectivityManager connectivityManager =
                (ConnectivityManager) appContext.getSystemService(Context.CONNECTIVITY_SERVICE);
        if (connectivityManager == null) {
            return;
        }
        for (Network network : connectivityManager.getAllNetworks()) {
            NetworkCapabilities caps = connectivityManager.getNetworkCapabilities(network);
            if (caps != null && caps.hasTransport(NetworkCapabilities.TRANSPORT_WIFI)) {
                WifiCallBackFuc.getInstance().setLocalNetwork(network);
                return;
            }
        }
    }

    public boolean isAllowedLocalHost(String host) {
        return host != null && LOCAL_CAMERA_HOSTS.contains(host);
    }

    public Set<String> localCameraHosts() {
        return LOCAL_CAMERA_HOSTS;
    }

    private boolean hasExpectedPrefix(String ssid) {
        if (ssid == null) {
            return false;
        }
        for (String prefix : DEVICE_PREFIXES) {
            if (ssid.toLowerCase(Locale.US).startsWith(prefix.toLowerCase(Locale.US))) {
                return true;
            }
        }
        return false;
    }

    private boolean isCellularActive() {
        ConnectivityManager connectivityManager =
                (ConnectivityManager) appContext.getSystemService(Context.CONNECTIVITY_SERVICE);
        if (connectivityManager == null) {
            return false;
        }
        Network activeNetwork = connectivityManager.getActiveNetwork();
        if (activeNetwork == null) {
            return false;
        }
        NetworkCapabilities caps = connectivityManager.getNetworkCapabilities(activeNetwork);
        return caps != null && caps.hasTransport(NetworkCapabilities.TRANSPORT_CELLULAR);
    }

    private String sanitizeSsid(String ssid) {
        if (ssid == null) {
            return "";
        }
        if (ssid.length() >= 2 && ssid.startsWith("\"") && ssid.endsWith("\"")) {
            return ssid.substring(1, ssid.length() - 1);
        }
        return ssid;
    }

    public static final class WifiSnapshot {
        public final String ssid;
        public final String gateway;
        public final boolean expectedDevice;
        public final boolean localGateway;
        public final boolean cellularActive;

        public WifiSnapshot(String ssid, String gateway, boolean expectedDevice, boolean localGateway, boolean cellularActive) {
            this.ssid = ssid;
            this.gateway = gateway;
            this.expectedDevice = expectedDevice;
            this.localGateway = localGateway;
            this.cellularActive = cellularActive;
        }

        public String securitySummary() {
            if (expectedDevice && localGateway && !cellularActive) {
                return "Local-only route ready";
            }
            if (cellularActive) {
                return "Cellular is active; SecureeEar will not request it";
            }
            if (!expectedDevice) {
                return "Waiting for device Wi-Fi";
            }
            return "Checking local route";
        }
    }
}
