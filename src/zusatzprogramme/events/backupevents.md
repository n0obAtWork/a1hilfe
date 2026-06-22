# Backupevents

<!-- source: https://amic.de/hilfe/_backupevents.htm -->

Backups, also Datensicherungen der Datenbank sind ein wesentlicher Bestandteil der Datensicherheit in Ihrem System. Deshalb sollten Sie regelmäßig Sicherungen Ihrer Datenbank vornehmen lassen. Backupevents sind Events, wie alle anderen auch. Sie sorgen lediglich für die Erstellung eines Backups und werden mit Hilfe eines eigenen Pflegers erstellt, der Ihnen die wesentlichen Parameter setzt.

Neues Backup Event

Mit dieser Funktion lassen sich Backupevents schnell einrichten.

| Feld | Beschreibung |
| --- | --- |
| Backup Name | Der Name des Backups ist zugleich der Name des Backup-Events. Deshalb darf der Name keine Leerzeichen enthalten. Eingegebene Leerzeichen werden beim Verlassen des Eingabefeldes in Unterstriche ‚_‘ gewandelt. |
| BackupTyp | Wählen Sie beim Backuptyp zwischen  
• **Vollbackup ohne Logfileaufbewahrung:** Hier wird die komplette Datenbank in das Backupverzeichnis kopiert und das aktuelle Transaktionslog nach Erstellung der Sicherungskopie verkürzt und neu gestartet.  
• **Vollbackup mit Logfileaufbewahrung:** Hier wird die komplette Datenbank in das Backupverzeichnis kopiert. Der Datenbankserver wird veranlasst, das aktuelle Transaktionslog nach Erstellen der Sicherungskopie umzubenennen. Die Sicherungskopie des Transaktionslogs erhält einen Namen mit dem Format *JJMMTTnn.log*, um der umbenannten Kopie des aktuellen Transaktionslogs zu entsprechen.  
• **Tagesbackup ohne Logfileaufbewahrung:** Hier wird NUR das Transaktionslog kopiert und das aktuelle Transaktionslog nach Erstellung der Sicherungskopie verkürzt und neu gestartet.  
• **Tagesbackup mit Logfileaufbewahrung:** Hier wird das Transaktionslog kopiert. Der Datenbankserver wird veranlasst, das aktuelle Transaktionslog nach Erstellen der Sicherungskopie umzubenennen. Die Sicherungskopie des Transaktionslogs erhält einen Namen mit dem Format *JJMMTTnn.log*, um der umbenannten Kopie des aktuellen Transaktionslogs zu entsprechen. |
| Backup Directory | Geben Sie hier an, in welches Verzeichnis das Backup erstellt werden soll.  
Bitte beachten Sie, dass Sie dieses Verzeichnis exklusiv für dieses Backup nutzen sollten, weil im Fehlerfall alle Dateien aus diesem Verzeichnis gelöscht werden. |
| Archiv Backup Directory  
 | Geben Sie hier an, in welches Verzeichnis das Backup der Archivdatenbank erstellt werden soll. Dieses Feld ist nur betretbar, wenn Sie die Option **Ext. Archiv Sichern** aktiviert haben.  
Bitte beachten Sie, dass Sie dieses Verzeichnis exklusiv für dieses Backup nutzen sollten, weil im Fehlerfall alle Dateien aus diesem Verzeichnis gelöscht werden. |
| Startzeit | Geben Sie hier an, zu welcher Zeit der Event gestartet werden soll. |
| Ext. Archiv Sichern | Wenn Ihr Archiv in einer externen Datenbank gespeichert wird, so steht Ihnen die Option „ext. Archiv sichern“ zur Verfügung. Aktivieren Sie diese Option, wenn Sie diese Datenbank ebenfalls sichern wollen.  
Hinweis: Beachten Sie beim Backup einer externen Archivdatenbank, dass die gleiche Archivdatenbank für verschiedene Mandantenserver zur Verfügung stehen kann. Nur einer der Server darf das Backup der Archivdatenbank verwalten. |
| Backup Validieren | Bei der Validierung wird eine Kopie des Backups angelegt und diese auf Inkonsistenzen in der Datenbank geprüft. Dies geschieht auf einer Kopie des Backups aus zweierlei Gründen:  
• Die Validierung der aktiven Datenbank erfordert die Inaktivität aller Nutzer, es kann also niemand mehr produktiv mit der Datenbank arbeiten  
• Die Validierung startet die Datenbank direkt, was mit einem Backup niemals geschehen darf.  
Hinweise:  
• Die Kopie des Backups erfordert zusätzlichen Platz auf dem Backup-Datenträger.  
• Die Kopie erfordert Schreibrechte auf dem Datenträger  
• Die Kopie erfolgt im Unterordner „temp“ des Backup-Verzeichnisses. Dieses Verzeichnis muss vorhanden sein!  
• Die anschließende Validierung kann sehr zeitintensiv sein und Ressourcen des Rechners verbrauchen. |
| Online-Validierung | Mit dieser Option lässt sie die Datenbank vor dem Backup im laufenden Betrieb validieren.  
Hinweise:  
• Dies kann jedoch nur für die Datenbank selbst und nicht für eine externe Archivdatenbank geschehen!  
• Eine fehlerhafte Validierung stoppt die Datenbank sofort. Eine Fortsetzung der Arbeiten ist nur nach einer Reparatur / Wiederherstellung möglich.  
• Ein Backup (auch das Backup einer externen Archivdatenbank) findet bei fehlerhafter Validierung nicht mehr statt. |
| Erweiterte Optionen | In den erweiterten Optionen können Sie einstellen, an welchen Tagen das Backup zur angegebenen Uhrzeit laufen soll. |

Der Einrichter erstellt selbständig die zugehörige Verarbeitungsroutine.

Siehe auch [Technische Hilfe zur Prozedur AMIC_EVT_BACKUP_DATABASE](http://www.amic.de/ihilfe/XMLDocuments/iAeins/html/M_SQL_function_AMIC_EVT_Backup_Database_6_d6759f01.htm)
