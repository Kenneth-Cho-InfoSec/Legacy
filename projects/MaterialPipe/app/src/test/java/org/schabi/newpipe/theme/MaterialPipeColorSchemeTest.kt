package org.schabi.newpipe.theme

import androidx.compose.ui.graphics.Color
import org.junit.Assert.assertEquals
import org.junit.Assert.assertNotEquals
import org.junit.Assert.assertTrue
import org.junit.Test

/**
 * Verifies the token → Material 3 ColorScheme derivation in [Color.kt].
 *
 * This is the "single source of truth" parity guard for the Compose theme: it
 * proves the scheme is built from the same values the View layer renders and
 * that the brand tinting behaves as designed.
 */
class MaterialPipeColorSchemeTest {

    private val lightTokens = MaterialPipeTokens(
        windowBackground = Color(0xFFFFFBFE),
        textPrimary = Color(0xDE000000),
        textSecondary = Color(0x99000000),
        cardBackground = Color(0xFFF8F8F8),
        border = Color(0x33000000),
        contrastBackground = Color(0x1FA6A6A6),
        brandPrimary = Color(0xFFE53935),
        darkTheme = false
    )

    private val darkTokens = lightTokens.copy(
        windowBackground = Color(0xFF222222),
        textPrimary = Color(0xFFDDDDDD),
        textSecondary = Color(0x99FFFFFF),
        cardBackground = Color(0xFF313131),
        border = Color(0x33FFFFFF),
        contrastBackground = Color(0x1FFFFFFF),
        brandPrimary = Color(0xFF992722),
        darkTheme = true
    )

    private val blackTokens = darkTokens.copy(
        windowBackground = Color(0xFF000000),
        cardBackground = Color(0xFF0F0F0F),
        border = Color(0x25FFFFFF)
    )

    @Test
    fun lightSchemeKeepsSurfacesFromTokens() {
        val scheme = colorSchemeFromTokens(lightTokens)
        assertEquals(Color(0xFFFFFBFE), scheme.background)
        assertEquals(Color(0xFFFFFBFE), scheme.surface)
        assertEquals(Color(0xFFF8F8F8), scheme.surfaceVariant)
        assertEquals(Color(0xDE000000), scheme.onSurface)
        assertEquals(Color(0x33000000), scheme.outline)
        assertEquals(Color(0x1FA6A6A6), scheme.outlineVariant)
    }

    @Test
    fun lightSchemeIsBrandTinted() {
        val scheme = colorSchemeFromTokens(lightTokens)
        assertEquals(Color(0xFFE53935), scheme.primary)
        assertEquals(Color(0xFFE53935), scheme.surfaceTint)
        assertEquals(Color.White, scheme.onPrimary)
        assertNotEquals(scheme.primary, scheme.secondary)
        assertNotEquals(scheme.primary, scheme.tertiary)
    }

    @Test
    fun darkSchemeUsesDarkBrandAndElevatesContainers() {
        val scheme = colorSchemeFromTokens(darkTokens)
        assertEquals(Color(0xFF992722), scheme.primary)
        assertEquals(Color(0xFF222222), scheme.surface)
        assertEquals(Color.White, scheme.onPrimary)
        assertTrue(
            "primaryContainer must be darker than primary in dark mode",
            relativeLuminance(scheme.primaryContainer) < relativeLuminance(scheme.primary)
        )
    }

    @Test
    fun contrastColorChoosesReadableText() {
        assertEquals(Color.Black, contrastColor(Color(0xFFFFFFFF)))
        assertEquals(Color.White, contrastColor(Color(0xFF000000)))
        assertEquals(Color.White, contrastColor(Color(0xFFE53935)))
        assertEquals(Color.White, contrastColor(Color(0xFF992722)))
    }

    @Test
    fun secondaryAndTertiaryDifferByHue() {
        val scheme = colorSchemeFromTokens(lightTokens)
        assertNotEquals(scheme.secondary, scheme.tertiary)
        assertNotEquals(scheme.primary, scheme.secondary)
    }

    @Test
    fun surfaceContainersAreComplete() {
        val scheme = colorSchemeFromTokens(lightTokens)
        assertEquals(scheme.background, scheme.surfaceContainerLowest)
        assertNotEquals(scheme.surface, scheme.surfaceContainerHighest)
        assertNotEquals(scheme.surfaceVariant, scheme.surfaceContainerLowest)
    }

    @Test
    fun rotateHueChangesColorAndIsReversible() {
        val original = Color(0xFFE53935)
        val rotated = rotateHue(original, 25f)
        val back = rotateHue(rotated, -25f)
        assertNotEquals(original, rotated)
        // HSL round trip lands back on the same hue; values are normalized to
        // sRGB, so compare with a small tolerance instead of exact equality.
        assertTrue(
            "rotating +25 then -25 should return near the original color",
            channelDistance(original, back) < 0.01f
        )
    }

    @Test
    fun blackThemeUsesPureBlackSurfaces() {
        val scheme = colorSchemeFromTokens(blackTokens)
        assertEquals(Color.Black, scheme.background)
        assertEquals(Color.Black, scheme.surface)
        assertEquals(Color(0xFF0F0F0F), scheme.surfaceVariant)
        // Black containers sit between pure black and the card surface.
        assertTrue(
            "surfaceContainer must be darker than surfaceContainerHighest",
            relativeLuminance(scheme.surfaceContainer) <
                relativeLuminance(scheme.surfaceContainerHighest)
        )
    }

    @Test
    fun darkenToAndLightenToShiftLuminance() {
        val brand = Color(0xFFE53935)
        val darkened = darkenTo(brand, 0.16f)
        assertTrue(
            "darkenTo must push luminance to at most 0.16",
            relativeLuminance(darkened) <= 0.16f + 1e-6
        )
        assertTrue(relativeLuminance(darkened) < relativeLuminance(brand))

        val lightened = lightenTo(brand, 0.75f)
        assertTrue(
            "lightenTo must push luminance to at least 0.75",
            relativeLuminance(lightened) >= 0.75f - 1e-6
        )
        assertTrue(relativeLuminance(lightened) > relativeLuminance(brand))
    }

    @Test
    fun lightSecondaryAndTertiaryAreAaCleanDarkShades() {
        val scheme = colorSchemeFromTokens(lightTokens)
        assertTrue(
            "light secondary must be a dark shade for AA-clean white text",
            relativeLuminance(scheme.secondary) <= SECONDARY_MAX_LUMINANCE + 1e-6
        )
        assertTrue(
            "light tertiary must be a dark shade for AA-clean white text",
            relativeLuminance(scheme.tertiary) <= SECONDARY_MAX_LUMINANCE + 1e-6
        )
        assertNotEquals(scheme.secondary, scheme.tertiary)
    }

    private fun channelDistance(a: Color, b: Color): Float =
        maxOf(
            kotlin.math.abs(a.red - b.red),
            kotlin.math.abs(a.green - b.green),
            kotlin.math.abs(a.blue - b.blue),
            kotlin.math.abs(a.alpha - b.alpha)
        )
}
