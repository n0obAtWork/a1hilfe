# Benutzung Warenreorganisation

<!-- source: https://amic.de/hilfe/_benutzungwarenreorganisation.htm -->

Hauptmenü > Systempflege > Abstimmung > Warenreorganisation

oder Direktsprung [WAREO]

Die Funktionen aus dem Bereich WAREO sollen die Folgen außergewöhnlicher Zustände im Bereich der Warenwirtschaft prüfen und gegebenenfalls richtigstellen. Im Einzelnen können dies unkontrollierte Abbrüche, fehlerhafte Einrichtungen oder auch Fehlbedienungen sein.

Veranlassung zum Ausführen eines solchen Reorganisationsvorganges kann zum Beispiel eine festgestellte Differenz im Bereich „Ware abstimmen [WABST]“ sein. Die Funktionen sollten grundsätzlich nur in Abstimmung mit den zuständigen Supportern verwendet werden. Besonders gilt dies für die Funktionen im unteren Bereich der Maske.

Im oberen Bereich des Bildschirmes wird der Fortschritt einer laufenden Aktion dargestellt, im unteren Bereich erfolgt die Funktionsauswahl.

Auswahl einer Funktion mit Richtungstasten oder Mausklick. Nach Anwahl einer Funktion erscheint ein kurzes Beschreibungsfeld, hier wird der eigentliche Start der Aktion ausgelöst.

Inhalt und Anwendungsbereich der Funktionen werden im Folgenden beschrieben.

| Funktion | Beschreibung |
| --- | --- |
| Konsistenzprüfung | Siehe Analysefunktionen |
| Gesamt Reorganisation Ware | Konfigurierbare Kombination mehrerer Reorganisationsschritte zum Ablauf ohne Benutzereingriff |
| Vorlauf: Buchungstabellen leeren | Löschen der Summen aller Artikel ( fakturierte Summen, Lagerplatzsummen, Bestandssummen). Ermöglicht eine komplette Neuberechnung in der Funktion **Bestandsdateien reorganisieren** |
| Bestandsdateien reorganisieren | Berechnet fehlende Artikelbestände und Artikelsummen auf Basis der Einzelbewegungen des Warenbuches neu. |
| Nachkalkulation der Artikelbewertung | Die Bewertungen incl. der Überträge Periode im Bereich Artikelsummen werden hier auf Basis der Einzelbewegungen in der relevanten Reihenfolge neu berechnet. |
| Konsistenz Artikel | Ruft ein Modul mit diversen Prüf- und Reorganisationsfunktionen für ausgewählte Artikel auf. |
| Korrekturfelder zurücksetzen | Wird ein Beleg während der Erfassung abgebrochen (Absturz), bleiben unter Umständen die schon im Artikelbestand berücksichtigten Korrekturmengen erhalten, was zu Fehlinterpretationen führt. |
| Vorgangstapelpositionen bereinigen | Datensätze, die sich in der Relation Vorgstapelposit befinden und deren V_Id nicht im Vorgangstamm existiert, werden entfernt. |
| Kontraktbestände reorganisieren | Errechnet die Kontraktbuchungen und Kontraktbestände auf der Basis der Einzelbewegungen des Warenbuches neu |
| Partiebestände reorganisieren | Errechnet die Partiebuchungen und Partiebestände auf der Basis der Einzelbewegungen des Warenbuches neu |
| Teildisposition reorganisieren | Die Verweise und Restmengen innerhalb von teildisponierten Belegen incl. der Belegbezüge (welche Quelle, welches Ziel) werden hier neu eingetragen. |
| Leergut reorganisieren | Die Leergutbewegungen werden aus den vorhandenen Warenbewegungen neu aufgebaut. |
| Baustellen reorganisieren | Die Baustellenbewegungen werden aus den vorhandenen Warenbewegungen neu aufgebaut und die Summen der Baustellenartikel neu berechnet. |
| Kundensummen reorganisieren | Die Kunden- und Lieferantensummen werden hier neu berechnet. |
| Fehlerhafte Vorgänge aus der Ware entf. | Wird ein Beleg während der Erfassung abgebrochen (Absturz), bleiben unter Umständen Teile des Beleges erhalten (Belegkopf ohne Positionen oder Positionen ohne Belegkopf), was zu Fehlinterpretationen führt. |
| Fehlende Mandantenservereinträge | Es werden hier alle Belege auf korrekte Verarbeitung durch den Mandantenserver geprüft und evtl. fehlende Einträge in den Datenstrom eingestellt. Voraussetzung ist der Stopp des Mandantenservers ohne anstehende Aufträge. |
| Perioden angleichen | Ausgehend von der Buchungsperiode Fibu wird die Belegperiode und anschließend die Periode der Bewegung gesetzt, falls Abweichung vorhanden. Hiermit wird nachträglich der Gleichstand der Periodenzuordnung eines Beleges erzwungen. |
| Rech.Gut.Perioden laut Belegdatum | Periodenzuordnung wird zwingend zum Datum des Beleges gesetzt.  
ACHTUNG!!! Dies ist nicht in allen Betrieben so gewünscht. |
| Warenbuch reorganisieren | Ermittelt Abweichungen zwischen Summe der Einzelpositionswerte und dem  
Belegwert und bereinigt evtl. Abweichungen. |
| Problemfälle VorgReservierung | Bearbeitung von Problemfällen in der Vorgangsreservierung. |

Die Reorganisation kann jetzt auch per Event gestartet werden. Mehr dazu unter [Wareo](../../zusatzprogramme/events/wareo.md).
