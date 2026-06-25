# SHOW TRIGGER Statement

<!-- source: https://amic.de/hilfe/showtriggerstatement.htm -->

<p class="just-emphasize">Syntax</p>

```text
SHOW TRIGGER | ON RELATION |
| TRIGGERNAME |
```

<p class="just-emphasize">Purpose</p>

Anzeige eines Triggers, aller Trigger oder aller Trigger zu einer Relation

<p class="just-emphasize">Anwendung</p>

Befehlszeile

<p class="just-emphasize">Berechtigung</p>

Alle Anwender

<p class="just-emphasize">Siehe auch</p>

[SHOW BUFFER](./show_buffer_statement.md), [SHOW CURSOR](./show_cursor.md), [SHOW TABLE](./show_table_statement.md), [SHOW VIEW](./show_view_statement.md), [SHOW PROC](./show_procedure_statement.md)

<p class="just-emphasize">Beschreibung</p>

SHOW TRIGGER hat drei Ausprägungen. Die erste wäre SHOW TRIGGER ohne irgendwelche sonstigen Parameter. Dadurch werden alle Trigger mit dem zugehörigen Creator angezeigt.

Gibt man den Namen des Triggers an, wird die Definition dieses Triggers in eine Datei ( "SHOWTRIG.TMP" ) geschrieben.

Verwendet man das Schlüsselwort ON mit einem Relationsname, werden nur die Trigger zu dieser Relation angezeigt.

<p class="just-emphasize">Beispiel</p>

```text
SHOW TRIGGER ON FiBuVorgPosition
// ERGEBNIS
Name
Relation
Event
FiBuVorgPosition_aftdel
fibuvorgposition       DELETE
fibuvorgposition_aftins
fibuvorgposition       INSERT
FiBuVorgPosition_aftupd_akz
fibuvorgposition       UPDATE
FiBuVorgPosition_aftupd_Konto
fibuvorgposition       UPDATE
FiBuVorgPosition_aftupd_opk
fibuvorgposition       UPDATE
FiBuVorgPosition_aftupd_VAL
fibuvorgposition       UPDATE
```
