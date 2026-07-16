# Artikel / Sorte / Lager

<!-- source: https://amic.de/hilfe/artikelsortelager.htm -->

Zuerst wird versucht, eine Sortennummer zu lesen. Klappt dies, so wird daraus in Abhängigkeit vom Parameter ART_AUS_SORTx eine Artikelnummer per Umsetztabelle ermittelt (ART_AUS_SORT=1) oder auch nicht (ART_AUS_SORT=0). Der Fehlwert des Artikels für die Umsetzung ist im Parameter ART_DEFAULT hinterlegbar. Kann dieser Parameter nicht ausgewertet werden, so ist ART_DEFAULT = „###“, was sicher einen Fehler herbeiführt. ART_AUS_SORTx steht, wenn nicht anders bestimmt defaultmäßig auf 0.

Schlägt das Lesen der Sortennummer jedoch fehl, so wird in jedem Falle versucht, eine Artikelnummer einzulesen.

(Konvertierungsparameter für die Umsetzung von Sorte nach Artikel: SORTARTxx, weitere Parameter: ART_DEFAULT, ART_AUS_SORTx, Positionsparameter: SOR_SAx, ART_SAx)

Anschließend wird die Lagernummer gelesen. Schlägt dies fehl, wird die Lagernummer aus dem 2. Scriptaufrufparameter verwendet. Ist auch diese leer, zieht der Wert aus LAGER_DEFAULT. Ist dieser leer, wird die Lagernummer mit der kleinsten im Artikelstamm vorgefundenen Lagernummer für den betreffenden Artikel belegt. Die Paarung Artikel – Lager wird validiert. Bei einem Fehler wird folgender Satz ins Fehlerprotokoll geschrieben: „ART. [...] o. LG. [...] falsch, Datei [...], Übern. #..., Zl. #...“

Der Satz wird dann nicht importiert.

(Positionsparameter: LG_SAx, weitere Parameter: LAGER_DEFAULT)

Abhängig vom Parameter SORT_AUS_ARTx wird nun aus einer Umsetztabelle aus der Artikelnummer eine Sortennummer ermittelt (SORT_AUS_ARTx=1) oder auch nicht (SORT_AUS_ARTx=0). SORT_AUS_ARTx und ART_AUS_SORTx einer Satzart x können nicht gleichzeitig auf 1 gesetzt werden.

SORT_AUS_ARTx steht, wenn nicht anders bestimmt defaultmäßig auf 0.

Falls SORT_AUS_ART = 0, folgt das Einlesen der Sortennummer. Falls dies nicht klappt, wird die in Parameter SORTE_DEFAULT eingestellte Sorte vorgegeben, die auf 0 steht, wenn nicht anders definiert.

(Parameter: SORT_AUS_ARTx, SORTE_DEFAULT)

Über den Parameter KONV_SORTE schließlich kann eine Konvertierung der Sortennummer von der Waage in eine Aeins-Sortennummer veranlaßt werden (KONV_SORTE=1) oder auch nicht (KONV_SORTE=0). Kann KONV_SORTE nicht ermittelt werden, so ist 0 die Defaulteinstellung. Der Fehlwert der Sorte für die Konvertierung ist SORTE_DEFAULT, die auf 0 steht, wenn nicht anders definiert.

Eine Validierung findet nicht statt.

(Konvertierungsparameter SORTSORTxx, weitere Parameter: KONV_SORTE)
