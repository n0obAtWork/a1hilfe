# Umbuchungen bei Wechsel der Forderungsgruppe

<!-- source: https://amic.de/hilfe/umbuchungenbeiwechselderforder.htm -->

Man hat die Möglichkeit für Personenkonten die Forderungsgruppe zu wechseln. Wenn man dies macht, ergibt sich das Problem, dass auf den „alten“ Forderungs-/Verbindlichkeitskonten Beträge stehen, die aber ab dem Zeitpunkt des Wechsels auf die „neuen“ Forderungs-/Verbindlichkeitskonten gehören würden:

| | Personenkonto | „altes“  
Forderungskonto | „neues“ Forderungskonto |
| --- | --- | --- | --- |
| Eröffnung | 10.000,00 | 10.000,00 | 0,00 |
| Bewegungen laufendes Jahr | 2.000,00 | 2.000,00 | 0,00 |
|   
Ab Periode x des laufenden Jahres wird im Personenkonto eine neue Forderungsgruppe und somit ein neues Forderungskonto eingetragen.  
 |
| Bewegungen ab Periode x | 295,00 | 0,00 | 295,00 |
| Saldo der einzelnen Konten | 12.295,00 | 12.000,00 | 295,00 |
|   
Beim Jahreswechsel wird jedoch, genau wie in den Normalperioden, der Saldo des Forderungskontos aus den Buchungen des Personenkontos in der Abschlussperiode gebildet. Dieser Buchung kann nur **einem** Forderungskonto zugewiesen werden:  
 |
| Jahreswechsel | 12.295,00 | 0 | 12.295,00 |
|   
Das ist auch richtig, denn es fehlt eine Umbuchung vom „alten“ auf das „neue“ Forderungskonto. Würde man diese Umbuchung weglassen, so würden auf dem „alten“ Forderungskonto die Beträge auf alle Ewigkeit stehen bleiben und das „neue“ Forderungskonto hätte irgendwann einen negativen Saldo.  
Also erfolgt bei Jahreswechsel automatisch eine Umbuchung in der letzten Normalperiode. Es wird hier empfohlen – genau wie für den automatischen Abschluss von Unterkonten auf ihre Hauptkonten - eine 13. Normalperiode einzurichten  
 |
| Umbuchung | | \-12.000,00 | 12.000,00 |
| Saldo der einzelnen Konten | 12.295,00 | 12.000,00 | 295,00 |
| Umbuchung + Saldo | 12.295,00 | 0,00 | 12.295,00 |
|   
Soll beim Jahreswechsel diese automatische Umbuchung nicht durchgeführt werden, so kann man mit Hilfe des Steuerungsparameters 968 „Forderungskonten umbuchen“ dieses Verhalten abschalten. |
