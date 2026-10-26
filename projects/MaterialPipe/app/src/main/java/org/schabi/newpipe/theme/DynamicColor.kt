package org.schabi.newpipe.theme

import android.content.Context
import android.os.Build
import androidx.compose.material3.ColorScheme
import androidx.compose.material3.dynamicDarkColorScheme
import androidx.compose.material3.dynamicLightColorScheme
import androidx.compose.runtime.Composable
import androidx.compose.runtime.remember
import androidx.preference.PreferenceManager
import org.schabi.newpipe.R

/**
 * Color-scheme resolution for PipePipe's Compose UI.
 *
 * Prefers the system Material-You palette when the user enables the
 * "Dynamic color" setting on Android 12+; otherwise it falls back to the
 * brand color scheme derived from the View theme tokens via
 * [colorSchemeFromTokens].
 */
@Composable
fun rememberMaterialPipeColorScheme(context: Context): ColorScheme =
    remember(context) {
        resolveMaterialPipeColorScheme(context)
    }

private fun resolveMaterialPipeColorScheme(context: Context): ColorScheme {
    val darkTheme = isDarkTheme(context)
    val preferences = PreferenceManager.getDefaultSharedPreferences(context)
    val dynamicColorEnabled = preferences.getBoolean(
        context.getString(R.string.dynamic_color_key),
        false
    )
    if (dynamicColorEnabled && Build.VERSION.SDK_INT >= Build.VERSION_CODES.S) {
        return if (darkTheme) {
            dynamicDarkColorScheme(context)
        } else {
            dynamicLightColorScheme(context)
        }
    }
    return colorSchemeFromTokens(resolveMaterialPipeTokens(context))
}

/**
 * Resolves the current theme tokens from the View theme attributes so the
 * Compose scheme always matches what `ThemeHelper` applied to the activity.
 */
fun resolveMaterialPipeTokens(context: Context): MaterialPipeTokens {
    val resources = context.resources
    return MaterialPipeTokens(
        windowBackground = colorAttr(context, R.attr.windowBackground),
        textPrimary = colorAttr(context, android.R.attr.textColorPrimary),
        textSecondary = colorAttr(context, android.R.attr.textColorSecondary),
        cardBackground = colorAttr(context, R.attr.card_item_background_color),
        border = colorAttr(context, R.attr.border_color),
        contrastBackground = colorAttr(context, R.attr.contrast_background_color),
        brandPrimary = colorAttr(context, R.attr.colorPrimary),
        darkTheme = isDarkTheme(context)
    )
}

private fun colorAttr(context: Context, attrRes: Int): androidx.compose.ui.graphics.Color {
    val value = org.schabi.newpipe.util.ThemeHelper.resolveColorFromAttr(context, attrRes)
    return androidx.compose.ui.graphics.Color(value)
}

/**
 * Returns true when the app should render with a dark color scheme.
 *
 * Mirrors the existing `theme`/`night_theme` preference semantics so Compose
 * and the View layer agree.
 */
fun isDarkTheme(context: Context): Boolean {
    val resources = context.resources
    val preferences = PreferenceManager.getDefaultSharedPreferences(context)
    val selectedTheme = preferences.getString(
        context.getString(R.string.theme_key),
        resources.getString(R.string.default_theme_value)
    )
    return when (selectedTheme) {
        resources.getString(R.string.light_theme_key) -> false
        resources.getString(R.string.auto_device_theme_key) -> {
            if (org.schabi.newpipe.util.ThemeHelper.isDeviceDarkThemeEnabled(context)) {
                true
            } else {
                false
            }
        }
        else -> true
    }
}
