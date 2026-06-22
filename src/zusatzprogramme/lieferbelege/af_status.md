# af_Status

<!-- source: https://amic.de/hilfe/afstatus.htm -->

Dieses Format sollte gepflegt werden bevor die Anwendung Lieferbeleg genutzt wird.  
Hier legt man fest wie der Status der Lieferbelegpositionen sein kann. Z.B. verloren

Für die Abgrenzung der Auswahlliste nach dem Status mit Hilfe der Funktion Bereich/Profile F2 ist es notwendig das Feld Kommentar,Schnipsel in diesen Format wie folgt zu pflegen:

AND (lbp.lbp_status = Nummer des Formatausdruckes der aktuellen Zeile )

z.B. für die Nummer 1

AND (lbp.lbp_status = 1)
