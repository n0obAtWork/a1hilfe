# Leermeldung unterbinden bei Automatischer Leermeldung (SPA 1010)

<!-- source: https://amic.de/hilfe/_steupa_silo_option.htm -->

Mit diesem Steuerparameter können Optionen für die Silo Verwaltung eingestellt werden.

| Option | Wert | Bedeutung |
| --- | --- | --- |
| LEERMELDUNGUNTERBINDENBEIRESTMENGE | 0 ist Nein  
1 ist Ja | Mit diesem Steuerparameter kann die automatische Leermeldung ausgestellt werden. Bei der automatischen Leermeldung wurden automatisch Waagenbelege angelegt, welche die Menge des Silos auf 0 setzt. Danach wird das Silo erst leergemeldet.  
Ist der Steuerparameter auf „Ja“ gestellt, so muss das Silo manuell auf die Menge 0 gesetzt werden. Danach darf das Silo erst leergemeldet werden. |
| BISABSMENGELEERMELDUNGOHNEWAAGENBELEG | Menge von 0 bis …  
Standard Wert ist 0,01 | Mit dieser Option kann eingestellt werden, bis zur welcher Grenzmenge eine Leermeldung ohne Waagenbeleg erzeugt werden darf. Ist die Menge auf dem Silo kleiner als die Grenzmenge, so wird auch bei der gesetzten Option „ Leermeldung unterbinden bei Restmenge“ auch die Leermeldung durchgeführt. |
