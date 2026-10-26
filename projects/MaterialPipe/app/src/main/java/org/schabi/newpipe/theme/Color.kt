package org.schabi.newpipe.theme

import androidx.compose.material3.ColorScheme
import androidx.compose.material3.darkColorScheme
import androidx.compose.material3.lightColorScheme
import androidx.compose.ui.graphics.Color
import kotlin.math.pow

/**
 * Material 3 color roles for PipePipe, derived from the existing
 * (NewPipe-style) theme tokens so the Compose world and the View world
 * share one source of truth.
 *
 * This file is intentionally pure Kotlin (no Android runtime types other than
 * [Color]) so the derivation logic is unit-testable. The actual token values
 * are resolved from the View theme attributes in [Theme.kt].
 */

/**
 * The resolved token set consumed by [colorSchemeFromTokens].
 * Values come from the XML theme that `ThemeHelper` applies to the activity.
 */
data class MaterialPipeTokens(
    val windowBackground: Color,
    val textPrimary: Color,
    val textSecondary: Color,
    val cardBackground: Color,
    val border: Color,
    val contrastBackground: Color,
    val brandPrimary: Color,
    val darkTheme: Boolean
)

/**
 * Builds a complete Material 3 [ColorScheme] from the theme tokens.
 *
 * - Surfaces keep the exact colors the View layer already renders.
 * - [MaterialPipeTokens.brandPrimary] tints the primary/secondary/tertiary
 *   roles, matching the per-service brand theming of the rest of the app.
 */
fun colorSchemeFromTokens(tokens: MaterialPipeTokens): ColorScheme {
    val isDark = tokens.darkTheme
    val primary = tokens.brandPrimary
    val background = tokens.windowBackground
    val onSurface = tokens.textPrimary
    val surface = background
    val surfaceVariant = tokens.cardBackground
    val onSurfaceVariant = tokens.textSecondary
    val outline = tokens.border
    val outlineVariant = tokens.contrastBackground

    val onPrimary = contrastColor(primary)
    val primaryContainer =
        if (isDark) blend(primary, Color.Black, 0.55f) else blend(primary, Color.White, 0.85f)
    val onPrimaryContainer =
        if (isDark) blend(primary, Color.White, 0.70f) else blend(primary, Color.Black, 0.20f)

    val secondary = if (isDark) rotateHue(primary, 25f)
    else darkenTo(rotateHue(primary, 25f), SECONDARY_MAX_LUMINANCE)
    val onSecondary = contrastColor(secondary)
    val secondaryContainer =
        if (isDark) blend(secondary, Color.Black, 0.55f) else blend(secondary, Color.White, 0.85f)
    val onSecondaryContainer =
        if (isDark) blend(secondary, Color.White, 0.70f) else blend(secondary, Color.Black, 0.20f)

    val tertiary = if (isDark) rotateHue(primary, -25f)
    else darkenTo(rotateHue(primary, -25f), SECONDARY_MAX_LUMINANCE)
    val onTertiary = contrastColor(tertiary)
    val tertiaryContainer =
        if (isDark) blend(tertiary, Color.Black, 0.55f) else blend(tertiary, Color.White, 0.85f)
    val onTertiaryContainer =
        if (isDark) blend(tertiary, Color.White, 0.70f) else blend(tertiary, Color.Black, 0.20f)

    val error = if (isDark) Color(0xFFF2B8B5) else Color(0xFFB3261E)
    val onError = if (isDark) Color(0xFF601410) else Color(0xFFFFFFFF)
    val errorContainer = if (isDark) Color(0xFF8C1D18) else Color(0xFFF9DEDC)
    val onErrorContainer = if (isDark) Color(0xFFF9DEDC) else Color(0xFF410E0B)

    // Surface containers: gradual blends between the background and the
    // card surface so components can express tonal elevation.
    val surfaceContainerLowest =
        if (isDark) blend(surface, Color.Black, 0.20f) else surface
    val surfaceContainerLow = blend(surface, surfaceVariant, 0.33f)
    val surfaceContainer = blend(surface, surfaceVariant, 0.66f)
    val surfaceContainerHigh = blend(surfaceVariant, onSurface, 0.04f)
    val surfaceContainerHighest = blend(surfaceVariant, onSurface, 0.10f)

    return if (isDark) {
        darkColorScheme(
            primary = primary,
            onPrimary = onPrimary,
            primaryContainer = primaryContainer,
            onPrimaryContainer = onPrimaryContainer,
            secondary = secondary,
            onSecondary = onSecondary,
            secondaryContainer = secondaryContainer,
            onSecondaryContainer = onSecondaryContainer,
            tertiary = tertiary,
            onTertiary = onTertiary,
            tertiaryContainer = tertiaryContainer,
            onTertiaryContainer = onTertiaryContainer,
            background = background,
            onBackground = onSurface,
            surface = surface,
            onSurface = onSurface,
            surfaceVariant = surfaceVariant,
            onSurfaceVariant = onSurfaceVariant,
            surfaceTint = primary,
            inverseSurface = onSurface,
            inverseOnSurface = surface,
            inversePrimary = primaryContainer,
            error = error,
            onError = onError,
            errorContainer = errorContainer,
            onErrorContainer = onErrorContainer,
            outline = outline,
            outlineVariant = outlineVariant,
            scrim = Color.Black,
            surfaceContainerLowest = surfaceContainerLowest,
            surfaceContainerLow = surfaceContainerLow,
            surfaceContainer = surfaceContainer,
            surfaceContainerHigh = surfaceContainerHigh,
            surfaceContainerHighest = surfaceContainerHighest
        )
    } else {
        lightColorScheme(
            primary = primary,
            onPrimary = onPrimary,
            primaryContainer = primaryContainer,
            onPrimaryContainer = onPrimaryContainer,
            secondary = secondary,
            onSecondary = onSecondary,
            secondaryContainer = secondaryContainer,
            onSecondaryContainer = onSecondaryContainer,
            tertiary = tertiary,
            onTertiary = onTertiary,
            tertiaryContainer = tertiaryContainer,
            onTertiaryContainer = onTertiaryContainer,
            background = background,
            onBackground = onSurface,
            surface = surface,
            onSurface = onSurface,
            surfaceVariant = surfaceVariant,
            onSurfaceVariant = onSurfaceVariant,
            surfaceTint = primary,
            inverseSurface = onSurface,
            inverseOnSurface = surface,
            inversePrimary = primaryContainer,
            error = error,
            onError = onError,
            errorContainer = errorContainer,
            onErrorContainer = onErrorContainer,
            outline = outline,
            outlineVariant = outlineVariant,
            scrim = Color.Black,
            surfaceContainerLowest = surfaceContainerLowest,
            surfaceContainerLow = surfaceContainerLow,
            surfaceContainer = surfaceContainer,
            surfaceContainerHigh = surfaceContainerHigh,
            surfaceContainerHighest = surfaceContainerHighest
        )
    }
}

/**
 * Returns black or white depending on which reads better against
 * [background]. The 0.45 threshold keeps the brand colors (e.g. the YouTube
 * red, which has a relative luminance of ~0.20) with white content, matching
 * the app's current brand look, while still returning black for near-white
 * surfaces.
 */
fun contrastColor(background: Color): Color =
    if (relativeLuminance(background) > 0.45f) Color.Black else Color.White

/**
 * Relative luminance of a color in the 0..1 range (WCAG definition).
 */
fun relativeLuminance(color: Color): Double {
    fun channel(value: Float): Double {
        val channelValue = value.toDouble()
        return if (channelValue <= 0.03928) {
            channelValue / 12.92
        } else {
            ((channelValue + 0.055) / 1.055).pow(2.4)
        }
    }
    return 0.2126 * channel(color.red) +
        0.7152 * channel(color.green) +
        0.0722 * channel(color.blue)
}

internal fun blend(start: Color, end: Color, fraction: Float): Color {
    val t = fraction.coerceIn(0f, 1f)
    return Color(
        red = lerp(start.red, end.red, t),
        green = lerp(start.green, end.green, t),
        blue = lerp(start.blue, end.blue, t),
        alpha = lerp(start.alpha, end.alpha, t)
    )
}

/**
 * Max luminance for the derived light-scheme secondary/tertiary roles, tuned
 * so white text on them clears WCAG AA (>= 4.5:1) while keeping a distinct hue
 * family from the primary.
 */
internal const val SECONDARY_MAX_LUMINANCE = 0.16f

/**
 * Upper bound for [darkenTo]/[lightenTo]. Compose `Color` packs sRGB floats to
 * 8-bit, so tiny blend steps can quantize-cycle without changing the color;
 * the guards below guarantee termination regardless.
 */
private const val TONAL_STEP_CAP = 256

/**
 * Blends [color] toward black until its relative luminance is at most
 * [maxLuminance], mimicking a dark M3 tonal-palette shade.
 */
internal fun darkenTo(color: Color, maxLuminance: Float): Color {
    var c = color
    var previous = color
    repeat(TONAL_STEP_CAP) {
        if (relativeLuminance(c) <= maxLuminance) return c
        previous = c
        c = blend(c, Color.Black, 0.01f)
        if (c == previous) return c
    }
    return c
}

/**
 * Blends [color] toward white until its relative luminance is at least
 * [minLuminance], mimicking a light M3 tonal-palette shade. Uses a larger step
 * than [darkenTo] because 8-bit quantization otherwise traps luminance in a
 * cycle below the target.
 */
internal fun lightenTo(color: Color, minLuminance: Float): Color {
    var c = color
    var previous = color
    repeat(TONAL_STEP_CAP) {
        if (relativeLuminance(c) >= minLuminance) return c
        previous = c
        c = blend(c, Color.White, 0.05f)
        if (c == previous) return c
    }
    return c
}

private fun lerp(start: Float, end: Float, fraction: Float): Float =
    start + (end - start) * fraction

internal fun rotateHue(color: Color, degrees: Float): Color {
    val hsl = FloatArray(3)
    rgbToHsl(color, hsl)
    hsl[0] = (hsl[0] + degrees) % 360f
    if (hsl[0] < 0f) {
        hsl[0] += 360f
    }
    return hslToRgb(hsl)
}

private fun rgbToHsl(color: Color, hsl: FloatArray) {
    val r = color.red
    val g = color.green
    val b = color.blue
    val max = maxOf(r, g, b)
    val min = minOf(r, g, b)
    val l = (max + min) / 2f
    var h = 0f
    var s = 0f
    if (max != min) {
        val d = max - min
        s = if (l > 0.5f) d / (2f - max - min) else d / (max + min)
        h = when (max) {
            r -> (g - b) / d + (if (g < b) 6f else 0f)
            g -> (b - r) / d + 2f
            else -> (r - g) / d + 4f
        }
        h *= 60f
    }
    hsl[0] = h
    hsl[1] = s
    hsl[2] = l
}

private fun hslToRgb(hsl: FloatArray): Color {
    val h = hsl[0] / 360f
    val s = hsl[1]
    val l = hsl[2]
    if (s == 0f) {
        return Color(l, l, l, 1f)
    }
    val q = if (l < 0.5f) l * (1f + s) else l + s - l * s
    val p = 2f * l - q
    fun hue2rgb(p: Float, q: Float, tIn: Float): Float {
        var t = tIn
        if (t < 0f) {
            t += 1f
        }
        if (t > 1f) {
            t -= 1f
        }
        return when {
            t < 1f / 6f -> p + (q - p) * 6f * t
            t < 1f / 2f -> q
            t < 2f / 3f -> p + (q - p) * (2f / 3f - t) * 6f
            else -> p
        }
    }
    return Color(
        red = hue2rgb(p, q, h + 1f / 3f),
        green = hue2rgb(p, q, h),
        blue = hue2rgb(p, q, h - 1f / 3f),
        alpha = 1f
    )
}
