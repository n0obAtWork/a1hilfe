# Kundenkreditlöschkennzeichen

<!-- source: https://amic.de/hilfe/_KundKredLoeKennz.htm -->

Das Kreditlimitlöschkennzeichen hat genau zwei Ausprägungen.

Aktiv: Wert gleich 0 / teilt mit, das es sich um ein aktives Kreditlimit handelt

Nicht aktiv: Wert ungleich 0 / teilt mit, das es sich um ein nicht aktives Kreditlimit handelt

Nur aktive Kreditlimits können zur Berechnung herangezogen werden.

WICHTIG:

*Private Einrichtungen, in denen das Kreditlimit verwendet wird, müssen auf die korrekte Berücksichtigung des Kreditlimitlöschkennzeichens überprüft werden. Hinweise hierzu findet man ggf. auch im Fehlerprotokoll **[FEHLP].** Zur korrekten Verwendung müssen die privaten Einrichtungen um den folgenden Wert erweitert werden:*

*KundKredLoekennz = 0 für aktive Limit*

*KundKredLoekennz != 0 für nicht aktive Limits*
