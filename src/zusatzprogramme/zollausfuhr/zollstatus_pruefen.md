# Zollstatus prüfen

<!-- source: https://amic.de/hilfe/zollstatusprfen.htm -->

Wurde eine Zollanmeldung erfolgreich versendet, so wird sie irgendwann vom Zoll bearbeitet worden sein. Sie können manuell den Status dieser Anmeldung beim Datendienstleister anfragen.

Dazu wählen Sie die von Ihnen bearbeitete Zollanmeldung aus der Auswahlliste in der Anwendung Zoll-Lieferscheine mit dem Direktsprung [LIZO] mit dem Status „versendet“ aus und wählen die Funktion „Zollstatus prüfen“.

Diese Prüfung kann auch periodisch durch ein sog. Event erfolgen. Mehr dazu im Abschnitt Automatische Prüfung und Abholung.

Mögliche weitere Status sind:

• nicht genehmigt

• ABD bereitgestellt

• ABD abgeholt

• AgV bereitgestellt

• AgV abgeholt

Zur Statusprüfung wird die Prozedur AMIC_STATUS_FUNKTION aufgerufen. Die Prozedur erstellt das für den Webservice notwendige XML und sendet es über die Prozedur *getAtlasExportTransaction* an den an den AEB-Webservice. Die Rückmeldung wird verarbeitet, Status-Informationen gesetzt und Meldungen zusammengestellt. (Näheres siehe Dokumentation zur Datenbankprozedur.)
