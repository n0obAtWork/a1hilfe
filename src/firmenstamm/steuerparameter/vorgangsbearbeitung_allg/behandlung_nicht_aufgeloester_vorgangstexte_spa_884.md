# Behandlung nicht aufgelöster Vorgangstexte (SPA 884)

<!-- source: https://amic.de/hilfe/_SPA_884.htm -->

Den Vorgangsklassen können Texte zugeordnet werden, die Platzhalter enthalten, welche schließlich durch manuelle Eingabe aufgelöst werden. Diese Auflösung findet je nach Einstellung vor Beginn der Positionsteilerfassung oder vor dem Abschluss der Erfassung statt.

Bei der Umwandlung eines Vorgangs in einen neuen Vorgang einer anderen Vorgangsklasse (z.B. von Angebot in Auftrag) findet keine Erfassung statt und damit auch keine Auflösung dieser Texte. Aus diesem Grund ist eine Behandlung der Texte erforderlich, damit im erstellten Beleg keine Platzhalter dargestellt werden.

Mit Hilfe dieses Steuerparameters kann die Behandlung der Vorgangstexte bei Umwandlung festgelegt werden.

Die gleiche Behandlung wird bei der Erstellung eines Vorgangs mit Makro durchgeführt, da auch hier keine Auflösung durch manuelle Eingabe erfolgen kann.

| Einstellungen |
| --- |
| 0 – nichts ändern | Diese Einstellung ist die voreingestellte Behandlung wie bisher. Es findet keinerlei Aktion statt. |
| 1 – nur warnen | Eine Warnung weist bei Umwandlung darauf hin, dass nicht aufgelöste Texte vorhanden sind. |
| 2 – Auflösen und warnen | Platzhalter werden bei Umwandlung durch Leerzeichen ersetzt.  
Eine Warnung weist darauf hin. |
| 3 – Auflösen | Platzhalter werden durch Leerzeichen ersetzt. |
| 4 – Löschen und warnen | Nicht aufgelöste Textzeilen werden bei Umwandlung entfernt.  
Eine Warnung weist darauf hin. |
| 5 – Löschen | Nicht aufgelöste Textzeilen werden entfernt. |
