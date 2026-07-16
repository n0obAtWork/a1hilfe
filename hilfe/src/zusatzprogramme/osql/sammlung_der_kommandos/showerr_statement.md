# SHOWERR Statement

<!-- source: https://amic.de/hilfe/showerrstatement.htm -->

#### Syntax

SHOWERR Feldname;

#### Purpose

Anzeige des letzten Datenbankfehlers in einem Maskenfeld;

#### Anwendung

Kommandodatei, Befehlszeile

#### Berechtigung

Alle Anwender

#### Siehe auch

[SHOW BUFFER](./show_buffer_statement.md), [SHOW CURSOR](./show_cursor.md), [SHOW TABLE](./show_table_statement.md), [SHOW VIEW](./show_view_statement.md), [SHOW TRIGGER](./show_trigger_statement.md), [SHOW PROC](./show_procedure_statement.md)

#### Beschreibung

Werden die Dialoge die den Datenbankfehler anzeigen unterdrückt, kann man mit diesem Befehl eventuelle Fehlermeldungen ausgeben.

#### Beispiel

```sql
// Statusline ist die Zeile unterhalb der Eingabezeile
SET ERROR NODISPLAY;
SET ERROR CONTINUE;
Select * From fibuvorgstamm where FIID=10002;
IF(DBERR!=0)
{
  SHOWERR STATUSLINE;
}
EXIT;
```
