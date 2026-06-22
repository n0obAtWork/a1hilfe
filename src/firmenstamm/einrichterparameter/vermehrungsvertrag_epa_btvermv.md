# Vermehrungsvertrag (EPA BTVERMV)

<!-- source: https://amic.de/hilfe/_EPA_BTVERMV.htm -->

| Bezeichnung | Standardwert | Erklärung |
| --- | --- | --- |
| Teil der Artikelnummer | % | Hier kann ein konstanter Teil der Artikelnummer angegeben werden, wenn alle für Vermehrungsverträge heranzuziehende Artikelnummern diesen enthalten. Beispiele:  
VM% - Artikelnummer beginnt mit \`VM´,  
%VM - Artikelnummer endet mit \`VM´,  
%VM% - Artikelnummer enthält \`VM´ |
| Eingabe Aussaatmonat prüfen, nur im Pfleger vorhandene Aussaattermine zulassen | Ja | |
| Lagerabfrage aktiv | Nein | |
| Vorbelegung Lager als in (.,.,.,...), leer VKONS | | Für die Auswahl der Artikel, Sorten und Kategorien kann eine Liste von Lagernummern angegeben werden. Wird hier nichts eingetragen, so wird die in den Vorgangskonstanten gesetzte Lagernummer herangezogen. |
| oberste Vermehrernummer | 399999 | |
| unterste Vermehrernummer | 300000 | |
| Nur Sorte/Kategorie erlaubt, ohne Artikel | Nein | Nein: Es werden nur Sorten und Kategorien zugelassen, für die bereits Artikel in den unter ‚Vorbelegung Lager als …‘ angegebenen Lägern existieren.  
Ja: Es werden alle Sorten und Kategorien zugelassen, auch wenn noch keine Artikel dafür gefunden werden. Im letzteren Fall bleibt die Artikelnummer leer. Artikel sind dann gegebenenfalls später zu erfassen und zuzuordnen. Solange noch keine Artikelnummer angegeben ist, kann keine Schlagzuordnung vorgenommen werden. |
| Sorte/Kategorie änderbar | Nein | |
| Pro Schlag mehrere Sorten zulassen! | Nein | |
| VO und Aufbereiterfeld aktiv | Nein | |
| Aktuelles Jahr als Erntejahr verwenden, sonst Geschäftsjahr. | Ja | Ist der Parameter auf Ja gesetzt, wird das Erntejahr standardmäßig mit dem aktuellen Jahr vorbelegt (bisheriger Standard).  
Ist der Parameter auf Nein gesetzt, wird das Erntejahr mit dem Geschäftsjahr vorbelegt. |
