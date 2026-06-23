# SET OUTERR Statement

<!-- source: https://amic.de/hilfe/setouterrstatement.htm -->

<p class="just-emphasize">Syntax</p>

SET OUTERR [Dateiname]

<p class="just-emphasize">Purpose</p>

Fehlermeldungen umlenken in Datei (append)

<p class="just-emphasize">Anwendung</p>

Kommandodatei, Befehlszeile

<p class="just-emphasize">Berechtigung</p>

Alle Anwender

<p class="just-emphasize">Siehe auch</p>

[CONTINUE](./continue_statement.md), [SET ERROR](./set_error_statement.md)

<p class="just-emphasize">Beschreibung</p>

Fehlermeldungen, die während des Laufes einer Kommandodatei entstehen können in eine Datei umgelenkt werden, um sie Anschließend zu kontrollieren. Dies ist umso wichtiger, wenn man die Fehlermeldung auf dem Bildschirm ausgeschaltet hat. Geschlossen wird die Fehlerdatei wieder, wenn SET OUTERR ohne Dateinamen angegeben wird.

<p class="just-emphasize">Beispiel</p>

SET ERROR NODISPLAY;

SET OUTERR c:\\ERR.TXT;

Select \* from DIESERELATIONEXISTIERTNICHT;

SET ERROR NODISPLAY;

SET OUTERR;
