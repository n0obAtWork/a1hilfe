# Container

<!-- source: https://amic.de/hilfe/_container.htm -->

Siehe auch: [Container einrichten](./einrichten_eines_containers.md)

„Die Archivfunktion ist jetzt auf ein Containermodell umgestellt worden. Ein Archiv kann aus beliebig vielen Containern bestehen, die in schreibgeschützter Form zur Anzeige eingebunden sein müssen.“

| | | |
| --- | --- | --- |
| Container | Pflichtfeld | Eindeutiger Name des Containers.<br>Container die keine Datenbank-Relationen im Aeins sind, dürfen nicht den Namen einer eben solchen haben. |
| Abstufung | Pflichtfeld | Die Abstufung impliziert ein Rangsystem und kann z.B. dazu verwendet werden, eine Recherche-Reihenfolge vorzugeben. |
| Status | Information | Im Falle von Datenbank-Relationen wird die momentane Verfügbarkeit ermittelt. |
| Verfügbarkeit | Information | Gibt Auskunft über den Status im Falle einer Datenbank-Relation. |
| Datenbank-Recherche | Optional | Anbindung einer privaten Datenbank-Funktion in der die Container-behandlung abgewickelt wird.<br>Hier ist z.B. frei definierbar, ob und wie die Namen der hinterlegten Container und Abstufungen behandelt werden.<br>Erste Schritte lassen sich mit Hilfe des AMIC-Templates „p_ArchivContainer“ erzielen; dieses wird über die Funktion „recherche-Funktion …“ bei leerem Datenbank-Recherche-Feld einmalig im System initiiert. Das Template ist als Vorschlag und erstes Grundgerüst für private Recherchen zu verstehen.<br> <br>Die Datenbank-Recherche-Funktionen werden von A.eins im Rahmen der „Archiv anzeigen“-Funktionen aufgerufen, sobald A.eins mit internen Mitteln kein Dokument in der Relation Archiv finden kann. Archiv-Dokumente werden anhand ihrer GUID identifiziert. Die Rückgabe der Datenbank-Recherche muss die folgenden Felder enthalten:<br>archiv_status:0 = OK, 1=Information, 2=Error, 3=keine Reaktion<br>archiv_blob: das recherchierte Dokument<br>archiv_message: ggf. User-Information, die via archiv_stati 2,3 zugestellt werden kann. |
