# Replikation defekt! Was nun?

<!-- source: https://amic.de/hilfe/replikationdefektwasnun.htm -->

In ALLEN Fällen kann man zunächst folgende Schritte in dieser Reihenfolge vornehmen, um ein Problem mit der Replikation zu ermitteln:

1. Jede Replikation legt LOG-Dateien im Ordner „dbrexp“ an. Diese Log-Dateien tragen hier nach Anleitung den Namen des Publishers für die Datenbank die Repliziert werden soll. Diese Log-Datei öffnen und ans Ende scrollen. Hier wird dokumentiert, was zuletzt vom DBRemote gemeldet wurde!

2. Sofern Zugriff auf Aeins möglich, hier den Direktsprung [RINFO](./stammdaten/replikation_informationen/index.md) verwenden um Informationen zur Replikation und den angeschlossenen DBRemote-Usern zu erhalten.

3. Prüfen Sie unter Dienste den Status den Dienst des DatenbankServers und den Status des DBRemote-Dienstes. Starten Sie diese Dienste ggf. neu.

4. Prüfen Sie, wenn verwendet das Event „dbrexp-schedule“

Wird festgestellt, dass die Replikation bereits dermassen defekt ist, zum Beispiel infolge eines versäumten Updates der Filialdatenbank, so hilft meist nur ein Neueinrichten der defekten Seite.

Führen Sie zunächst ein Extrakt auf Seiten der korrekten Datenbank durch:

1. Starten Sie scjview.exe und verbinden sich mit der Datenbank

2. Suchen Sie den Eintrag „SQL Remote-Benutzer“

3. Klicken Sie mit der rechten Maustaste den dort in der Liste geführten Remote-User an und klicken auf „Datenbank extrahieren“

4. Beim Extrakt sollen Struktur und Daten mitgenommen werden

5. Verwenden sie die reload.sql Option zum späteren leichteren Einspielen

6. Nach Abschluss des Extraktes müssen diese Daten auf die defekte Seite geschafft und dort wieder eingespielt werden.

7. Um Fehler beim Einspielen zu vermeiden muss:

7.1 die Pfadangaben im reload.sql entsprechend den örtlichen Strukturen angepasst werden

7.2 Die Tabellen, welche durch das reload.sql Skript erstellt werden sollen, müssen entfernt werden

8. Nach dem Start des Skripts verfolgen Sie die Arbeitsschritte und bewerten und reagieren Sie entsprechend auf die gezeigten Meldungen

9. Nehmen Sie nach erfolgreicher Erstellung die Replikation wieder in Betrieb

10. Achten Sie hierbei darauf, ob der Adressen des Publishers und der Remote-User korrekt sind

  - Ob die Remote-User stimmig sind
  - Subskriptionen gestartet sind
  - Event gestartet ist

    11. Führen Sie folgenden Befehl auf beiden Seiten aus:

  - „REMOTE RESET &lt;Remote-User>; COMMIT;“

    Wichtig hierbei ist, dass die DBRemote-Nachrichtenagenten NICHT vorher gestartet wurden!
