# SET OUTPUT Statement

<!-- source: https://amic.de/hilfe/setoutputstatement.htm -->

<p class="just-emphasize">Syntax</p>

SET OUTPUT [TRIMED] [Filename]

<p class="just-emphasize">Purpose</p>

Öffnet / schließt eine Ausgabedatei im Modus „Überschreiben“

<p class="just-emphasize">Anwendung</p>

Kommandodatei, Befehlszeile

<p class="just-emphasize">Berechtigung</p>

Alle Anwender

<p class="just-emphasize">Siehe auch</p>

[SET APPEND](./set_append_statement.md), [SET OUTFILE](./set_outfile_statement.md), [SET TITLE](./set_title_statement.md)

<p class="just-emphasize">Beschreibung</p>

Ist ein Dateiname angegeben wird diese Datei geöffnet und die Daten bzw. die Ausgaben werden zusätzlich in diese Datei geschrieben. Die Datei wird überschrieben! Der unterschied zu SET APPEND und SET OUTFILE ist die Genauigkeit und Menge der Ausgabe. SET OUTPUT gibt in die Datei nicht nur die Daten aus, sonder zusätzlich die Überschriften und die abgegebenen Statements. Es wird auch nur das ausgegeben, was angezeigt wurde. Daher ist es nicht zu verwenden. Wird kein Dateiname angegeben, wird die offene Ausgabedatei geschlossen. Ist keine Datei offen wird dieser Befehl ignoriert.  
    
Der optionale Parameter TRIMED sorgt dafür, dass in der Ausgabedatei Leerzeichen am Ende einer Zeile wegoptimiert werden.

<p class="just-emphasize">Beispiel</p>

SET OUTPUT c:\\ZINS.SQL;

Select \* from fibuvorgstamm;

SET OUTPUT;
