# Lagerumbuchung mit Teildispo

<!-- source: https://amic.de/hilfe/_lvs20_lguteildispo.htm -->

Eine klassische Lagerumbuchung in A.eins ist ein „Ganz oder gar nicht“-Prozess. Das bedeutet, dass die teilweise Ausführung einer Planung damit nicht möglich ist.

Um dennoch die Lagerumbuchung mit Teildisposition im LVS zu vollziehen, wendet man die folgende Strategie an:

#### Einrichtung

1. Jedes Lager bekommt einen Kontokorrentkunden, der als steuerfreier Kunde eingerichtet sein muss.

2. Im Lagerstamm [LGS] wird dieser Kunde im jeweiligen Lager hinterlegt.

3. In der Vorgangsunterklasse der Bestellung wird in Kontrollmakro folgender Code eingetragen:

```sql
//INCLUDEMAKRO AMIC_LVS_Lagerumbuchung
public void Vorgang_Nach_Speichern(IVorgang vorg, int modus)
{
  int v_id = 0;
  vorg.GetValue(VORGANG.ID_V_ID, out v_id);
  int techBeleg = 0;
  vorg.GetValue(VORGANG.ID_TECHNISCHERBESTAND, out techBeleg);
  if (techBeleg == 1)
  {
    return;
  }
  int cnt = D.GetExecuteScalar(0, @"select count(*)
                                    from amic_v_vorgaenge vs
                                    join lagerstamm lgs on lgs.KundIdGegenBeleg = kundidzuord
                                    where vs.v_id = ?", v_id);
  if (cnt > 0)
  {
    AMIC_LVS_Lagerumbuchung.AMIC_LVS_LGU lgu = new AMIC_LVS_Lagerumbuchung.AMIC_LVS_LGU();
    lgu.Gegenbeleg_erstellen(v_id, 10);
  }
}
```

#### Ausführung

1. Es wird eine Bestellung erfasst mit dem Kunden des Quell-Lagers und den Artikeln des Ziel-Lagers. Dieser beleg bekommt das Kennzeichen technischer Bestand und wird alle Preise aus dem Bewertungspreis des Quell-Artikels vorschlagen.

2. Beim Speichern wird über das Kontrollmakro ein Aufruf der Lagerumbuchung aus „AMIC_LVS_Lagerumbuchung“ gemacht. Es entsteht ein Auftrag für den Kunden des Ziel-Lagers mit Artikeln des Quell-Lagers.

3. Dieser Auftrag kann nun mittels Ladeschein teilweise ausgeliefert werden.

4. Ladeschein wird wie alle anderen Aufträge abgearbeitet. Erst beim Abschluss wird statt der Löschung der Ladeträger eine Artikelumbuchung im LVS vorgenommen.

5. Es entstehen ein Lieferschein und ein Eingangslieferschein.
