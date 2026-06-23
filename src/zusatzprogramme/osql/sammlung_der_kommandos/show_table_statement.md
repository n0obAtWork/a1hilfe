# SHOW TABLE Statement

<!-- source: https://amic.de/hilfe/showtablestatement.htm -->

<p class="just-emphasize">Syntax</p>

SHOW TABEL [ALL] [[creator].table-name];

<p class="just-emphasize">Purpose</p>

Anzeige der Felder einer Relation

<p class="just-emphasize">Anwendung</p>

Kommandodatei, Befehlszeile

<p class="just-emphasize">Berechtigung</p>

Alle Anwender

<p class="just-emphasize">Siehe auch</p>

[SHOW BUFFER](./show_buffer_statement.md), [SHOW CURSOR](./show_cursor.md), [SHOW VIEW](./show_view_statement.md), [SHOW TRIGGER](./show_trigger_statement.md), [SHOW PROC](./show_procedure_statement.md)

<p class="just-emphasize">Beschreibung</p>

SHOW TABLE oder kurz SHOW TAB zeigt die Felder der angegeben Tabelle an, sowie die Felddefinitionen. Existiert die Relation nicht bzw. ist sie unter einem anderen Benutzer angelegt, wird nichts ausgegeben. Es ist möglich den Creator mit Punkt vom Tabellennamen getrennt mit anzugeben Gibt man keinen Tabellennamen an, so werden alle Relationen die unter Admin angelegt wurden angezeigt. Will man alle Relationen - also auch die Systemtabellen - sehen verwendet man das Schlüsselwort ALL (SHOW TAB ALL).  
 Felder einer Relation kann man sich auch über CTRL F1 ansehen!

<p class="just-emphasize">Beispiel</p>

```text
SHOW TABLE FiBuVorgPosition
```
