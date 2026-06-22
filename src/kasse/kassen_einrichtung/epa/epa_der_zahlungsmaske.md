# EPA der Zahlungsmaske

<!-- source: https://amic.de/hilfe/epaderzahlungsmaske.htm -->

Auf der Zahlungsmaske ziehen folgende EPAs, die auch für andere Finanzvorgänge ziehen, die über diese Maske abgewickelt werden:

| Auf der Zahlungsmaske ziehen folgende EPAs, die auch für andere Finanzvorgänge ziehen, die über diese Maske abgewickelt werden: |
| --- |
| EPA | Beschreibung |
| Abfrage beim Abschluss der Zahlung? | Ist dieser EPA auf Ja gesetzt, wird beim Validieren des Zahlungsbetrages bei Barvorgängen noch eine Abfrage geschaltet; bei Finanzvorgängen, die auf derselben Maske operieren, erfolgt keine Abfrage, da dort der Vorgang explizit mit F9 erfolgt. |
| Soll Zahlungsart Scheck aktiv sein? | Durch diesen EPA kann die Zahlungsart Scheck deaktiviert werden. |
| Soll Zahlungsart Gutschein aktiv sein? | Durch diesen EPA kann die Zahlungsart Gutschein deaktiviert werden |
| Soll Zahlungsart Kreditkarte aktiv sein? | Durch diesen EPA kann die Zahlungsart Kreditkarte deaktiviert werden. |
| Soll Zahlungsart Bankeinzug aktiv sein? | Durch diesen EPA kann die Zahlungsart Bankeinzug deaktiviert werden. |
| Sollen Einzahlungen,... über Formularsteuerung gedruckt werden? | Ist dieser EPA auf Ja gesetzt, werden bei Finanzvorgängen zusätzliche Formulare 51-55 aus dem Formularwesen auf den in DRZ hinterlegten Drucker gedruckt. Bei der Tresenkasse hat dieser EPA keine Auswirkungen. Ist er auf Nein gesetzt, wird nur ein Standardbeleg über den Finanzvorgang ausgedruckt. |
| Soll ein Scheck gedruckt werden? | Nur wenn dieser EPA auf Ja gesetzt ist, wird auf den in DRZ hinterlegten Drucker bei Zahlungsart Scheck ein Scheckformular mit den erfassten Informationen gedruckt. Dieses Scheckformular ist über den Wert in Kasseneinstellungen, Formularnummer Scheck im Formularwesen unter entsprechendem Wert hinterlegt. Ist hier Nein eingetragen, wird kein Scheck gedruckt. |
| Funktion Schublade öffnen auf Maske laden? | Nur wenn dieser EPA auf Ja gesetzt ist, erscheint die Funktion zum Schublade öffnen auf der Maske. Ansonsten ist die Öffnung der Schublade explizit an Zeitpunkte im Programm gebunden. |
| Soll bei Geldentnahme Steuersatz wählbar sein? | Dieser EPA zieht nur bei Geldentnahmen und lässt dort eine Wahl des Steuersatzes zu (wenn das entsprechende Sachkonto diese Änderung zulässt). |
| Darf Skonto gewährt werden? | Durch diesen EPA kann man bei Barvorgängen die Gewährung von Skonto abschalten. |
| Soll Journal bei Kassenvorgängen gedruckt werden? | Dieser EPA zieht nur bei Finanzvorgängen. |
| Eine Rechnung pro Zahlungsmeldung? | Ist dieser EPA auf Nein gesetzt ist es möglich, beim Finanzvorgang Zahlungsmeldung nacheinander mehrere Zahlungsmeldung zu erfassen, die dann zu einer Meldung zusammengefasst werden und zusammen beglichen werden können. |
| Reduzierte Displayanzeige? | Ist dieser EPA auf Ja gesetzt, erscheint bei gewissen internen Finanzvorgängen keine Displayanzeige. |
| Soll ein Lastschrift-Formular gedruckt werden? | Nur wenn dieser EPA auf Ja gesetzt ist, wird auf den in DRZ hinterlegten Drucker bei Zahlungsart Kreditkarte ein EC-Lastschriftformular mit den erfassten Informationen gedruckt. Dieses Formular ist über den Wert in Kasseneinstellungen, Formularnummer EC-Karte im Formularwesen unter entsprechendem Wert hinterlegt. Ist hier Nein eingetragen, wird kein Lastschriftformular gedruckt. |
| Automatische Steuer bei Entnahmen ? | Ist dieser EPA auf Ja gesetzt, wird bei Geldentnahmen automatisch der Steuersatz aus dem Sachkontenstamm des gewählten Kontos. .übernommen |
