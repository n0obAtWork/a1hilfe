# AIS

<!-- source: https://amic.de/hilfe/_formularzuordnung_ais.htm -->

Auf der Registerkarte „AIS“ können eine oder mehrere AIS-Gruppen den jeweiligen Vorgangsmasken zugeordnet werden. Dabei ist zu beachten, **dass alle AIS-Gruppen** die einer Vorgangsmaske zugewiesen worden sind auch beim Aktualisierungsaufruf des AIS aktualisiert werden. **Deswegen ist darauf zu achten, dass keine Zeitintensiven SQL Statements ausgeführt werden, da diese den Erfassungsablauf massiv stören könnten**.

| Gridfelder | Beschreibung |
| --- | --- |
| Gruppenname | In diesem Feld wird die AIS Gruppe hinterlegt |
| AIS Makro | Das AIS Makro wird aus der AIS-Gruppe gelesen. Ist kein Makro hinterlegt wird das AIS nur an den Standardpunkten im Vorgang aktualisiert z.B. Kundenwechsel auf der SVMAIN Maske. |
| Unit Name | Funktion des Makro welches aufgerufen werden soll, wenn eine Aktualisierung des AIS vorgenommen wird. |

Eine Beispieleinrichtung und Beschreibung für im AIS im Vorgang finden sie [hier.](../ais_im_vorgang/index.md)
