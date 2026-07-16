# Maskenfelder im SQL verwenden

<!-- source: https://amic.de/hilfe/maskenfelderimsqlverwenden.htm -->

Man kann in den SQL-Statements, die man bei Datenherkunft **SQL** auch auf Maskenfelder zugreifen. Man muss dann nur einen Doppelpunkt vor den Name des Maskenfeldes schreibe und unbedingt Groß- und Kleinschreibung beachten. Beispiel:

```sql
select AdressAnrede||' '||AdressVorName ||' '|| Adressname
  from Anschriftstamm a join Kundenstamm k on k.Adressidhauptadr=a.adressid
  where Kontonummer=:ais1.KontoNummer$
```

<strong>Achtung: </strong><em>So wie das Statement hier formuliert ist kommt es zu einem Syntaxfehler, sollte das Feld ais1.KontoNummer$ keine Daten enthalten. Daher sollte man bei der Verwendung von Maskenfeldern immer einfache Hochkomma verwenden:</em>

```sql
select AdressAnrede||' '||AdressVorName ||' '|| Adressname
  from Anschriftstamm a join Kundenstamm k on k.Adressidhauptadr=a.adressid
  where Kontonummer=':ais1.KontoNummer$'
```

Die Typkonvertierung wird dann automatisch von der Datenbank vorgenommen.
