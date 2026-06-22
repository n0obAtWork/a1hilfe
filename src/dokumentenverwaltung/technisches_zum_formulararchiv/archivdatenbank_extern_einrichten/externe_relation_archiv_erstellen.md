# Externe Relation Archiv erstellen

<!-- source: https://amic.de/hilfe/_externerelationarchi.htm -->

Etwaige Massendaten der Relation Archiv machen eine Auslagerung dieser Daten in eine extra dafür vorgesehene Datenbank von Nöten.

*Bevor Sie weitermachen kommen Sie bitte Ihrer Sorgfaltspflicht nach und überzeugen sich, dass sie eine lauffähige Sicherung der beteiligten Datenbanken haben, um im Bedarfsfalle möglicherweise auftretende Problemfälle notfalls dadurch rückgängig machen zu können, dass Sie die Sicherung einspielen können.*

Schritt 1:

Dazu nehme man eine Kopie der aktuellen Datenbank. Letztere sollte man mit dem „Nullsetzer“ bearbeiten (Nicht Archiv!), anschließende Reorganisation wird auch hier empfohlen.

Schritt 2:

Richten Sie auf dem Datenbankserver eine System-ODBC Verbindung zur Archiv-Datenbank ein.

Schritt 3:

Damit befindet sich die Relation Archiv schon in der Zieldatenbank und kann somit abgebaut werden.

```sql
Drop table
archiv
```

```sql
Create
existing table admin.archiv at ‘archiv;;admin;Archiv’
```

Schritt4:

Führen Sie im Bedienerstamm die Funktion ***Fremdserver Rechte zuordnen*** aus.
