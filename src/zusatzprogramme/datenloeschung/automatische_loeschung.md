# Automatische Löschung

<!-- source: https://amic.de/hilfe/_datloeautloesch.htm -->

Neben den Parametern zur Verbindung mit der Datenbank, kann ein weiterer hinzugefügt werden, um die Löschung zu automatisieren. Dieser sieht wie folgt aus:

Profil=profil-ID

Dieses Profil kann in der Tabelle Datenloeschung_Profile erstellt und angepasst werden. Dabei gibt es folgende Spalten:

| Spaltenname | Beschreibung |
| --- | --- |
| Dl_Profil_Id | Ist der Identifikator und muss im Startparameter angegeben werden. |
| Dl_LoescheBis | Gibt das Jahr an, bis zu welchem gelöscht werden soll. |
| Dl_LoeschZeit | Entspricht dem Timer in Minuten, wie lang dieser Löschprozess andauern soll. |
| Dl_LogVerzeichnis | Beinhaltet den Dateipfad, in welchem die Logdateien gespeichert werden sollen.  
Ist der angegebene Pfad nicht vorhanden, so wird dieser automatisch angelegt. |
| Dl_ArchivTabelle | Falls eine der Kategorien Archiv, Archivanhänge, oder Formulararchiv ausgewählt ist, wird der Name dieser Tabelle hier verlangt. |
| Dl_Cb_\*\* | Diese Felder entsprechen den Kategorien in der Anwendung. Wenn eine Kategorie gelöscht werden soll, wird in das jeweilige Feld eine 1 eingetragen, ansonsten bleibt es leer bzw. mit 0 gefüllt. |

Falls ungültige Daten, wie unmögliche Jahre oder falsche Archivtabellen, angegeben wurden, wird die Löschung nicht gestartet.
