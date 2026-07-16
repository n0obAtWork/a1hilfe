# Formulare in der Basis-DB

<!-- source: https://amic.de/hilfe/formulareinderbasisdb.htm -->

| Formular | Bezeichnung | Formulartyp |
| --- | --- | --- |
| 100 | Angebot Druck | Standardvorgang |
| 101 | Angebot Vorschau | Standardvorgang |
| 102 | Angebot Erfassung | Standardvorgang |
| 111 | Formular für Itembox-Druck | Listen |
| 112 | Formular ITEM-Box HP-Drucker | Listen |
| 190 | Storno Angebot Druck | Standardvorgang |
| 191 | Storno Angebot Vors. | Standardvorgang |
| 192 | Storno Angebot Erf. | Standardvorgang |
| 400 | Auftrag Druck | Standardvorgang |
| 401 | Auftrag Vorschau | Standardvorgang |
| 402 | Auftrag Erfassung | Standardvorgang |
| 490 | Storno Auftrag Druck | Standardvorgang |
| 491 | Storno Auftrag Vorschau | Standardvorgang |
| 492 | Storno Auftrag Erfassung | Standardvorgang |
| 600 | Lieferschein Druck | Standardvorgang |
| 601 | Lieferschein Vorschau | Standardvorgang |
| 602 | Lieferschein Erfassung | Standardvorgang |
| 690 | Storno Lieferschein Druck | Standardvorgang |
| 691 | Storno Lieferschein Vorschau | Standardvorgang |
| 692 | Storno Lieferschein Erfassung | Standardvorgang |
| 700 | Rechnung Druck | Standardvorgang |
| 701 | Rechnung Vorschau | Standardvorgang |
| 702 | Rechnung Erfassung | Standardvorgang |

Im Programmteil ***Formularzuordnung* [FRZ]** werden die WaWi - Formulare an

Vorgänge gebunden.

FIBU - Formulare werden entweder direkt vor Druck abgefragt

(Buchungsjournal, Kontenblatt) oder als Konstanten des jeweiligen Programmteils

(Zahlungsverkehr, Mahnen) verwaltet.

| Vorg.-Klasse | Ukl-Nr | U-Klasse | Druck | Vorschau | Bildschirm |
| --- | --- | --- | --- | --- | --- |
| Angebot | 0 | Angebot | 100 | 101 | 102 |
| Angebot | 1 | Bestätigung | 400 | 401 | 402 |
| Angebot | 9998 | Ordersatz | 100 | 101 | 102 |
| Storno Angebot | 0 | Storno Angebot | 190 | 191 | 192 |
| Auftrag | 0 | Auftrag | 400 | 401 | 402 |
| Auftrag | 9998 | Objekt | 400 | 401 | 402 |
| Storno Auftrag | 0 | Storno Auftrag | 490 | 491 | 492 |
| Ladeschein | 0 | VK-Ladeschein | 600 | 601 | 602 |
| Lieferschein | 0 | VK-Lieferschein | 600 | 601 | 602 |
| Lieferschein | 1 | Tresen-Lieferschein | 600 | 601 | 602 |
| Storno Lieferschein | 0 | Storno Lieferschein | 690 | 691 | 692 |
| Rechnung | 0 | VK-Rechnung | 750 | 701 | 702 |
| Rechnung | 9900 | Barverkauf | 710 | 711 | 712 |
| Storno Rechnung | 0 | Storno Rechnung | 790 | 791 | 792 |
| Gutschrift | 0 | VK-Gutschrift | 800 | 801 | 802 |
| Gutschrift | 9900 | Gutschrift Barverkauf | 810 | 811 | 812 |
| Storno Gutschrift | 0 | Storno Gutschrift | 890 | 891 | 892 |
| Bestellanfrage | 0 | Bestellanfrage | 1100 | 1101 | 1102 |
| Bestellanfrage | 9998 | Ordersatz Einkauf | 1400 | 1401 | 1402 |
| Storno Bestellanfrage | 0 | Storno Bestellanfrage | 1490 | 1491 | 1492 |
| Bestellung | 0 | Bestellung | 1400 | 1401 | 1402 |
| Storno Bestellung | 0 | Storno Bestellung | 1490 | 1491 | 1492 |
| Eingangsladeschein | 0 | EK-Ladeschein | 1600 | 1601 | 1602 |
| Eingangslieferschein | 0 | EK-Lieferschein | 1600 | 1601 | 1602 |
| Eingangslieferschein | 9999 | Rohwarenanlieferung | 903 | 903 | 1602 |
| Storno Eingangslieferschein | 0 | Storno Eingangslieferschein | 1690 | 1691 | 1692 |
| Eingangsrechnung | 0 | ER Ware | 1700 | 1701 | 1702 |
| Eingangsrechnung | 1 | Getreide Gutschrift | 1700 | 1701 | 1702 |
| Eingangsrechnung | 9900 | Bareinkauf | 1710 | 1711 | 1712 |
| Eingangsrechnung | 9999 | Rohwarenanlieferung | 903 | 903 | 1702 |
| Storno Eingangsrechnung | 0 | Storno Eingangsrechnung | 1790 | 1791 | 1792 |
| Eingangsgutschrift | 0 | EK-Gutschrift | 1800 | 1801 | 1802 |
| Eingangsgutschrift | 9900 | Bareinkauf Gutschrift | 1810 | 1811 | 1812 |
| Storno Eingangsgutschrift | 0 | Storno Eingangsgutschrift | 1890 | 1891 | 1892 |
| Lagerplatzumbuchung | 0 | Lagerplatzumbuchung | 5100 | 5101 | 5102 |
| Lagerumbuchung | 0 | Lagerumbuchung | 5110 | 5111 | 5112 |
| Artikelumbuchung | 0 | Artikelumbuchung | 5120 | 5121 | 5122 |
