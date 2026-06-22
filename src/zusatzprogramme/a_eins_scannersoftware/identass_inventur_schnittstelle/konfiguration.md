# Konfiguration

<!-- source: https://amic.de/hilfe/_identasskonfiguration.htm -->

Scanner:

Um eine Verbindung zu dem Rechner herzustellen, auf dem die PC Software installiert wurde gehen Sie nun wie folgt vor:

Klicken Sie auf dem Desktop des Scanners auf „**MyDevice**“, anschließend auf „**Program Files**“, „**mde**“ und starten das Konfigurationstool über die **MultiLinkIP.exe** Datei.

| Feld | Funktion |
| --- | --- |
| Dropdown-Menü | Auswahl der Verbindungsdaten |
| Großer, leerer Knopf dahinter | Öffnet Eingabe für die Namen oder IP des Rechners, zu dem verbunden werden soll.  
\- Eingeben und anschließend OK klicken |
| : Zahl | Angabe des zu verwendenden Ports 8591 |
| Knopf „Add“ | Die Angaben auf dem großen Knopf und des Ports werden zusammengeführt und in die Liste zur Auswahl im Dropdown-Menü hinzugefügt. |
| Test | Testet die ausgewählte Verbindung. |
| Write | Erzeugt einen Eintrag in den Einstellungen des Scanner-Programms für den ausgewählten Rechner. |
| Quit | Verlassen des Programms |

Nach dem Start des Konfigurationstools klicken Sie auf den großen, leeren Knopf. Ein Eingabebereich wird geöffnet und Sie tragen den Namen des Zielrechners ein. Bestätigen Sie die Eingabe mit Ok rechts unten. Der verwendete Port ist 8591. Mit einem Klick auf den Knopf „Add“ werden die Daten in das Programm übernommen. Nun wählen Sie die neue Verbindung oben aus und klicken anschließend auf den Knopf „Write“ um die Konfiguration abzuschließen.

Die Scanner-IP muss anschließend in die Datei ScannerIP.txt eingetragen werden. Diese befindet sich unter **MyDevice, Application, mde.** Öffnen Sie diese Datei mit dem Editor oder einem ähnlichen Programm, tragen die Scanner-IP ein und speichern Sie die Datei.

Starten Sie das Programm **Multilink.exe** auf Ihrem Rechner aus dem Verzeichnis **Program Files**. Sie sehen nun das Log. Oben befinden sich die Menüs Datei, Einstellungen, Log und Info.

Zum Konfigurieren der Software wählen Sie hier das Menü Einstellungen, PlugIn Einstellungen und GenericDatabase aus.

Die Konfiguration kann bequem über das Userinterface vorgenommen werden und ermöglicht die Konfiguration der Datenbankverbindung und Datenbankbefehlen als Untermenge der Verbindungen.

In der oberen linken Box werden alle bestehenden Datenbankverbindungen (Slotnummer: Name(DB Typ)) aufgelistet. „None“ steht für eine leere, unkonfigurierte Verbindung und wird über den Knopf „Hinzufügen erzeugt“.

Mittig finden sich die Eigenschaften der links ausgewählten Verbindung. Diese lassen sich an dieser Stelle konfigurieren. 

| Feld | Wert |
| --- | --- |
| Connectionstring | |
| ConnectionString | DSN=Name der ODBC-Verbindung  
oder  
einen Connectionstring |
| ConnectionStringParameter | True oder False  
Gibt an, ob Werte im ConnectionString ersetzt werden sollen. Die Werte würden auf  
ScannerSeite der Eigenschaft ConnectionStringParameter angegeben. Die Konvention entsprichtdem des Haispeed.CF20.Utils.Arguments Objekt. (--parameter=Wert) |
| FeldAnfangKenner | { |
| FeldEndKenner | } |
| Konfiguration | |
| IsStatic | True oder False |
| SlotNr | Slotnummer |
| Type | Verbindungstyp: ODBC |
| Zusätzliche Informationen | |
| Bemerkung | Bemerkungstext der Verbindung |
| Name | Name der Verbindung |

Mittig unter den Verbindungseigenschaften, unter Sonstiges können die Datenbankbefehle konfiguriert werden.

| Feld | Wert |
| --- | --- |
| Sonstiges | |
| Befehl | Verschiedene Datenbankbefehle möglich:  
Select \* from IhrFunktionsname('@Artikel',@Menge,'Scanner1') für unsere Inventur-Prozedur (IhrFunktionsname ist entsprechend zu ersetzen!)  
Aktuelles Beispiel:  
Select \* from IdentassScannerInventurTest ( {0},{1},‘{2}‘ )  
{0}: EAN-Code (wird gescannt)  
{1}: Menge  
{2}: ScannerID  
 |
| CommandTimeout | Setzt die Zeit, die gewartet werden soll, bis der Versuch einer Befehlsausführung beendet und ein Fehler generiert wird. -1 lässt den Default-Wert der Datenbankverbindung bestehen. |
| Name | Name, unter dem der SQL-Befehl ausgeführt werden soll. |
| Type | Definiert den Typ des Befehls (z.B. Select) |
| UseDBParameter | Gibt an, ob die Übergebenen Parameter mittels String.Format (False) oder DbParameter übergeben werden sollen. |
| UseTransaction | Gibt an ob vor dem Ausführen des Befehls eine Transaktion gestartet werden soll. |

Über Hinzufügen lassen sich weitere konfigurieren.

Mit Speichern werden die angegebenen Daten übernommen.
