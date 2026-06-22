# Lieblingsdruckerdruck

<!-- source: https://amic.de/hilfe/lieblingsdruckerdruck.htm -->

Diese Funktion ermöglicht es, ausgewählten Vorgängen für den Druck einen anderen als den Standarddrucker zuzuordnen.

Zusätzlich kann zum Ausdruck ein anderes Formular durch Markieren der Unterklasse gewählt werden.

| Felder | Erweiterte Druckoptionen |
| --- | --- |
| Kontonr. | Kontonummer |
| Kunde | Kunde |
| Belegnummer | Belegnummer |
| Datum | Datum |
| Formulare gedruckt | Anzeige laufendendes Formular von Gesamt-Formularen |
| Nr. | Formularnummer |
| Formular | Formularbezeichnung |
| Unter | Vorgangsunterklasse |
| Unterklasse | Vorgangsunterklassenbezeichnung |
| VKNR. | Vorgangsklasse |
| Vorgangsklasse | Vorgangsklassenbezeichnung |
| Makro | Der Name des Makros, welches bei einer Vorgangsdruckklasse hinterlegt ist. |
| Druckernummer | Druckernummer |
| Druckerbezeichnung | Zugehörige Druckerbezeichnung aus Druckerstamm  
In der Spalte danach bedeutet ein „Stern“, das der Drucker der momentane A.eins-Standard-Drucker ist. |
| Nulldrucker | Zugehöriges Nulldrucker-Kennzeichen aus Druckerstamm |
| Archiv unterdrücken | Zugehöriges Archiv unterdrücken-Kennzeichen aus Druckerstamm |
| Senden an | Zugehöriges Senden an-Kennzeichen aus Druckerstamm |
| Druck ohne FIBU-Übertrag | |
| Druck mit FIBU-Übertrag | |
| Einzelbeleg | |
| Archivierung unterdrücken | |
| VRGD Makros ausführen | Hiermit wird festgelegt, ob die Makros ausgeführt werden, die bei den [Vorgangsdruckklassen](../../firmenstamm/druckereinrichtung/vorgangsdruckklassen.md) hinterlegt sind. Die Standardvorbelegung lässt sich in den Steuerparametern ([SPA 907](../../firmenstamm/steuerparameter/allgemeine_programmsteuerung/druckeinstellungen_spa_907.md)) einstellen. |
| Nur drucken | Wenn aktiv werden die Formular-Daten nach VKNR gerafft und es lässt sich ein Formular und ein Drucker auswählen auf welche dann gedruckt wird.  
Es sei ausdrücklich darauf hingewiesen, dass nur ein „auf Papier bringen“ durchgeführt wird, keine Sonderfunktionen.  
Handelt es sich bei dem Drucker um einen „Senden an“ erfolgt eine Druckaufbereitung mittels „PDF-Erzeugung“ die zur Ansicht gebracht wird – es erfolgt aber keine Archivierung. |

| Funktionen | Erweiterte Druckoptionen |
| --- | --- |
| Formulardruck | Startet den Druck |
| Druckerauswahl | Öffnet einen speziellen Dialog zur Drucker-Auswahl |
| Private Druckfunktion anlegen | Bei Ausführung dieser Funktion wird in der Optionbox von der aus dieser „Erweiterte-Druckoptionen“ Dialog aufgerufen wurde automatisch eine private Funktion (im Folgenden kurz PF genannt) angelegt.  
Die Beschriftung der PF ergibt sich aus der gewählten *Formularbezeichnung* und durch Komma getrennt der gewählten *Druckerbezeichnung*.  
Nach technischer Einrichtung der PF durch das Programm erfolgt automatisch die Öffnung des [Rollenkontextpflegers](../../firmenstamm/firmenkonstanten/zuordnung_von_funktionen_zu_bedienerklassen_rollen/rollenkontext/rollenkontext_pfleger.md).  
Der „Controlstring“ der PF ist z.B. ^jpl [lieblingsdrucker](http://www.amic.de/ihilfe/default.htm?turl=XMLDocuments%2FiAeins%2Fhtml%2FM_JPL_JFunktionen_lieblingsdrucker_3_fc05408f.htm) 5 7005.  
   
Die Einstellung von „Nur drucken“ wird sich ebenfalls gemerkt. |

| EPA | Einrichtungsparameter |
| --- | --- |
| Maske nach vollständigem Druck verlassen | Verlässt nach Abschluss des Drucks die Maske |
