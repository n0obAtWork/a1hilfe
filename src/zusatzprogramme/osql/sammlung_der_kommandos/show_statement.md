# SHOW Statement

<!-- source: https://amic.de/hilfe/showstatement.htm -->

<p class="just-emphasize">Syntax</p>

SHOW Feldname Text;

<p class="just-emphasize">Purpose</p>

Anzeige eines Textes in einem Maskenfeld;

<p class="just-emphasize">Anwendung</p>

Kommandodatei, Befehlszeile

<p class="just-emphasize">Berechtigung</p>

Alle Anwender

<p class="just-emphasize">Siehe auch</p>

[SHOW BUFFER](./show_buffer_statement.md), [SHOW CURSOR](./show_cursor.md), [SHOW TABLE](./show_table_statement.md), [SHOW VIEW](./show_view_statement.md), [SHOW TRIGGER](./show_trigger_statement.md), [SHOW PROC](./show_procedure_statement.md)

<p class="just-emphasize">Beschreibung</p>

Wenn es sich weder um SHOW BUFFER, SHOW CURSOR oder SHOW TABLE handelt, wird versucht das zweite Argument als Feldnamen zu interpretieren. Der Text der daraus folg wird in dieses Feld geschrieben. Dadurch kann man z.B. Fortschrittsanzeigen innerhalb eines Skriptes bewerkstelligen.

<p class="just-emphasize">Beispiel</p>

```text
// Statusline ist die Zeile unterhalb der
Eingabezeile
SHOW STATUSLINE Ende des Skripts;
```
