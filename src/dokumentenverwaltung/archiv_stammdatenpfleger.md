# Archiv-Stammdatenpfleger

<!-- source: https://amic.de/hilfe/_archivstammdatenpfle.htm -->

Sie erreichen den Stammdaten-Pfleger in den Anwendungen und Varianten des Archivs. Außerdem ist er über die „**Archiv anzeigen**“ verfügbar.

| Felder |
| --- |
| Beleg-Referenz | Referenz  
   
Die Referenz stellt eine Art „Klammer“ dar, die über das Archiv hinweg „zusammengehörige“ Dokumente strukturiert.  
Sie wird bei der Neuanlage von Objekten (z.B. Vorgänge, Artikel, usw.) mittels der „Archiv-Fakte“ ermittelt und im Laufe der Operationen – wie zum Beispiel beim Archivieren von Drucken – dem so entstandenen Dokument im Archiv zugeordnet. Die „Archiv anzeigen“-Methodiken erlauben dann in diesem Kontext diese Archiv-Einträge zu recherchieren.  
   
Grundsätzlich ist aber hier die „Beleg-Referenz“ frei wählbar. |
| Belegnummer | Eine Beleg-Nummer.  
Diese wird standardmäßig beim Druck von Vorgängen die Beleg-Nummer sein, es können aber je nach Kontext auch z.B. externe Beleg-Nummern sein. |
| Belegdatum | Das Beleg-Datum.  
Im Falle von Hinzufügungen (Belegklasse „Hinzufügung“, 9003) kann dieses Datum geändert werden. |
| Kundennummer | Die zugeordnete Kundennummer.  
In aller Regel eine A.eins-Kundennummer. |
| Belegklasse | Eine Dokument-Typisierung gemäß des AMIC-Formates FAKLASSE.  
Die Beleg-Klasse ergibt sich immer aus dem aktuellen Workflow und Entstehung des Archiv-Dokuments. |
| Kontierung | Verarbeitungskennzeichen im Rahmen der Vorkontierung (in Entwicklung) |
| Klassifizierung | Auf Basis des Anwendungsformates AF_FA_KLASSE mögliche individuelle Klassifizierung eines Beleges. |
| Barcode | Archiv Barcode |
| Mandant | Mandant |
| Bedienerklasse | Jedes Dokument ist einer Bedienerklasse zugeordnet.  
Bei Neuanlage ist es die Bedienerklasse des anlegenden Bedieners. Über das Sichtschutz-Konzept des Formulararchivs steuern Sie welche Bedienerklassen welche Dokumente anderer Bedienerklassen-Zuordnungen in den jeweiligen „Archiv anzeigen“-Auflistungen angezeigt bekommen. |
| Datei | Im Hinzufügen-Modus ist hier der Pfad auf das hinzufügende Dokument erfassbar. |
| Archiv/Druckdatum | Zeitpunkt der Archivierung bzw. des Druckes |
| **Info-Felder** | • Name der Drucker-Queue auf dem das Dokument gedruckt wurde.  
• Mime-Typ des Dokuments.  
• Die dem Mime-Typ entsprechende Datei-Extension.  
• Grösse des Dokumentes in Bytes. |
| Belegtyp | Ein freiwählbarer Text, bei bestimmten Abläufen finden sich hier unterstützende, durch die jeweiligen Systeme generierte Hinweise (z.B. beim Vorgangsdruck die Formularbezeichnung) |

| Datentabelle „Belegfluss“ |
| --- |
| angefordert | Anforderungen an Belegfluss Dokumente |
| genehmigt | Archivanforderungsgenehmigungen |

| Register „Allgemein“ |
| --- |
| Betreff | Optionale Zusatzinformation. |
| Kategorie | Optionale Zusatzinformation. |
| Stichwörter | Optionale Zusatzinformation. |
| Kommentar | Optionale Zusatzinformation. |
| Titel | Optionale Zusatzinformation. |
| Autor | Optionale Zusatzinformation. |
| Dateiname | Optionale Zusatzinformation.  
Bei der Angabe dieser Information wird diese beim Anzeigen eines Archiv-Dokumentes (PDF) in der Titelleiste mit angegeben. |
| Register „Gruppe“ |
| Gruppentyp | Formulararchiv-Gruppen |
| Gruppentext | Formulararchiv-Gruppen |
| Gruppennr | Formulararchiv-Gruppen |
| Sortierung | Formulararchiv-Gruppen |

| Funktionen |
| --- |
| Speichern F9 | Speichern |
| Löschen F7 | Löscht den Formulararchiv-Eintrag.  
   
Siehe auch [Archiveinträge löschen](./archiveintraege_loeschen.md) |
| Beleg-Referenz erzeugen F10 | Spezial-Funktion  
Wenn möglich aus der Belegnummer und der Belegklasse unter Zuhilfenahme der [Archiv-Fakte](./archiv_fakte.md) die Belegreferenz des dazu passenden Vorgangs ermitteln. |
| Archiv anzeigen CF12 | Öffnet hinterlegtes Dokument des Archives zur Ansicht. |
