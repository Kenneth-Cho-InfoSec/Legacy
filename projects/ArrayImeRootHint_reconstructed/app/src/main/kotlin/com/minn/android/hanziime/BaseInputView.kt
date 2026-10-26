package com.minn.android.hanziime

import android.content.Context
import android.util.AttributeSet
import android.widget.LinearLayout

class BaseInputView(context: Context, attrs: AttributeSet?) : LinearLayout(context, attrs) {
    override fun onWindowVisibilityChanged(visibility: Int) {
        super.onWindowVisibilityChanged(visibility)
    }
}
