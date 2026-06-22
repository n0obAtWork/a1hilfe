# MaskenTitel (EPA SVWARE)

<!-- source: https://amic.de/hilfe/_EPA_SVWARE.htm -->

| Bezeichnung | Standardwert | Erklärung |
| --- | --- | --- |
| Bildschirm für Addon aufbauen | Nein | |
| Artikel/Artikelstamm immer im Zusammenhang anlegen | Ja | |
| Bei Baustelle nur Baustellenartikel in IB anzeigen | Nein | Bei NEIN steigt man in die Itembox „Nach Nummern“ intern „IB_ARTIKEL_BAUSTELLE“ ein.  
Bei JA steigt man in die Itembox „nur Objektartikel“ intern „IB_ARTIKEL_BAUSTELLE_LANGSAM“ ein.  
Die Itembox „IB_ARTIKEL_BAUSTELLE_LANGSAM“ zeigt nur die Artikel an, welche in dem Objektstamm eingetragen wurden. |
| Beim Drücken von RETURN im Feld Menge wird in die nächste Spalte gesprungen | Nein | |
| Folgezeilen sofort rekalkulieren | Ja | |
| Gebindemaske ohne Abfrage weiterschalten | Ja | |
| Geschäftsart abfragen | Nein | |
| Label Geschäftsart | Gesch.Art | |
| Länge des Anzeigefeldes Geschäftsart | 10 | |
| Stückliste: F3-Auswahl ab 1 Stückliste | Nein | |
| Stückliste: F3-Auswahl ESC = keine | Nein | |
| Die Artikelnummer wird im NEU Fall IMMER mit dem letzten Artikel vorbelegt | Nein | |
| Lagerplatz und Lagerplatzort durch die Bezeichnung auswählbar | Nein | |
| Nachkommastellen der Warenmenge(höchstens 4) | 3 | |
| Merkmalsleisten Neuartikelprozedur | | Wird ein Artikel über die [Merkmalsleiste](../../artikelstamm_und_artikel/merkmalsleiste/artikeleingabe_in_der_vorgangserfassung.md) neu angelegt, so wird bei gesetztem EPA (Prozedurname) diese Prozedur vor Aufruf der eigentlichen Artikelanlage gestartet. Signatur der Prozedur ist:  
( ':ArtikelNummer$', :LagerNummer$ ) |
| Gebinde ohne Folgeabfragen | Ja | |
| Verschiebung der Warenerfassung | 12 | |
| Preisänderungen verbieten, F3 Auswahl ist aber erlaubt. | Nein | |
| Auto.F3 im Preisfeld bei Korrektur | Ja | |
| sofortige Preisfindung durchführen | Ja | |
| Bei F9 Abschluss sofort in Belegabschluss | Nein | |
| Zusatz 1 mit F3-Auswahl | Nein | |
| Feldname für Zusatz1 in F3-Auswahl | | |
| Bezeichnung Zusatztext 1 | Info 1 | |
| Zusatztext 1 Länge | 40 | |
| Vorbelegung Zusatz1 | | |
| Zusatz 2 mit F3-Auswahl | Nein | |
| Feldname für Zusatz2 in F3-Auswahl | | |
| Bezeichnung Zusatztext 2 | Info 2 | |
| Zusatztext 2 Länge | 40 | |
| Vorbelegung Zusatz2 | | |
| Zusatztext 1 abfragen | Nein | |
| Zusatztext 2 abfragen | Nein | |
| Vorgangsklassen, für die Zwangspartien angelegt werden (mit Komma getr.) | | |
| Unterklassen, für die Zwangspartien angelegt werden (mit Komma getr.) | | |
| Merkmalsleisten Läger (getrennt mit ,) für Neuanlage | Default: LEER, Beispiel : 0,3,4,6 | Liste der Lagernummern, die mit dem Merkmalsleistenartikel bei Neuanlage mit dem Artikel aufgefüllt werden. |
