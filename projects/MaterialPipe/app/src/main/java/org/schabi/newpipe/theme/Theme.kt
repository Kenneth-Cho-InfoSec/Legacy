package org.schabi.newpipe.theme

import android.content.Context
import androidx.compose.material3.MaterialTheme
import androidx.compose.runtime.Composable

/**
 * The single Compose theme entry point for PipePipe.
 *
 * Replaces the previous per-screen ad-hoc themes (the search-filter dialog's
 * hard-coded schemes and the info-list helper's attr scheme) with one
 * implementation backed by [rememberMaterialPipeColorScheme], the M3
 * typography scale and the M3 shape scale.
 */
@Composable
fun MaterialPipeTheme(
    context: Context,
    content: @Composable () -> Unit
) {
    val colorScheme = rememberMaterialPipeColorScheme(context)
    MaterialTheme(
        colorScheme = colorScheme,
        typography = PipePipeTypography,
        shapes = PipePipeShapes,
        content = content
    )
}
