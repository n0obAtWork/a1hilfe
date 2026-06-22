# Weitere Anwendungsmöglichkeit Kasse

<!-- source: https://amic.de/hilfe/weitereanwendungsmglichkeitkas.htm -->

Es gab den Wunsch, dass mehrere Kassenarbeitsplätze auf denselben Drucker im Netz drucken sollen.

Um dieses zu realisieren, kann man auf oben beschriebene Vorgehensweise zurückgreifen. Der Drucker, auf den die Arbeitsplätze drucken sollen ist als Windows-Drucker im Netz freizugeben. Für die Barvorgänge muss dieser Drucker durch Eintrag in den Vorgangsdruckklassen (evtl. mit individueller Druckumleitung) angesprochen werden. Für Einzahlungen,... muss in der Kassensystemverwaltung dieser Drucker über seine IP-Adresse und dem Freigabenamen angesprochen werden (siehe oben).

ACHTUNG: Ich bin mir nicht sicher, ob in dieser Konstellation mit mehreren Kassen an einem Drucker der POS-Abverkauf möglich ist (denn hier muss der komplette Beleg hintereinander gedruckt werden, was nur über BVVE sichergestellt ist, damit keine andere Kasse mal eine Zeile „dazwischen druckt“).

Aber das Erfassen an der POS-Kasse mit zeilenweisem Druck funktioniert durchaus (kleine Zeitverzögerung beim Drucken eingerechnet), wenn der Drucker nur einer Kasse zugeordnet ist und obige Einstellungen in den Vorgangsdruckklassen, Druckerstamm und Druckerumleitung gemacht wurden.
