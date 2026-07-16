# Dokumente mehrfach hinzufügen

<!-- source: https://amic.de/hilfe/_dokumentemehrfachhin.htm -->

A.eins bietet die Möglichkeit, mehreren gleichartigen Objekten (nämlich genau denen aus einer Auswahlliste mit zugehöriger Ansichtsdefinition ein Dokument zuzuordnen, welches dann aber nur genau einmal im Archiv existiert, die anderen Einträge sind nur Links/Verweise auf den Ursprung!). Diese Funktionalität kann man sich per privater Funktion mit gleichem Controlstring, erweitert um eine angehängte „1“, einrichten.

Also z.B. ^jpl fa_view AMIC_KUNDE 1 führt nach Auswahl von mehreren Kunden dazu, dass ein Dateiauswahl-Dialog aufgeht, in dem man das zu archivierende Dokument angegeben kann. Danach werden mit den einschlägigen Funktionen jeweilige Umgebungen berechnet und die notwendigen Einträge im Formulararchiv veranlasst.
