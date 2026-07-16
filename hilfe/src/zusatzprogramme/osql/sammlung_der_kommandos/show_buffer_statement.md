# SHOW BUFFER Statement

<!-- source: https://amic.de/hilfe/showbufferstatement.htm -->

#### Syntax

SHOW BUFFER [buffer-name];

#### Purpose

Anzeige der/des Buffers

#### Anwendung

Kommandodatei, Befehlszeile

#### Berechtigung

Alle Anwender

#### Siehe auch

[SHOW CURSOR](./show_cursor.md), [SHOW TABLE](./show_table_statement.md), [SHOW VIEW](./show_view_statement.md), [SHOW TRIGGER](./show_trigger_statement.md), [SHOW PROC](./show_procedure_statement.md)

#### Beschreibung

SHOW BUFFER ohne Name des Buffers zeigt alle aktive Datenbuffer an. Wird ein Name mit angegeben, werden die Daten, die von diesem Buffer gehalten werden ausgegeben.

#### Beispiel

SHOW BUFFER KINFO;
