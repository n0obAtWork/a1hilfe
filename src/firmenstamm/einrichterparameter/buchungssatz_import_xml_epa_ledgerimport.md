# Buchungssatz Import XML (EPA LEDGERIMPORT)

<!-- source: https://amic.de/hilfe/_EPA_LEDGERIMPORT.htm -->

| Bezeichnung | Standardwert | Erklärung |
| --- | --- | --- |
| Script darf geändert werden | Nein | |
| VBA Script welches mir die Datei holt | | Hier kann ein VBA Script angegeben werden, welches mir die XML von einem FTP Server lokal auf meinen Rechner speichert. |
| VBS Script welches mir die Datei holt | | Hier kann ein VBS Script angegeben werden, welches mir die XML von einem FTP Server lokal auf meinen Rechner speichert. |
| Schlüsselklasse des Importumsetzers | | Wenn das Hauptkonto in der XML Struktur Alphanumerisch ist so muss diese noch in eine numerische Kontozahl umgeschlüsselt werden. Hier wird die Schlüsselklasse eingetragen. |
| Dateiprüfung und Einspielung passiert im privaten VBA oder VBS Script | | Mit diesem Parameter kann eingeschaltet werden, ob eine private Behandlung zum Holen, prüfen und einspielen der Dateien in dem privaten Skript vorhanden ist. Wird dies ausgeschaltet, so brauch in das Maskenfeld nur der Speicherort angegeben werden. Existiert keine private Prüfung kann immer nur eine Datei heruntergeladen und verarbeitet werden. |
