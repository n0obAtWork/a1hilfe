# SHOW PROCEDURE Statement

<!-- source: https://amic.de/hilfe/showprocedurestatement.htm -->

Syntax

SHOW PROC [[Creator.]procedurename];

Purpose

Anzeige aller Prozeduren unter admin oder einer speziellen Prozedur.

Anwendung

Befehlszeile

Siehe auch

[SHOW BUFFER](./show_buffer_statement.md), [SHOW CURSOR](./show_cursor.md), [SHOW TABLE](./show_table_statement.md), [SHOW VIEW](./show_view_statement.md), [SHOW TRIGGER](./show_trigger_statement.md)

Beschreibung

SHOW PROCEDURE zeigt alle Prozeduren in der Datenbank an. Will man nur die Prozeduren sehen, die unter einem bestimmten Benutzer angelegt wurden, so muss man den Creator gefolgt von .\* angeben.

```text
SHOW PROC admin.*
```

Wird ein spezielle Prozedur angegeben, so wird die Definition in eine Datei ausgegeben ( "SHOWPROC.TMP"), die gleich zur Bearbeitung geöffnet wird. Hierbei ist es möglich, den Creator mit anzugeben, um auch die Systemprozeduren anzeigen zu können, die ja bekannterweise nicht unter Admin angelegt werden.  
    

Beispiel

```text
SHOW PROC dbo.sa_conn_info
```
