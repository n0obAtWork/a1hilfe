# Den SQL Remote-Nachrichtenagent als Dienst in Sybase Central anlegen

<!-- source: https://amic.de/hilfe/densqlremotenachrichtenagental1.htm -->

Zum Anlegen des Nachrichtenagenten dbremote als Service / Dienst unter Windows gehen Sie bitte wie folgt vor:

1. Starten Sie Sybase Central unter: ..\\Aeins\\bin64\\scjview.exe

2. Verbinden Sie sich mit der gewünschten Datenbank

3. Doppelklicken Sie auf SQL Anywhere 17

4. Wählen Sie die Registerkarte „Dienste“

5. Klicken Sie auf einer freien Stelle auf der Registerkarte „Dienste“ mit der RECHTEN Maustaste

6. Wählen Sie „Neu“ → „Dienst“. Es öffnet sich der Assistent zum Erstellen eines neuen Dienstes

7. Geben Sie einen Namen für Dienst an der gestartet werden soll und gehen anschließend auf „Weiter“

8. Markieren Sie nun „SQL Remote-Nachrichtenagent“ in der Liste aus und gehen anschließend auf „Weiter“

9. Klicken Sie nun auf durchsuchen, wählen die Datei „dbremote.exe“ aus dem „..\\Aeins\\bin\\“ Verzeichnis aus und gehen anschließend auf „Weiter“

10. Als Parameter wird dem Dienst der Speicherort der Konfigurationsdatei übergeben und sieht beispielsweise wie folgt aus: @C:\\Aeins\\config\\serviceparameter_&lt;Datenbankname>.txt

   (Wert in der spitzen Klammer ersetzen)

11. Da alle anderen Einstellungen so bleiben können, klicken Sie nun auf „Fertig stellen“
