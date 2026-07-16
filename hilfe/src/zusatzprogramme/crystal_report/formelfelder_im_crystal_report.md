# Formelfelder im Crystal Report

<!-- source: https://amic.de/hilfe/formelfelderimcrystalreport.htm -->

A.eins versorgt bestimmte Formelfelder des Reports automatisch mit Daten. Diese sind:

| Formelfeld | Bedeutung |
| --- | --- |
| LABEL1 … n | Wenn die Kommunikation des Auswahlbereichs über Referenzvariablen erfolgt, stehen hier nur die Bezeichnungen der aktiven Abfragen.<br> |
| AUSWAHLVON1…n | Wenn die Kommunikation des Auswahlbereichs über Referenzvariablen erfolgt, stehen hier nur die Von-Eingaben der aktiven Abfragen.<br> |
| AUSWAHLBIS1…n | Wenn die Kommunikation des Auswahlbereichs über Referenzvariablen erfolgt, stehen hier nur die Bis-Eingaben der aktiven Abfragen.<br> |
| VON1…N | Vonwert, wie er eingegeben wurde. Bei FS Formaten der Wert, der unter Schnipsel steht<br> |
| BIS1…N | Biswert.<br> |
| VONWERT1…N | Vonwert, wie er eingegeben wurde. Bei FS-Formaten immer die textliche Darstellung.<br> |
| BISWERT1…N | Biswert.<br> |
| WAEHRUNG | Die Währung, in der der Report dargestellt wird. In der Finanzbuchhaltung gibt es bei diversen Reporten die Möglichkeit das Ergebnis in einer anderen Währung darzustellen.<br> |
| TITEL | Der Titel, wie er in der Reportdefinition hinterlegt ist. Bei Testdatenbanken enthält dieser Titel zusätzlich den Text „[TEST]“<br> |
| ZUSATZTEXT1…10 | Texte aus LDB_CRWZUSATZTEXT. Diese können beliebig in Vorlauffunktionen gefüllt werden.<br> |
| FIRMA | Wenn die Firmenbezeichnung laut CRW-Optionen am Seitenende erscheinen soll dann die Firmenbezeichnung, sonst leer.<br> |
| FIRMAIMKOPF | Wenn die Firmenbezeichnung laut CRW-Optionen im Berichtskopf erscheinen soll dann die Firmenbezeichnung, sonst leer.<br> |
| USER | Je nach Einstellung in den CRW-Optionen entweder den Benutzernamen oder das Benutzerkürzel<br> |
| DISPLAYHEADER | „TRUE“ wenn nur auf der ersten Seite der Berichtskopf angezeigt werden soll<br> |
| SHADOW | „TRUE“ wenn der Berichtskopf und Berichtsfuß grau eingefärbt werden soll.<br> |
| SHADOWLINE | „TRUE“ wenn jede zweite Zeile grau eingefärbt werden soll.<br> |
| BUCHWAEHRUNG | Die aktuelle Buchwährung als Zeichenfolge<br> |
| LOGO | Enthält den Wert 1, wenn unter CRW-Optionen Anzeige des Firmenlogos ausgewählt wurde.<br> |
| FORMATTEXTSOLL | Enthält den Text ‚Soll‘ bzw. den entsprechenden Text in einer anderen Sprache.<br> |
| FORMATTEXTHABEN | Enthält den Text ‚Haben‘ bzw. den entsprechenden Text in einer anderen Sprache.<br> |
