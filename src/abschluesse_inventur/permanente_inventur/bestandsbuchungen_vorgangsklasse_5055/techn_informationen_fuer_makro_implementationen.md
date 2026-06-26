# techn. Informationen für Makro-Implementationen

<!-- source: https://amic.de/hilfe/techninformationenfrmakroimple.htm -->

Folgende ID\`s können im Rahmen einer Vorgangserzeugung gesetzt werden, um den Ausbuchpreis zu setzen.

| Wert | ID | Bedeutung |
| --- | --- | --- |
| 566 | ID_AUSBUCH_PREIS | Preis mit dem die Ware ausgebucht wird. |
| 567 | ID_AUSBUCH_PREISEINHEIT | Preiseinheit mit der die Ware ausgebucht wird. |
| 568 | ID_AUSBUCH_ME | Mengeneinheit des Preises mit dem die Ware ausgebucht wird. |
| 571 | ID_INVENTUR_PREISTYP | Preistyp des Ausbuchpreises als Kennzeichen (s.u.) |
| 572 | ID_VORGLINKID | Gibt die Link-ID des Vorgangs aus. |
| 573 | ID_SETLINKID | Setzt die Link-ID des verlinkten Vorgangs als Linktyp |

Mögliche Werte für einen Preistyp

| Wert | ID | Bedeutung |
| --- | --- | --- |
| 0 | INVENTUR_PREISTYP_UNBEPREIST | Unbepreist – Hier liegt kein Ausbuchpreis vor. |
| 1 | INVENTUR_PREISTYP_MANUELL | Manuell erfasster Ausbuchpreis |
| 5 | INVENTUR_PREISTYP_AUTO_TEMP | Hier wurde der zum Zeitpunkt der Erstellung gültige Bewertungspreis der Ware temporär festgelegt. |
| 10 | INVENTUR_PREISTYP_AUTOMATIK | Dieser Preis wurde automatisch eingefügt. |

Mögliche Werte für den Linktyp

| Wert | ID | Bedeutung |
| --- | --- | --- |
| 0 | VORGLINKTYP_NIX | Kein gültiger Linktyp |
| 1 | VORGLINKTYP_PROD_INVENTUR | Hier werden Produktionen und Inventuren verknüpft. Diese Art der Verknüpfung bedeutet:<br><ul><li>Die Stornierung einer der beiden Belege verursacht automatisch die Stornierung des anderen.</li><li>Die Korrektur beider Belege ist jeweils gesperrt.</li></ul> |

### Verknüpfung Produktion und Inventur

Es ist möglich, Produktionsbelege mit Inventuren zu verknüpfen.

Der Hintergrund ist der, dass ein Teil der aufgefundenen Ware zu einem anderen (meist minderwertigen) Verwendungszweck umdeklariert werden soll (Produktion) und der Rest der Ware nunmehr inventarisiert werden soll.

Dazu ist vom zuerst erstellten Beleg die eigene LinkID festzustellen und beim zweiten Beleg mit der entsprechenden ID und den Linktyp 1 einzutragen.
