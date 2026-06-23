# SHOWERR Statement

<!-- source: https://amic.de/hilfe/showerrstatement.htm -->

<p class="just-emphasize">Syntax</p>

SHOWERR Feldname;

<p class="just-emphasize">Purpose</p>

Anzeige des letzten Datenbankfehlers in einem Maskenfeld;

<p class="just-emphasize">Anwendung</p>

Kommandodatei, Befehlszeile

<p class="just-emphasize">Berechtigung</p>

Alle Anwender

<p class="just-emphasize">Siehe auch</p>

[SHOW BUFFER](./show_buffer_statement.md), [SHOW CURSOR](./show_cursor.md), [SHOW TABLE](./show_table_statement.md), [SHOW VIEW](./show_view_statement.md), [SHOW TRIGGER](./show_trigger_statement.md), [SHOW PROC](./show_procedure_statement.md)

<p class="just-emphasize">Beschreibung</p>

Werden die Dialoge die den Datenbankfehler anzeigen unterdrückt, kann man mit diesem Befehl eventuelle Fehlermeldungen ausgeben.

<p class="just-emphasize">Beispiel</p>

```sql
// Statusline ist die Zeile unterhalb der
Eingabezeile
SET ERROR NODISPLAY;
SET ERROR CONTINUE;
Select * From fibuvorgstamm where FIID=10002;
IF(DBERR!=0)
{
  SHOWERR STATUSLINE;
}
EXIT;
```
