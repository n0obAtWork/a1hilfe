# SET APPEND Statement

<!-- source: https://amic.de/hilfe/setappendstatement.htm -->

Syntax

SET APPEND [TRIMED] [Filename]

Purpose

Öffnet / schließt eine Ausgabedatei im Modus „Anhängen“

Anwendung

Kommandodatei, Befehlszeile

Berechtigung

Alle Anwender

Siehe auch

[SET OUTFILE](./set_outfile_statement.md), [SET OUTPUT](./set_output_statement.md), [SET TITLE](./set_title_statement.md)

Beschreibung

Ist ein Dateiname angegeben wird diese Datei geöffnet und die Daten bzw. die Ausgaben in diese Datei umgelenkt. Die Datei wird nicht überschrieben sondern die Daten werden an die bestehenden angehängt! Wird kein Dateiname angegeben, wird die offenen Ausgabedatei geschlossen. Ist keine Datei offen wird dieser Befehl ignoriert.  
Der optionale Parameter TRIMED sorgt dafür, dass in der Ausgabedatei Leerzeichen am Ende einer Zeile wegoptimiert werden.

Beispiel

SET APPEND c:\\ZINS.SQL;

Select \* from fibuvorgstamm;

SET APPEND;
