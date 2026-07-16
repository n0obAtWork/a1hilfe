# SHOW Statement

<!-- source: https://amic.de/hilfe/showstatement.htm -->

#### Syntax

SHOW Feldname Text;

#### Purpose

Anzeige eines Textes in einem Maskenfeld;

#### Anwendung

Kommandodatei, Befehlszeile

#### Berechtigung

Alle Anwender

#### Siehe auch

[SHOW BUFFER](./show_buffer_statement.md), [SHOW CURSOR](./show_cursor.md), [SHOW TABLE](./show_table_statement.md), [SHOW VIEW](./show_view_statement.md), [SHOW TRIGGER](./show_trigger_statement.md), [SHOW PROC](./show_procedure_statement.md)

#### Beschreibung

Wenn es sich weder um SHOW BUFFER, SHOW CURSOR oder SHOW TABLE handelt, wird versucht das zweite Argument als Feldnamen zu interpretieren. Der Text der daraus folg wird in dieses Feld geschrieben. Dadurch kann man z.B. Fortschrittsanzeigen innerhalb eines Skriptes bewerkstelligen.

#### Beispiel

```text
// Statusline ist die Zeile unterhalb der Eingabezeile
SHOW STATUSLINE Ende des Skripts;
```
