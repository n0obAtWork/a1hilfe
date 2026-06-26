# SHOW TABLE Statement

<!-- source: https://amic.de/hilfe/showtablestatement.htm -->

#### Syntax

SHOW TABEL [ALL] [[creator].table-name];

#### Purpose

Anzeige der Felder einer Relation

#### Anwendung

Kommandodatei, Befehlszeile

#### Berechtigung

Alle Anwender

#### Siehe auch

[SHOW BUFFER](./show_buffer_statement.md), [SHOW CURSOR](./show_cursor.md), [SHOW VIEW](./show_view_statement.md), [SHOW TRIGGER](./show_trigger_statement.md), [SHOW PROC](./show_procedure_statement.md)

#### Beschreibung

SHOW TABLE oder kurz SHOW TAB zeigt die Felder der angegeben Tabelle an, sowie die Felddefinitionen. Existiert die Relation nicht bzw. ist sie unter einem anderen Benutzer angelegt, wird nichts ausgegeben. Es ist möglich den Creator mit Punkt vom Tabellennamen getrennt mit anzugeben Gibt man keinen Tabellennamen an, so werden alle Relationen die unter Admin angelegt wurden angezeigt. Will man alle Relationen - also auch die Systemtabellen - sehen verwendet man das Schlüsselwort ALL (SHOW TAB ALL).  
 Felder einer Relation kann man sich auch über CTRL F1 ansehen!

#### Beispiel

```text
SHOW TABLE FiBuVorgPosition
```
