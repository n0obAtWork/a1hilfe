# Eingangslieferschein

<!-- source: https://amic.de/hilfe/_scanner_eingangslieferschein.htm -->

Mit diesem Modul ist es möglich ein Eingangslieferschein mittels eines Scanners zu erfassen. Bei der Erfassung eines Eingangslieferscheins besteht die Möglichkeit den erfassten Artikel einem bestimmten Lagerplatz zu zuweisen.

Das Erfassen von Partien ist bislang noch nicht berücksichtigt worden.

#### Itembox zur Darstellung der Daten auf dem Scanner

IB_CE_VIMP_Eingangslieferschein

#### Voraussetzungen

Als erstes müssen folgende [Einrichtungen](../../identass_inventur_schnittstelle/index.md) vorgenommen werden.

Des Weiteren müssen folgende Texte im EAN 128 Konvertiert werden um eine Lagerplatzumbuchung auf dem Scanner zu starten. . An dieser [Stelle](./beispiel_scancodes_fuer_den_eingangslieferschein.md) sind die Beispiel Scancodes für die Eingangslieferscheine hinterlegt worden.

1. EL

2. ELENDE

3. STORNO

4. Etiketten für die einzelnen Lagerplätze. Der Lagerplatznummer muss der [AI-Code](../../technische_daten/application_identifier_des_ean_128_ucc_128.md) 91 vorangestellt werden. z.B. 9112345 wobei 12345 der Lagerplatz ist

5. Die Lieferanten ILN. Der Lieferanten ILN muss der [AI-Code](../../technische_daten/application_identifier_des_ean_128_ucc_128.md) 00 und eine 3 Vorangestellt werden z.B. 003123456

Der Scanner muss auf das aktuelle Lager eingestellt sein.

#### Ablauf

Als erstes wird der Startscancode **EL** mittels Scanner erfasst.

#### Lieferanteneingabe

Die Lieferanteneingabe erfolgt nach der Erfassung des Startscancodes. Die Eingabe des Lieferanten kann auch übersprungen werden, denn der Lieferant kann im Nachlauf unter [Vorgangsimport](../../../../vorgangsabwicklung/vorgangsimport/index.md) [**VIMP**] hinzugefügt werden kann. Die ILN Nummer wird im [Lieferanten- / Kundenstamm](../../../../kunden_und_lieferanten/uebersicht_kunden_und_lieferanten.md) im Feld ILN hinterlegt

#### Artikeleingabe

Als nächstes wird der Artikel erfasst. Hier kann der EAN-Code entweder per Scanner gescannt werden, oder per Hand eingegeben werden. Des Weiteren ist es möglich die Artikelnummer per Hand zu erfassen. Sollte der erfasste Artikel nicht gefunden werden, so wird in der Relation ImportVorgPosition auch ein neuer Datensatz angelegt. Diesem kann im Nachlauf unter [Vorgangsimport](../../../../vorgangsabwicklung/vorgangsimport/vorgangsimport_mit_opentrans/zeitgesteuerter_importprozess.md) [**VIMP**] bearbeitet werden ein Artikel hinzugefügt werden.

Hauptmenü > Externe Kommunikation > Stammdatenimport > Vorgangsimport

**Wenn ein Artikel nicht gefunden worden ist, kann dies folgende Ursachen haben.**

5. Der Artikel befindet sich nicht auf dem Aktuellen Lager des Scannerbedieners.

6. Der Artikel ist nicht mehr gültig

7. Der Artikel existiert nicht im System

8. Die EAN-Nummer ist nicht im Sekundschlüssel hinter legt worden.

#### Mengeneingabe

Nach dem Artikel erfasst worden ist wird die Eingabe der Menge erwartet. Es ist möglich eine 0 Menge einzugeben. Ist die Menge nicht bekannt während der Erfassung muss hier eine 0 eingegeben werden. Ansonsten kann der Position kein Lagerplatz zugewiesen werden.

**Gebindebehandlung**

Die Gebindefaktoren werden über zwei Unterschiedliche Wege bestimmt.

1. Der erste Weg ist per EAN Code des erfassten Artikels. Ist die erfasste Artikel-EAN im [Sekundschlüsse](../../../../artikelstamm_und_artikel/parameter_des_artikelstamms/sekundaerschluessel.md)l in der Gruppe 11 hinterlegt, so entspricht die Zeile in der Gruppe im Sekundschlüssel den Gebindefaktor. Dies bedeutet, Zeile 1 ist der Gebindefaktor1. Dann wird nur der Gebindefaktor1 übernommen. Mit der Datenbankfunktion [Gebinde](http://www.amic.de/ihilfe/XMLDocuments/iAeins/html/M_SQL_FUNCTION_Gebinde_12_8bab589d.htm) werden dann alle weiteren Gebindefaktoren bestimmt. **Diese muss dann bei der Vorgangserzeugung in der Positionsanlage aufgerufen werden um die restlichen Gebindefaktoren zu bestimmen**.

2. Der zweite Weg ist die Suche über die [Mengeneinheit](../../../../artikelstamm_und_artikel/konstanten_der_artikelverwaltung/mengeneinheiten.md). Hier werden die [Gebindefaktoren](../../../../artikelstamm_und_artikel/konstanten_der_artikelverwaltung/mengeneinheiten_mit_umrechnungen_ergebnismengeneinheit/mengeneinheiten_mit_gebinde/index.md) anhand des Gebindetyps bestimmt. In diesem Fall werden alle Gebindefaktoren übernommen.

Nach dem die Menge eingegeben worden ist muss jetzt der Lagerplatz gescannt werden

#### Lagerplatz

Hier ist es auch möglich den Lagerplatz per Hand einzugeben. Der AI-Code 91muss bei der Handeingabe nicht mit angegeben werden.

#### Storno

Mit dem Befehl **STORNO** ist es möglich solange der Eingangslieferschein nicht abgeschlossen worden ist, diese erfasste Position oder die komplette Eingangslieferschein zu stornieren.

Durch das erstmalige Scannen des Befehls **STORNO** wird der Scanner in den Stornomodus gesetzt. Es wurde noch nichts gelöscht. Durch nochmaliges Scannen des Befehls **STORNO** wird die Position gelöscht sofern eine Position erfasst worden ist. Der Befehl **STORNO** kann mehrmals hintereinander gescannt werden. Wenn keine Position mehr vorhanden ist, so wird der Eingangslieferschein storniert. Wird nach einem **STORNO** Kommando ein nicht **STORNO** Befehl erfasst, so wird die normale Verarbeitung fortgeführt.

Um eine Eingangslieferschein mit dem Scanner abzuschließen muss der Befehl **ELENDE** erfasst werden.

Die erfassten Daten werden in der Anwendung [Vorgangsimport](../../../../vorgangsabwicklung/vorgangsimport/index.md) [**VIMP**] gespeichert und können in der Anwendung noch bearbeitet werden.

Hauptmenü > Externe Kommunikation > Stammdatenimport > Vorgangsimport

#### Einstiegspunkte in der Prozedur ScannerEingangslieferschein

#### Folgende Einstiegspunkte sind realisiert worden

| Punkt | Bereich | Typ | |
| --- | --- | --- | --- |
| Eingangslieferschein Anfang Blockanfang | 1 | 0 | Direkt nach dem EL Kommando |
| Eingangslieferschein Anfang Blockende | 1 | 1 | Vor der Ausgabe der Anzeige |
| Artikel Blockanfang | 2 | 0 | Direkt nach dem Beginn des Artikel Blocks |
| Artikel Blockende | 2 | 1 | Vor der Ausgabe der Anzeige |
| Menge Blockanfang | 3 | 0 | Direkt nach dem Blockstart der Mengeneingabe |
| Menge Blockende | 3 | 1 | Vor der Ausgabe der Anzeige |
| Lagerplatzblock Anfang | 4 | 0 | Direkt nach dem Beginn des Blockes |
| Lagerplatzblock Ende | 4 | 1 | Vor der Ausgabe der Anzeige |
| Eingangslieferschein Ende Blockanfang | 5 | 0 | Direkt nach dem LGPUENDE Kommando. Hier gibt es kein Blockende, da nur der Status auf 3 gesetzt wird. Der Ladeschein wird später weiterverarbeitet |

<p class="siehe-auch">Siehe auch:</p>

- [Beispiel Scancodes für den Eingangslieferschein](./beispiel_scancodes_fuer_den_eingangslieferschein.md)
