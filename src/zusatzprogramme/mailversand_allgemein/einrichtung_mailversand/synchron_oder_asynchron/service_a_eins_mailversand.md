# Service A.eins Mailversand

<!-- source: https://amic.de/hilfe/_mailserviceeinrichten.htm -->

Service einrichten für den A.eins Mailversand

Mit Hilfe des Steuerparameters „SPA 1019 Mailversand per“ kann der Mailversand über „Datenbank“ oder „Dienst oder Exe“ ausgewählt werden. Zum Versenden der Emails per Service ist hier „Dienst oder Exe“ auszuwählen.

Um den Service zu installieren, öffnen Sie die „Eingabeaufforderung(Administrator)“. Mit Hilfe der „InstallUtil.exe“ wird jetzt der Service installiert.

Beispiel: &lt;Pfad zu InstallUtil.exe>InstallUtil.exe &lt;Pfad zu A.eins.MailSvc.exe>A.eins.MailSvc.exe

C:\\Windows\\Microsoft.NET\\Framework64\\v4.0.30319\\InstallUtil.exe C:\\Aeins\\bin64\\a.eins.mailsvc.exe

Der Service wird jetzt installiert und Sie werden aufgefordert den Benutzernamen und das zugehörige Passwort einzugeben, über den der Service gestartet werden soll.

In der Registrierung muss der Eintrag des A.eins.Mailservice erweitert werden.

Beispiel: Computer\\HKEY_LOCAL_MACHINE\\SYSTEM\\CurrentControlSet\\Services\\A.eins.MailService\\

Wählen Sie „neu“ und selektieren Sie „Schlüssel“. Name: Parameters

Im Schlüssel „Parameters“ wieder „neu“ auswählen und dieses Mal „Zeichenfolge“ selektieren.

Name: Parameters

Als Wert wird hier der Startparameter für den Service hinterlegt. Dieses kann sein die Datenbankverbindung gefolgt vom Sendeintervall in Minuten oder eine Pfadangabe zur Datei „A.eins.MailService.ini“.

Beispiel:

Datenbankverbindung mit Sendeintervall von 1 Minute;

eng=entw;dbn=entw;links=tcpip; 1

Pfadangabe zur Datei:

@C:\\Aeins\\bin64\\A.eins.MailService.ini
