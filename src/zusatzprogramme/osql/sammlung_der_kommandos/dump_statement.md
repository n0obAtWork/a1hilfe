# DUMP Statement

<!-- source: https://amic.de/hilfe/dumpstatement.htm -->

Syntax

DUMP INTO Dateiname [APPEND];

Purpose

Daten in Datei schreiben.

Anwendung

Befehlszeile

Berechtigung

Alle Anwender

Siehe auch

[SET OUTPUT](./set_output_statement.md), [SET OUTFILE](./set_outfile_statement.md)

Beschreibung

Dump into ... dient dazu, die aktuell angezeigten Daten in eine Datei zu übernehmen. Der Vorteil ist hier, dass das Statement, welches eventuell sehr lange gelaufen ist, nicht ein zweites mal ausgeführt werden muss, da alle bereits gelesenen – und auch nur die bisher gelesenen - Daten in die Datei geschrieben werden.

Dump into überschreibt standardmäßig die Datei. Das Schlüsselwort APPEND sorgt dafür, dass die Daten an die bestehende Datei angehängt werden

Beispiel

DUMP INTO c:\\AEINS\\BIN\\output.TXT APPEND
