# Lagerplatzumbuchung

<!-- source: https://amic.de/hilfe/_scanner_lagerplatzumbuchung.htm -->

Bei der Lagerplatzumbuchung können einzelne Artikel vom Lagerplatz A zum Lagerplatz B in einem Lager umgebucht werden.

Es kann zurzeit pro Lagerplatzumbuchungsblock auf dem Scanner nur ein Artikel umgebucht werden.

Partien werden bislang noch nicht berücksichtigt.

Itembox zur Darstellung der Daten auf dem Scanner

IB_CE_Lagerplatzumbuchung

Voraussetzungen

Als erstes müssen folgende [Einrichtungen](../../identass_inventur_schnittstelle/index.md) vorgenommen werden.

Des Weiteren müssen folgende Texte im EAN 128 Konvertiert werden um eine Lagerplatzumbuchung auf dem Scanner zu starten. An dieser [Stelle](./beispiel_scancodes_fuer_die_lagerplatzumbuchung.md) sind die Beispiel Scancodes für die Lagerplatzumbuchung hinterlegt worden.

1. LGPU

2. LGPUENDE

3. STORNO

4. Etiketten für die einzelnen Lagerplätze. Der Lagerplatznummer muss der [AI-Code](../../technische_daten/application_identifier_des_ean_128_ucc_128.md) 91 vorangestellt werden. z.B. 9112345 wobei 12345 der Lagerplatz ist

Ablauf

Als erstes wird der Startscancode **LGPU** mittels Scanner erfasst.

Artikeleingabe

Nach dem der Scancode erfolgreich verarbeitet worden ist, muss als nächstes der Artikel erfasst werden. Hier kann der EAN-Code entweder per Scanner gescannt werden, oder per Hand eingegeben werden. Des Weiteren ist es möglich die Artikelnummer per Hand zu erfassen. Sollte der erfasste Artikel nicht gefunden werden, so wird in der Relation ImportVorgPosition auch ein neuer Datensatz angelegt. Dieser kann im Nachlauf unter [Vorgangsimport](../../../../vorgangsabwicklung/vorgangsimport/index.md) [VIMP]bearbeitet werden.

**Wenn ein Artikel nicht gefunden worden ist, kann dies folgende Ursachen haben.**

1. Der Artikel befindet sich nicht auf dem Aktuellen Lager des Scannerbedieners.

2. Der Artikel ist nicht mehr gültig

3. Der Artikel existiert nicht im System

4. Die EAN-Nummer ist nicht im Sekundschlüssel hinter legt worden.

Mengeneingabe

Nach dem Artikel erfasst worden ist wird die Eingabe der Menge erwartet. Es ist möglich eine 0 Menge einzugeben.

**Gebindebehandlung**

Die Gebindefaktoren werden über zwei Unterschiedliche Wege bestimmt.

1. Der erste Weg ist per EAN Code des erfassten Artikels. Ist die erfasste Artikel-EAN im [Sekundschlüsse](../../../../artikelstamm_und_artikel/parameter_des_artikelstamms/sekundaerschluessel.md)l in der Gruppe 11 hinterlegt, so entspricht die Zeile in der Gruppe im Sekundschlüssel den Gebindefaktor. Dies bedeutet, Zeile 1 ist der Gebindefaktor1. Dann wird nur der Gebindefaktor1 übernommen. Mit der Datenbankfunktion [Gebinde](http://www.amic.de/ihilfe/XMLDocuments/iAeins/html/M_SQL_FUNCTION_Gebinde_12_8bab589d.htm) werden dann alle weiteren Gebindefaktoren bestimmt. **Diese muss dann bei der Vorgangserzeugung in der Positionsanlage aufgerufen werden um die restlichen Gebindefaktoren zu bestimmen**.

2. Der zweite Weg ist die Suche über die [Mengeneinheit](../../../../artikelstamm_und_artikel/konstanten_der_artikelverwaltung/mengeneinheiten.md). Hier werden die [Gebindefaktoren](../../../../artikelstamm_und_artikel/konstanten_der_artikelverwaltung/mengeneinheiten_mit_umrechnungen_ergebnismengeneinheit/mengeneinheiten_mit_gebinde/index.md) anhand des Gebindetyps bestimmt. In diesem Fall werden alle Gebindefaktoren übernommen.

Nach dem die Menge eingegeben worden ist muss jetzt der Abgangslagerplatz gescannt werden. Die Menge auf der erzeugten Lagerplatzumbuchung im A.eins System wird dann aber auf die Grundmengeinheit zurückgeführt.

Abgangslagerplatz - Zugangslagerplatz

Hier ist es auch möglich den Lagerplatz per Hand einzugeben. Der AI-Code 91 muss nicht mit angegeben werden.

Nach der Eingabe des Abgangslagerplatzes wird der Zugangslagerplatz erwartet. Sollte das Abgangslagerplatz oder das Zugangslagerplatz nicht im System vorhanden sein, so wird der Standard Lagerplatz 0 gewählt.

Storno

Mit dem Befehl **STORNO** ist es möglich solange die Lagerplatzumbuchung nicht abgeschlossen worden ist, diese erfasste Position oder die komplette Lagerplatzumbuchung zu stornieren.

Durch das erstmalige Scannen des Befehls **STORNO** wird der Scanner in den Stornomodus gesetzt. Es wurde noch nichts gelöscht. Durch nochmaliges Scannen des Befehls **STORNO** wird die Position gelöscht sofern eine Position erfasst worden ist. Der Befehl **STORNO** kann mehrmals hintereinander gescannt werden. Wenn keine Position mehr vorhanden ist, so wird die Lagerplatzumbuchung storniert. Wird nach einem **STORNO** Kommando ein nicht **STORNO** Befehl erfasst, so wird die normale Verarbeitung fortgeführt.

Um eine Lagerplatzumbuchung mit dem Scanner abzuschließen muss der Befehl **LPGUENDE** erfasst werden.

Die erfassten Daten werden in der Anwendung [Vorgangsimport](../../../../vorgangsabwicklung/vorgangsimport/index.md) [VIMP] gespeichert und können in der Anwendung noch bearbeitet werden.

Hauptmenü > Externe Kommunikation > Stammdatenimport > Vorgangsimport

Einstiegspunkte in der Prozedur ScannerLagerplatzumbuchung

Folgende Einstiegspunkte sind realisiert worden

| Punkt | Bereich | Typ | |
| --- | --- | --- | --- |
| Lagerplatzumbuchung Anfang Blockanfang | 1 | 0 | Direkt nach dem LGPU Kommando |
| Lagerplatzumbuchung Anfang Blockende | 1 | 1 | Vor der Ausgabe der Anzeige |
| Artikel Blockanfang | 2 | 0 | Direkt nach dem Beginn des Artikel Blocks |
| Artikel Blockende | 2 | 1 | Vor der Ausgabe der Anzeige |
| Menge Blockanfang | 3 | 0 | Direkt nach dem Blockstart der Mengeneingabe |
| Menge Blockende | 3 | 1 | Vor der Ausgabe der Anzeige |
| Lagerplatzblock gilt für den Abgangslagerplatz sowie für den Zugangslagerplatz | 4 | 0 | Direkt nach dem Beginn des Blockes |
| Lagerplatzblock gilt für den Abgangslagerplatz sowie für den Zugangslagerplatz | 4 | 1 | Vor der Ausgabe der Anzeige |
| Lagerplatzumbuchung Ende Blockanfang | 5 | 0 | Direkt nach dem LGPUENDE Kommando. Hier gibt es kein Blockende, da nur der Status auf 3 gesetzt wird. Der Ladeschein wird später weiterverarbeitet |

<p class="siehe-auch">Siehe auch:</p>

- [Beispiel Scancodes für die Lagerplatzumbuchung](./beispiel_scancodes_fuer_die_lagerplatzumbuchung.md)
