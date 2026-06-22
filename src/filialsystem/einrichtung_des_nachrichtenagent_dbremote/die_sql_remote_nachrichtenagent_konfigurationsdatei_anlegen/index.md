# Die SQL Remote-Nachrichtenagent Konfigurationsdatei anlegen

<!-- source: https://amic.de/hilfe/diesqlremotenachrichtenagentko.htm -->

1. Starten Sie den Editor

2. Klicken Sie nun auf Datei à Speichern unter

3. Bewegen Sie sich im Dateiexplorer in das Verzeichnis „..\\Aeins\\config\\“

4. Speichern Sie die Datei unter dem Namen „serviceparameter_Datenbankname.txt“ ab. ACHTUNG! Ersetzen Sie das Wort Datenbankname in der Dateibezeichnung auch wirklich durch den Datenbanknamen!

5. Zurück im Editor, müssen nun folgende [Optionen für den Start des SQL Remote-Nachrichtenagenten](http://dcx.sybase.com/1200/de/sqlremote/dbrem.html) konfiguriert bzw. angegeben werden:

\-c "uid=admin;pwd=\*\*\*\*\*\*;eng=&lt;ServerName>;dbn=&lt;DatenbankName>;links=tcpip"

\-x 50m

\-os 5m

\-rd 30s

\-v

\-r

\-s

\-ro C:\\aeins\\dbrexp\\&lt;RemoteUserName>_err.log

\-o C:\\aeins\\dbrexp\\&lt;PublisherName>.log

c:\\aeins\\daten\\&lt;DatenbankName>

6. Ändern Sie die Werte in den Spitzen Klammern entsprechend!

7. Speichern Sie die Konfigurationsdatei unter Datei à Speichern

<p class="siehe-auch">Siehe auch:</p>

- [SQL Remote-Nachrichtenagent Optionen](./sql_remote_nachrichtenagent_optionen.md)
