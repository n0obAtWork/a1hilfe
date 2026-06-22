# Einrichten eines Containers

<!-- source: https://amic.de/hilfe/_container_einrichten.htm -->

1. Backup der aktuellen Datenbank erzeugen.

2. Das Backup auf dem Server ablegen, wo die Datenbank Zugriff hat.

3. Das Programm „scjview.exe“ im a.eins-Verzeichnis „Aeins\\bin64“ ausführen.

4. An die DB anmelden.

5. Ordner „Remote Servers“ auswählen.

6. Im Popup-Menü den Eintrag „New“ -> „Remote Server“ wählen.

7. Namen des Remoteservers angeben und Schaltfläche „Next >“ drücken

8. Typ des Servers auswählen, hier „SQL Anywhere“ und Schaltfläche „Next >“ drücken.

9. Verbindungs-Information für Datenbank-Verbindung erzeugen: „driver=sql anywhere 12;  
eng=xxx;dbf=Pfad\\DB.db; dbn=yyy;links=tcpip“ und Schaltfläche „Next >“ drücken.

• Die Information „xxx“ und „yyy“ sind in den Systeminformationen [SYSIN] im Feld „Verbindungsparameter“ zu finden.

• Die Angabe „Pfad“ entspricht dem Verzeichnis aus Schritt 2.

• Die Angabe „DB“ entspricht der Datenbankdatei aus Schritt 2.

10. Den im letzten Schritt erzeugte Verbindungsinformation in das Feld „Connection Information“ eingeben und Schaltfläche „Next >“ drücken.

11. Auswahlfeld „Make this remote server a read-only data source“ deaktivieren und Schaltfläche  
„Next >“ drücken.

12. Auswahlfeld „Create an external login“ deaktivieren und Schaltfläche „Test Connection“ drücken.

13. Wenn der Test fehlgeschlagen ist, wiederholen ab Schritt 5. Ansonsten Schaltfläche „Next >“ drücken.

14. Schaltfläche „Finish“ drücken.

15. Bei der Orginal-Datenbank die Tabelle Archiv leeren  
(mittels OSQL-Befehl: „truncate table archiv“).

16. Prüfen, ob Dokumente im Archiv vorhanden sind.
