# Beispielszenarien für Belegversand

<!-- source: https://amic.de/hilfe/beispielszenarienfrbelegversan.htm -->

| Szenario | Kunde | [VRGD](./index.md)<br>[Belegversand](./index.md) |
| --- | --- | --- |
| Sie möchten keinen Belegversand für den Kunden | Kein Belegversand | Nein |
| Sie möchten eine Rechnung vorab per Mail versenden – der Postversand bleibt wie zuvor<br> | <ul><li>Mit Rechnungsdruck</li><li>Mailadresse in der Hauptanschrift des Rechnungskunden des Beleges hinterlegen</li><li>Vorgangsdruckklasse definieren</li></ul> | Ja |
| Sie möchten eine Rechnung per Mail versenden, jedoch den Druck für den Versand nicht durchführen | <ul><li>Statt Rechnungsdruck</li><li>Mailadresse in der Hauptanschrift des Rechnungskunden des Beleges hinterlegen</li><li>Vorgangsdruckklasse definieren</li></ul> | Ja |
| Sie möchten eine Rechnung per Mail versenden, jedoch ein Formular mit Briefkopf-Grafik verwenden, weil dies sonst auf dem Druckpapier dargestellt wird | <ul><li>Statt Rechnungsdruck</li><li>Mailadresse in der Hauptanschrift des Rechnungskunden des Beleges hinterlegen</li><li>Vorgangsdruckklasse definieren</li></ul> | Eigenes Formular als Exklusiv kennzeichnen |

| Szenario | [[FRZ]](./index.md) | |
| --- | --- | --- |
| Sie möchten Belege zunächst sammeln und später versenden | <ul><li>Richten Sie die Prozedur AMIC_Belegversand_Ware_Spaeter oder eine private Ableitung davon ein</li><li>Richten Sie ein Event zum Versand der Belege ein</li></ul> | Sichten Sie Belege zum Versand unter [MAIL] |
| Sie möchten Belege sofort beim Druck versenden | <ul><li>Richten Sie die Prozedur AMIC_Belegversand_Ware_Sofort oder eine private Ableitung davon ein</li></ul> | |
