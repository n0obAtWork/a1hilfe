# PDF-Drucken

<!-- source: https://amic.de/hilfe/pdfdrucken.htm -->

Diese Funktion ist über [Dokumentenverwaltung- Multifunktionsleiste](./index.md) und über die Anwendung

[Anwendung Formulararchiv](../../archiv_administration/anwendung_formulararchiv/index.md) verfügbar.

Nach Auswahl von Dokumenten werden die PDF-Dokumente gefiltert und zum Druck angeboten.

| Felder: |
| --- |
| Drucker | Pflichtfeld | Angabe des Windows-Druckers<br>Die Online-Verfügbarkeit wird geprüft und optisch durch einen grünen Haken belegt.<br>![](../../../ImagesExt/image8_848.png)<br>Mausklick auf dieses Sysmbol oder betätigen der F3-Taste ruft den „Windows-Drucker-Auswahl-Dialog“ auf.<br>Die Angabe des Druckers wird sich für den erneuten Aufruf gemerkt und ist bei Ersteintritt der Windows-Standard-Drucker. |
| PDF-Dokumente | Anzeige | Anzahl der zu druckenden Dokumente |

| Funktionen: |
| --- |
| Drucken | F9 | Druckt die vorgesehenen Dokumente auf den ausgewählten Drucker |

Der PDF-Druck ist programmatisch durchführbar.

1) Für Makro2 siehe IArchiv.PrintPDF.

2) Für andere Scriptsprachen steht die JPP-Methode „PrintPdf“ im JPP-Objekt „JFA_View“ zur Verfügung.

| Parameter: |
| --- |
| fa_id | | Schlüssel |
| fa_mndnr | | Schlüssel (Angabe optional) |
| printer | | Drucker |
