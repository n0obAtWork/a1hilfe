# Wareneingang

<!-- source: https://amic.de/hilfe/_lvs20_wareneingang.htm -->

Im Wareneingang geht es darum, die in A.eins erfassten Daten mit den Ladeträgern zu verknüpfen. In der Bestellung werden in der Regel noch keine Partien festgelegt. Dies geschieht erst bei Anlieferung im Entladeschein.

Die Ladepapiere werden also dazu verwendet, einen Entladeschein zu erstellen, der die angelieferten Mengen aus den Bestellpositionen zusammenfasst. Dabei können auch Anlieferungen aus verschiedenen Bestellungen unter Umständen sogar von verschiedenen Lieferanten zusammengefasst werden.

Hier werden nun auch Partien zugeordnet.

Der Entladeschein wird gedruckt und kann im Lager abgearbeitet werden.

**Empfohlener Arbeitsablauf Scanner:**

1. Scan der Entladescheinposition (ggf. mit Partie)

2. Scan der NVE (Ladeträgeridentifikation)

3. Eingabe der Menge auf dem Ladeträger

Somit sind Ladeträger, der Artikel, die Partie und die Menge miteinander verbunden.

Beim Abschluss des Wareneingangs wird eine Summe über die einzelnen Positionen gebildet und der Eingangslieferschein aus der Bestellung teildisponiert. Dabei werden Partie-Informationen und Mengen aus dem Entladeschein verwendet.

Die Datenbank-Funktion „AMIC_LVS_WE_Abschluss“ sorgt dafür, dass die gesammelten Vorgangsimporte (LVS) in eine Teildispo umgewandelt werden.

Beide Sätze müssen anschließend in einen Verarbeitungsstatus gesetzt werden. (2 für LVS, 5 für Ladeschein)

Die eigentliche Belegerstellung erfolgt durch die Verarbeitung im Vorgangsimport.

Achtung! Um die Partieinformationen aus dem Ladeschein zu verwenden, muss das Vorgangsimportkontrollmakro des Eingangslieferscheins dies unterstützen.

Der folgende Code macht dies beispielsweise:

```sql
public void
WPos_Nach_Ladeschein_zu_ReLi(IVorgang vorg, int
modus, int wabewId_SRC = 0, int wabewid_LD = 0, bool teilUmwandlung = false)
{
  if (wabewid_LD
!= 0)
  {
    //
Partien nachpflegen
    DataTable dt = D.GetSql(@"select distinct ivp.PartieId,isnull(vp.PartieArtiPosit,
pa.PartieArtiPosit) as PartieArtiPosit, vp.V_PosiParMenge

from WARENBEWEGUNG wb

join V_POSIWARE vw on wb.wabewid = vw.wabewid

join artikel a on a.artikelid = wb.artikelid

left outer join V_POSIPARTIE vp on vw.v_id = vp.v_id and vw.v_posizaehler =
vp.v_posiZaehler

join ImportVorgPosition ivp on ivp.internereferenz = wb.wabew_guid and
ivp.satzid = 2

join partiestamm ps on ps.partieid = ivp.partieid

join partieartikel pa on pa.artikelid = a.artistammid and pa.partieid =
ivp.partieid

where wb.wabewid = ?", wabewid_LD);

    if
(dt.Rows != null && dt.Rows.Count >
0)
    {
      vorg.StartPartie();
      float ergebnisMenge = 0;

vorg.GetValPos(WARENPOSITION.ID_MENGE, out
ergebnisMenge);

      foreach (DataRow zeile in dt.Rows)
      {
        float menge = (float)D.Get<decimal>(zeile["V_PosiParMenge"], (decimal)ergebnisMenge);
        int partieid = D.Get<Int32>(zeile["PartieId"], 0);
        int partieArtiPosit = D.Get<Int32>(zeile["PartieArtiPosit"], 0);

vorg.AddPartieMenge(partieid: partieid, partieartiposit: partieArtiPosit, menge:
menge);
      }
      vorg.EndPartie();
    }
  }
}
```
