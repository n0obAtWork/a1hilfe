# SET OUTFILE Statement

<!-- source: https://amic.de/hilfe/setoutfilestatement.htm -->

Syntax

SET OUTFILE [TRIMED] [Filename]

Purpose

Öffnet / schließt eine Ausgabedatei im Modus „Überschreiben“

Anwendung

Kommandodatei, Befehlszeile

Berechtigung

Alle Anwender

Siehe auch

[SET APPEND](./set_append_statement.md), [SET OUTPUT](./set_output_statement.md), [SET TITLE](./set_title_statement.md)

Beschreibung

Ist ein Dateiname angegeben wird diese Datei geöffnet und die Daten bzw. die Ausgaben in diese Datei umgelenkt. Die Datei wird überschrieben! Wird kein Dateiname angegeben, wird die offene Ausgabedatei geschlossen. Ist keine Datei offen wird dieser Befehl ignoriert.  
 Der optionale Parameter TRIMED sorgt dafür, dass in der Ausgabedatei Leerzeichen am Ende einer Zeile wegoptimiert werden.

Beispiel

SET OUTFILE c:\\ZINS.SQL;

Select \* from fibuvorgstamm;

SET OUTFILE;
