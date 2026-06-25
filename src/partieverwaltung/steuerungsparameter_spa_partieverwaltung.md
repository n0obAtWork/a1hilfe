# Steuerungsparameter [SPA] Partieverwaltung

<!-- source: https://amic.de/hilfe/_steuerungsparameters.htm -->

Hauptmenü > Administration > Steuerung > Steuerparameter zeigen

oder Direktsprung **[SPA]**

Nachfolgend die Beschreibung für SPA-Einstellung 1 bis 62:

| SPA | Beschreibung |
| --- | --- |
| 1 | Globale Einstellung ob mit Partieverwaltung gearbeitet wird (Ja/Nein) |
| 3 | Anzahl Ziffern für eine Partienummer (1 bis 8) |
| 4 | Aktualisierung der Mengen und Werte bereits bei Vorgangserfassung (Ja/Nein) |
| 9 | Wahl der Voreinstellung für die Anlage der Partieartikel (Artikelstamm oder Artikel-Lager)<br><ul><li>&nbsp;&nbsp;&nbsp; Artikelstamm: Zuordnung einer Artikelnummer aus dem Artikelstamm</li><li>&nbsp;&nbsp;&nbsp; Artikel/Lager:&nbsp; Zuordnung eines Artikels aus einem Lager</li></ul> |
| 10 | Steuert die Artikelanzeige in der Partieverwaltung (Partien ändern + F2 Artikel)<br><ul><li>&nbsp;&nbsp;&nbsp; Ja:&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp; Anzeige für Lager und Saldo</li><li>&nbsp;&nbsp;&nbsp; Nein:&nbsp;&nbsp; Sollmengen Einkauf und Verkauf werden dargestellt</li></ul> |
| 18 | Steuert die autom. Partieauswahl nur für die Auftragserfassung<br><ul><li>&nbsp;&nbsp;&nbsp; Nein:&nbsp;&nbsp; Keine autom. Partieauswahl bei der Auftragserfassung</li><li>&nbsp;&nbsp;&nbsp; Verkauf: Autom. Partieauswahl nur bei der Auftragserfassung im Verkauf</li><li>&nbsp;&nbsp;&nbsp; Einkauf: Autom. Partieauswahl nur bei der Bestellerfassung im Einkauf</li><li>&nbsp;&nbsp;&nbsp; Ja:&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp; Autom. Partieauswahl bei der Auftragserfassung im Einkauf und Verkauf</li></ul> |
| 20 | Steuert die autom.Partieauswahl für die Vorgangserfassung (ab 2 Partien/generell/nie/auch ohne Partien)<br><ul><li>&nbsp;&nbsp;&nbsp; ab 2 Partien: Autom. Anzeige der Partieauswahl, wenn mind. 2 Partien für diesen Artikel vorhanden</li><li>&nbsp;&nbsp;&nbsp; generell: Autom. Anzeige der Partieauswahl, wenn mind. 1 Partie für diesen Artikel vorhanden</li><li>&nbsp;&nbsp;&nbsp; nie: Keine autom. Anzeige der Partieauswahl</li><li>&nbsp;&nbsp;&nbsp; ohne Partien: Autom. Anzeige der Partieauswahl, auch wenn keine Partie für diesen Artikel vorh.</li></ul> |
| 21 | Steuert die autom. Partieauswahl nur für Vorgangsänderungen<br><ul><li>&nbsp;&nbsp;&nbsp; Ja:&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp; Bei Änderung eines Vorgangs erfolgt eine autom. Anzeige der Partieauswahl</li><li>&nbsp;&nbsp;&nbsp; Nein:&nbsp;&nbsp; Keine autom. Anzeige der Partieauswahl bei Vorgangsänderungen</li></ul> |
| 23 | Steuert das Format der Partieauswahl<br><ul><li>&nbsp;&nbsp;&nbsp; 1 Zeile:&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp; &nbsp;Einzeilige Darstellung der Partien</li><li>&nbsp;&nbsp;&nbsp; 2 Zeilen:&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp; &nbsp;Zweizeilige Darstellung der Partien</li><li>&nbsp;&nbsp;&nbsp; m. Partiegruppe: Zweizeilige Darstellung der Partien mit Partiegruppe</li></ul> |
| 25 | Steuert das Neuanlegen einer Partie in der Vorgangserfassung (Ja/Nein/Verkauf/Einkauf)<br><ul><li>&nbsp;&nbsp;&nbsp; Nein:&nbsp;&nbsp; Vorgangserfassung im Einkauf und Verkauf ist keine Partieneuanlage möglich</li><li>&nbsp;&nbsp;&nbsp; Verkauf:Partieneuanlage in Vorgangserfassung nur im Verkauf möglich</li><li>&nbsp;&nbsp;&nbsp; Einkauf:Partieneuanlage in Vorgangserfassung nur im Einkauf möglich</li><li>&nbsp;&nbsp;&nbsp; Ja:&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp; Partieneuanlage in Vorgangserfassung im Einkauf und Verkauf möglich</li></ul> |
| 26 | Erfasste Sollmengen um Istmengen korrigieren?<br><ul><li>&nbsp;&nbsp;&nbsp; Verkauf: Ist-Menge aus dem Verkaufsvorgang wird der Partie-Sollmenge zugebucht</li><li>&nbsp;&nbsp;&nbsp; Einkauf: Ist-Menge aus dem Einkaufsvorgang wird der Partie-Sollmenge zugebucht</li><li>&nbsp;&nbsp;&nbsp; Nein:&nbsp;&nbsp; Keine autom. Veränderung der Partie-Sollmenge</li></ul> |
| 42 | Steuert die Restmengenanzeige des Partieauswahlfensters<br><ul><li>&nbsp;&nbsp;&nbsp; Ja:&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp; Die Restmengen einer Partie werden mit angezeigt</li><li>&nbsp;&nbsp;&nbsp; Nein:&nbsp;&nbsp; Die Restmengen einer Partie werden nicht angezeigt</li></ul> |
| 50 | Partiezwang bei Saatgutlieferungen (Ja/Nein/Verkauf/Einkauf) |
| 60 | Rabatte je Partie zugelassen<br><ul><li>&nbsp;&nbsp;&nbsp; Ja:&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp; Rabatte greifen auch in Verbindung mit den hinterlegten Partiepreisen</li><li>&nbsp;&nbsp;&nbsp; Nein:&nbsp;&nbsp; Rabatte greifen nicht in Verbindung mit den hinterlegten Partiepreisen</li></ul> |
| 61 | Zu-/Abschläge je Partie zugelassen<br><ul><li>&nbsp;&nbsp;&nbsp; Ja:&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp; Zu-/Abschläge greifen auch in Verbindung mit den hinterlegten Partiepreisen</li><li>&nbsp;&nbsp;&nbsp; Nein:&nbsp;&nbsp; Zu-/Abschläge greifen nicht in Verbindung mit den hinterlegten Partiepreisen</li></ul> |
| 62 | Frachten je Partie zugelassen<br><ul><li>&nbsp;&nbsp;&nbsp; Ja:&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp; Frachten greifen auch in Verbindung mit den hinterlegten Partiepreisen</li><li>&nbsp;&nbsp;&nbsp; Nein:&nbsp;&nbsp; Frachten greifen nicht in Verbindung mit den hinterlegten Partiepreisen</li></ul> |
