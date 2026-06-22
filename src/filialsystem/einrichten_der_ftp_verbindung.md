# Einrichten der FTP-Verbindung

<!-- source: https://amic.de/hilfe/einrichtenderftpverbindung.htm -->

Wird der Datenaustausch innerhalb der Replikation über verschiedene Standorte der Datenbankserver realisiert, der Austausch der Nachrichten über FTP geregelt. Hierfür benötigen wir ein FTP-Skript (FTP.PS1), welches die Verbindung zum FTP-Server aufbaut und den Datenaustausch intelligent regelt.

**!! WICHTIG !!**

**Das Event „**[**dbrexp_schedule**](./einrichten_des_events_dbrexp_schedule/index.md)**“ darf nicht aktiviert sein!**

Die Erstellung des FTP-Skriptes wird durch die Datenbank Prozedur „***amic_remote_schedule_ftp()***“ durchgeführt. Hierzu gehen Sich in Sybase Central wie folgt vor:

1. Starten Sie Sybase Central unter: ..\\Aeins\\bin64\\scjview.exe

2. Verbinden Sie sich mit der gewünschten Datenbank

3. Wählen Sie in der Ordneransicht den Punkt aus in dem der Datenbankname steht (

4. Klicken Sie diesen mit der RECHTEN Maustaste an und klicken anschließend auf „Interactive SQL öffnen“

5. Geben Sie nun im neuen Fenster die folgende SQL Befehlskette ein: „***call amic_remote_schedule_ftp()***“ und drücken Sie die Funktionstaste F9 zum ausführen

6. Schließen Sie das Fenster wieder

7. Öffnen Sie den Dateiexplorer und bewegen sich in das Verzeichnis „..\\Aeins\\dbrexp\\“

8. Suchen Sie hier die Datei FTP.PS1 , klicken diese mit der RECHTEN Maustaste an und klicken anschließend auf „bearbeiten“

9. Suchen Sie im jetzt geöffneten Editor / Notepad nach dem Stichwort „. Main“ (Dies sollte die letzte Zeile in dem Skript sein)

10. Passen Sie die hier zu findenden Parameter an:

b. –bstpath Pfad zum lokalen dbrexp-Verzeichnis (z.B. c:\\aeins\\dbrexp)

c. –bst Remoteusername für den die Verbindung aufgebaut wird (z.B. BST2)

d. –bstip IP des FTP-Servers

e. –bstuser Benutzername zur Anmeldung am FTP-Server

f. –bstpwd Kennwort zur Anmeldung am FTP-Server

11. Speichern Sie die Änderungen und schließen den Editor / Notepad

Die FTP Einrichtung ist damit abgeschlossen.

*Hinweis:*

*Das* [*Ausführen von Skripten*](./technisches_zu_replikation_mit_und_in_a_eins/ausfuehren_von_skripten_zulassen.md) *muss erlaubt sein.*
