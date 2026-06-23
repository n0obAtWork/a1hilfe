# SHOW BUFFER Statement

<!-- source: https://amic.de/hilfe/showbufferstatement.htm -->

<p class="just-emphasize">Syntax</p>

SHOW BUFFER [buffer-name];

<p class="just-emphasize">Purpose</p>

Anzeige der/des Buffers

<p class="just-emphasize">Anwendung</p>

Kommandodatei, Befehlszeile

<p class="just-emphasize">Berechtigung</p>

Alle Anwender

<p class="just-emphasize">Siehe auch</p>

[SHOW CURSOR](./show_cursor.md), [SHOW TABLE](./show_table_statement.md), [SHOW VIEW](./show_view_statement.md), [SHOW TRIGGER](./show_trigger_statement.md), [SHOW PROC](./show_procedure_statement.md)

<p class="just-emphasize">Beschreibung</p>

SHOW BUFFER ohne Name des Buffers zeigt alle aktive Datenbuffer an. Wird ein Name mit angegeben, werden die Daten, die von diesem Buffer gehalten werden ausgegeben.

<p class="just-emphasize">Beispiel</p>

SHOW BUFFER KINFO;
