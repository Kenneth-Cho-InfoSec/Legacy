package p001b;

import android.content.Context;
import android.content.res.Resources;
import android.content.res.XmlResourceParser;
import android.util.Log;
import com.minn.android.hanziime.R;
import java.util.ArrayList;

/* JADX INFO: renamed from: b.d */
/* JADX INFO: loaded from: classes.dex */
public final class C0032d {

    /* JADX INFO: renamed from: e */
    public static int f221e = 0;

    /* JADX INFO: renamed from: f */
    public static int f222f = 1;

    /* JADX INFO: renamed from: g */
    public static int f223g = 4;

    /* JADX INFO: renamed from: h */
    public static int f224h;

    /* JADX INFO: renamed from: b */
    public C0031c f226b;

    /* JADX INFO: renamed from: c */
    public int f227c;

    /* JADX INFO: renamed from: d */
    public final C0030b[] f228d = new C0030b[5];

    /* JADX INFO: renamed from: a */
    public final ArrayList f225a = new ArrayList();

    public C0032d(Context context) {
        XmlResourceParser xml = context.getResources().getXml(R.xml.hzim_config);
        Resources resources = context.getResources();
        loop0: while (true) {
            boolean z = false;
            while (true) {
                try {
                    int next = xml.next();
                    if (next == 1) {
                        break loop0;
                    }
                    if (next == 2) {
                        String name = xml.getName();
                        if ("InputMethod".equals(name)) {
                            this.f225a.add(new C0031c(resources, context, xml));
                            z = true;
                        } else {
                            "HzimConfig".equals(name);
                        }
                    } else if (next != 3 || !z) {
                    }
                } catch (Exception e) {
                    Log.e("HzimConfig", "Parse error:" + e);
                    e.printStackTrace();
                }
            }
        }
        this.f227c = -1;
    }

    /* JADX INFO: renamed from: a */
    public final boolean m81a() {
        return this.f226b.f212d[11];
    }

    /* JADX INFO: renamed from: b */
    public final boolean m82b(String str) {
        String str2 = this.f226b.f215g;
        if (str2 == null) {
            return false;
        }
        return str2.contains(str);
    }

    /* JADX INFO: renamed from: c */
    public final void m83c(int i) {
        C0031c c0031c = (C0031c) this.f225a.get(i);
        this.f226b = c0031c;
        C0030b c0030bM80a = c0031c.m80a(0);
        C0030b[] c0030bArr = this.f228d;
        c0030bArr[0] = c0030bM80a;
        c0030bArr[1] = this.f226b.m80a(1);
        c0030bArr[2] = this.f226b.m80a(2);
        c0030bArr[3] = this.f226b.m80a(3);
        c0030bArr[4] = this.f226b.m80a(4);
        C0030b c0030b = c0030bArr[3];
        if (c0030b != null) {
            this.f227c = c0030b.f207c;
        } else {
            this.f227c = 1;
        }
    }
}
