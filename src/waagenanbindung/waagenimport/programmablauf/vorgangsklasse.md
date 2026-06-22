# Vorgangsklasse

<!-- source: https://amic.de/hilfe/vorgangsklasse.htm -->

Die Vorgangsklasse wird anhand der in den Parametern BELARTKZ_xxx hinterlegten Kennungen ermittelt. Kann die Vorgangsklasse nicht ermittelt werden, zieht die BELART_DEFAULT abgelegte Vorgangsklasse. Ist auch diese nicht auswertbar, wird Lieferschein (v_klassnummer = 600) angenommen. Wurde ein Wert aus den Importdaten gelesen, der jedoch keiner Vorgangsklasse zugeordnet werden kann, oder handelt es sich um einen Rohwarenbeleg , wird die Vorgangsklasse auf 0 gesetzt. Belege, die unberechtigt mit der Vorgangsklasse 0 belegt werden, können später nicht in Vorgänge umgewandelt werden!

Lager- bzw. Lagerplatzumbuchungen werden automatisch unterschieden.

(Zugehörige Positionsparameter: BELART_SAx, weitere Parameter: BELARTKZ_xxx, BELART_DEFAULT)
