# Erm.Kreditlimit mit P_IndivKreditLimit(SPA 594)

<!-- source: https://amic.de/hilfe/_SPA_594.htm -->

Bei „Ja“ wird das Kreditlimit eines Kunden nicht aus dem Kundenstamm sondern durch Aufruf der Datenbankfunktion P_IndivKreditLimit ermittelt. Diese Funktion muss einen Integer-Parameter besitzen, der mit der Kundid gefüllt wird. Als Ergebnis wird eine numeric(15,4) Variable erwartet.
