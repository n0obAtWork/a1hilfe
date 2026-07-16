# Aktualisierung der Nachhaltigkeitswerte

<!-- source: https://amic.de/hilfe/_nachhaltigkeit_aktualisierung.htm -->

Bei der Änderung von [Stammdaten](../stammdaten/index.md) müssen die Nachhaltigkeitsbewegungen nachkalkuliert werden. Da es sich bei den Bewegungen um eine große Anzahl von Daten handeln kann und die erneute Berechnung der Werte einige Zeit in Anspruch nimmt, wird die Berechnung durch den Mandantenserver vorgenommen.

#### Aktualisierungseinstellungen

Auf der Maske lassen sich für einzelne Wochentage die abzuarbeitenden Vorgänge je Intervall einstellen. Dadurch kann man den Mandantenserver so einstellen, dass er am Tag wenig und in der Nacht viele Belege abarbeitet. Somit kann verhindert werden, dass der Mandantenserver zu Stoßzeiten zu lange blockiert wird.

Wenn Aktualisierungseinstellungen existieren, aber kein passender Eintrag gefunden wurde, werden keine Belege abgearbeitet. Sollten jedoch keine Aktualisierungseinstellungen existieren, wird die Standardanzahl von 50 Belegen verwendet.

Zusätzlich zu diesen Einstellungen lassen sich ein paar Einstellungen am [Steuerparameter 844](../../../firmenstamm/steuerparameter/optionen_warenwirtschaft/nachhaltigkeitseinstellungen_spa_844.md) ändern.

Folgende Felder stehen zur Verfügung.

| Feld | Beschreibung |
| --- | --- |
| Wochentag | Hier kann der Wochentag eingerichtet werden, für den die Anzahl gelten soll. |
| Von | Die Start Uhrzeit für die Anzahl Belege. |
| Bis | Die End Uhrzeit für diese Anzahl Belege. |
| Anzahl | Die Anzahl der zu verarbeitenden Belege. |

Bei sich überschneidenden Zeiträumen gilt immer der Eintrag, der am nächsten an der aktuellen Zeit dran ist und die niedrigste Anzahl hat.

Beispiel:

| Wochentag | Von | Bis | Anzahl |
| --- | --- | --- | --- |
| Montag | 00:00 | 23:59 | 100 |
| Montag | 08:00 | 12:00 | 25 |
| Montag | 09:00 | 13:00 | 30 |

- Um 6:45 würde der Wert 100 verwendet werden.
- Um 8:35 würde der Wert 25 verwendet werden.
- Um 9:43 würde der Wert 30 verwendet werden.
- Um 12:51 würde der Wert 30 verwendet werden.
- Um 14:42 würde der Wert 100 verwendet werden.

### Staffelung

Zusätzlich existiert dann noch eine Staffelung der Belege, je nachdem wieviel der Mandantenserver aktuell zu tun hat. Der ermittelte Wert aus Wochentag und Uhrzeit wird dann mit dem Faktor multipliziert.

| Anzahl Belege | Faktor | Anzahl | Beispiel |
| --- | --- | --- | --- |
| Keine Belege vorhanden | 2 | 25 | 50 Belege |
| Belege vorhanden | 1 | 25 | 25 Belege |

Insgesamt werden jedoch nie mehr als 500 Belege auf einmal abgearbeitet. Bei der Einstellung sollte zusätzlich beachtet werden, dass die Verarbeitung nicht mehr als 2 Minuten dauern darf, da der Mandantenserver ansonsten neugestartet wird.
