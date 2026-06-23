# SET OUTFILE Statement

<!-- source: https://amic.de/hilfe/setoutfilestatement.htm -->

<p class="just-emphasize">Syntax</p>

SET OUTFILE [TRIMED] [Filename]

<p class="just-emphasize">Purpose</p>

Öffnet / schließt eine Ausgabedatei im Modus „Überschreiben“

<p class="just-emphasize">Anwendung</p>

Kommandodatei, Befehlszeile

<p class="just-emphasize">Berechtigung</p>

Alle Anwender

<p class="just-emphasize">Siehe auch</p>

[SET APPEND](./set_append_statement.md), [SET OUTPUT](./set_output_statement.md), [SET TITLE](./set_title_statement.md)

<p class="just-emphasize">Beschreibung</p>

Ist ein Dateiname angegeben wird diese Datei geöffnet und die Daten bzw. die Ausgaben in diese Datei umgelenkt. Die Datei wird überschrieben! Wird kein Dateiname angegeben, wird die offene Ausgabedatei geschlossen. Ist keine Datei offen wird dieser Befehl ignoriert.  
 Der optionale Parameter TRIMED sorgt dafür, dass in der Ausgabedatei Leerzeichen am Ende einer Zeile wegoptimiert werden.

<p class="just-emphasize">Beispiel</p>

SET OUTFILE c:\\ZINS.SQL;

Select \* from fibuvorgstamm;

SET OUTFILE;
