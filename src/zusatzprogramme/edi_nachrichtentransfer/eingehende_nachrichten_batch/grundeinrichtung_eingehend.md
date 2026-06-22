# Grundeinrichtung (eingehend)

<!-- source: https://amic.de/hilfe/grundeinrichtungeingehend.htm -->

Hier werden die grundlegenden Einstellungen für die Verwendung von Rosi vorgenommen.

Steuerparameter anpassen

Mit dem Steuerparameter 1016 wird festgelegt, mit welchen Vorgangsklassen Rosi benutzt werden soll.

1. Die Anwendung „Steuerparameter“ mit dem Direktsprung [SPA] aufrufen.

2. Den Parameter mit der „SPA_Nr.“ 1016 markieren und die Taste „F5“ betätigen.  
\=> Die Maske zum Ändern des Steuerparameters wird geöffnet.

3. Im Feld „Gültig ab“ den Begriff „heute“ eingeben.

4. Im Feld „EDI-Typ“ die Taste „F3“ drücken und den Eintrag „Rosi“ auswählen.

5. Im Feld „Klasse“ die Zahl ‚400‘ eingeben.

6. Das Feld „ggf. Unterklasse“ bleibt leer.

7. Die Eingaben mit der Taste „F9“ speichern. Die erscheinende Frage „Sollen die Daten übernommen werden?“ mit „Ja“ bestätigen.  
\=> Die Maske zum Ändern des Steuerparameters wird geschlossen.

Allgemeine Rosi-Einrichtungen

Es werden die Ordner für die Archivierung von fehlerbehaften und korrekten eingehenden EDI-Nachrichten eingerichtet.

1. Die Anwendung „Rosi Einrichtung“ mit dem Direktsprung [Rosie] aufrufen.

2. Die Variante „Rosi Konfiguration“ auswählen.

3. Mit der Taste „F8“ die Maske zum Anlegen eines Rosi Konfigurations-Parameters aufrufen.  
\=> Die Maske zum Anlegen des Rosi Konfigurations-Parameters wird geöffnet.

4. Im Feld „Gruppe“ die Taste „F3“ drücken und den Eintrag „Ordner“ auswählen.

5. Im Feld „Parameter“ die Taste „F3“ den Eintrag „ArchivEingehende“ auswählen.

6. Im Feld „Wert“ den Pfad „..\\Import\\Rosi“ eintragen.

7. Die Eingaben mit der Taste „F9“ speichern.  
\=> Datensatz wird gespeichert. Ein neuer Datensatz kann nun eingegeben werden.

8. Im Feld „Gruppe“ die Taste „F3“ drücken und den Eintrag „Ordner“ auswählen.

9. Im Feld „Parameter“ die Taste „F3“ den Eintrag „FehlerEingehende“ auswählen.

10. Im Feld „Wert“ den Pfad „..\\Import\\Rosi“ eintragen.

11. Die Eingaben mit der Taste „F9“ speichern. Anschließend die Maske mit der Taste „ESC“ schließen.

Eingeben der eingenen ILN-Nummer

Damit EDI-Nachrichten importiert werden können ist die Eingabe der eigenen ILN-Nummer notwendig. Es kann sonst keine eingehende EDI-Nachricht zugeordnet werden.

1. Die Anwendung „Mandantenstamm“ mit dem Direktsprung [MND] aufrufen.

2. Den Tabreiter „Allgemein“ auswählen.

3. Im Abschnitt „EDIFact“ im Feld „Sende ILN“ die eigene ILN eintragen.

4. Die Eingabe mit der Taste „F9“ speichern. Anschließend die Maske mit der Taste „ESC“ verlassen.
