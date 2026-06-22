# Archiv Barcode

<!-- source: https://amic.de/hilfe/_archivbarcode.htm -->

Das A.eins-Archiv unterstützt nun direkt die zentrale Erfassung von Belegen über Barcode-Systeme.

Zu diesem Zweck ist ein Feld „Barcode“ ins Archiv eingeführt worden und steht somit in allen betreffenden Dialogen und Auswahlen zur Verfügung.

Bei der Einrichtung des zentralen Imports der Belege ist darauf zu achten, dass der ermittelte Barcode dem dafür zugewiesenen neuen Feld zugeordnet wird (In FA-Spalte „Barcode“) und das die Belegklasse mit der Konstanten 8019 (Belegklasse Barcode) ausgewiesen wird.

Die Zuordnung eines solchen integrierten Barcode-Beleges kann dann kontextabhängig an allen Stellen, wo eine „Archiv anzeigen“-Funktionalität verfügbar ist, über die dortige Funktion ***Barcode zuweisen*** durchgeführt werden. In diesem Falle hat man idealerweise das Dokument zum Abscannen des Barcodes vorliegen. Intern werden dann alle entsprechenden Archiv-Barcode-Belege – sofern noch nicht geschehen – mit den Archiv-Kontextdaten (z.B. Referenz, Kundennummern Belegnummer sofern verfügbar) angereichert und stehen somit direkt im jeweiligen Kontext zur Verfügung.

| Felder |
| --- |
| Referenz | Referenz des Programm-Kontextes |
| Belegnummer | Belegnummer des Programm-Kontextes |
| KndNr. | Kundennummer des Programm-Kontextes |
| Barcode | Hier ist der zu suchende Barcode anzugeben bzw. einzuscannen |

| Funktionen |
| --- |
| Hinzufügen [F9] | Versucht den angegebenen Barcode im Formulararchiv zu finden und gemäß obigen Vorgaben zu verschlagworten. |
