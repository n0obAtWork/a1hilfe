# Einstellung eines Statements in die Replikation

<!-- source: https://amic.de/hilfe/einstellungeinesstatementsindi.htm -->

Grundsätzlich muss man hier unterscheiden, ob es sich um strukturverändernde Befehle ( alter | create | drop) oder um datenverändernde Statements handelt. Strukturverändernde Befehle müssen immer auf allen Datenbanken der Replikation ausgeführt werden. Dafür existieren zwei Möglichkeiten dies der Replikation mitzuteilen:

1) Wenn der Steuerungsparameter 851 „Passthrough aktivieren“ auf **Ja** steht wird mit der Datenbankfunktionalität [Passthrough](http://infocenter.sybase.com/help/index.jsp?topic=/com.sybase.help.sqlanywhere.12.0.1/dbreference/passthrough-statement.html) der Befehl automatisch von A.eins weitergereicht.  
    

2) Wenn der Steuerungsparameter auf **Nein** steht, werden strukturverändernde Befehle nicht mehr direkt ausgeführt und zwar auch nicht auf der initiierenden Datenbank. Soll ein Befehl trotzdem weitergeleitet werden, so kann man in A.eins (z.B. unter OSQL) dem Befehl ein Sternchen **\*** voranstellen. Dann wird dieser Befehl so verarbeitet, als ob der Steuerungsparameter auf **Ja** steht. Beispiel:

```sql
*create table admin.MusterTabelle (Musterspalte1 integer not null default autoincrement primary key)
```

Datenverändernden Statements werden dann automatisch weitergereicht, wenn die angesprochenen Tabellen in den Publikationen entsprechend eingerichtet sind. Dies geschieht unabhängig von dem Steuerungsparameter 851 „Passthrough aktivieren“. **Aber** es können auch Daten für Tabellen, die nicht in den Publikationen stehen an alle Datenbanken des Replikationssystems weitergeleitet werden! Dazu muss dem Statement ein Sternchen **\*** vorangestellt werden.

```sql
*insert into admin.MusterTabelle (Musterspalte1) values (4711)
```
