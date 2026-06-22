# SET TITLE Statement

<!-- source: https://amic.de/hilfe/settitlestatement.htm -->

Syntax

SET TITLE text;

Purpose

Schreibt Text in eine offene Ausgabedatei

Anwendung

Kommandodatei, Befehlszeile

Berechtigung

Alle Anwender

Siehe auch

[SET APPEND](./set_append_statement.md), [SET OUTFILE](./set_outfile_statement.md), [SET OUTPUT](./set_output_statement.md)

Beschreibung

Um in eine Ausgabedatei Zusatzstatement oder Beschreibungen zu schreiben, kann SET TITLE verwendet werden;

Beispiel

SET OUTFILE c:\\FIBUKL.SQL;

SET COMMAND_DELIMITER #;

SET TITEL delete from FiBuVorgKlasse;#

SET TITLE LOAD;#

SET TITLE insert into FiBuVorgKlasse (…) values(%s)#

SET DELIMITER ,#

Select \* from fibuvorgstamm#

SET TITLE LOAD;#

SET COMMAND_DELIMITER ;#

SET DELIMITER;

SET OUTFILE;
