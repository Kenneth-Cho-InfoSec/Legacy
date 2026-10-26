package p001b;

import android.content.Context;
import android.util.Log;
import java.io.ByteArrayOutputStream;
import java.io.IOException;
import java.io.InputStream;
import java.nio.ByteBuffer;
import java.util.Iterator;

/* JADX INFO: renamed from: b.b */
/* JADX INFO: loaded from: classes.dex */
public final class C0030b implements Iterator {

    /* JADX INFO: renamed from: e */
    public static int f201e = -1;

    /* JADX INFO: renamed from: f */
    public static String f202f = null;

    /* JADX INFO: renamed from: g */
    public static boolean f203g = false;

    /* JADX INFO: renamed from: h */
    public static String[] f204h;

    /* JADX INFO: renamed from: a */
    public String f205a;

    /* JADX INFO: renamed from: b */
    public String f206b;

    /* JADX INFO: renamed from: c */
    public int f207c;

    /* JADX INFO: renamed from: d */
    public int f208d;

    public C0030b(String str, Context context) {
        this.f205a = null;
        this.f206b = null;
        this.f207c = 1;
        this.f208d = 1;
        try {
            InputStream inputStreamOpen = context.getAssets().open("ImeData/" + str);
            byte[] bArr = new byte[4];
            inputStreamOpen.read(bArr);
            this.f207c = ByteBuffer.wrap(bArr).getInt();
            inputStreamOpen.read(bArr);
            this.f208d = ByteBuffer.wrap(bArr).getInt();
            inputStreamOpen.read(bArr);
            int i = ByteBuffer.wrap(bArr).getInt();
            ByteArrayOutputStream byteArrayOutputStream = new ByteArrayOutputStream();
            byte[] bArr2 = new byte[i];
            inputStreamOpen.read(bArr2);
            byteArrayOutputStream.write(bArr2, 0, i);
            this.f205a = byteArrayOutputStream.toString("UTF-8").toUpperCase();
            inputStreamOpen.read(bArr);
            int i2 = ByteBuffer.wrap(bArr).getInt();
            byte[] bArr3 = new byte[i2];
            byteArrayOutputStream.reset();
            inputStreamOpen.read(bArr3);
            byteArrayOutputStream.write(bArr3, 0, i2);
            this.f206b = byteArrayOutputStream.toString("UTF-8");
            inputStreamOpen.close();
            byteArrayOutputStream.close();
        } catch (IOException unused) {
            Log.e("b", "Key-Value file loading fail");
        }
    }

    /* JADX INFO: renamed from: a */
    public final int m74a(String str) {
        f202f = str;
        String str2 = "";
        if (str.length() > 0) {
            for (int length = this.f207c - str.length(); length > 0; length--) {
                str2 = str2 + " ";
            }
            str2 = str + str2;
        }
        int iM75b = m75b(str2, false);
        f201e = iM75b;
        if (iM75b >= 0) {
            return iM75b + 1;
        }
        return -1;
    }

    /* JADX INFO: renamed from: b */
    public final int m75b(String str, boolean z) {
        String str2 = this.f205a;
        int length = str2.length();
        int i = this.f207c;
        int i2 = (length / i) - 1;
        if (!z) {
            int iM76c = m76c(0, i2, str);
            return iM76c >= 0 ? m78e(0, iM76c - 1, str) : iM76c;
        }
        if (i2 < 0) {
            return 0;
        }
        int i3 = i2 >> 1;
        int i4 = i3 + 1;
        String strSubstring = str2.substring(i3 * i, i * i4);
        int iCompareToIgnoreCase = str.compareToIgnoreCase(strSubstring);
        if (strSubstring.startsWith(str)) {
            iCompareToIgnoreCase = 0;
        }
        return iCompareToIgnoreCase <= 0 ? m78e(0, i3 - 1, str) : m78e(i4, i2, str);
    }

    /* JADX INFO: renamed from: c */
    public final int m76c(int i, int i2, String str) {
        if (i > i2) {
            return -1;
        }
        int i3 = ((i2 - i) >> 1) + i;
        int i4 = this.f207c;
        int i5 = i3 + 1;
        int iCompareToIgnoreCase = str.compareToIgnoreCase(this.f205a.substring(i3 * i4, i4 * i5));
        return iCompareToIgnoreCase < 0 ? m76c(i, i3 - 1, str) : iCompareToIgnoreCase > 0 ? m76c(i5, i2, str) : i3;
    }

    /* JADX INFO: renamed from: d */
    public final String m77d(int i) {
        int i2 = this.f208d;
        return this.f206b.substring(i * i2, (i + 1) * i2);
    }

    /* JADX INFO: renamed from: e */
    public final int m78e(int i, int i2, String str) {
        if (i > i2) {
            return i;
        }
        int i3 = ((i2 - i) >> 1) + i;
        int i4 = this.f207c;
        int i5 = i3 + 1;
        return str.compareToIgnoreCase(this.f205a.substring(i3 * i4, i4 * i5)) <= 0 ? m78e(i, i3 - 1, str) : m78e(i5, i2, str);
    }

    @Override // java.util.Iterator
    /* JADX INFO: renamed from: f, reason: merged with bridge method [inline-methods] */
    public final String next() {
        if (f202f.equals("*") || f202f.equals("?")) {
            String str = f202f;
            f202f = "";
            return str;
        }
        String strM77d = m77d(f201e);
        f201e++;
        return strM77d;
    }

    /* JADX WARN: Code restructure failed: missing block: B:24:0x006d, code lost:
    
        r1 = true;
     */
    /* JADX WARN: Code restructure failed: missing block: B:59:0x010a, code lost:
    
        p001b.C0030b.f201e = r5.length() / r6;
     */
    /* JADX WARN: Code restructure failed: missing block: B:60:0x0111, code lost:
    
        return false;
     */
    /* JADX WARN: Removed duplicated region for block: B:115:0x00b5 A[SYNTHETIC] */
    @Override // java.util.Iterator
    /*
        Code decompiled incorrectly, please refer to instructions dump.
    */
    public final boolean hasNext() {
        int iCompareToIgnoreCase;
        String str = f202f;
        char c = 0;
        if (str == null) {
            return false;
        }
        boolean zEquals = str.equals("*");
        String str2 = this.f205a;
        int i = this.f207c;
        if (zEquals || f202f.equals("?")) {
            f201e = str2.length() / i;
            return true;
        }
        String str3 = "";
        if (!f203g) {
            for (int length = i - str.length(); length > 0; length--) {
                str3 = str3 + " ";
            }
            String str4 = str + str3;
            int i2 = f201e;
            if ((i2 + 1) * i > 0 && (i2 + 1) * i <= str2.length()) {
                int i3 = f201e;
                iCompareToIgnoreCase = str4.compareToIgnoreCase(str2.substring(i3 * i, (i3 + 1) * i));
            } else {
                iCompareToIgnoreCase = -1;
            }
            if (iCompareToIgnoreCase == 0) {
                return true;
            }
            f201e = -1;
            return false;
        }
        boolean z = false;
        while (true) {
            boolean z2 = (f201e + 1) * i > str2.length();
            if (z2) {
                return false;
            }
            int i4 = f201e;
            String strReplaceAll = str2.substring(i4 * i, (i4 + 1) * i).replaceAll("\\s+", "");
            String str5 = f204h[c];
            String str6 = f202f;
            if (str6.startsWith("*")) {
                char[] r12 = f202f.toCharArray();
                int r13 = r12.length;
                if (r13 == 2) {
                    if (z) {
                        return true;
                    }
                } else if (z) {
                }
            } else if (f202f.startsWith("?")) {
                if (f202f.length() == strReplaceAll.length()) {
                    char[] charArray = f202f.toCharArray();
                    char[] charArray2 = strReplaceAll.toCharArray();
                    for (int iIndexOf = str6.indexOf("?"); iIndexOf < str6.length() && iIndexOf >= 0; iIndexOf = str6.indexOf("?", iIndexOf + 1)) {
                        charArray[iIndexOf] = ' ';
                        charArray2[iIndexOf] = ' ';
                    }
                    if (String.valueOf(charArray).equals(String.valueOf(charArray2))) {
                        return true;
                    }
                }
            } else if (f202f.contains("?")) {
                if (!strReplaceAll.startsWith(str5)) {
                    break;
                }
                if (f202f.length() == strReplaceAll.length()) {
                    char[] charArray3 = f202f.toCharArray();
                    char[] charArray4 = strReplaceAll.toCharArray();
                    for (int iIndexOf2 = str6.indexOf("?"); iIndexOf2 < str6.length() && iIndexOf2 >= 0; iIndexOf2 = str6.indexOf("?", iIndexOf2 + 1)) {
                        charArray3[iIndexOf2] = ' ';
                        charArray4[iIndexOf2] = ' ';
                    }
                    if (String.valueOf(charArray3).equals(String.valueOf(charArray4))) {
                        return true;
                    }
                }
            } else {
                if (!strReplaceAll.startsWith(str5)) {
                    break;
                }
                int length2 = f204h.length;
                if (length2 == 1) {
                    if (f202f.charAt(f202f.length() - 1) == '*' && strReplaceAll.length() >= f202f.length()) {
                        return true;
                    }
                }
                String str7 = f204h[0];
                if (length2 == 2 && strReplaceAll.length() >= f202f.length()) {
                    char[] charArray5 = strReplaceAll.toCharArray();
                    int iIndexOf3 = f202f.indexOf(42);
                    for (int length3 = strReplaceAll.length() - f202f.length(); length3 >= 0; length3--) {
                        charArray5[iIndexOf3] = ' ';
                        str7 = str7 + " ";
                        if ((str7 + f204h[1]).equals(String.valueOf(charArray5))) {
                            return true;
                        }
                        iIndexOf3++;
                    }
                }
            }
            f201e++;
            if (z || z2) {
                return false;
            }
            c = 0;
        }
        return false;
    }

    @Override // java.util.Iterator
    public final void remove() {
    }
}
