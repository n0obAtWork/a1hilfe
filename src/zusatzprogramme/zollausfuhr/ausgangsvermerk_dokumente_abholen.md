# Ausgangsvermerk-Dokumente abholen

<!-- source: https://amic.de/hilfe/ausgangsvermerkdokumenteabhole.htm -->

Hat die Ware zum Ausfuhr-Vorgang das Zollgebiet der EU verlassen, so wird in der Regel von der Ausgangs-Zollstelle ein Ausgangsvermerk erstellt. Liegt dieses Dokument beim Dienstleister vor, so wird dieses zur Abholung bereitgestellt. Der Status der Zollanmeldung ist dann „AgV liegt bereit“. Sie können nun das Ausgangsvermerk-Dokument abholen.

Dazu wählen Sie die von Ihnen bearbeitete Zollanmeldung aus der Auswahlliste in der Anwendung Zoll-Lieferscheine mit dem Direktsprung [LIZO] mit dem Status „AgV liegt bereit“ aus und wählen die Funktion „Ausgangsvermerk-Dokumente abholen“.

Diese Abholung kann auch periodisch durch ein sog. Event erfolgen. Mehr dazu im Abschnitt Automatische Prüfung und Abholung.

Der Status der Zeile wechselt von „AgV liegt bereit“ auf „AgV abgeholt“.

Zur Abholung wird die Prozedur *AMIC_ABD_FUNKTION* aufgerufen. Die Prozedur erstellt das für den Webservice notwendige XML und sendet es über die Prozedur *getAtlasExportTransaction* an den an den AEB-Webservice. Die Rückmeldung wird verarbeitet, das Ausgangsvermerk-Dokument ins Formulararchiv geschrieben und die Status-Informationen gesetzt. (Näheres siehe Dokumentation zur Datenbankprozedur.)
