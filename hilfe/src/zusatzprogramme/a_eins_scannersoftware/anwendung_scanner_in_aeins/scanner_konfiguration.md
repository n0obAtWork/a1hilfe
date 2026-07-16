# Scanner Konfiguration

<!-- source: https://amic.de/hilfe/_scanner_konfiguration.htm -->

In dieser Variante wir das Verhalten des Androids-Scanners oder Windows CE lösung eingestellt.

| Maskenfelder | Bedeutung |
| --- | --- |
| IP-Adresse | Zur Identifikation des Scanners kann in der Konfiguration entweder die IP-Adresse oder ein benutzerdefinierter Bezeichner (z. B. Scanner1) eingetragen werden.<br>Sobald der Scanner einen Wert übermittelt, wird dieser zur Erkennung verwendet und die dazugehörige Konfiguration automatisch geladen. |
| Scannerprozedur | In diesem Feld wird die **Prozedur hinterlegt**, die beim **Scannen eines Befehls** automatisch aufgerufen wird.<br>So lassen sich gezielte Aktionen oder Abläufe direkt durch das Scannen auslösen. |
| Stylesheet-Prozedur | In diesem Feld kann eine benutzerdefinierte Stylesheet-Prozedur hinterlegt werden, um die HTML-Anzeige auf dem Scanner individuell zu gestalten und zu steuern. |
| EAN8 Länge 8 akzeptieren | Hier kann festgelegt werden, ob 8-stellige EAN-Codes als EAN-8 erkannt werden sollen.<br>Diese Einstellung ist erforderlich, wenn eigene Scancodes im EAN-128-Format erstellt wurden, die als EAN-8 interpretiert werden sollen. |
| EAN8 Länge 13 akzeptieren | Hier kann festgelegt werden, ob 13-stellige EAN-Codes als EAN-13 erkannt werden sollen.<br>Diese Einstellung ist erforderlich, wenn eigene Scancodes im EAN-128-Format erstellt wurden, die als EAN-13 interpretiert werden sollen. |
| Passwd nach IDLE | Diese Einstellung betrifft ausschließlich Windows CE Scanner.<br>Der Schalter steuert, ob beim Verlassen des Idle-Modus die Passworteingabe für den Bediener erneut erforderlich ist.<br>Dies dient der Sicherung des Zugriffs nach Inaktivität. |
| Max Länge der Menge | In diesem Feld kann eine **Zahl definiert** werden, die angibt, **wie viele Zeichen** bei der **Mengeneingabe über die Tastatur** maximal erlaubt sind.<br>Dies dient zur Begrenzung der Eingabelänge und zur Sicherstellung gültiger Werte. |
| Sound | Hier kann festgelegt werden, **welcher Typ von Sound bei bestimmten Aktionen abgespielt werden darf**. Es stehen folgende Optionen zur Verfügung:<br><ul><li><strong>Datalogic</strong> – Gilt ausschließlich für <strong>Datalogic-Geräte</strong>.</li><li><strong>Andere</strong> – Für <strong>nicht-Datalogic-Geräte</strong> mit <strong>Windows CE</strong>.</li><li><strong>Aus</strong> – <strong>Deaktiviert</strong> die Soundausgabe vollständig.</li><li><strong>Wave</strong> – Ermöglicht das <strong>Abspielen von WAV-Dateien</strong>, die sich im <strong>Anwendungsverzeichnis auf dem Scanner</strong> befinden müssen.<br>&nbsp;</li></ul> |
| Beep nur bei Fehler | Mit dieser Einstellung kann festgelegt werden, ob ausschließlich Fehlerbeeps auf dem Scanner abgespielt werden sollen.<br>Andere akustische Signale (z. B. bei erfolgreichem Scan) werden in diesem Fall unterdrückt. |
| Erfolgs-Sound | Standardsound für erfolgreiche Scannungen |
| Fehler-Sound | Standardsound für fehlerhafte Scannungen |
| ESC-Taste weiterreichen | Diese Einstellung gilt ausschließlich für die Windows CE Lösung.<br>Beim Drücken der ESC-Taste wird das Signal an die Anwendung weitergeleitet und führt nicht zum Schließen der Applikation.<br>So kann die ESC-Taste innerhalb der Anwendung gezielt verwendet werden. |
| HTML anzeigen | Diese Einstellung wird nur in der Windows CE Lösung ausgewertet.<br>Sie bestimmt, ob eine HTML-Seite zur Darstellung der Scanvorgänge angezeigt wird.<br>Dies ermöglicht eine visuelle Rückmeldung direkt auf dem Scannerdisplay. |
| FNC1 übersetzen | In diesem Feld wird das FNC-Zeichen hinterlegt, das zur Trennung von EAN-Codes im EAN128-Standard verwendet wird. |
| ersetzen durch | In diesem Feld kann ein Ersatzzeichen eingetragen werden, das anstelle des FNC1-Zeichens verwendet wird.<br>Diese Funktion wurde eingeführt, um nicht druckbare Zeichen durch ein druckbares Zeichen zu ersetzen, da die Datenbankprozedur mit druckbaren Zeichen besser umgehen kann. |

Für den Android Scanner kann dann noch für einen bestimmten Status das Verhalten des Scanners gesteuert werden.

| Feld | Bedeutung |
| --- | --- |
| Status | Der Scannerstatus aus der Tabelle TCPIP-Scanner |
| Hardwaretastatur | Hier kann eingestellt werden, ob die Hardwaretastatur, wenn vorhanden ist verwendet werden soll. Ansonsten wird die On-Screen Tastatur angezeigt. |
| Eingabefeld Focus | Soll der Focus auf dem Eingabefeld liegen. |
| Tastatur layout | Hier kann eingestellt werden, welches Tastatur Layout für die On-Screentastatur verwendet werden soll. |
