# Escapesteuerung im Scanner(SPA 963)

<!-- source: https://amic.de/hilfe/_SPA_963.htm -->

Mit diesem Steuerparameter kann eingestellt werden, ob die Escape Taste das Programm nicht schließt, sondern das der verarbeitenden Prozedur mitgeteilt als String „Escape“ wird, dass die Escape Taste gedrückt wurde. Dies wird nicht im Standard ausgewertet.

| Einstellung | Bedeutung |
| --- | --- |
| Nein | Escape schließt die Maske |
| Ja | Das Programm wird nicht geschlossen, es wird der verarbeitenden Datenbank Prozedur mitgeteilt, dass die Escape Taste gedrückt wurde. Kommt dann eine 999 im Statusfeld zurück wird die Scannersoftware geschlossen. |
