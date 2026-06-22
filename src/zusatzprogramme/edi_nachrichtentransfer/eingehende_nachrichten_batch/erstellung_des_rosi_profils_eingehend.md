# Erstellung des Rosi-Profils (eingehend)

<!-- source: https://amic.de/hilfe/erstellungdesrosiprofilseingeh.htm -->

Für die Erstellung eines Rosi-Profils sind die folgenden Schritte durchzuführen:

• Nachrichtenprofil anlegen

• Kommunikationsbatch anlegen

• EDI-Partner anlegen

Nachrichtenprofil anlegen

In dem Nachrichtenprofil wird angegeben, in welche Richtung und um welchen Typ von EDI-Nachricht es sich handelt. Es wird eine EDI-Nachricht vom Typ „ORDERS“ verwendet.

1. Die Anwendung „Rosi Einrichtung“ mit dem Direktsprung [ROSIE] aufrufen.

2. Die Variante „Rosi Nachrichtenprofil“ auswählen.

3. Mit der Taste „F8“ die Maske zum Anlegen eines neuen Nachrichtenprofils aufrufen.  
\=> Die Maske zum Anlagen des Nachrichtenprofils wird geöffnet.

4. Die Zahl im Feld „ID“ wird vom Programm automatisch vergeben. Es kann aber auch vom Benutzer eine Zahl eingegeben werden. Wird eine existierende Nummer eingegeben, so führt dies beim Speichern zu einer Fehlermeldung.

5. Im Feld „Bezeichnung“ die Bezeichnung „Rosi ORDERS Test“ eingeben.

6. Im Feld „Richtung“ die Taste „F3“ drücken und die Richtung „eingehend“ auswählen.

7. Im Feld „Typ“ die Taste „F3“ drücken und den EDI-Nachrichtentyp „ORDERS“ auswählen.

8. Im Feld „Mapping-ID“ die Zahl ‚0‘ eintragen.

9. Das Feld „Makroname“ bleibt leer.

10. Die Eingaben mit der Taste „F9“ speichern. Anschließend die Maske mit der Taste „ESC“ schließen.

Kommunikationsbatch anlegen

In dem Kommunikationsbatch wird angegeben: Die Richtung und das Quellverzeichnis der EDI-Nachricht.

1. Die Anwendung „Rosi Einrichtung“ mit dem Direktsprung [ROSIE] aufrufen.

2. Die Variante „Rosi Konfiguration Batch“ auswählen.

3. Mit der Taste „F8“ die Maske zum Anlegen eines neuen Kommunikationsbatches aufrufen.  
\=> Die Maske zum Anlagen des Kommunikationsbatches wird geöffnet.

4. Die Zahl im Feld „ID“ wird vom Programm automatisch vergeben. Es kann aber auch vom Benutzer eine Zahl eingegeben werden. Wird eine existierende Nummer eingegeben, so führt dies beim Speichern zu einer Fehlermeldung.

5. Im Feld „Bezeichnung“ die Bezeichnung „Rosi Komm. Batch Test“ eingeben.

6. Im Feld „Ordner Lokal“ den Pfad „..\\Import\\Rosi-Test“ eingeben.

7. Im Feld „Richtung“ die Taste „F3“ drücken und die Richtung „eingehend“ auswählen.

8. Die Felder „Programm“ und „Programm-Parameter“ bleiben leer.

9. Die Eingaben mit der Taste „F9“ speichern. Anschließend die Maske mit der Taste „ESC“ schließen.

EDI-Partner anlegen

In dem Profil EDI-Partner wird die Teilnehmer-ILN, das Kommunikationsprofil und der EDI-Nachrichtentyp hinterlegt.

1. Die Anwendung „Rosi Einrichtung“ mit dem Direktsprung [ROSIE] aufrufen.

2. Die Variante „Rosi Einrichtung“ auswählen.

3. Mit der Taste „F8“ die Maske zum Anlegen eines neuen EDI-Partners aufrufen.  
\=> Die Maske zum Anlagen des EDI-Partners wird geöffnet.

4. Die Zahl im Feld „ID“ wird vom Programm automatisch vergeben. Es kann aber auch vom Benutzer eine Zahl eingegeben werden. Wird eine existierende Nummer eingegeben, so führt dies beim Speichern zu einer Fehlermeldung.

5. Im Feld „Profilbezeichnung“ die Bezeichnung „ORDERS Test IN“ eingeben.

6. Im Feld „Teilnehmer ILN“ die ILN des Kunden eintragen. Diese Angabe steht im Feld „GLN-Nr.“ im Kundenstamm für den betreffenden Kunden.

7. Im Feld „Komm.-Profil eingehend Typ“ die Taste „F3“ drücken und den Typ „Batch“ auswählen.

8. Im Feld „Komm.-Profil eingehend ID“ die Taste „F3“ drücken und den Eintrag „Rosi Komm. Batch Test“ auswählen.

9. Im Feld „Prefix“ den String „ORDERS_Test_“ eingeben.

10. Im Feld „Nachrichten Profil ID“ die Taste „F3“ drücken und den Eintrag „Rosi ORDERS Test“ auswählen.

11. Die Felder „Komm.-Profil ausgehend Typ“, „Komm.-Profil ausgehend“ und „Nummernkreis …“ werden nicht geändert.

12. Die Eingaben mit der Taste „F9“ speichern. Anschließend die Maske mit der Taste „ESC“ schließen.
