package com.kennethchoinfosec.secureeear;

import android.content.Intent;
import android.content.pm.PackageManager;
import android.graphics.Bitmap;
import android.graphics.Color;
import android.graphics.Matrix;
import android.graphics.Typeface;
import android.os.Build;
import android.os.Bundle;
import android.provider.Settings;
import android.view.Gravity;
import android.view.View;
import android.widget.ImageView;
import android.widget.LinearLayout;
import android.widget.ScrollView;
import android.widget.TextView;

import androidx.activity.ComponentActivity;

import com.google.android.material.button.MaterialButton;
import com.google.android.material.card.MaterialCardView;
import com.jni.CallBack.WifiCallBack;
import com.kennethchoinfosec.secureeear.camera.NativeCameraController;
import com.kennethchoinfosec.secureeear.security.PermissionPolicy;
import com.kennethchoinfosec.secureeear.security.SecurityMonitor;

import java.util.ArrayList;
import java.util.List;
import java.util.Locale;

public final class MainActivity extends ComponentActivity implements NativeCameraController.Listener {
    private static final int TEAL = Color.rgb(0, 121, 107);
    private static final int TEAL_DARK = Color.rgb(0, 77, 64);
    private static final int TEAL_SOFT = Color.rgb(218, 244, 239);
    private static final int SURFACE = Color.rgb(248, 251, 249);
    private static final int SURFACE_ALT = Color.rgb(237, 247, 244);
    private static final int TEXT = Color.rgb(18, 33, 31);
    private static final int MUTED = Color.rgb(81, 104, 100);

    private SecurityMonitor securityMonitor;
    private NativeCameraController cameraController;
    private TextView routeBadge;
    private TextView wifiValue;
    private TextView cameraStatus;
    private TextView deviceValue;
    private TextView frameValue;
    private TextView securityValue;
    private TextView nativeRiskValue;
    private ImageView preview;
    private MaterialButton cameraButton;
    private MaterialButton rotateButton;
    private Bitmap latestFrame;
    private int previewRotationDegrees;

    @Override
    protected void onCreate(Bundle savedInstanceState) {
        super.onCreate(savedInstanceState);
        securityMonitor = new SecurityMonitor(this);
        cameraController = new NativeCameraController(this, this);
        WifiCallBack.getInstance().setDeviceStatusInterface(cameraController);
        buildUi();
        requestRequiredPermissions();
        refreshSecurityState();
    }

    private void buildUi() {
        ScrollView scrollView = new ScrollView(this);
        scrollView.setFillViewport(true);
        scrollView.setBackgroundColor(SURFACE);

        LinearLayout root = new LinearLayout(this);
        root.setOrientation(LinearLayout.VERTICAL);
        root.setPadding(dp(20), dp(22), dp(20), dp(24));
        scrollView.addView(root, new ScrollView.LayoutParams(
                ScrollView.LayoutParams.MATCH_PARENT,
                ScrollView.LayoutParams.WRAP_CONTENT));

        LinearLayout titleRow = new LinearLayout(this);
        titleRow.setOrientation(LinearLayout.HORIZONTAL);
        titleRow.setGravity(Gravity.CENTER_VERTICAL);
        root.addView(titleRow, matchWrap());

        TextView title = new TextView(this);
        title.setText("SecureeEar");
        title.setTextColor(TEXT);
        title.setTextSize(30);
        title.setTypeface(Typeface.DEFAULT_BOLD);
        titleRow.addView(title, new LinearLayout.LayoutParams(0, LinearLayout.LayoutParams.WRAP_CONTENT, 1f));

        routeBadge = pillLabel("Local only");
        titleRow.addView(routeBadge);

        TextView subtitle = text("Cybersecurity-focused ear camera", 15, MUTED, Typeface.NORMAL);
        subtitle.setPadding(0, dp(4), 0, dp(18));
        root.addView(subtitle, matchWrap());

        preview = new ImageView(this);
        preview.setScaleType(ImageView.ScaleType.FIT_CENTER);
        preview.setBackgroundColor(Color.rgb(229, 242, 239));
        MaterialCardView previewShell = panel();
        previewShell.addView(preview, new MaterialCardView.LayoutParams(
                MaterialCardView.LayoutParams.MATCH_PARENT, dp(310)));
        root.addView(previewShell, matchWrapWithBottom(16));

        LinearLayout actionRow = new LinearLayout(this);
        actionRow.setOrientation(LinearLayout.HORIZONTAL);
        actionRow.setGravity(Gravity.CENTER);
        root.addView(actionRow, matchWrapWithBottom(14));

        MaterialButton wifiButton = pillButton("Wi-Fi", false);
        wifiButton.setOnClickListener(new View.OnClickListener() {
            @Override
            public void onClick(View view) {
                startActivity(new Intent(Settings.ACTION_WIFI_SETTINGS));
            }
        });
        actionRow.addView(wifiButton, weightedButtonParams());

        cameraButton = pillButton("Start", true);
        cameraButton.setOnClickListener(new View.OnClickListener() {
            @Override
            public void onClick(View view) {
                toggleCamera();
            }
        });
        actionRow.addView(cameraButton, weightedButtonParams());

        rotateButton = pillButton("", false);
        rotateButton.setIconResource(R.drawable.ic_rotate_90);
        rotateButton.setIconGravity(MaterialButton.ICON_GRAVITY_TEXT_START);
        rotateButton.setIconPadding(0);
        rotateButton.setContentDescription("Rotate preview 90 degrees");
        if (Build.VERSION.SDK_INT >= 26) {
            rotateButton.setTooltipText("Rotate preview");
        }
        rotateButton.setOnClickListener(new View.OnClickListener() {
            @Override
            public void onClick(View view) {
                rotatePreview();
            }
        });
        actionRow.addView(rotateButton, weightedButtonParams());

        root.addView(statusPanel(), matchWrapWithBottom(12));
        root.addView(securityPanel(), matchWrapWithBottom(12));
        setContentView(scrollView);
    }

    private View statusPanel() {
        LinearLayout body = panelBody();
        body.addView(sectionTitle("Connection"));
        wifiValue = value("Wi-Fi: checking");
        cameraStatus = value("Camera: idle");
        deviceValue = value("Device: not connected");
        frameValue = value("Frames: 0 | audio: 0");
        body.addView(wifiValue);
        body.addView(cameraStatus);
        body.addView(deviceValue);
        body.addView(frameValue);
        return wrapPanel(body);
    }

    private View securityPanel() {
        LinearLayout body = panelBody();
        body.addView(sectionTitle("Security"));
        securityValue = value("Permissions: checking");
        nativeRiskValue = value("Native bridge: local-only wrapper active");
        body.addView(securityValue);
        body.addView(nativeRiskValue);
        return wrapPanel(body);
    }

    private void toggleCamera() {
        refreshSecurityState();
        securityMonitor.bindNativeCallbacksToCurrentWifi();
        if (cameraController.isStarted()) {
            cameraController.stop();
            cameraButton.setText("Start");
            return;
        }
        cameraController.start();
        cameraButton.setText("Stop");
    }

    private void requestRequiredPermissions() {
        List<String> missing = new ArrayList<>();
        for (String permission : PermissionPolicy.runtimePermissions()) {
            if (checkSelfPermission(permission) != PackageManager.PERMISSION_GRANTED) {
                missing.add(permission);
            }
        }
        if (!missing.isEmpty()) {
            requestPermissions(missing.toArray(new String[0]), 100);
        }
    }

    private void refreshSecurityState() {
        SecurityMonitor.WifiSnapshot snapshot = securityMonitor.wifiSnapshot();
        routeBadge.setText(snapshot.expectedDevice && !snapshot.cellularActive ? "Local only" : "Check route");
        wifiValue.setText(String.format(Locale.US, "Wi-Fi: %s | gateway %s", clean(snapshot.ssid), clean(snapshot.gateway)));
        securityValue.setText(snapshot.securitySummary());
        nativeRiskValue.setText("Allowed hosts: " + securityMonitor.localCameraHosts());
    }

    @Override
    public void onStatus(String value) {
        cameraStatus.setText("Camera: " + value);
        if (!cameraController.isStarted()) {
            cameraButton.setText("Start");
        }
    }

    @Override
    public void onDevice(String value) {
        deviceValue.setText("Device: " + value);
    }

    @Override
    public void onFrame(Bitmap bitmap, String metadata) {
        latestFrame = bitmap;
        renderPreview();
        frameValue.setText("Preview: " + metadata + " | rotate " + previewRotationDegrees + "°");
    }

    @Override
    public void onMetrics(long frames, long audioPackets) {
        frameValue.setText("Frames: " + frames + " | audio: " + audioPackets);
    }

    @Override
    protected void onResume() {
        super.onResume();
        refreshSecurityState();
    }

    @Override
    protected void onDestroy() {
        cameraController.shutdown();
        super.onDestroy();
    }

    private MaterialButton pillButton(String label, boolean filled) {
        MaterialButton button = new MaterialButton(this);
        button.setText(label);
        button.setAllCaps(false);
        button.setCornerRadius(dp(28));
        button.setMinHeight(dp(54));
        button.setTextSize(15);
        button.setTypeface(Typeface.DEFAULT_BOLD);
        if (filled) {
            button.setBackgroundColor(TEAL);
            button.setTextColor(Color.WHITE);
        } else {
            button.setBackgroundColor(TEAL_SOFT);
            button.setTextColor(TEAL_DARK);
        }
        return button;
    }

    private void rotatePreview() {
        previewRotationDegrees = (previewRotationDegrees + 90) % 360;
        renderPreview();
        frameValue.setText("Preview rotation: " + previewRotationDegrees + "°");
    }

    private void renderPreview() {
        if (latestFrame == null) {
            return;
        }
        if (previewRotationDegrees == 0) {
            preview.setRotation(0f);
            preview.setImageBitmap(latestFrame);
            return;
        }
        try {
            Matrix matrix = new Matrix();
            matrix.postRotate(previewRotationDegrees);
            Bitmap rotated = Bitmap.createBitmap(latestFrame, 0, 0, latestFrame.getWidth(), latestFrame.getHeight(), matrix, true);
            preview.setRotation(0f);
            preview.setImageBitmap(rotated);
        } catch (RuntimeException exception) {
            preview.setRotation(0f);
            preview.setImageBitmap(latestFrame);
            frameValue.setText("Preview rotation failed; showing original frame");
        }
    }

    private TextView pillLabel(String value) {
        TextView label = text(value, 13, TEAL_DARK, Typeface.BOLD);
        label.setGravity(Gravity.CENTER);
        label.setPadding(dp(14), dp(7), dp(14), dp(7));
        label.setBackgroundColor(TEAL_SOFT);
        return label;
    }

    private TextView sectionTitle(String value) {
        TextView title = text(value, 18, TEXT, Typeface.BOLD);
        title.setPadding(0, 0, 0, dp(8));
        return title;
    }

    private TextView value(String value) {
        TextView text = text(value, 14, MUTED, Typeface.NORMAL);
        text.setPadding(0, dp(4), 0, dp(4));
        return text;
    }

    private TextView text(String value, int sp, int color, int style) {
        TextView text = new TextView(this);
        text.setText(value);
        text.setTextSize(sp);
        text.setTextColor(color);
        text.setTypeface(Typeface.DEFAULT, style);
        return text;
    }

    private MaterialCardView panel() {
        MaterialCardView card = new MaterialCardView(this);
        card.setRadius(dp(8));
        card.setCardElevation(0);
        card.setStrokeWidth(dp(1));
        card.setStrokeColor(Color.rgb(207, 225, 221));
        card.setCardBackgroundColor(SURFACE_ALT);
        return card;
    }

    private LinearLayout panelBody() {
        LinearLayout body = new LinearLayout(this);
        body.setOrientation(LinearLayout.VERTICAL);
        body.setPadding(dp(16), dp(15), dp(16), dp(15));
        return body;
    }

    private MaterialCardView wrapPanel(View body) {
        MaterialCardView card = panel();
        card.addView(body);
        return card;
    }

    private LinearLayout.LayoutParams matchWrap() {
        return new LinearLayout.LayoutParams(
                LinearLayout.LayoutParams.MATCH_PARENT,
                LinearLayout.LayoutParams.WRAP_CONTENT);
    }

    private LinearLayout.LayoutParams matchWrapWithBottom(int bottomDp) {
        LinearLayout.LayoutParams params = matchWrap();
        params.setMargins(0, 0, 0, dp(bottomDp));
        return params;
    }

    private LinearLayout.LayoutParams weightedButtonParams() {
        LinearLayout.LayoutParams params = new LinearLayout.LayoutParams(0, dp(56), 1f);
        params.setMargins(dp(4), 0, dp(4), 0);
        return params;
    }

    private String clean(String value) {
        return value == null || value.length() == 0 || "<unknown ssid>".equals(value) ? "unknown" : value;
    }

    private int dp(int value) {
        return Math.round(value * getResources().getDisplayMetrics().density);
    }
}
