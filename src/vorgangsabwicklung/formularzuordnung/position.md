# Position

<!-- source: https://amic.de/hilfe/position.htm -->

| Feld | Beschreibung |
| --- | --- |
| Datenladeroutine | |
| Bestellte Menge abfragen | |
| Gebindefaktor 1 abfragen | |
| Gebindefaktor 2 | |
| Gebindemenge | |
| Preis ME | |
| Preis abfragen | |
| Preis pro | |
| Zusatz 1 abfragen | |
| Zusatz 2 abfragen | |
| Addonfeldname für Auf | |
| Doppelerfassungsartik | |
| IB-Artikel in Menge | |
| Artikel in MSA J/N/F | |
| Druckauswahlfenster | |
| Druckvorbelegung | |
| Artikelnummer verstecken | |
| Bestellte Menge anz | |
| Letzter VK anz. | |
| | |
| Offene Belegeprozedur | |
| ME anzeigen | |
| MENR Verhalten | |
| IB Zusatz2 | |
| Ohne Preise | |
| Leergut | |
| Wertartikel | |
| Ltz. VK Prozedur | |
| Partieitembox | |
| Entryprozedur Preisbezug | Hier wird der Name einer Prozedur angegeben, die anhand der gegebenen Werte eine Vorbelegung für den Preisbezug errechnet (sofern relevant)<br>Beispiel:<br><pre><code>CREATE PROCEDURE p_mas_testteildispo(&#10; in in_wabewid integer,&#10; in in_DestKlassNummer integer,&#10; in in_DestUKlassNummer integer,&#10; in in_MengeGesamt numeric(15,4),&#10; in in_MengeDisp numeric(15,4),&#10; in in_MengeCurr numeric(15,4),&#10; in in_MengeRest numeric(15,4),&#10; in in_PreisBezugGesamt numeric(15,4),&#10; in in_PreisBezugDisp numeric(15,4),&#10; in in_PreisBezugCurr numeric(15,4),&#10; in in_PreisBezugRest numeric(15,4) )&#10;result( Result numeric(15,4) )&#10;begin&#10; select in_PreisBezugCurr from dummy&#10;end</code></pre><br> <br>Als Vorbelegung bekommt die Prozedur über eine Dreisatzberechnung den Wert „in_PreisBezugCurr„ gegeben.<br> |
| Ltz. VK Bezeichnung | |
| Folgeartikel Prozedur | |
| Wertberechnungsprozedur | |
| Knopfbelegung D1/D1 | |
