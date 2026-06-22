# SQL Remote-Nachrichtenagent Optionen

<!-- source: https://amic.de/hilfe/sqlremotenachrichtenagentoptio.htm -->

(Version 12.0.1.3851)

Optionen (genau in der gezeigten Schreibweise eingeben):

 -a Empfangene Transaktionen nicht anwenden

 -b Stapelverarbeitung

 -c "Schlüsselwort=Wert; ..."

 Datenbank-Verbindungsparameter angeben

 -dl Lognachrichten auf dem Bildschirm anzeigen

 -ek &lt;Schlüssel> Datenbank-Chiffrierschlüssel angeben

 -ep Eingabeaufforderung für Chiffrierschlüssel der

 Datenbank

 -g &lt;n> Gruppentransaktionen weniger als &lt;n> Vorgänge (Standard 20)

 -l &lt;Länge> Maximale Nachrichtenlänge (Minimum 10000, Standard 50000)

 -m &lt;Größe> Speicher für Nachrichten- und Datei-IO-Caching (Standard 2048 kB)

 -ml &lt;Verz> Verzeichnis für umbenannte Logspiegel

 -o &lt;Datei> Ausgabenachrichten in Datei protokollieren

 -os &lt;Größe> Maximale Ausgabedateigröße festlegen (Min. 10240)

 -ot &lt;Datei> Datei kürzen und Logausgabe in diese Datei umleiten

 -p Nachrichten nicht bereinigen

 -q In minimiertem Fenster ausführen

 -qc Dialogfeld nach dem Beenden schließen

 -r Nachrichten empfangen

 -rd &lt;Zeit> Empfangs-Abrufperiode (Standard 1 Minute)

 -ro &lt;Datei> Ausgabe von entfernter Datenbank in Datei protokollieren

 -rp &lt;n> Anzahl von Empfangsabfragen, bevor eine Nachricht als

 verloren eingestuft wird

 -rt &lt;Datei> Kürzen und Ausgabe von entfernter Datenbank in Datei

 protokollieren

 -ru &lt;Zeit> Wartezeit zum Neu-Scan des Logs nach Neusendeaufforderung

 (Standard: automatisch richtigen Wert auslesen)

 -s Nachrichten senden

 -sd &lt;Zeit> Sende-Abrufperiode (Standard 1 Minute)

 -t Triggeraktionen replizieren

 -u Nur gesicherte Transaktionen senden

 -v Vorgang ausführlich darstellen

 -w &lt;n> Worker-Threads zur Anwendung von Nachrichten (Standard 0,

 Maximum 50)

 -x &lt;Größe> Transaktionslog umbenennen und neu starten (Standard 0)
