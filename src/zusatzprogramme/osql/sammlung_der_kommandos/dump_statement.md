# DUMP Statement

<!-- source: https://amic.de/hilfe/dumpstatement.htm -->

<p class="just-emphasize">Syntax</p>

DUMP INTO Dateiname [APPEND];

<p class="just-emphasize">Purpose</p>

Daten in Datei schreiben.

<p class="just-emphasize">Anwendung</p>

Befehlszeile

<p class="just-emphasize">Berechtigung</p>

Alle Anwender

<p class="just-emphasize">Siehe auch</p>

[SET OUTPUT](./set_output_statement.md), [SET OUTFILE](./set_outfile_statement.md)

<p class="just-emphasize">Beschreibung</p>

Dump into ... dient dazu, die aktuell angezeigten Daten in eine Datei zu übernehmen. Der Vorteil ist hier, dass das Statement, welches eventuell sehr lange gelaufen ist, nicht ein zweites mal ausgeführt werden muss, da alle bereits gelesenen – und auch nur die bisher gelesenen - Daten in die Datei geschrieben werden.

Dump into überschreibt standardmäßig die Datei. Das Schlüsselwort APPEND sorgt dafür, dass die Daten an die bestehende Datei angehängt werden

<p class="just-emphasize">Beispiel</p>

DUMP INTO c:\\AEINS\\BIN\\output.TXT APPEND
