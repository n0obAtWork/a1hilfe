# Kontraktartikel (AIS)

<!-- source: https://amic.de/hilfe/kontraktartikelais.htm -->

In der Kontraktartikelerfassungsmaske kann nicht sichergestellt werden, dass beim Löschen eines Kontraktartikels auch die dazugehörigen AIS-Daten mitgelöscht werden.

Aus diesem Grund muss die private Tabelle mit einem „Fremdschlüssel“ versehen werden, der beim Löschen eines Kontraktartikels den dazugehörigen AIS-Datensatz löscht.

Beispiel:

```sql
create table
admin.kontraktartikelAddon
( ktrid integer
  ,ktrartiPosit integer
  ,primary key (ktrid, ktrartiPosit)
  ,foreign Key (ktrid, ktrartiposit)
       References
kontraktartikel ( ktrid, ktrartiposit )
       ON DELETE CASCADE
CHECK ON COMMIT
)
```

Hier eine kleine Erläuterung zum Anlegen des Fremdschlüssels.

| Statement | Beschreibung |
| --- | --- |
| foreign Key (ktrid, ktrartiposit) | Hiermit wird angegeben aus welchen Spalten der Fremdschlüssel besteht.  
 |
| References kontraktartikel ( ktrid, ktrartiposit ) | Dieser Teil legt fest auf welche Tabelle der Fremdschlüssel zeigt und um welche Spalten es sich hierbei handelt. Die Reihenfolge der Spalten muss die gleich wie im „foreign key“ Teil sein.  
 |
| ON DELETE CASCADE CHECK ON COMMIT | “ON DELETE CASCADE” bedeutet, dass beim Löschen die Abhängigen Daten dieser Tabelle auch gelöscht werden.  
Der “CHECK ON COMMIT” Teil sagt aus, dass die Überprüfung erst beim COMMIT erfolgen soll.  
 |
