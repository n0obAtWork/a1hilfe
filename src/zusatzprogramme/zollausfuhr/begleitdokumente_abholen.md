# Begleitdokumente abholen

<!-- source: https://amic.de/hilfe/begleitdokumenteabholen.htm -->

Wurde eine Zollanmeldung von der Zolldienststelle genehmigt, so wird ein Ausfuhrbegleitdokument bereitgestellt. Der Status der Zollanmeldung ist dann „ABD liegt bereit“. Sie können nun das Ausfuhrbegleitdokument abholen.

Dazu wählen Sie die von Ihnen bearbeitete Zollanmeldung aus der Auswahlliste in der Anwendung Zoll-Lieferscheine mit dem Direktsprung [LIZO] mit dem Status „ABD liegt bereit“ aus und wählen die Funktion „Begleitdokumente abholen“.

Diese Abholung kann auch periodisch durch ein sog. Event erfolgen. Mehr dazu im Abschnitt Automatische Prüfung und Abholung.

Der Status der Zeile wechselt von „ABD liegt bereit“ auf „ABD abgeholt“.

Zur Abholung wird die Prozedur *AMIC_ABD_FUNKTION* aufgerufen. Die Prozedur erstellt das für den Webservice notwendige XML und sendet es über die Prozedur *getAtlasExportTransaction* an den an den AEB-Webservice. Die Rückmeldung wird verarbeitet, das Ausfuhrbegleitdokument ins Formulararchiv geschrieben und die Status-Informationen gesetzt. (Näheres siehe Dokumentation zur Datenbankprozedur.)
