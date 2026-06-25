# Uhrzeit auf dem Bon ausdrucken

<!-- source: https://amic.de/hilfe/uhrzeitaufdembonausdrucken.htm -->

Durch folgendes SQLK besteht die Möglichkeit, sich die Uhrzeit auf dem Bon ausdrucken zu lassen:

```sql
select SUBSTR(BelegDatum, 10) Uhrzeit
from AcashBelg
where BelegId=:V_Id
```

Dieses ist ein privates SQLK mit Namen XYZ. Im entsprechenden Barverkaufsformular ist dann an entsprechender Position die Druckposition 7 SQL Zugriff auf Daten einzutragen mit Festtext XYZ, Uhrzeit. Dieses zieht zurzeit allerdings nur bei der Tresenkasse.
