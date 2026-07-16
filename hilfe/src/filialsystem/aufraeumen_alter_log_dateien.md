# Aufräumen alter Log-Dateien

<!-- source: https://amic.de/hilfe/_alteLogs.htm -->

Um die Übersicht innerhalb der Log-Dateien zu gewährleisten, werden die Log-Dateien nach Erreichen einer von uns vorher festgelegten Speichergröße umbenannt und neu angelegt.

Durch das Umbenennen dieser Log-Dateien wird natürlich Speicherplatz auf dem Datenträger belegt. Im ungünstigsten Fall kann zu Fehlern und/oder Problemen kommen, da der Datenträger nicht mehr genügend Speicherplatz zur Verfügung stellen kann.

Es ist also hier zwingend erforderlich, die alten Log-Dateien von Zeit zu Zeit aufzuräumen und ggf. zu entfernen.

Alte, umbenannte Transaktionslogdateien mit der Endung „.dbr“ befinden sich im Verzeichnis „..\\Aeins\\dbrexp\\Log\\alte_DBRLogs“.

Alte, umbenannte Datenbanklogdateien befinden sich im Verzeichnis der Datenbank. Über den Direktsprung [RINFO] in A.eins, erhält man unter anderem den Pfad zum Datenbanklogverzeichnis.

Mit der Datenbank-Replikationsoption „delete_old_logs“ werden alte, umbenannte Transaktionslogdateien behandelt. Der Standardwert ist „**Off**“. Setzen Sie den Wert der Option auf „**On**“.

```sql
SET OPTION public.delete_old_logs = 'ON';
```
