# Kommandozeile

<!-- source: https://amic.de/hilfe/_scannerkommandozeilenparameter.htm -->

Folgende Kommandozeilenparameter stehen ab der Version 8.3.2.XXX zur Verfügung.

Die Scanner Software kann auch per Kommandozeile gestartet werden. Beim Start des Scanners per Kommandozeile besteht die Möglichkeit eine XML Datei anzugeben, welche dann Automatisch startet.

| Parameter | Wert |
| --- | --- |
| conn= | Wird eine Verbindungszeichenekette(Connectionstring) übergeben, so verbindet sich der Scanner mit der angegebenen Datenbank. Die Verbindungsdaten aus der dbconfig.xml werden nicht berücksichtig.  
Beispiel:  
eng=dbserver;dbn=datenbankname;uid=USER;pwd=PASSWORD;links=tcpip{HOST=ServerIp};pooling=false;idle=60;lto=30; |
| cf= | Mit dem Parameter kann ein Pfad zu einer alternativen Datei mit Verbindungsparameter angegeben werde. |
| scans= | An diesem Parameter kann ein Pfad zu einer XML Datei mit [Scanbefehlen](./beispiel_xml_datei.md) angegeben werden, die beim Starten der Software automatisch ausgeführt wird |
| scip= | Mit diesem Parameter kann dem Scanner eine IP-Adresse zugewiesen werden. Anhand dieser IP-Adresse werden die Steuerparameter aus dem A.eins System geladen. Diese IP-Adresse wird auch dazu verwendet um die erfassten Daten einem Scanner zuzuweisen. |
| Pip | Mit diesem kann dem Scanner eine Profil IP-Adresse mit gegeben werden. Anhand der Profil IP-Adresse wird aus der Datei mit den Verbindungsparameter der richtige Datensatz gelesen. Ist die Profil IP-Adresse in der Datei nicht vorhanden, so wird der Standard Datensatz gelesen. |

Durch die Möglichkeit die Scanner mit Übergabe Parameter ab der Version 8.3.2.XXX zu starten ist es sinnvoll auf den Scannern bat Dateien anzulegen, mit denen dann die Software gestartet werde kann. Beispiel für eine Bat Datei auf dem Scanner

cd \\windows\\aeins

start a.eins.scanner.exe conn=uid=USER;pwd=PASSWORT;dbn=DBN;eng=ENG;links=tcpip

Bei der Bat Datei für den Scanner ist darauf zuachten, dass **immer** in das A.eins Verzeichnis gewechselt werden muss, um die Scannersoftware zu starten.

Überblick für die zu Benutzenden Tasten

| TastenCodes | Bedeutung |
| --- | --- |
| F2 / TAB | Ist mit dem Scanvorgang gleichzusetzten (Eingabe im Textfeld muss vorhanden sein) |
| ENTER | Ist gleichzusetzen mit der Mengen Eingabe beim Scanner |
| F3 | Öffnet ein Dialogfenster wo eine XML Datei ausgewählt werden kann |
