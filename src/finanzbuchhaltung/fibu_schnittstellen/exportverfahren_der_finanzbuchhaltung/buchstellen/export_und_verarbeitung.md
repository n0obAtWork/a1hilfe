# Export und Verarbeitung

<!-- source: https://amic.de/hilfe/exportundverarbeitung.htm -->

Export der Daten

Das Exportieren der Daten erfolgt beim Buchen der Belege. Die Buchungsdaten werden dabei in eine XML-Struktur umgewandelt und mit der XML-Kopfstruktur aus dem „[Buchstellen Firmenstamm](./einrichtung_buchstellen.md)“ zusammengeführt. Danach wird die komplette XML-Datei in den festgelegten Pfad abgelegt.

Von dort aus muss die Datei z.B. per FTP an die Buchstelle übermittelt werden.

Verarbeitung

Nachdem die Daten vom Webservice empfangen wurden, erfolgt eine Prüfung des Passworts und der Sender-E-Mailadresse. Sind die Daten korrekt erfolgt eine weitere Verarbeitung der Daten, die Daten werden entschlüsselt und auf einem Server bereitgestellt.

Dort können die Daten von den entsprechenden Buchstellen abgeholt werden.
