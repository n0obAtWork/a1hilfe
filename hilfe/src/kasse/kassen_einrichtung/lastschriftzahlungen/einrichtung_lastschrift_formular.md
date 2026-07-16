# Einrichtung Lastschrift-Formular

<!-- source: https://amic.de/hilfe/einrichtunglastschriftformular.htm -->

Damit diese Funktionalität genutzt werden kann, ist folgendes einzurichten:

1. Im Formularwesen muss ein Lastschrift-Formular eingerichtet werden, das dann später z.B. auf dem Schacht des Druckers gedruckt werden soll. Dabei stehen folgende Druckpositionen zur Verfügung:

| Variablenname | Druckposition | Druckbereich | Bedeutung |
| --- | --- | --- | --- |
| EC_Firma | 3 Textvariable | 950 Hauptteil EC_Lastschrift | Mandanten / Firmenname |
| EC_Betrag | 4 Zahlvariable | 950 Hauptteil EC_Lastschrift | Betrag der Lastschrift in erfasster Währung |
| EC_Waehrung | 3 TextVariable | 950 Hauptteil EC_Lastschrift | Währungskürzel, in der Lastschrift erfasst wurde |
| EC_Datum | 11 Tagesdatum | 950 Hauptteil EC_Lastschrift | Tagesdatum, an dem Lastschrift erfasst wurde |
| EC_Zeit | 3 Textvariable | 950 Hauptteil EC_Lastschrift | Uhrzeit, an der Lastschrift erfasst wurde |
| EC_KartNr | 3 Textvariable | 950 Hauptteil EC_Lastschrift | Kartennummer der EC_Karte |
| EC_KontoNr | 3 Textvariable | 950 Hauptteil EC_Lastschrift | Kontonummer der EC_Karte |
| EC_BLZ | 3 Textvariable | 950 Hauptteil EC_Lastschrift | Bankleitzahl der EC_Karte |
| EC_GueltigBis | 3 Textvariable | 950 Hauptteil EC_Lastschrift | Gültigkeitsdatum der EC_Karte |
| EC_BelegNr | 4 Zahlvariable | 950 Hauptteil EC_Lastschrift | Referenz auf die Belegnummer, bei dem mit Karte bezahlt wurde |
| EC_BonNr | 4 Zahlvariable | 950 Hauptteil EC_Lastschrift | Laufende Ident-Nummer des Zahlungsmittelsatzes |
| EC_Kasse | 4 Zahlvariable | 950 Hauptteil EC_Lastschrift | Nummer der Kasse, an der mit dieser Karte gezahlt wurde |

2. Auf der Zahlungsmaske / Maske der POS-Kasse muss der EPA „Soll ein Lastschrift-Formular gedruckt werden“ auf Ja gesetzt werden. Dann wird das unter 1. Angelegte Formular auf den in DRZ zugeordneten Drucker gedruckt.

3. In den Kasseneinstellungen kann man der Kasse unter der OptiGruppe Formular mit der Nummer 2 das zu druckende Lastschrift-Formular zuordnen, das gedruckt werden soll.

4. In den Optionalen Parametern [OPT] muss unter ECDTABANKNUMMER die Nummer der Hausbank eingetragen werden, an den Datenträgr geschickt werden soll.
