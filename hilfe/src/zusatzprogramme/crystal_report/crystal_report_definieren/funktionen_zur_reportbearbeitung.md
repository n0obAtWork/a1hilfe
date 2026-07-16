# Funktionen zur Reportbearbeitung

<!-- source: https://amic.de/hilfe/funktionenzurreportbearbeitung.htm -->

Hauptmenü > Administration > Werkzeuge > Anwendung Reports > Funktion ***Ändern F5***

Direktsprung **[ANWR]**.

Wenn man einen Report ausgewählt und ihn zum Ändern geöffnet hat, stehen folgende Funktionen zur Verfügung:

| Funktion | Bedeutung |
| --- | --- |
| ***Report aktivieren*** | Nur wenn das Register **Reportdateien** aktiv ist. Sind mehrere Reporte für diese Definition angegeben, so kann man mit dieser Funktion einen dieser Reporte als den aktiven Report definieren. Man muss dazu vorher die Schreibmarke (den Cursor) in der entsprechenden Zeile auf dem Register „Reportdateien“ platziert haben. Es erscheint dann in der ersten Spalte für den angewählten Report ein Stern (\*).<br> |
| ***Speichern*** | Die Reportdefinition wird gespeichert und der nächste Datensatz wird aufgerufen.<br> |
| ***Crystal View edit.*** | Hier kann direkt das angegebene View editiert werden. Es öffnet sich dann der Editor mit der Viewdefinition. Sind mehrere Views eingetragen, so wird entweder das erste View genommen, oder das, auf dem die Schreibmarke (der Cursor) steht.<br> |
| ***Crystal View create*** | Das Programm versucht alle Views neu anzulegen. Eventuelle auftretende Fehler werden angezeigt.<br> |
| ***Zugehöriger Bereich*** | Hier kann man direkt den Bereich, den man unter Auswahlbereich angegeben hat, bearbeiten.<br> |
| ***Verbinden*** | Hier kann man den Aufruf des Reports direkt einer Anwendung / Optionbox oder einem Menü zuordnen. Es wird dabei eine Anwendungsfunktion mit einem Controlstring „jpl aw_list Ident“ erstellt und dem Bereich zugeordnet.<br> |
| ***Report aktualisieren*** | Alle Reporte werden zusätzlich in der Datenbank gespeichert und aus dieser aufgerufen. Die Reporte in der Datenbank werden nur dann aktualisiert, wenn der Report, der sich im rpt-Verzeichnis befindet, jünger ist als der in der Datenbank. Hat man nun eine Änderung im Report vorgenommen, sich dann aber später entschieden, doch den ursprünglichen Report wieder zu verwenden, so wird dieser nicht aktualisiert, da der Report im Verzeichnis ja dann älter ist als der in der Datenbank.<br>Mit dieser Funktion wird das Programm angewiesen den Report – unabhängig vom Datum –beim nächsten Aufruf erneut von der Festplatte zu lesen.<br> |
| ***Export*** | Die Reportdefinition und der in der Datenbank hinterlegte Report werden als SQL-Text ausgelagert, so dass sie z.B. in eine andere Datenbank eingespielt werden können.<br> |
| ***Umbenennen in..*** | Man kann die einmal vergebene Ident ändern.<br> |
| ***Template für*** | Es wird eine Kopie dieses Reports erstellt und unter der neu angegebenen Ident gespeichert.<br> |
