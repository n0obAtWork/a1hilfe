# Umschlüsselungen Excel zu Aeins

<!-- source: https://amic.de/hilfe/umschlsselungenexcelzuaeins.htm -->

| Excel-Datentyp | Aeins-Relationstyp | Aeins-FIELD-Typ | Auswahllisten-Cast (siehe \*) |
| --- | --- | --- | --- |
| String | long varchar | char,30 | cast a char(255) |
| Double | double | N4,12 | |
| Datetime | date | D4,16 | |
| Decimal | numeric(15,4) | N4,12 | |
| Integer | Shortint o. integer | I2, I4, FS_xxxxx | \*\* |

Zu (\*):

Die Anzeige von String-Feldern ist in der Auswahlliste auf 255 Zeichen beschränkt, die Suche erfolgt aber in der kompletten Information der Datenbank-Spalte.

(\*\*)

Boolsche Felder müssen im Zahlenformat 0 (FALSE) oder 1 (TRUE) vorliegen. Eine Konvertierung eines Texts wie „true“ oder „false“ in ein boolsches Feld ist NICHT möglich. Die Darstellung in A.eins erfolgt über FS-Formate
