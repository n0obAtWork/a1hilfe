# Wareo

<!-- source: https://amic.de/hilfe/wareo.htm -->

Die Reorganisation kann jetzt einfach per Event gestartet werden.  
Zunächst ist der **Eventname** auf dem Tabreiter **Allgemein** anzugeben. Wird kein Name angegeben, so wird bei der Generierung des Events automatisch der Eventname *Wareo* zugewiesen. Ein Zeitplan wird nur generiert, wenn der Eventname *Wareo* lautet, da unterschiedlich Wareo-Events sich zeitlich nicht überschneiden sollten. Grundsätzlich muss der Zeitplan auf dem Tabreiter **Bedingungen** an die Gegebenheiten angepasst werden.

Auf dem Tabreiter **Vorlagen** können nun zunächst die gewünschten Reorganisationsmaßnahmen ausgewählt werden. Wird keine Auswahl der Wareo-Funktionen getroffen, so wird bei der Erzeugung der Verarbeitungsroutine als auszuführende Wareo-Funktion die <em>Gesamte Reorganisation (Standard)</em> eingesetzt. Die Erzeugung der Verarbeitungsroutine wird mit Betätigung des Buttons **Wareo-Event erzeugen** ausgelöst und generiert einen Prozeduraufruf, dessen Syntax auf dem Tabreiter **Verarbeitungsroutine** dargestellt wird. Der Prozeduraufruf muss noch bezüglich der Parameter **in_user**, **in_passwort** und **in_kill** ergänzt werden. Der Systempfad wird beim Start des Events automatisch aus den im Datenbankserver vorhandenen Informationen generiert, wenn der Parameter **in_systempfad** leer ( ' ' ) ist.

**Funktionsweise eines Wareo-Events:**

Das A.eins Programmstart.vbs Script kopiert die Aeins.exe und benennt diese dann in den Wert des Parameters <strong>in_</strong>exe um. Danach werden alle aktiven A.eins Prozesse auf dem Datenbank Server gestoppt, wenn der Parameter **in_kill** nicht leer ( ' ' ) ist. Erst danach Startet das A.eins Programm mit der ausgewählten Reorganisation.

**Achtung der Parameter Kill killt alle A.eins Prozesse die auf dem Datenbankserver laufen.** **Deswegen wird der Parameter nicht Vorbelegt. Um zu testen, ob der Wareo Aufruf funktioniert, kann dieser unter OSQL getestet werden. Aber beim Test muss der Kill Parameter leer sein.**

**Aufruf der Wareo Prozedur**

```sql
begin

  call
wareo_automatisch(

in_systempfad   = 'c:\\aeins\\bin\\aeins_programmstart.vbs',

in_section      = 'entw',

in_schema       = '',

in_schemadatei  = '',

in_user         = '<User>',

in_passwort     = '<Password>',

in_kill         = '',

in_exe          = 'A1entw',

in_dsn          = 'entw',

in_dauer        = 0,

in_wareo_aufruf = 1)
end
```

| **Parameter Übersicht** | **Bedeutung** |
| --- | --- |
| in_systempfad | Pfad zum VBS Script A.eins Programmstart |
| in_section | Section der Ahoi.ini |
| in_schema | Schema des Datenbank Servers |
| in_schemadatei | Schemadatei des Datenbank Servers |
| in_user | Anmeldungs Benutzer |
| in_passwort | Anmeldungs Passwort |
| in_kill | Name der Exe unter der das Wareo laufen soll |
| in_exe | Name der Exe unter der das Wareo laufen soll |
| in_dsn | DSN Eintrag |
| in_dauer | Parameter Timeout nach dem der Prozess beendet wird |
| in_wareo_aufruf | Welches Wareo soll ausgeführt werden. |

Unterstützte Funktionen für die Warenreorganisation:

| Nummer | Reorganisation Funktion |
| --- | --- |
| 1 | Gesamte Reorganisation (Standard)<br>Es werden die wesentlichen Reorganisationen der Daten durchgeführt:<br>• **Korrekturfelder zurücksetzen**<br>• **Vorgangstapelpositionen bereinigen**<br>• **Bestandsreorganisation und Nachkalkulation**<br>• **Teildisposition reorganisieren**<br>• **Kontraktbestände reorganisieren**<br>• **Partiebestände reorganisieren**<br> |
| 2 | Bestandsreorganisation und Nachkalkulation |
| 3 | Konsistenz Artikel |
| 4 | Korrekturfelder zurücksetzen |
| 5 | Kontraktbestände reorganisieren |
| 6 | Partiebestände reorganisieren |
| 7 | Teildisposition reorganisieren |
| 8 | Wareo-Kombinations-Element:<br>Bestandsreorganisation und Nachkalkulation (ohne vorhergehende Löschung) |
| 16 | Wareo-Kombinations-Element:<br>Bestandsreorganisation und Nachkalkulation mit<br>Löschen der Summen aller Artikel (fakturierte Summen, Lagerplatzsummen, Bestandssummen), ermöglicht die komplette Neuberechnung durch<br>Bestandsreorganisation und Nachkalkulation |
| 32 | Wareo-Kombinations-Element:<br>Teildisposition reorganisieren |
| 64 | Wareo-Kombinations-Element:<br>Kontraktbestände reorganisieren und fehlende Kontraktbewegungen nachtragen |
| 128 | Wareo-Kombinations-Element:<br>Kontrakbewegungen löschen und Kontraktbestände und Bewegungen reorganisieren |
| 256 | Wareo-Kombinations-Element:<br>Partiebestände reorganisieren |
| 512 | Wareo-Kombinations-Element:<br>Kundensummen reorganisieren |
| 1024 | Wareo-Kombinations-Element:<br>Leergut reorganisieren |
| 2048 | Wareo-Kombinations-Element:<br>Baustellen reorganisieren |

Die Funktionen 8 bis 2048 sind per Addition der Funktionswerte kombinierbar. In einem extra dafür vorgesehenem Feld wird der Kombinationswert angezeigt. Bei der Neuanlage eines WAREO-Events können diese durch Aktivierung der entsprechenden Merkmale vor der Eingabe des Wertes ‚*Ja*‘ im Feld **Wareo** ausgewählt werden. Bei der Erzeugung der Verarbeitungsroutine wird der somit ermittelte Wert automatisch berücksichtigt. Bei Änderungen muss der Funktionskombinationswert als Wert des Parameters **in_wareo_aufruf** manuell eingetragen werden.
