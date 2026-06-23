# Optionen (F10)

<!-- source: https://amic.de/hilfe/optionenf10.htm -->

Wenn man unter OSQL die Funktion ***Optionen*** **F10** auswählt, so öffnet sich folgender Dialog mit zwei Reitern:

<p class="just-emphasize">Anwendung:</p>

 ![](../../ImagesExt/image8_1456.png)

| | |
| --- | --- |
| (F3) Arbeitsverzeichnis<br>(F3) Dateinamenserweiterung<br>(F3) Zuletzt verwendete Datei | Diese Einstellungen beziehen sich auf die Dialogmaske, die man über die Funktion ***SQL ausführen*** **F3** erreicht. |
| (F9) Arbeitsverzeichnis<br>(F9) Datei<br>(F9) Dateinamenserweiterung | Diese Einstellungen beziehen sich auf die Dialogmaske, die man über die Funktionen ***Sichern Eingabe* SCF9, *Ausführen Statement* CF9** und ***Editieren Statement* SF9** erreicht. |
| Ausgabedatei | Dieser Dateiname wird dort als Vorbelegung verwendet, wo OSQL Daten in eine Datei schreiben soll.<br> |
| Bei TAB Tabellennamen ergänzen<br> | Es wird, wenn man die TAB-Taste drückt, der nächste Tabellenname – bei Shift-TAB der vorherige – ergänzt.<br>Beispiel:<br><code>Select * from Waehr&lt;TAB&gt;</code><br> <br>Ergibt<br><code>Select * from WaehrIsoList</code><br> <br>Beim erneuten drücken von Tab<br><code>Select * from WaehrUmrechInfo</code><br> <br> |
| Dateityp …. | Die Dateitypen, die in den Dateiauswahldialogen angeboten werden sollen. Der Syntax dazu ist wie folgt:<br>("Text", "Suchkriterium")<br>Wobei der Text die Beschreibung enthält, wie z.B. „SQL-Skript (\*.SQL)“. Der gesamte Test müsste folgendermaßen lauten:<br>(SQL-Skript (\*.SQL)“, „\*.sql“)<br> |
| Trotz Fehler Skripte weiter ausführen<br> | Im Normalfall werden SQL-Skripte trotz Fehler weiter ausgeführt. Trägt man hier FALSE ein, so wird die Ausführung des Skripts sofort beendet. |
| Fehlermeldungen<br> | Man kann OSQL dazu bringen, dass keine Fehlermeldungen angezeigt werden. Dazu trägt man hier den Wert FALSE ein. |
| Warnungen anzeigen<br> | Sybase unterscheidet zwischen Fehlern und Warnungen. Warnungen der Datenbank werden von A.eins im Normalfall nicht ausgegeben. Unter OSQL werden Warnungen jedoch angezeigt (TRUE). Stellt man hier FALSE ein, so verhält sich OSQL wie der Rest des Programms.<br> |
| Spaltentitel<br> | TRUE => Es wird immer der gesamte Spaltentitel angezeigt und die Spalte ggf. verbreitert.<br>FALSE => Die Spalte wird in der Standardbreite des Feldes angezeigt. Will man hier den kompletten Spaltentitel sehen, kann man auf den Titel klicken und die Spalte wird dann ggf. verbreitert.<br> |
| Vollbild (ZOOM) | Hiermit wird voreingestellt, ob die Maske den gesamten Bildschirm ausfüllt (TRUE) oder nur einen Teil des Bildschirms(FALSE)<br> |
| Max. Zeilen<br>(0=Bildschirm füllen,<br>\-1=alles Lesen)<br> | Hier existieren zwei Modi:<br>0 Es wird immer nur der Bildschirm gefüllt. Wenn man mehr Daten haben will, so muss man mit den Blättern-Tasten „Bild rauf“ und „Bild runter“ weiterblättern. Man erkennt, ob man alle Daten gelesen hat, an der Statuszeile. Dort steht dann „Gelesene Datensätze 99999“.<br>\-1 Es werden alle Daten geladen. Während der Ladephase kann bereits das nächste Stamement erfasst werden. Sobald eine Funktion aufgerufen wird, bricht das laden ab.<br> |
| Max. Spalten (bis 200). | Anzahl der Spalten die angezeigt werden. Wenn nicht bereits anders eingestellt, wird hier 49 vorgeschlagen.<br> |

<p class="just-emphasize">Datenbank:</p>

Hier werden die Optionen der Datenbank angezeigt. Ein Ändern der Werte ist nicht möglich. Lesen sie dazu die Sybase Dokumentation zu „Database Options“

![](../../ImagesExt/image8_1457.png)
