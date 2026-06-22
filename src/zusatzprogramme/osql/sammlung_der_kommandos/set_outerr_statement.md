# SET OUTERR Statement

<!-- source: https://amic.de/hilfe/setouterrstatement.htm -->

Syntax

SET OUTERR [Dateiname]

Purpose

Fehlermeldungen umlenken in Datei (append)

Anwendung

Kommandodatei, Befehlszeile

Berechtigung

Alle Anwender

Siehe auch

[CONTINUE](./continue_statement.md), [SET ERROR](./set_error_statement.md)

Beschreibung

Fehlermeldungen, die während des Laufes einer Kommandodatei entstehen können in eine Datei umgelenkt werden, um sie Anschließend zu kontrollieren. Dies ist umso wichtiger, wenn man die Fehlermeldung auf dem Bildschirm ausgeschaltet hat. Geschlossen wird die Fehlerdatei wieder, wenn SET OUTERR ohne Dateinamen angegeben wird.

Beispiel

SET ERROR NODISPLAY;

SET OUTERR c:\\ERR.TXT;

Select \* from DIESERELATIONEXISTIERTNICHT;

SET ERROR NODISPLAY;

SET OUTERR;
