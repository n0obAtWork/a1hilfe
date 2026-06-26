# SHOW VIEW Statement

<!-- source: https://amic.de/hilfe/showviewstatement.htm -->

#### Syntax

SHOW VIEW [[Creator.]Viewname];  
SHOW VIEWS ON Tablename;

#### Purpose

Anzeige aller Views unter admin, eines speziellen Views oder aller Views auf eine bestimmte Tabelle.

#### Anwendung

Befehlszeile

#### Siehe auch

[SHOW BUFFER](./show_buffer_statement.md), [SHOW CURSOR](./show_cursor.md), [SHOW TABLE](./show_table_statement.md), [SHOW TRIGGER](./show_trigger_statement.md), [SHOW PROC](./show_procedure_statement.md)

#### Beschreibung

SHOW VIEW zeigt alle Views in der Datenbank an. Will man nur die Views sehen, die unter einem bestimmten Bediener angelegt wurden, so muss man den Creator gefolgt von .\* mit angeben. Z.B.:

```text
SHOW VIEW ADMIN.*
```

Wird ein spezielles View angegeben, so wird die Definition in eine Datei ausgegeben ( "SHOWVIEW.TMP"), die gleich zur Bearbeitung geöffnet wird. Hierbei ist es möglich, den Creator mit anzugeben, um auch die Crystalviews anzeigen zu können, die ja bekannterweise nicht unter Admin angelegt werden. Will man alle VIEWS -also auch die Systemviews oder Crystalviews - verwendet man das Schlüsselwort ALL (SHOW VIEW ALL). Um herauszubekommen, welche Views es zu einer bestimmten Tabelle gibt, so verwendet man das Statement SHOW VIEW ON .... Es werden dann die Views mit dem Creator angezeigt.

#### Beispiel

```text
SHOW VIEW PS.AMIC_CRW_VERKAUSAUSWERTUNG_VR
SHOW VIEWS ON Fibuvorgposition
```
