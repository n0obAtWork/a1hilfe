# Ratierliche Einstellungen (SPA 846)

<!-- source: https://amic.de/hilfe/_SPA_846.htm -->

In diesem Steuerparameter können Optionen für die ratierliche Verteilung und/oder ratierliche Restmengenanzeige der Zeitraum-Tabellen im Kontraktstammpflegemodul eingestellt werden.

Zur Einstellung stehen verschiedene Typen zur Verfügung.

| Typ | Wert |
| --- | --- |
| MENGEUEBER | Für die Anzeige ratierlicher Monatsmengen in der Standardauswahlliste des Kontraktstamm-Pflegemoduls wie auch bei der variablen Zeitraumwahl (Option „Variable Kontraktzeitraumzuordnung“) während der Warenpositionsbearbeitung eines Vorgangs gilt bei der Einstellung dieser Option mit dem Wert „Ja“:  
 Ist bei der Verteilung die gelieferte Menge größer als die ratierliche Menge des Monats, wird die Übermenge in den nächsten Monat übernommen.  
Übermengen die über die Zeiträume hinausgehen, werden im speziellen Feld „UEBERMENGE“ in den jeweiligen Tabellen gespeichert.  
 |
| Variable Kontraktzeitraumzuordnung | Bei der Einstellung dieser Option mit dem Wert „Ja“ wird bei der Funktion „Kontraktzeitraum“ in der Warenpositionsmaske der Vorgangsbearbeitung eine erweiterte Kontraktzeitraummaske genutzt, die es ermöglicht, einen anderen als den automatisch zugeordneten Zeitraum zur Kontraktpositionszuordnung zu nutzen. |
| Ktr-Anzeige Minusrest in Folgezeitraum | Diese Option schaltet bei der Einstellung mit dem Wert „Ja“ auf den Zeitraummasken des Kontraktpflegemoduls (Mengenzeiträume, Artikelmengen) eine zusätzliche Tabellen-Spalte „Rest>0“ frei, in der negative Zeitraum-Restmengen/-werte (Übermengen/Überwerte) in den jeweils nächsten Zeitraum zur Verrechnung mit der dortigen Restmenge/Restwert übertragen werden. Die Restmenge/Restwert der Zeile selbst wird mit dem Wert „0“ ausgewiesen. |
| Ktr-Anzeige Kumulierte Zeitraum-Reste | Diese Option schaltet bei der Einstellung mit dem Wert „Ja“ auf den Zeitraummasken des Kontraktpflegemoduls (Mengenzeiträume, Artikelmengen) eine zusätzliche Tabellen-Spalte „Rest kumuliert“ frei, in denen die Restmengen/Restwerte kumuliert dargestellt werden. |
