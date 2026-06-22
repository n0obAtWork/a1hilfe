# Optionen (F10)

<!-- source: https://amic.de/hilfe/optionenf10.htm -->

Wenn man unter OSQL die Funktion ***Optionen*** **F10** auswählt, so öffnet sich folgender Dialog mit zwei Reitern:

Anwendung:

 ![](../../ImagesExt/image8_1456.png)

| | |
| --- | --- |
| (F3) Arbeitsverzeichnis  
(F3) Dateinamenserweiterung  
(F3) Zuletzt verwendete Datei | Diese Einstellungen beziehen sich auf die Dialogmaske, die man über die Funktion ***SQL ausführen*** **F3** erreicht. |
| (F9) Arbeitsverzeichnis  
(F9) Datei  
(F9) Dateinamenserweiterung | Diese Einstellungen beziehen sich auf die Dialogmaske, die man über die Funktionen ***Sichern Eingabe*** **SCF9,** ***Ausführen Statement*** **CF9** und ***Editieren Statement*** **SF9** erreicht. |
| Ausgabedatei | Dieser Dateiname wird dort als Vorbelegung verwendet, wo OSQL Daten in eine Datei schreiben soll.  
 |
| Bei TAB Tabellennamen ergänzen  
 | Es wird, wenn man die TAB-Taste drückt, der nächste Tabellenname – bei Shift-TAB der vorherige – ergänzt.  
Beispiel:  
   
Ergibt  
   
Beim erneuten drücken von Tab  
   
 |
| Dateityp …. | Die Dateitypen, die in den Dateiauswahldialogen angeboten werden sollen. Der Syntax dazu ist wie folgt:  
("Text", "Suchkriterium")  
Wobei der Text die Beschreibung enthält, wie z.B. „SQL-Skript (\*.SQL)“. Der gesamte Test müsste folgendermaßen lauten:  
(SQL-Skript (\*.SQL)“, „\*.sql“)  
 |
| Trotz Fehler Skripte weiter ausführen  
 | Im Normalfall werden SQL-Skripte trotz Fehler weiter ausgeführt. Trägt man hier FALSE ein, so wird die Ausführung des Skripts sofort beendet. |
| Fehlermeldungen  
 | Man kann OSQL dazu bringen, dass keine Fehlermeldungen angezeigt werden. Dazu trägt man hier den Wert FALSE ein. |
| Warnungen anzeigen  
 | Sybase unterscheidet zwischen Fehlern und Warnungen. Warnungen der Datenbank werden von A.eins im Normalfall nicht ausgegeben. Unter OSQL werden Warnungen jedoch angezeigt (TRUE). Stellt man hier FALSE ein, so verhält sich OSQL wie der Rest des Programms.  
 |
| Spaltentitel  
 | TRUE => Es wird immer der gesamte Spaltentitel angezeigt und die Spalte ggf. verbreitert.  
FALSE => Die Spalte wird in der Standardbreite des Feldes angezeigt. Will man hier den kompletten Spaltentitel sehen, kann man auf den Titel klicken und die Spalte wird dann ggf. verbreitert.  
 |
| Vollbild (ZOOM) | Hiermit wird voreingestellt, ob die Maske den gesamten Bildschirm ausfüllt (TRUE) oder nur einen Teil des Bildschirms(FALSE)  
 |
| Max. Zeilen  
(0=Bildschirm füllen,  
\-1=alles Lesen)  
 | Hier existieren zwei Modi:  
0 Es wird immer nur der Bildschirm gefüllt. Wenn man mehr Daten haben will, so muss man mit den Blättern-Tasten „Bild rauf“ und „Bild runter“ weiterblättern. Man erkennt, ob man alle Daten gelesen hat, an der Statuszeile. Dort steht dann „Gelesene Datensätze 99999“.  
\-1 Es werden alle Daten geladen. Während der Ladephase kann bereits das nächste Stamement erfasst werden. Sobald eine Funktion aufgerufen wird, bricht das laden ab.  
 |
| Max. Spalten (bis 200). | Anzahl der Spalten die angezeigt werden. Wenn nicht bereits anders eingestellt, wird hier 49 vorgeschlagen.  
 |

Datenbank:

Hier werden die Optionen der Datenbank angezeigt. Ein Ändern der Werte ist nicht möglich. Lesen sie dazu die Sybase Dokumentation zu „Database Options“

![](../../ImagesExt/image8_1457.png)
