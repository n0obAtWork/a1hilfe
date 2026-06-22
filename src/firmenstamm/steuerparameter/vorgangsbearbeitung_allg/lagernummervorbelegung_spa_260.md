# Lagernummervorbelegung (SPA 260)

<!-- source: https://amic.de/hilfe/_SPA_260.htm -->

| SPA-Einstellungen |
| --- |
| 0 – nie vorbelegen | Die Lagernummer wird nie vorbelegt |
| 1 – wie letzte Auswahl | Die Lagernummer wird aus den Vorgangskonstanten vorbelegt. Eine Änderung der Lagernummer ändert auch die Vorgangskonstanten (nicht empfohlen) \*\* |
| 2 – aus Vorgangskonstanten | Die Lagernummer wird bei jedem neuen Vorgang aus den Vorgangskonstanten vorbelegt. \*\* |
| 3 – aus VKONS b. Mehrbeleg wie vorheriger Vorg. | Die Lagernummer wird bei der ersten Erfassung aus den Vorgangskonstanten vorbelegt. Im Fall der Mehrbelegserfassung wird die letzte verwendete Lagernummer weiterverwendet. Eine Änderung der Vorgangskonstanten wie in Einstellung 2 findet jedoch nicht statt. \*\* |

\*\* Bei Einstellung 1-3 – Es wird die Vorbelegung in Abhängigkeit folgender Einstellungen vorgenommen, die einander (von oben nach unten) überlagern können:

| Einstellungen |
| --- |
| Vorgangskonstanten | Die in [VKONS] eingetragene Lagernummer wird vorbelegt |
| WWW-Konstante des Bedieners | Im Bedienerstamm können sog. WWW-Konstanten definiert werden. Diese überlagern bei dem aktuellen Bediener die Einstellung in den Vorgangskonstanten |
| UFLD-Feld | Eingaben in einem UFLD-Feld überlagern sowohl die Vorgangskonstanten als auch die WWW-Kontante des Bedieners |
