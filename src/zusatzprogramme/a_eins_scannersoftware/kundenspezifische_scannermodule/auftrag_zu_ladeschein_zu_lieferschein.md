# Auftrag zu Ladeschein zu Lieferschein

<!-- source: https://amic.de/hilfe/_scanner_au_zu_la_zu_li.htm -->

Ladescheinliste

Vorbereitung

Um die Abarbeitung des Ladescheins mit dem Scanner zu ermöglichen, wurde eine Vorlage für eine Ladescheinliste im System hinterlegt.

Diese Ladescheinliste befindet sich in der Anwendung „AMIC Etikettendruck“ in der Variante „Vorlagen AMIC Etikettendruck Reporte“. Diese Vorlage muss in den privaten Bereich übernommen werden. Wurde die Vorlage erfolgreich in den privaten Bereich übernommen, so muss diese Vorlage mit der Anwendung Ladeschein verbunden werden.

Anhand dieser Vorlage kann dann eine Individuelle Ladescheinliste erstellt werden. Folgende Scancodes müssen sich auf der Ladescheinliste befinden, damit die Ladescheinliste mit dem Scanner abgearbeitet werden kann.

| Scancode | Bedeutung |
| --- | --- |
| LAB + Ladescheinnummer | Startet die Erfassung des Ladeschein |
| LABENDE | Beendet die Erfassung des Ladescheins |
| AUFLADEN + Ladescheinnummer | Starten das Aufladen |
| AUFLADENENDE | Beendet die Erfassung des Aufladens. |
| STORNO | Mit diesem Befehl ist es möglich, eine erfasste Scannerposition aus dem Ladeschein zu stornieren. |

Achtung: Die privaten Funktion zur Erstellung des Ladescheins und drucken des Ladescheins müssen angepasst werden. Dies gilt, wenn anstelle eines Formulars ein AMIC Etikettendruck Dokument ausgedruckt werden soll. Des Weiteren ist im AMIC Etikettendruck Dokument dann die Vorlauf Funktion zu entfernen.

| Private Funktion | Bedeutung |
| --- | --- |
| ^jpl VorlaufScannerLadeschein 1 | Drucken des Ladescheins |
| ^jpl VorlaufScannerLadeschein 0 | Editieren des Ladescheins |

Ablauf

Mit diesem Modul können [Aufträge](../../../vorgangsabwicklung/vorgangsbearbeitung_allgemein/vorgangsklassen_in_a_eins/auftrag/index.md), die zu einem Ladeschein umgewandelt worden sind, bearbeitet werden. Aus diesen Ladeschein wird dann ein [Lieferschein](../../../vorgangsabwicklung/vorgangsbearbeitung_allgemein/vorgangsklassen_in_a_eins/lieferschein/index.md) erzeugt, und die Lieferscheinmenge wird dann per Teildisposition vom Auftrag abgebucht.

Als erstes muss ein Auftrag mit den aufzuladenden Positionen erfasst werden. Es können auch mehrere Aufträge sein. In der Anwendung „[Aufträge](../../../vorgangsabwicklung/vorgangsbearbeitung_allgemein/vorgangsklassen_in_a_eins/auftrag/index.md)“ und der Variante „Aufträge mit Positionen“ wird dann mit der Funktion „[Ladeschein zusammenstellen](../../../vorgangsabwicklung/vorgangsbearbeitung_allgemein/vorgangsklassen_in_a_eins/auftrag/ladeschein_zusammenstellen.md)“ ein Ladeschein über alle markierten Positionen erstellt.

Als zweite Möglichkeit kann in der Anwendung „Ladeschein“ ein neuer Ladeschein angelegt werden. In diesem Ladeschein kann auf der Positionsebene über die Funktion „Schnelle Teildisposition“ ein Artikel aus einem Auftrag hinzugefügt werden.

Nach dem der Ladeschein zusammengestellt worden ist, und die Ladescheinliste mit der Anwendung Ladeschein verknüpft wurde, kann jetzt die Ladescheinliste ausgedruckt werden. Dazu wird der Ladeschein in der Anwendung [Ladeschein](../../../vorgangsabwicklung/vorgangsbearbeitung_allgemein/vorgangsklassen_in_a_eins/ladeschein/index.md) markiert und mit der Funktion „Scanner Ladeschein drucken“ wird die Ladeliste ausgedruckt.

Nach dem die Liste ausgedruckt wurde, wird diese dann mit dem Scanner abgearbeitet.

Als erstes ist der Befehl **LAB + Ladescheinnummer** von der Ladeliste zu scannen. Dadurch wird die Bearbeitung der Ladeliste mittels Scanner gestartet.

Als nächstes werden dann die Artikelpositionen mit dem Scanner erfasst, welche sich auf der Ladeliste befinden. Es besteht die Möglichkeit entweder die Artikelnummer oder die Seriennummer zu Scannen. Hierbei ist zu beachten, dass bei der Seriennummer Erfassung gleich der Artikel mit der Menge 1 erfasst wird. Bei der Erfassung eines Artikels ohne Seriennummer kann die Menge manuell angegeben werden. Die Artikelnummer oder Seriennummer kann auch manuell eingegeben werden. Dabei ist zu beachten, dass nur Artikelnummern mit mehr als fünf Zeichen als Artikelnummer erkannt werden.

Artikel die sich nicht auf der Ladeliste befinden können mit dem Scanner nicht erfasst werden.

Nach der Erfassung des Artikels wird dann die Menge eingegeben. Es kann nur eine Menge eingegeben werden, welche kleiner als 10000 ist. Die Menge einer Artikelposition kann solange geändert werden, bis ein neuer Artikel erfasst worden ist. Es ist auch möglich eine kleinere Menge als die Artikelmenge einzugeben.

Nach dem alle Artikelpositionen abgearbeitet worden sind wird die Ladescheinbearbeitung mit dem Befehl **LABENDE** beendet. Jetzt wird noch kein Lieferschein erzeugt.

Als nächstes wird der Befehl **AUFLADEN + Ladescheinnummer** gescannt.

Jetzt können, bevor die Wandlung zum Lieferschein passiert, Positionen aus dem erfassten Ladeschein storniert werden. Des Weiteren ist es möglich die aufzuladende Menge zu verringern. Dazu wird der Artikel oder die Seriennummer gescannt und die Menge eingegeben, welche nicht mehr auf das Transportmittel passt.

Mit dem Befehl **AUFLADENENDE** wird automatisch mit dem Modul [Rechnung Lieferschein aus Ladeschein](../../../vorgangsabwicklung/vorgangsbearbeitung_allgemein/vorgangsklassen_in_a_eins/ladeschein/rechnung_lieferschein_aus_ladeschein.md) ein Lieferschein erstellt.

Besonderheiten

Parken

Es besteht die Möglichkeit eine bestehende Abarbeitung eines Ladescheins zu unterbrechen oder von einem anderen Scanner zu Ende zu führen. Die Park Funktion funktioniert nur in diesem Modul. Soll ein neuer Ladeschein bearbeitet werden, so muss nur der LAB Befehl gescannt werden. Jetzt wird dieser Ladeschein aktiv bearbeitet. Der andere Ladeschein ist zurückgestellt und kann später wie gewohnt abgearbeitet werden.

Wird der Ladeschein von einem anderen Scanner bearbeitet, so muss nach der Erfassung des LAB Befehles noch die Zahlenkombination 9999 eingegeben werden, damit der Ladeschein jetzt von diesem Scanner aus bearbeitet werden kann. Der andere Scanner kann jetzt nicht mehr auf dem Ladeschein arbeiten. Es werden alle bisher erfassten Positionen übernommen.

Dies gilt auch für die Aufladen Funktionalität.

Eine parallele Bearbeitung von einem Ladeschein mit mehreren Scannern ist nicht möglich.

Stornierung einer Position und Neustart eines Lieferscheins

Eine erfasste Position kann wie folgt storniert werden. Dazu wird der Stornobefehl gescannt. Danach kann entweder per Scannung des Artikels oder der Seriennummer die letzte erfasste Position des Artikels gelöscht werden. Des Weiteren gibt es die Möglichkeit die zu löschende Positionsnummer manuell über die Tastatur einzugeben. Es wird immer die komplette Position gelöscht.

Durch erneutes Mehrmaliges Scannen von „**LAB + Ladescheinnummer“** wird die bisherige Abarbeitung des Ladescheins zurückgesetzt und die Erfassung kann von vorne beginnen.

Ist ein Ladeschein abgearbeitet worden, so wird mit dem Befehl „**LABENDE**“ die Abarbeitung des Ladescheins beendet. Nach dem der Ladeschein mit „**LABENDE**“ abgeschlossen worden ist, wird aus diesem Beleg noch kein Lieferschein erstellt.

Verhalten bei einer Teillieferung einer Position

In der Anwendung [Vorgangsunterklasse](../../../vorgangsabwicklung/formularzuordnung/formular_formularzuordnungen_zum_vorgang_unterklasse.md) [FRZ] für die Klasse 500 „Ladeschein“ auf der Registerkarte „[Sperren](../../../vorgangsabwicklung/formularzuordnung/sperren.md)“ wird eingestellt, ob nach der Erzeugung des Lieferscheins oder der Rechnung der Ladeschein gesperrt oder Storniert werden soll. Die Standard Einstellung ist das stornieren des Ladescheins.

Soll der Ladeschein gesperrt werden, so bleiben alle Positionen im Ladeschein nach der Umwandlung erhalten und die Weiterverarbeitungssperre wird gesetzt.

Wird der Schalter auf „Ja“ (Standard Einstellung) gestellt, so wird der Ladeschein nur dann komplett storniert wenn alle Positionen des Ladescheins bearbeitet wurden. Als Bearbeitung einer Position gilt auch eine Teillieferung einer Ladescheinposition. Sind Positionen im Ladeschein vorhanden, die nicht bearbeitet wurden, bleibt der Ladeschein mit diesen Positionen im System bestehen. Alle anderen Positionen sind aus dem Ladeschein storniert worden.

Mit dem Steuerparameter „[Stornierung einer Ladescheinposition im Modul Ladeschein zu Lieferschein (1026)](../../../firmenstamm/steuerparameter/vorgangsbearbeitung_warenposition/stornierung_einer_ladescheinposition_im_modul_ladeschein_zu_.md)“ kann eingestellt werden, das teilgelieferte Ladescheinpositionen mit der Restmenge stehen bleiben. Dies gilt nur für die Scanner Abarbeitung.

Dies hat zur Folge, dass ein Ladeschein nicht storniert wird, wenn noch teilgelieferte Positionen im Ladeschein stehen.

Drucken des Lieferscheins

Um einen erstellten Lieferschein automatisch auszudrucken, muss ein Bediener LaLiDru im System eingerichtet sein. Auf dem Standard Drucker des Bedieners wird dann der Lieferschein ausgedruckt.

Die erfassten Daten werden in der Anwendung [Vorgangsimport](../../../vorgangsabwicklung/vorgangsimport/index.md) [VIMP] gespeichert.

Hauptmenü > Externe Kommunikation > Stammdatenimport > Vorgangsimport

Folgender Status können die Erfassten Sätze haben

| Status | Status Bedeutung | Bedeutung |
| --- | --- | --- |
| 0 | Übernahem Läuft | Ladeschein wird erfasst |
| 2 | Bereit | Ladescheinerfassung beendet |
| 3 | Konvertierung Läuft | Konvertierung läuft. Aufladen läuft |
| 5 | Konvertierung Ende | Aufladen ende |
| 6 | Belegerzeugung Läuft | Belegerzeugung läuft. |
| 7 | Fehler Belegerzeugung | Es ist ein Fehler bei der Abarbeitung aufgetreten |
| 8 | erledigt | Belegerzeugung abgeschlossen. |
| 9 | Gelöscht | Diese Position ist gelöscht worden z.B. durch das Zusammenfassen von Artikelpositionen |

| Menge Blockende | 3 | 1 | Vor der Ausgabe der Anzeige |
| --- | --- | --- | --- |
| Artikel Blockanfang | 2 | 0 | Direkt nach dem Ende des Menge Blocks. Der Artikelblock hat keinen Separtenstartpunkt |
| Artikel Blockende | 2 | 1 | Vor der Ausgabe der Anzeige |
