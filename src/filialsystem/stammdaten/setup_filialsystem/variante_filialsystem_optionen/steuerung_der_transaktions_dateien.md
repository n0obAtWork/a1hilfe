# Steuerung der Transaktions-Dateien

<!-- source: https://amic.de/hilfe/_steuerungdertransaktionsdateien.htm -->

| **Felder** | |
| --- | --- |
| Betriebsstätte | Im Neu-Fall F3-Auswahl der eingerichteten Betriebsstätten aus dem Filialstamm. Die Auswahl zeigt alle Betriebsstätten an, die noch nicht konfiguriert wurden. |
| Zentrale | Zeigt an ob es sich bei der Auswahl der Betriebstätte um eine Zentrale handelt |
| Wann oder wie sollen Transaktionslog-Dateien gelöscht werden? | Stellt den Wert der Datenbank-Option „**delete_old_logs**“ dar.  
Mögliche Einstellungen:  
\- **Off** ( Standard )  
\- **On**  
\- **Delay**  
\- **Sieben Tage**  
\- **Dreißig Tage** |
| Maximale Dateigröße der Transaktionslog-Datei? | Hier kann die maximale Größe der Transaktionslog-Datei angegeben werden. Im ersten Eingabefeld wird die Zahl eingetragen. Unter „in“ kann die Speichermengeneinheit angegeben werden.  
\- **Byte**  
\- **kB** ( kilo Byte)  
\- **MB** ( MegaByte )  
\- **GB** ( GigaByte )  
Die Angebe entspricht dem Wert des Steuerungsparameters „**\-x**“ für den SQL Remote-Nachrichtenagenten **dbremote.** |
| Maximale Dateigröße der Auslagerungslog-Datei? | Hier kann die maximale Größe der Auslagerungslog-Datei angegeben werden. Im ersten Eingabefeld wird die Zahl eingetragen. Unter „in“ kann die Speichermengeneinheit angegeben werden.  
\- **Byte**  
\- **kB** ( kilo Byte)  
\- **MB** ( MegaByte )  
\- **GB** ( GigaByte )  
Die Angebe entspricht dem Wert des Steuerungsparameters „**\-os**“ für den SQL Remote-Nachrichtenagenten **dbremote.** |
| Name der Prozedur für Fehlerbehandlungen? | Stellt den Wert der Datenbank-Option „**replication-error**“ dar. Bleibt das Feld leer ( Standard ), so werden Replikationsfehler nicht behandelt. |
| Ausführliche Ausgabe im Log? | Auswahl Ja / Nein  
Legt dabei fest, ob der SQL Remote-Nachrichtenagent **dbremote** eine ausführliche Protokollierung ausführen soll ( entspricht dem Steuerungsparameter „**\-v**“ ) |

| **Funktionen** | |
| --- | --- |
| Speichern F9 | Speichert die Eingaben in der Tabelle Filialsystemoptionen. |
| Optionen setzen | Wird nur dann in der Funktionsliste angezeigt, wenn die im Feld Betriebsstätte ausgewählte Nummer, der der aktuellen Betriebsstätte entspricht.  
Mit dieser Funktion werden die Datenbank-Optionen „delete_old_logs“ und „replication_error“ in der Datenbank gesetzt. |
