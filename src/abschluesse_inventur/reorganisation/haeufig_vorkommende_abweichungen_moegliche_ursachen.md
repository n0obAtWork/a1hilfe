# Häufig vorkommende Abweichungen / mögliche Ursachen

<!-- source: https://amic.de/hilfe/_hufigvorkommendeabwe.htm -->

- Warenbelege sind noch nicht an die FIBU übertragen

  Es kommt leider häufiger vor, dass in den üblichen Auswahllisten die Bereichseingrenzungen nicht korrekt eingestellt werden. Das kann dann dazu führen, dass einige Belege beim FIBU-Übertrag übersehen wurden.

  Mit ***Konsistenzprüfungen*** kann man sich einen Überblick über sämtliche noch nicht in der FIBU vorhandenen Belege verschaffen (Funktion: ***Belege ohne Fibuübertrag***).

- Unterschiedliche Periodenführung

  <strong>[WABST]</strong> ist eine streng nach Buchungsperioden abgegrenzte Auswertung. In A.eins gibt es mehrere SPA-Einstellungen bezüglich der Periodenzuordnung. Sie sollten auf jeden Fall darauf achten, dass Fibu- und Warenperioden gleichlaufen. Beachten Sie bitte, dass es bei der Erstellung von Sammelrechnungen auch zu unterschiedlichen Perioden der einzelnen Warenpositionen kommen kann, wenn der SPA **Rechnungstrennung durch Periode** auf **nein** steht (Empfehlung: auf **neu** stellen, das heißt alle Warenpositionen erhalten die Periode des neu erstellten Beleges!).

  Die Konsistenz-Funktion ***Belege mit abweichenden Perioden*** erstellt hierfür eine Übersicht. Sollten Periodenunstimmigkeiten auftreten, so können Sie mit der WAREO-Funktion, ***Perioden angleichen*** eine Periodenstimmigkeit erzeugen. Dabei wird folgendermaßen vorgegangen:

  1. Belege, die schon an die FIBU übertragen wurden, werden periodenmäßig an die FIBU angeglichen (d.h. die FIBU-Periode wird nicht verändert, jedoch die Periodenzugehörigkeit in der Ware!).

  2. Unstimmigkeiten der Perioden der Warenpositionen innerhalb eines Beleges werden durch Anpassung an die Periode des Gesamtbeleges aufgelöst.

  **ACHTUNG:** Nach diesem Lauf ist auf jeden Fall auch wieder eine Gesamtreorganisation fällig, da die internen Periodenstatistiken angepasst werden müssen.

- Fehlerhafte Einträge im Mandantenserver

  Im Mandantenserver können sich noch Einträge befinden, die von AMIC-Mitarbeitern zwischenzeitlich deaktiviert wurden (DS_STATUS = 2). Ferner bleiben dort eventuell Einträge mit dem DS_STATUS = 3, wenn der Mandantenserver während des Abarbeitens eines Eintrages abbricht. Hierzu gibt es in der Konsistenzprüfung mehrere Übersichten.

  Die mit DS_STATUS = 3 gekennzeichneten Belege können mit der Konsistenzfunktion**!!! ***Mandantenserver Status 3 freigeben***** wieder aktiviert werden.

  Die Einträge mit DS_STATUS = 2 erfordern eine Bearbeitung von AMIC-Mitarbeitern und sollten nur nach ANWEISUNG bearbeitet werden.

  DS-Status: 0 = noch nicht durchgelaufen, 1= erledigt, 2 = manuell zurückgestellt, 3 = abgebrochen.

- Warenbewegungen / Vorgangsdifferenzen

  Falls unter <strong>[WABST]</strong> Differenzen zwischen den Bereichen WARENBELEGE und WARENPOSITIONEN auftreten und der oben beschriebene Punkt ‚Unterschiedliche Periodenführung’ keine negativen Ergebnisse liefert, sollte man die Konsistenzfunktion ***Warenbewegung / Vorgangsdifferenzen*** starten.

  Als Ergebnis erhält man eine Übersicht über alle Belege, deren Gesamtbelegsumme nicht mit der Summe aus den Einzelpositionen übereinstimmen. In der Regel handelt es sich hier um Auswirkungen von früheren Programmfehlern. Folgende Differenzen können mit folgenden Reparaturanweisungen behoben werden:

- Die Warensumme ist 0.0, die Belegsumme ist ungleich 0.0, WBUANZEIGE ist 0.0.

  Dann sind die Warenbewegungen nicht korrekt in das Warenbuch übernommen worden. Wahrscheinlich ist der Beleg nicht durch den Mandantenserver verarbeitet worden. Hierfür dient unter **[WAREO]** der Punkt ***Mandanten Server Einträge nachtragen***, wodurch der Eintrag in den Mandantenserver nachgeholt wird.

- Die Belegsumme entspricht genau der zweifachen Warensumme

  Hierbei handelt es sich um einen Programmfehler. Mit der Konsistenzfunktion ***!!! Vorgänge mit doppelten Beträgen*** werden diese Belege automatisch richtiggestellt. (Die doppelte Summe wurde beim Übertrag in die FIBU nicht berücksichtigt, die Fibu ist mit diesen Belegen stimmig!)

**Beispiele für evtl. Fehler:**

| Fehler | Fehlermöglichkeiten | Lösungsmöglichkeiten |
| --- | --- | --- |
| Die Summe Erlöse /Aufwand ist in der Ware höher als in der Fibu | Fehler im Fehlerprotokoll Fibu-Übertrag (falsche EKZZ)<br>Fibu-Übertrag nicht gestartet<br>nicht alle Belege übertragen<br>Mandantenserver nicht gelaufen | Fehlerprotokoll prüfen<br>Mandantenserver starten<br>Auswahlbereiche überprüfen<br> |
| Unterschiedliche Periodenführung in der Ware und der Fibu | SPA-Einstellung: Variante Periodenermittlung für FIBU = Datum<br>SPA-Einstellung: Rechnungstrennung durch Periode = NEIN<br>Abweichende Einrichtung Fibu- / Warenperioden | Konsistenzprüfung: Belege mit abweichenden Perioden<br>Wenn Unstimmigkeit, dann in **[WAREO]:**<br>Perioden angleichen<br>Gesamtreorganisation<br> |
| Fehlerhafte Einträge im Mandanten | Absturz, während der Mandantenserver lief<br>Kaputter Eintrag vom AMIC-Support zurückgestellt (DS_STATUS = 2)<br> | Konsistenzprüfung: ***Zurückgestellte Mandantenservereinträge***<br>Konsistenzprüfung: ***Redundante Mandantenservereinträge löschen***<br>Konsistenzprüfung: ***Mandantenserver Status 3 freigeben***<br>alle Prüfungen mit Mandantenserver checken |
| Warenbelegsumme und Artikelsummen stimmen nicht überein | Stammdateneinrichtung (Mengeneinheiten / Gebinde / etc.) | Stammdaten korrigieren<br>AMIC-Support |
| Es fehlen Belege in der Ware, die aber in der Fibu vorhanden sind | Programmfehler<br>Fehler durch AMIC-Support oder Anwender (gelöscht mit OSQL) | Storno-Buchung in der Fibu erzeugen (AR - Betrag)<br>Beleg in der Ware neu erfassen und dann Fibu-Übertrag<br>Fibu-Beleg per OSQL löschen (Nur durch den AMIC-SUPPORT!!) |
| Warenbewegung / Vorgangsdifferenzen | Programmfehler | Konsistenzprüfung: ***Warenbewegung / Vorgangsdifferenzen***<br>Konsistenzprüfung: ***Vorgänge mit doppelten Beträgen***<br>WAREO: ***Mandanten Server Einträge nachtragen***<br>WAREO: ***Abgleich Warenbuch***<br>WABST<br>AMIC-Support |
