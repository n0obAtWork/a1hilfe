# Grundeinrichtung (ausgehend)

<!-- source: https://amic.de/hilfe/grundeinrichtungausgehend.htm -->

Hier werden die grundlegenden Einstellungen für die Verwendung von Rosi vorgenommen.

<p class="just-emphasize">Steuerparameter anpassen</p>

Mit dem Steuerparameter 1016 wird festgelegt, mit welchen Vorgangsklassen Rosi benutzt werden soll.

1. Die Anwendung „Steuerparameter“ mit dem Direktsprung [SPA] aufrufen.

2. Den Parameter mit der „SPA_Nr.“ 1016 markieren und die Taste „F5“ betätigen.  
\=> Die Maske zum Ändern des Steuerparameters wird geöffnet.

3. Im Feld „Gültig ab“ den Begriff „heute“ eingeben.

4. Im Feld „EDI-Typ“ die Taste „F3“ drücken und den Eintrag „Rosi“ auswählen.

5. Im Feld „Klasse“ die Zahl ‚700‘ eingeben.

6. Das Feld „ggf. Unterklasse“ bleibt leer.

7. Die Eingaben mit der Taste „F9“ speichern. Die erscheinende Frage „Sollen die Daten übernommen werden?“ mit „Ja“ bestätigen.  
\=> Die Maske zum Ändern des Steuerparameters wird geschlossen.

<p class="just-emphasize">Allgemeine Rosi-Einrichtungen</p>

Es werden die Ordner für die Archivierung von fehlerbehaften und korrekten ausgehenden EDI-Nachrichten eingerichtet.

1. Die Anwendung „Rosi Einrichtung“ mit dem Direktsprung [Rosie] aufrufen.

2. Die Variante „Rosi Konfiguration“ auswählen.

3. Mit der Taste „F8“ die Maske zum Anlegen eines Rosi Konfigurations-Parameters aufrufen.  
\=> Die Maske zum Anlegen des Rosi Konfigurations-Parameters wird geöffnet.

4. Im Feld „Gruppe“ die Taste „F3“ drücken und den Eintrag „Ordner“ auswählen.

5. Im Feld „Parameter“ die Taste „F3“ den Eintrag „ArchivAusgehende“ auswählen.

6. Im Feld „Wert“ den Pfad „..\\Export\\Rosi“ eintragen.

7. Die Eingaben mit der Taste „F9“ speichern.  
\=> Datensatz wird gespeichert. Ein neuer Datensatz kann nun eingegeben werden.

8. Im Feld „Gruppe“ die Taste „F3“ drücken und den Eintrag „Ordner“ auswählen.

9. Im Feld „Parameter“ die Taste „F3“ den Eintrag „FehlerAusgehende“ auswählen.

10. Im Feld „Wert“ den Pfad „..\\Export\\Rosi“ eintragen.

11. Die Eingaben mit der Taste „F9“ speichern. Anschließend die Maske mit der Taste „ESC“ schließen.

<p class="just-emphasize">Nummernkreis einrichten</p>

Für die EDI-Nachrichten und Dateien werden eindeutige Zahlen benötigt. Diese werden mit Hilfe eines eigenen Nummerkreises erzeugt. Für dieses Beispiel wird nur ein Nummernkreis verwendet.

1. Die Anwendung „Logische Nummernkreise“ mit dem Direktsprung [NKS] aufrufen.

2. Mit der Taste „F8“ die Maske zum Anlegen eines neuen Nachrichtenprofils aufrufen.  
\=> Die Maske zum Anlegen des Nummernkreises wird geöffnet.

3. Im Feld „Nummernkreis“ die Zahl ‚600‘ eingeben.

4. Im Nachbarfeld von „Nummernkreis“ die Bezeichnung „Rosi NK“ eingeben.

5. In den anderen Felden die Taste „F3“ drücken und den Eintrag „Nein“ auswählen.

6. Die Eingaben mit der Taste „F9“ speichern. Anschließend die Maske mit der Taste „ESC“ schließen.

7. Nummernkreis „600“ markieren und mit Taste „F5“ zum Bearbeiten öffnen.

8. Die Funktion „Neuer Gültigkeitszeitraum“ (oder Tastenkürzel „Shift + F8“) auswählen.  
\=> Die Maske „Zuordnungen NumKreise/Zählkreise“ wird geöffnet.

9. Im Feld „Nummerkreis“ die Zahl ‚600‘ eingeben.

10. Im Feld „gültig ab“ das Wort „heute“ eingeben.

11. Im Feld „mit Zählkreis“ die Zahl ‚1‘ eingeben.

12. Den Wert im Feld „bis“ nicht ändern.

13. Die Eingaben mit der Taste „F9“ speichern. Anschließend die Maske mit der Taste „ESC“ schließen.

14. Die Maske „Logische Nummernkreise“ mit der Taste „ESC“ schließen.
