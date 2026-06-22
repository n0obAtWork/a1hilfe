# Beispielszenarien für Belegversand

<!-- source: https://amic.de/hilfe/beispielszenarienfrbelegversan.htm -->

| Szenario | Kunde | [VRGD](./index.md)<br>[Belegversand](./index.md) |
| --- | --- | --- |
| Sie möchten keinen Belegversand für den Kunden | Kein Belegversand | Nein |
| Sie möchten eine Rechnung vorab per Mail versenden – der Postversand bleibt wie zuvor<br> | • Mit Rechnungsdruck<br>• Mailadresse in der Hauptanschrift des Rechnungskunden des Beleges hinterlegen<br>• Vorgangsdruckklasse definieren | Ja |
| Sie möchten eine Rechnung per Mail versenden, jedoch den Druck für den Versand nicht durchführen | • Statt Rechnungsdruck<br>• Mailadresse in der Hauptanschrift des Rechnungskunden des Beleges hinterlegen<br>• Vorgangsdruckklasse definieren | Ja |
| Sie möchten eine Rechnung per Mail versenden, jedoch ein Formular mit Briefkopf-Grafik verwenden, weil dies sonst auf dem Druckpapier dargestellt wird | • Statt Rechnungsdruck<br>• Mailadresse in der Hauptanschrift des Rechnungskunden des Beleges hinterlegen<br>• Vorgangsdruckklasse definieren | Eigenes Formular als Exklusiv kennzeichnen |

| Szenario | [[FRZ]](./index.md) | |
| --- | --- | --- |
| Sie möchten Belege zunächst sammeln und später versenden | • Richten Sie die Prozedur AMIC_Belegversand_Ware_Spaeter oder eine private Ableitung davon ein<br>• Richten Sie ein Event zum Versand der Belege ein | Sichten Sie Belege zum Versand unter [MAIL] |
| Sie möchten Belege sofort beim Druck versenden | • Richten Sie die Prozedur AMIC_Belegversand_Ware_Sofort oder eine private Ableitung davon ein | |
