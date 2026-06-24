# Private Tabellen

<!-- source: https://amic.de/hilfe/privatetabellen.htm -->

Man kann in AIS eigene private Tabellen definieren und ist nicht mehr auf die vorgegebenen eingeschränkt. Will man für diese Tabellen mit Hilfe der Maske Aezaddon (und Varianten von dieser Maske) Daten erfassen, so muss diese Tabelle mindestens das Feld **ident** haben und in der Tabelle **ident** muss ein Eintrag für diese Tabelle existieren. Will man also z.B. eine Tabelle für Geschäftsvorfallkodes erstellen, in der ein Feld für den Kode (integer) und ein Feld für den Beschreibungstext (char(255)) enthalten sein soll, so müsste das Statement für die Tabelle wie folgt aussehen:

```sql
create table
admin.p_Geschaeftsvorfaelle
( Ident integer,
  Kode integer,
  Beschreibgung
char(255),
  primary key (ident)
)
```

Zusätzlich muss dann noch ein Eintrag in der Tabelle **ident** erzeugt werden:

```sql
insert into ident
 ( IdentTableName, IdentColumnName, IdentIdent,
IdentAktivKont, IdentAngefKont)
  Values
 ( ' p_Geschaeftsvorfaelle ', 'Ident', 0, 1,
0)
```

<strong>Hinweis:</strong><em> Hat man vergessen einen Eintrag in der Tabelle Ident vorzunehmen, kommt es zu dem Effekt, dass man nur einen Datensatz (mit der Ident 0) erfassen kann.</em>

<p class="siehe-auch">Siehe auch:</p>

- [Kontraktartikel (AIS)](./kontraktartikel_ais.md)
