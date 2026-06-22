# Artikelmengen

<!-- source: https://amic.de/hilfe/_ktrartikelmengen.htm -->

Diese Maske steht nur für kontrakte mit mehreren Zeiträumen zur Verfügung.

Im oberen Bereich der Maske werden allgemeine Angaben dargestellt:

• Kontraktklasse

• Kontraktgruppe

• Hauptkunde

• Kontraktnummer

• Artikelnummer

Die Datentabelle weist für den Artikel die Mengen und die aktuellen Restmengen aller Kontrakt-Zeiträume des Artikels aus. Bei Einzelmengen-Kontrakten sind die Sollmengen in dieser Tabelle änderbar.

Änderungen von Soll-Mengen werden in einem Änderungsprotokoll dokumentiert. 

Zu Kontroll-Zwecken werden im unteren Bereich der Maske die aktuelle Gesamtsumme, die Restsumme und die ursprüngliche Gesamtsumme und Restsumme (vor Beginn der Kontraktänderung) sowie die jeweiligen Differenzen ausgewiesen.

| Feld | Beschreibung |
| --- | --- |
| Zeitraum | Beginn des Kontraktmengen-Zeitraums |
| Gesamtmenge | Sollmenge des Artikels im Kontrakt-Zeitraum.  
Bei Gesamtmengen-Kontrakten wird hier die gesamte Sollmenge des Kontrakt-Zeitraums dargestellt.  
Bei Freimengen-Kontrakten ist die Sollmenge immer mit 0 dargestellt. |
| Restmenge | Aktuelle Restmenge des Artikels im Kontrakt-Zeitraum  
Bei Gesamtmengen-Kontrakten wird hier die gesamte Restmenge des Kontrakt-Zeitraums dargestellt.  
Bei Freimengen-Kontrakten ist die Restmenge immer mit 0 dargestellt. |
| Rest>0 | Negativer Rest wird mit 0 dargestellt, aktueller Rest ist um negativen Rest des vorhergehenden Zeitraums reduziert.  
(Nur bei eingestellter Option [Steuerungsparameter](../../../firmenstamm/steuerparameter/kontraktwesen/ratierliche_einstellungen_spa_846.md) 846 „Ratierliche Einstellungen“ „Ktr-Anzeige Minusrest in Folgezeitraum“ mit dem Wert **Ja**). |
| Rest kumuliert | Summe der Werte aus vorhergehender Restspalte (Restmenge, Rest>0) bis einschließlich dem aktuellen Zeitraum  
(Nur bei eingestellter Option [Steuerungsparameter](../../../firmenstamm/steuerparameter/kontraktwesen/ratierliche_einstellungen_spa_846.md) 846 „Ratierliche Einstellungen“ „Ktr-Anzeige Kumulierte Zeitraum-Reste“ mit dem Wert **Ja**). |
