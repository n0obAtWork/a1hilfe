# Nützliche Controlstrings für Buttons

<!-- source: https://amic.de/hilfe/ntzlichecontrolstringsfrbutton.htm -->

| Funktion | Controlstring |
| --- | --- |
| Blätter an den Anfang der Ergebnistabelle | ^smx_con_exec SDINTERFACE 1 10 |
| Blätter an das Ende der Ergebnistabelle | ^smx_con_exec SDINTERFACE 1 11 |
| Einen Datensatz weiter Blättern | ^smx_con_exec SDINTERFACE 1 9 |
| Einen Datensatz zurück Blättern | ^smx_con_exec SDINTERFACE 1 8 |
| Speichern und nächster Datensatz | ^smx_con_exec SDINTERFACE 1 12 |
| Zwischenspeichern | ^smx_con_exec SDINTERFACE 6 0 |
| Aufruf Konteninformation | ^jpl koi_call :Kontonummer [[[:Jahrnummer] :Bereich] :Kontogesperrt]  
   
Wobei:  
• Wird als **Jahrnummer** 0 übergeben, so wird das aktuelle Jahr verwendet.  
• **Bereich**\= „PK“ bedeutet nur Personenkonten in der Auswahl zulassen.  
• **Kontogesperrt**\=1 bedeutet, dass das Konto nicht geändert werden kann. Es stehen dann auch nicht die Blätterbuttons zur Verfügung. |
| Aufruf OP-Verwaltung | ^jpl opv_call [:Kontonummer [ :Perdatum [:Jahrnummer :Periode] ] ]  
   
Wobei:  
• Wird keine **Kontonummer** angegeben, wird die OP-Verwaltung so aufgerufen, als ob man den Direktsprung **[OPV]** verwendet hätte.  
• **Perdatum** ist das Datum welches sonst bei der OP-Verwaltung beim Erstaufruf abgefragt wird. Wird kein Datum angegeben, so wird es beim Betreten der OP-Verwaltung abgefragt.  
• **Jahrnummer** und **Periode** sind optional und müssen immer zusammen angegeben werden. Sind sie nicht angegeben, wird die zum Datum passende Normalperiode verwendet. |
| Aufruf Kundenstamm | ^jpl ais_kundenstamm :KUNDID [Aendern|Ansehen] |
| Auruf Anschriftstamm | ^jpl ais_anschriften :AdressId [Aendern|Ansehen] |
