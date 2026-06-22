# IB_Box

<!-- source: https://amic.de/hilfe/_cescanneribbox.htm -->

| FIELD Beschreibung | Was ist es genau |
| --- | --- |
| Lagernummer | Bezeichnung einer Spalte auf der Maske |
| Lagernummer | Ausgabewert des Select Statements |
| I4 | Format des Feldes |
| 2 | Breite der Angezeigten Spalte auf der Maske |

```sql
// Priv. SQL Text
IB_SCANNER_ANZEIGE
TITLE Vorgang auf dem CE Scanner-3
FIELD Lagernummer,Lagernummer,I4,2
FIELD Artikelnummer,artikelid,I4,8
FIELD Artikelbezeich,artikelbezeich,char,20
SQL
     select TOP :TOP start at
:ZEILENNUMMER ar.Lagernummer,ar.artikelid,
       ar.artikelbezeich
from artikel ar
       join sekundschluessel sek on ( ar.artistammid =
sek.sekudatenid
       where sek.sekugruppe = 2 and sek.sekubegriff = ':SEKUNDS'
       order by ar.lagernummer asc
```
