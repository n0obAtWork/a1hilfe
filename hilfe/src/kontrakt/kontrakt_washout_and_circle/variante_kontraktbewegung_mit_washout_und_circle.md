# Variante Kontraktbewegung mit Washout und Circle

<!-- source: https://amic.de/hilfe/_ktrbewewaci.htm -->

Hauptmenü > Kontraktverwaltung > Kontrakt Stammdaten

oder **[KTR]**

In der Variante „Kontraktbewegung mit Washout und Circle“ werden alle Vorgänge angezeigt die an einem Washout oder Circle beteiligt sind.

In dieser Variante kann nach Circle, Washout oder beiden selektiert werden. Des Weiteren ist es möglich, sich die Mengenbuchungen, Wertbuchung oder beide Buchungstypen anzuschauen.

Die Vorgänge eines Washout oder eines Circle sind über eine [Vorgangsklammer](../../vorgangsabwicklung/vorgangsklammer.md) geklammert.

Wenn diese Variante privat abgeleitet werden soll, muss darauf geachtet werden, dass die Funktionen amic_func_bit_test(warenbewegung.wabewbits1, 8) für Washout und amic_func_bit_test(warenbewegung.wabewbits1, 9) für Circle

mit in der „select“ Anweisung berücksichtigt werden.
