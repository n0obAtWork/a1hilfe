# Anhang

<!-- source: https://amic.de/hilfe/anhang.htm -->

Die kalkulatorischen Frachten werden im Vorgang gespeichert in der Relation v_posiware als vp_warekalkfrach.

Folgende view liefert die Werte in eine Auswahlliste:

```sql
// Private View
P_Fracht_Kalk  ---  LA   28.07.2000
//
// Beschreibung:
//
//
//
CREATE VIEW P_Fracht_Kalk AS
//
 SELECT
a.artikelnummer,a.artikelbezeich,vp.vp_warekalkfrach,
 vp.artikelid,w.lagernummer
//
  FROM vorgangstamm vs JOIN v_posiware vp ON
(vs.v_id=vp.v_id)
  JOIN warenbewegung w ON
(w.wabewid=vp.wabewid)
  JOIN artikel a ON (a.artikelid=w.artikelid)
 //
 WHERE
 vs.v_klassnummer IN
(700,790,800,890,1700,1790,1800,1890) AND
 (vs.v_statusweiter
< 2)
```

Diese view kann dann z.B. in WBA in die Variante „Kum. Artikel Summen“ mit folgendem Befehl eingebaut werden:

(select sum(fra.vp_warekalkfrach) from P_Fracht_Kalk fra

where fra.artikelid=a.artikelid) KalkFra,
