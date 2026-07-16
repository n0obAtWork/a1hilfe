# Einkaufs-/Verkaufskennzeichen

<!-- source: https://amic.de/hilfe/einkaufsverkaufskennzeichen.htm -->

Das Einkaufs-/Verkaufskennzeichen wird nur bei CEREA-Datensätzen benötigt und hat keine Auswirkung auf Daten anderer Zielansprache. Letzter werden genauer über die Vorgangsklassen definiert.

Entspricht der eingelesene Wert keinem der in den Parametern EK_KENNUNG und VK_KENNUNG vorgegebenen Werte, so wir der in EKVK_DEFAULT eingestellte Wert angenommen. Ist auch dieser nicht auswertbar, so wird „Einkauf“ angenommen.

(Zugehörige Positionsparameter: EV_SAx)

*Lieferscheindatum*

Das Datum wird an späterer Stelle validiert und zuvor durch eine Konvertierung, die mit DATUMFORMAT parametrisiert wird, geschickt. In DATUMFORMAT ist hinterlegt, in welcher Form das Datum vorliegt (Mögliche Werte sind: TT.MM.JJ., TT.MM.JJJJ, JJJJ.MM.TT, JJ.MM.TT, TTMMJJ, TTMMJJJJ, JJMMTT, JJMMTT, JJJJMMTT, auf Groß- und Kleinschreibung kommt es bei diesen Werten nicht an. Stehen in den Importdaten statt der Trennpunkte andere Zeichen, so wird das Datum dennoch richtig erkannt.) Falls das Datumformat nicht explizit über die Scriptparameter angegeben ist, wird von dem Format TT.MM.JJ ausgegangen.

Kann das Lieferscheindatum nicht ermittelt werden, wird das Tagesdatum eingesetzt.

Innerhalb der Datumsvalidierung wird u. U. ein Datenbankfehler angezeigt, wenn sich das Datum nicht lesen lässt. („Cannot convert to a date ...“). Weiteres zu den Validierungen am Ende der Programmschleife.

(Zugehörige Positionsparameter: LD_SAx)
