# Problemfälle VorgReservierung

<!-- source: https://amic.de/hilfe/_WareoVorgReservierung.htm -->

Hier können Problemfälle in der Vorgangsreservierung korrigiert werden.

| Beschriftung | Funktion |
| --- | --- |
| V_Klassnummer | Hier kann die Nummer der Vorgangsklasse eingegeben werden. |
| V_NumNummer | Hier kann die Belegnummer eingegeben werden. |
| Problemfälle laden! | Sucht fehlerhafte Vorgänge. |
| Mit V_Id = 0 | Sucht fehlerhafte Vorgänge mit V_Id = 0 |
| VorgReserv. Löschen | Löscht den ausgewählten Vorgang aus der VorgReservierung. Das Löschen der VorgReservierung wird in der Relation WareoProtokoll protokolliert. |

Es werden fünf Problemfälle unterschieden:

Typ 1: Vorgang ist komplett mit der richtigen Vorgreservierung verbunden

Typ 2: Eintrag in Vorgreservierung fehlt

Typ 3: VorgangStamm fehlt

Typ 4: Vorgreservierung mit V_Id != 0 nicht in V_Position

Typ 5: Vorgreservierung mit V_Id != 0 nicht in Vorgangstamm

Die Spalte „\*“ zeigt Vorgänge mit gleicher ErfassId. Diese sind wahrscheinlich durch Korrektur hervor gegangen.
