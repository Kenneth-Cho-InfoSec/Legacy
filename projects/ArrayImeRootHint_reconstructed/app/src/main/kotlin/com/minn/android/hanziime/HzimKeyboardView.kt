package com.minn.android.hanziime

import android.content.Context
import android.content.res.TypedArray
import android.graphics.drawable.Drawable
import android.util.AttributeSet
import android.view.inputmethod.InputMethodSubtype
import p000a.AbstractC0020r
import p000a.C0006d
import p000a.C0007e
import p000a.C0013k

class HzimKeyboardView(context: Context, attrs: AttributeSet?) : KeyboardView(context, attrs) {
    var f262I0: TypedArray? = null

    override fun mo103j(key: C0013k): Boolean {
        if (key.f122a[0] != -3) {
            return super.mo103j(key)
        }
        getOnKeyboardActionListener().mo59a(-100, null)
        return true
    }

    fun setEnterKeyAs(value: Int) {
        val keyboard = getKeyboard() as C0007e
        val iconIndex = if (value == 0) 0 else 1
        keyboard.m51d(resources, iconIndex)
        m123f()
    }

    fun setKeyPreviewAs(value: String) {
        val keyboard = getKeyboard() as C0007e
        f262I0 = if (value == "") {
            resources.obtainTypedArray(R.array.ar30_12key_preview_array_nx)
        } else {
            try {
                resources.obtainTypedArray(
                    AbstractC0020r::class.java.getField("ar30_12key_preview_array_${value}x")
                        .getInt(null)
                )
            } catch (exception: Exception) {
                exception.printStackTrace()
                return
            }
        }

        val typedArray = f262I0 ?: return
        val keys = arrayOf(
            keyboard.f94t,
            keyboard.f95u,
            keyboard.f96v,
            keyboard.f97w,
            keyboard.f98x,
            keyboard.f99y,
            keyboard.f100z,
            keyboard.f88A,
            keyboard.f89B,
            keyboard.f90C,
        )
        keys.forEachIndexed { index, key ->
            setPreviewDrawable(key, typedArray.getResourceId(index, -1))
        }
        m123f()
        typedArray.recycle()
    }

    fun setSubtypeOnSpaceKey(inputMethodSubtype: InputMethodSubtype) {
        val keyboard = getKeyboard() as C0007e
        val drawable = resources.getDrawable(inputMethodSubtype.iconResId)
        keyboard.f93s?.f124c = drawable
        m123f()
    }

    private fun setPreviewDrawable(key: C0006d, resourceId: Int) {
        val drawable: Drawable = resources.getDrawable(resourceId)
        key.f125d = drawable
        drawable.setBounds(0, 0, drawable.intrinsicWidth, drawable.intrinsicHeight)
        key.f123b = null
    }
}
