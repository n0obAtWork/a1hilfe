# Rohwareparameter & Rohwarewaagenparameter löschen

<!-- source: https://amic.de/hilfe/rohwareparameterrohwarewaagenp.htm -->

Es werden die Daten in folgenden Tabellen gelöscht:

RohWareParamWert unter der Bedingung: where (RohWaPaAbDatum > '01-01-1901') or (Rohwarengruppe > 0) or (RohSorteId > 0)

RWWaagenParamWert unter der Bedingung: where AbDatum > '01-01-1901'
