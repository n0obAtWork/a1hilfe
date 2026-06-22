# RW-Auswertung Excel

<!-- source: https://amic.de/hilfe/rwauswertungexcel.htm -->

Hauptmenü > Rohwarenabrechnung > Excel-Kommunikation > RW-Auswertung bereitstellen

Direktsprung **[RWAUS]**

Mit diesem Aufruf wird die Rohwarenauswertung aufgerufen.  
Zur Erstellung der Auswertung wird zunächst eine Listenfeld-Definition angegeben, die die Spalteninhalte sowie die individuelle Behandlungsweise der Spalten bei Teilsummen- und Endsummenbildung beschreibt. Hierfür steht neben der direkten Eingabe der Definitionsnummer die Auswahlfunktion per **F3**\-Taste zur Verfügung, in der es auch eine Funktion **‚Stammdaten‘** zum Aufruf des Moduls zur Erstellung und Pflege der Listenfeld-Definitionen gibt.  
Ebenfalls angegeben werden muss die Selektions- und Gruppierungs-Definition, die die Reihenfolge der Datenzeilen und die Teilsummenauslösungskriterien angibt sowie die Vorbelegung der Selektionskriterien enthält. Hierfür steht ebenfalls neben der direkten Eingabe der Definitionsnummer die Auswahlfunktion per **F3**\-Taste zur Verfügung, in der es auch eine Funktion **‚Stammdaten‘** zum Aufruf des Moduls zur Erstellung und Pflege der Auswertungs-Sortierungs-Definitionen gibt.  
Die Trennung zwischen Listenfeld-Definition und Selektions- und Gruppierungs-Definition erfolgt, um Kombinationen dieser beiden Möglichkeiten zuzulassen.

Die vorbelegten Angaben zur Selektion können nun wie gewünscht geändert werden.

Im unteren Teil der Maske kann die Art der zu berücksichtigenden Belege durch aktivieren der/des entsprechenden Buttons (![](../../../ImagesExt/image8_409.jpg)nicht ausgewählt, ![](../../../ImagesExt/image8_410.jpg)ausgewählt) ausgewählt werden.

Ferner kann man den Ein- oder Ausschluss von Fremdwarebuchungen (Einlagerung und Vereinnahmung der Einlagerung) festlegen. Hier ist jedoch darauf zu achten, dass es im Falle der Berücksichtigung des Einlagerungs-/Vereinnahmungsstatus in der gewählten Sortierdefinition zu keinen ungewollten Einschränkungen kommt.

Mit der Funktion **‚Starte Aufbereitung …‘** wird das Modul zur Datengewinnung ausgeführt und die Ergebnisse in einer Auswahlliste dargestellt, um diese zum Beispiel mit einem Quickreport weiterzuverarbeiten.  
Die Funktion **‚Starte Aufbereitung Excel direkt‘** startet auch die Datengewinnungsfunktion, übergibt die gewonnenen Daten jedoch incl. der Strukturinformationen (Teilsummenbildungskriterien etc.) an das ELARA-Modul zur Erzeugung einer entsprechenden Darstellung in **MS-Excel**.

<p class="siehe-auch">Siehe auch:</p>

- [Rohware-Auswertung in Auswahlliste](./rohware_auswertung_in_auswahlliste.md)
- [Auswertungs-Listenfeld-Definition](./auswertungs_listenfeld_definition.md)
- [Auswertungs-Sortier-Definition](./auswertungs_sortier_definition.md)
