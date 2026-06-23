# SET APPEND Statement

<!-- source: https://amic.de/hilfe/setappendstatement.htm -->

<p class="just-emphasize">Syntax</p>

SET APPEND [TRIMED] [Filename]

<p class="just-emphasize">Purpose</p>

Öffnet / schließt eine Ausgabedatei im Modus „Anhängen“

<p class="just-emphasize">Anwendung</p>

Kommandodatei, Befehlszeile

<p class="just-emphasize">Berechtigung</p>

Alle Anwender

<p class="just-emphasize">Siehe auch</p>

[SET OUTFILE](./set_outfile_statement.md), [SET OUTPUT](./set_output_statement.md), [SET TITLE](./set_title_statement.md)

<p class="just-emphasize">Beschreibung</p>

Ist ein Dateiname angegeben wird diese Datei geöffnet und die Daten bzw. die Ausgaben in diese Datei umgelenkt. Die Datei wird nicht überschrieben sondern die Daten werden an die bestehenden angehängt! Wird kein Dateiname angegeben, wird die offenen Ausgabedatei geschlossen. Ist keine Datei offen wird dieser Befehl ignoriert.  
Der optionale Parameter TRIMED sorgt dafür, dass in der Ausgabedatei Leerzeichen am Ende einer Zeile wegoptimiert werden.

<p class="just-emphasize">Beispiel</p>

SET APPEND c:\\ZINS.SQL;

Select \* from fibuvorgstamm;

SET APPEND;
