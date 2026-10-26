package org.schabi.newpipe.theme

import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.material3.Shapes
import androidx.compose.ui.unit.dp

/**
 * Material 3 shape scale for PipePipe.
 *
 * Mirrors the official M3 shape scale (extraSmall…extraLarge) using the
 * canonical 4/8/12/16/24 dp corner sizes.
 */
val PipePipeShapes = Shapes(
    extraSmall = RoundedCornerShape(4.dp),
    small = RoundedCornerShape(8.dp),
    medium = RoundedCornerShape(12.dp),
    large = RoundedCornerShape(16.dp),
    extraLarge = RoundedCornerShape(24.dp)
)
