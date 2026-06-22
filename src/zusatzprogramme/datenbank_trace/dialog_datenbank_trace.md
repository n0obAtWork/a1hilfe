# Dialog „Datenbank Trace“

<!-- source: https://amic.de/hilfe/dialogdatenbanktrace.htm -->

Hauptmenü > Administration > Werkzeuge > Tracefile

oder Direktsprung [**TRON**]

Die Aeins-Trace-Funktionalitäten unterstützen eine Analyse der aeins-seitig gegen die Datenbank verbrachten Datenbank-Anweisungen. Zwar sind nicht durchgängig in allen Fällen alle tatsächlich verwendeten Parameter ermittelbar, aber für einen ersten Überblick sind detaillierte Angaben über Art und Beschaffenheit, sowie Laufzeitverhalten - auch ohne weitere Entwicklungswerkzeuge – gegeben.

| Felder | Dialog „Datenbank Trace“ |
| --- | --- |
| Dateiname | Textdatei mit den ermittelten Daten.  
Diese Textdatei stellt ein OSQL-Einspielskript in die Relation „amic_tracefile“ dar. ([Aufbau der Datenbank-Tracedatei](./aufbau_der_datenbank_tracedatei.md)) |
| wie öffnen | Überschreiben  
Anhängen |
| Status | Aus  
An  
Genau |
| Hochkomma umwandeln | Ja  
Nein |
| Insert am Ende | Ja  
Nein |
| Stammdateninterface protokollieren | Ja  
Nein |
| Menüaktivitäten protokollieren | Ja  
Nein |

| Funktionen | Dialog „Datenbank Trace“ |
| --- | --- |
| Datei editieren [F4] | Öffnet die Textdatei im Notepad. |
| Übernehmen [F9] | Übernimmt die getätigten Einstellungen |
