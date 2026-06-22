# Technische Änderungen

<!-- source: https://amic.de/hilfe/_technischenderungen.htm -->

Die Tabelle PartieArtiMenIst wird für Partien nicht mehr befüllt – sie ist allerdings weiterhin für das Streckenmodul aktiv! Private Auswahllistenwahllisten oder MAKROS müssen daraufhin überprüft werden.

In der Tabelle V_Posiware sind neue Felder für die interne Verwaltung bei der Teildisposition hinzugekommen. Das Feld V_PosiParWert enthält nicht immer korrekte Werte.

Die bisherige Tabelle PartieBestand gibt es nicht mehr, sie ist durch eine ‚baugleiche’ VIEW gleichen Namens ersetzt worden. Ehemalige Trigger auf Partiebestand wurden entfernt.

Die Partiebestände werden jetzt in der Relation PARTIEBESTANDPUR geführt. ACHTUNG: Das Feld REMENGE in dieser Relation wird nicht immer versorgt, es wird demnächst entfernt!

Für die Ermittlung des Partiebestandes ist folgen Datenbankfunktion geschaffen worden:

```text
// Ermittele den Partiebestand
aus
// Relation PartiebestandPur
// Korrekturmengen fließen mit ein!
// bei Lagerplatz_in = -1 wird nicht
lagerplatzspezifisch ermittelt
// bei artikelid_ist_stammid = 1 werden Summen über
alle Artikel des gleichen Stamms gemacht
// bei mit_dispo = 1 werden Bestellungen und Aufträge
mitgezählt
//---------------------------------------------------------------------
create function
AMIC_FUNC_PARTIEBESTAND
 (
 in partieid_in
integer,
 in artikelid_in
integer,
 in lagerplatz_in
integer default -1,
 in
artikelid_ist_stammid integer default
0,
 in mit_dispo
integer default 0)
returns decimal(20,8)
```
