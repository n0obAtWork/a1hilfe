# Einrichtung

<!-- source: https://amic.de/hilfe/_ueberwachungeinrichten.htm -->

| Feld | Beschreibung |
| --- | --- |
| Tabellenname | Auswahl der zu Überwachenden Tabelle.<br>Bei gesperrten oder geblockten Tabellen, ist die jeweilige Zeile rot gekennzeichnet.<br>Es gibt zwei Besonderheiten bei den Tabellen:<br>1) Wenn man die Tabelle „Kundenkredit“ überwacht, muss man auch das Feld KundKredit im Kundenstamm überwachen.<br>2) Wenn man die Tabelle „KundForderGruppe“ überwacht, so muss man auch das Feld ForGrupNummer im Kundenstamm überwachen.<br>Es erscheinen bei der Einrichtung entsprechende Meldungen.<br> |
| Tabellenfeld | Zeigt die Felder der ausgewählten Tabelle.<br> |
| Überwachen? | Hier kann entschieden werden, ob das Feld überwacht werden soll oder nicht.<br> |
| Hinweisfeld (unten links) | Es wird geprüft ob Tabellen aktuell für eine Bearbeitung durch einen Mitarbeiter gesperrt sind. Ist dies der Fall, so wird hier ein Hinweis mit der gesperrten Tabelle, sowie dem Benutzer, der die Sperre verursacht angezeigt.<br> |

<p class="just-emphasize">Hilfe F1</p>

Ruft diese Hilfe auf.

<p class="just-emphasize">Alle Felder überwachen F5</p>

Hiermit werden in alle Felder der Spalte „*überwachen?*“ eine **JA** eingetragen.

<p class="just-emphasize">Überwachung löschen F7</p>

Hiermit werden in alle Felder der Spalte „*überwachen?*“ eine **NEIN** eingetragen.

<p class="just-emphasize">Speichern F9</p>

Um die gewünschten Änderungen zu übernehmen, ist die Funktion „Speichern“ im Funktionsmenü zu verwenden. Hierbei werden intern private Trigger erstellt, welche eine Überwachung der gewählten Daten gewährleisten sollen.

Sind hierfür benötigte Tabellen aktuell gesperrt, so können keine Trigger angelegt werden. Erst nach Freigabe dieser Tabelle können diese privaten Trigger erstellt werden.

Sollte es einmal nötig sein diese privaten Trigger neu zu erstellen (z.B.: nah Änderungen am System oder den überwachten Tabellen), so können Sie alle überwachten Felder einer Tabelle auf „Nein“ stellen und diese Änderungen speichern. Die privaten Trigger dieser Tabelle werden entfernt. Nun können Sie die Feldüberwachung für die gewünschten Felder wieder aktivieren und speichern.

<p class="just-emphasize">Mail an Blocker F4</p>

Geblockte oder gesperrte Tabellen werden auf der Maske im Feld Tabellenname rot gekennzeichnet. Weiterhin wird im Hinweisfenster unten Links ein entsprechender Hinweis zu sehen sein. Über die Funktion „Mail an Blocker“ wird eine E-Mail an den Benutzer gesendet, welche diesen darauf hinweist die Tabelle freizugeben.
