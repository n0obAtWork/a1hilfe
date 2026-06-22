# „Kontraktbewegung zum Marktpreis“

<!-- source: https://amic.de/hilfe/_ktrpari_kbm.htm -->

Hauptmenü > Kontraktverwaltung > Paritätsstammdaten

oder Direktsprung **[PARI]**

Die „Kontraktbewertung zum Marktpreis“ Informationen befinden sich in den Varianten „Paritäten detailliert (KBM)“ und „Währungskurse (KBM)“. Dort befindet sich neben den Importen der Marktpreise und der Paritätssätze, auch die Auswertungen „Marktpreise (KBM)“ und „Paritätssatzliste (KBM)“.

Über die Importe lassen sich die Marktpreise und die Paritätssätze einlesen. Die Importdateien müssen sich dafür im Ordner „Aeins\\User“ befinden.

Aufbau Importdatei „Marktpreise“

Name der Datei ist „Marktpreise.xlsx“

| Spalte | Feldname | Beschreibung |
| --- | --- | --- |
| 1 | Datum | |
| 2 | Kunde | |
| 3 | Artikel | |
| 4 | Preis | |
| 5 | VE | |
| 6 | Stichtag | |
| 7 | Level | |
| 8 | Uplift | |
| 9 | Kosten | |
| 10 | Profit | |
| 11 | Preistyp | Beim Import kann hier der Preistyp angegeben werden. Standardmäßig wird dieser Wert auf „0“ gesetzt. Bei dem Wert „1“ handelt es sich um einen nachhaltigen Preis. |

Aufbau Importdatei „Paritätssätze“

Name der Datei ist „ParitaetImport.xls“

| Spalte | Feldname | Beschreibung |
| --- | --- | --- |
| 1 | Stichtag | |
| 2 | Gültig ab | |
| 3 | Nummer | |
| 4 | Satz | |
