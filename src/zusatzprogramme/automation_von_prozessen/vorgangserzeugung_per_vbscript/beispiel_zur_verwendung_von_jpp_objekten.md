# Beispiel zur Verwendung von JPP-Objekten

<!-- source: https://amic.de/hilfe/beispielzurverwendungvonjppobj.htm -->

Wir gehen hier davon aus das ein Objekt Namens „aeins“ im Script korrekt instanziert wurde.

Im Beispiel wird ein „JDBX“-Objekt verwendet

```sql
Option Explicit
dim aeins
set aeins = createobject("AMIC.Aeins")
aeins.connect(...
sub xyz
  dim sql
  dim hdl
  hdl = “xyz”
  sql = “SELECT irgendwas FROM
irgendwo”
  if aeins.jpp_new (hdl, "JDBX") then
    aeins.jpp_in hdl, "sql"  ,
sql
    aeins.jpp_do hdl , "exec"
    if aeins.jpp_do (hdl, "DBERR") = 0
then
      tu was
…
    End if
    aeins.jpp_delete hdl
  end if
end sub
```
