# SET TITLE Statement

<!-- source: https://amic.de/hilfe/settitlestatement.htm -->

<p class="just-emphasize">Syntax</p>

SET TITLE text;

<p class="just-emphasize">Purpose</p>

Schreibt Text in eine offene Ausgabedatei

<p class="just-emphasize">Anwendung</p>

Kommandodatei, Befehlszeile

<p class="just-emphasize">Berechtigung</p>

Alle Anwender

<p class="just-emphasize">Siehe auch</p>

[SET APPEND](./set_append_statement.md), [SET OUTFILE](./set_outfile_statement.md), [SET OUTPUT](./set_output_statement.md)

<p class="just-emphasize">Beschreibung</p>

Um in eine Ausgabedatei Zusatzstatement oder Beschreibungen zu schreiben, kann SET TITLE verwendet werden;

<p class="just-emphasize">Beispiel</p>

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
