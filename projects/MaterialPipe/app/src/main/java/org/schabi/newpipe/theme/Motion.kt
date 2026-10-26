package org.schabi.newpipe.theme

import android.content.Context
import android.provider.Settings

/**
 * Motion tokens for PipePipe.
 *
 * `material3` 1.4.0 already applies expressive spring motion inside its
 * component defaults; these constants are the project-level tokens used by
 * the View bridge and by custom animations. When `material3` 1.5.0 lands the
 * public [androidx.compose.material3.MotionScheme], this file becomes the
 * place that exposes the expressive motion scheme.
 */
object PipePipeMotion {
    /** Short effects: color, opacity. */
    const val DURATION_SHORT_MS = 150
    /** Standard spatial transitions. */
    const val DURATION_MEDIUM_MS = 250
    /** Long, full-screen transitions. */
    const val DURATION_LONG_MS = 350
}

/**
 * Returns true when the system animation scales are disabled (user preference
 * or "Remove animations"). Used to skip spring/expressive motion so the UI is
 * accessible to users with motion sensitivity.
 */
fun isReduceMotionEnabled(context: Context): Boolean =
    Settings.Global.getFloat(
        context.contentResolver,
        Settings.Global.ANIMATOR_DURATION_SCALE,
        1f
    ) == 0f
