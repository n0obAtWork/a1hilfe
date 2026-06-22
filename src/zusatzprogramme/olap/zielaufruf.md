# Zielaufruf

<!-- source: https://amic.de/hilfe/zielaufruf.htm -->

Der gezielte Aufruf einer Ansicht kann vom Support vorgenommen werden. Dies ist eine sehr technische Einrichtung, die hier beschrieben werden soll:

An die JPL namens OLAP.j werden folgende Parameter übergeben:

| JVAR 1975 |
| --- |
| COMMAND | Anzeigekommando<br>• Sollen die Daten nur angezeigt werden ohne Designer, dann wird hier „SHOW“ angegeben<br>Soll die Auswertung automatisiert gedruckt werden wird hier „PRINT“ angegeben |
| ANWENDUNG | Die Anwendung, als der die Daten kommen sollen |
| VARIANTE | Die Variante aus der die Daten kommen sollen |
| PROFIL | Das Profil aus dem der Filter kommen soll |
| TITEL | Der anzuzeigende Titel (Default leer) |
| PRINTER | Drucker, auf dem die Auswertung gedruckt werden soll (COMMAND==PRINT) |
| PRINTAREA | Hier gibt es drei Bereiche:<br>• RAW – die Rohdaten der Anwendung<br>• CHART – die grafische Auswertung des Charts<br>• PIVOT – die Pivottabelle |
