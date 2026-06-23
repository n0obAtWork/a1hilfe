# SHOW PROCEDURE Statement

<!-- source: https://amic.de/hilfe/showprocedurestatement.htm -->

<p class="just-emphasize">Syntax</p>

SHOW PROC [[Creator.]procedurename];

<p class="just-emphasize">Purpose</p>

Anzeige aller Prozeduren unter admin oder einer speziellen Prozedur.

<p class="just-emphasize">Anwendung</p>

Befehlszeile

<p class="just-emphasize">Siehe auch</p>

[SHOW BUFFER](./show_buffer_statement.md), [SHOW CURSOR](./show_cursor.md), [SHOW TABLE](./show_table_statement.md), [SHOW VIEW](./show_view_statement.md), [SHOW TRIGGER](./show_trigger_statement.md)

<p class="just-emphasize">Beschreibung</p>

SHOW PROCEDURE zeigt alle Prozeduren in der Datenbank an. Will man nur die Prozeduren sehen, die unter einem bestimmten Benutzer angelegt wurden, so muss man den Creator gefolgt von .\* angeben.

```text
SHOW PROC admin.*
```

Wird ein spezielle Prozedur angegeben, so wird die Definition in eine Datei ausgegeben ( "SHOWPROC.TMP"), die gleich zur Bearbeitung geöffnet wird. Hierbei ist es möglich, den Creator mit anzugeben, um auch die Systemprozeduren anzeigen zu können, die ja bekannterweise nicht unter Admin angelegt werden.  
    

<p class="just-emphasize">Beispiel</p>

```text
SHOW PROC dbo.sa_conn_info
```
