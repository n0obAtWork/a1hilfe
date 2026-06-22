# Auswerten des KundSammel-Kennzeichens(SPA 516)

<!-- source: https://amic.de/hilfe/_SPA_516.htm -->

Im Kundenstamm gibt es das Kennzeichen KundsammelKennz. Dieses wird wie folgt ausgewertet:

2 Einzelrechnungen, d.h. für diesen Kunden sind nur die Funktionen Rechnung aus Lieferschein, Rechnung aus Angebot und Rechnung aus Auftrag möglich

1 Sammelrechnung, d.h. für diesen Kunden sind nur die Funktionen Sammelrechnung aus Angebot, Sammelrechnung aus Auftrag und Sammelauftrag aus Lieferschein möglich 

0: alles ist möglich. 

Dieser Steuerparameter steuert, wie dieses Kennzeichen ermittelt wird:

0: es wird nicht ausgewertet

1: aus dem Kunden der Vorgangszuordnung

2: aus dem Kunden des Rechnungsempfängers 

3: aus dem Kunden des Zahlungspflichtigen
