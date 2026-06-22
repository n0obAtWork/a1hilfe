# Bediener clonen

<!-- source: https://amic.de/hilfe/_bedienierclonen.htm -->

Diese Funktion erlaubt es, einen Bediener mit allen Daten zu kopieren und nach Eingabe eines neuen eindeutigen Namens, mit diesen kopierten Daten zu erstellen.

Kopfdaten:

| Felder | Beschreibung |
| --- | --- |
| Nehme nächste freie ID ab | Beim Duplizieren eines Bedieners wird diese ID als Vorgabe verwendet.  
Sind mehrere Bediener zum duplizieren ausgewählt, dann wird diese Vorgabe jeweils um 1 erhöht.  
Nach Eingabe wird die Spalte „Clone-ID“ des Grids neu kalkuliert. |
| Clone-Kurzname vorne erweitern um | Der Kurzname eines Bedieners muss eindeutig im System.  
Nach Eingabe wird die Spalte „Clone-Kurzname“ im Grid neu kalkuliert. |
| jede Clone-Bedienerklasse setzen auf | Nach Auswahl einer Bedienerklasse wird diese in die Spalte „C-Bedienerklasse“ übernommen und die Spalte „C-Betriebsstätte“ angepasst, sowie die Spalte „C-Windows Login Name“ neu auf Plausibilität geprüft. |

<details>
<summary>Felder:</summary>

| Felder | Beschreibung |
| --- | --- |
| ID | BedienerID des Vorlage-Bedieners |
| Clone-ID | Clone-BedienerID |
| Kurzname | Kurzname des Vorlage-Bedieners |
| Clone-Kurzname | Clone-Kurzname |
| Bedienerklasse | Bedienerklasse des Vorlage-Bedieners |
| Clone-Bedienerklasse | Clone-Bedienerklasse |
| Betriebsstätte | Betriebsstätte des Vorlage-Bedieners |
| Windows Login Name | Windows Login Name des Vorlage-Bedieners |
| Clone-Windows Login Name | Clone-Windows Login Name |

</details>

<details>
<summary>Funktionen:</summary>

Die Eingaben im Grid werden auf mögliche Konflikte hin geprüft. Eingaben die nicht plausibel sind werden farblich hervorgehoben.

| Funktionen | Beschreibung |
| --- | --- |
| Bediener clonen | Dupliziert nach einer erneuten Plausibilitätsprüfung, die vorgegeben Bediener samt der zugehörigen Detailtabellen. |

</details>
