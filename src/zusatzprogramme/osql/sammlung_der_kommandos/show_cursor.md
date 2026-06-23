# SHOW CURSOR

<!-- source: https://amic.de/hilfe/showcursor.htm -->

<p class="just-emphasize">Syntax</p>

SHOW CURSOR [cursor-name];

<p class="just-emphasize">Purpose</p>

Anzeige der/des Cursor

<p class="just-emphasize">Anwendung</p>

Kommandodatei, Befehlszeile

<p class="just-emphasize">Berechtigung</p>

Alle Anwender

<p class="just-emphasize">Siehe auch</p>

[SHOW BUFFER](./show_buffer_statement.md), [SHOW TABLE](./show_table_statement.md), [SHOW VIEW](./show_view_statement.md), [SHOW TRIGGER](./show_trigger_statement.md), [SHOW PROC](./show_procedure_statement.md)

<p class="just-emphasize">Beschreibung</p>

SHOW CURSOR ohne Name zeigt alle aktive Datenbankcursor an. Wird ein Name mit angegeben, werden die Daten, die im letzten gelesenen Satz enthalten sind angezeigt.

<p class="just-emphasize">Beispiel</p>

SHOW CURSOR CSQL; 

//Csql ist der Cursor der von OSQL verwendet wird
