# Dateien laden

<!-- source: https://amic.de/hilfe/dateienladen.htm -->

Hauptmenü > Mahn-/Zahl-/Zinswesen > Zahlungsverkehr > e-Clearing > Funktion ***Daten laden***

Direktsprung **[ECL]**

Der Dateiname wird über eine Dialogmaske abgefragt. Den Dateiformaten werden feste Dateierweiterung zugeordnet, erkannt "\*.STA" für MT940, "\*.DTI" für das DTA-Format, "\*.C53", "\*.ZIP" oder ungepackt als "\*.XML" für das SEPA CAMT.053-Format und "\*.CSV" für PayPal. Das angegebene Verzeichnis und die ausgewählte Dateierweiterung werden vom Programm gespeichert, so dass beim nächsten Aufruf sofort auf dieses Verzeichnis zugegriffen wird. Die SEPA CAMT Dateien mit der Extension ZIP bzw. C53 werden in einem so genannten ZIP-SEPA Container zusammengefasst. Vor dem Einlesen wird dieser Container automatisch in ein Temporäres-Verzeichnis entpackt und von dort werden dann die Dateien eingespielt.  
Während des Ladens werden diverse Prüfungen auf Richtigkeit der Datei durchgeführt.  
    
DTA-Format: Dateierweiterung "\*.DTI"

• Stimmt die Struktur mit dem geforderten Datenträgerformat überein?

• Stimmt die Anzahl der Datensätze mit denen im Datenträgernachsatz überein?

• Stimmt die Summe der DM-Beträge der Datensätze mit denen im Datenträgernachsatz überein?

• Stimmt die Summe der Kontonummern der Datensätze mit denen im Datenträgernachsatz überein?

• Stimmt die Summe der Bankleitzahlen der Datensätze mit denen im Datenträgernachsatz überein?

• Stimmt die Summe der Euro-Beträge der Datensätze mit denen im Datenträgernachsatz überein?

Für MT940(Swift)-Format: Dateierweiterung "\*.STA"

• Stimmen alle Feldnummern überein?

• Stimmt der Anfangssaldo plus aller Bewegungen mit dem Endsaldo überein?

• Ist die Währung Euro (früher auch DM)?

Für das SEPA CAMT.053-Format: Dateierweiterung "\*.C53", "\*.ZIP" oder ungepackt als XML. Die Auswahl der Dateierweiterung erfolgt im Dateiauswahldialog.

• Stimmen alle Feldnamen überein?

• Ist die Währung Euro?

Für PayPal (CSV-Format): Dateierweiterung "\*.CSV"

• Stimmt der Anfangssaldo plus aller Bewegungen mit dem Endsaldo überein? Diese Überprüfung kann in den Optionen aktiviert/deaktiviert werden (siehe [Zahlungsdienstleister](./optionen.md#Zahlungdienstleister)).

• Anhand des Transaktionscodes wird geprüft, ob eine Transaktion bereits eingespielt wurde. Befindet sich in der Datei mindestens eine Transaktion, die bereits eingespielt wurde, so wird die gesamte Datei abgelehnt.

• Ist die Währung Euro?

Für Freien Import: Die Dateierweiterung wird in den Optionen festgelegt.

• Optional: Stimmt der Anfangssaldo plus aller Bewegungen mit dem Endsaldo überein? Diese Überprüfung kann in den Optionen aktiviert/deaktiviert werden (siehe [Zahlungsdienstleister](./optionen.md#Zahlungdienstleister)).

• Wurde die Auszugsnummer bereits importiert. Nur wenn das Feld „DTADiskAusZug“ von der Datenbankprozedur geliefert wird.

• Ist die Währung Euro? Dieser Test findet nur statt, wenn die Währungsnummer von der Datenbankprozedur geliefert wird.

Schlägt einer dieser Tests fehl, wird die gesamte Datei abgewiesen!

Um ein versehentlich doppeltes Einlesen zu verhindern, wird im DTA-Format geprüft, ob bereits eine Datei mit demselben Erstellungsdatum, derselben Anzahl Datensätze und denselben Summenfeldern existiert. Ist dies der Fall, wird die logische Datei (der Kontoauszug) abgewiesen. Unterscheidet sich die Datei nur im Erstelldatum, wird sie zugelassen, jedoch wird eine Warnmeldung ausgegeben.  
    
Im MT940 (Swift)-Format und im CAMT.053-Format bezieht sich diese Prüfung auf die Bankleitzahl, das Bankkonto, die Auszugnummer, das Buchungsdatum, die Anzahl der Datensätze und den Endsaldo des Kontoauszugs.  
    
Zusätzlich zu diesen maschinellen Prüfungen, kann man per [Option](./optionen.md) **F10** einstellen, dass die Datei direkt nach erfolgreicher Einspielung umbenannt werden soll, so dass sie nicht aus Versehen ein zweites Mal angefasst wird. Die Nummer der Einspielung – erste Spalte in der Auswahlliste – wird zur Bildung des neuen Namens verwendet, so dass Datei und Datenbestand jederzeit überprüfbar sind.  
    
Weiterhin gibt es die Möglichkeit Kontoauszüge, die nicht zu diesem Mandanten gehören, direkt beim Einlesen abweisen zu lassen. Dazu darf das Konto des nicht einzulesenden Mandanten nicht im der Hausbankenstamm hinterlegt sein und die Option „Bei fehlender Hausbank Daten nicht einlesen“ muss auf „Ja“ stehen. Diese Option sollte auf Ja stehen, um auch die Zeiten für die “[Kontenerkennung und automatische Auszifferung](./kontenerkennung_und_automatische_auszifferung.md)“ zu optimieren.
