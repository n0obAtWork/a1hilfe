# SHOW CURSOR

<!-- source: https://amic.de/hilfe/showcursor.htm -->

#### Syntax

SHOW CURSOR [cursor-name];

#### Purpose

Anzeige der/des Cursor

#### Anwendung

Kommandodatei, Befehlszeile

#### Berechtigung

Alle Anwender

#### Siehe auch

[SHOW BUFFER](./show_buffer_statement.md), [SHOW TABLE](./show_table_statement.md), [SHOW VIEW](./show_view_statement.md), [SHOW TRIGGER](./show_trigger_statement.md), [SHOW PROC](./show_procedure_statement.md)

#### Beschreibung

SHOW CURSOR ohne Name zeigt alle aktive Datenbankcursor an. Wird ein Name mit angegeben, werden die Daten, die im letzten gelesenen Satz enthalten sind angezeigt.

#### Beispiel

SHOW CURSOR CSQL; 

//Csql ist der Cursor der von OSQL verwendet wird
