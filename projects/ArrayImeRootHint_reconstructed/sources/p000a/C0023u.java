package p000a;

import android.content.res.XmlResourceParser;
import android.util.Log;
import com.minn.android.hanziime.R;
import java.io.IOException;
import java.util.ArrayList;
import java.util.HashMap;
import java.util.Iterator;
import org.xmlpull.v1.XmlPullParserException;

/* JADX INFO: renamed from: a.u */
/* JADX INFO: loaded from: classes.dex */
public final class C0023u implements InterfaceC0003b0 {

    /* JADX INFO: renamed from: a */
    public final HashMap f175a;

    /* JADX INFO: renamed from: b */
    public final AbstractC0010h f176b;

    /* JADX INFO: renamed from: c */
    public ArrayList f177c;

    /* JADX INFO: renamed from: d */
    public Iterator f178d;

    public C0023u(AbstractC0010h abstractC0010h) {
        this.f176b = abstractC0010h;
        HashMap map = new HashMap();
        this.f175a = map;
        map.put("e", m65e(R.xml.symbols_latin12_list));
        map.put("n", m65e(R.xml.symbols_default_list));
        this.f177c = null;
    }

    @Override // p000a.InterfaceC0003b0
    /* JADX INFO: renamed from: a */
    public final int mo45a(C0002b c0002b) {
        return 0;
    }

    @Override // p000a.InterfaceC0003b0
    /* JADX INFO: renamed from: b */
    public final C0005c0 mo46b() {
        Iterator it = this.f178d;
        if (it == null || !it.hasNext()) {
            return null;
        }
        return new C0005c0((String) this.f178d.next(), 0);
    }

    @Override // p000a.InterfaceC0003b0
    /* JADX INFO: renamed from: c */
    public final int mo47c(C0002b c0002b) {
        ArrayList arrayList = this.f177c;
        if (arrayList == null) {
            this.f178d = null;
            return 0;
        }
        this.f178d = arrayList.iterator();
        return 1;
    }

    /* JADX INFO: renamed from: e */
    public final ArrayList m65e(int i) {
        String str;
        ArrayList arrayList = new ArrayList();
        AbstractC0010h abstractC0010h = this.f176b;
        XmlResourceParser xml = abstractC0010h.getResources().getXml(i);
        while (true) {
            try {
                int next = xml.next();
                if (next == 1) {
                    break;
                }
                if (next == 2 && "string".equals(xml.getName())) {
                    int attributeResourceValue = xml.getAttributeResourceValue(null, "value", 0);
                    String attributeValue = attributeResourceValue == 0 ? xml.getAttributeValue(null, "value") : abstractC0010h.getString(attributeResourceValue);
                    if (attributeValue != null) {
                        arrayList.add(attributeValue);
                    }
                }
            } catch (IOException unused) {
                str = "Unable to read keyboard resource file";
                Log.e("HzimService", str);
            } catch (XmlPullParserException unused2) {
                str = "Ill-formatted keybaord resource file";
                Log.e("HzimService", str);
            }
            return arrayList;
        }
        xml.close();
        return arrayList;
    }

    /* JADX INFO: renamed from: f */
    public final void m66f(String str) {
        this.f177c = (ArrayList) this.f175a.get(str);
    }

    @Override // p000a.InterfaceC0003b0
    /* JADX INFO: renamed from: d */
    public final void mo48d() {
    }
}
