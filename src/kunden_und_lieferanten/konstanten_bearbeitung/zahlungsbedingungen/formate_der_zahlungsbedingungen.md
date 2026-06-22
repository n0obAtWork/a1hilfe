# Formate der Zahlungsbedingungen

<!-- source: https://amic.de/hilfe/_formatederzahlungsbe.htm -->

Hier finden Sie die Formate und dazugehörigen Beschreibungen, welche in der Zahlungsbedingung verwendet werden.

[Typ](./formate_der_zahlungsbedingungen.md#zabed_format_typ)

[Bezug](./formate_der_zahlungsbedingungen.md#zabed_format_bezug)

[Automatisch aufblenden](./formate_der_zahlungsbedingungen.md#zabed_format_auto_aufblenden)

[Formel](./formate_der_zahlungsbedingungen.md#zabed_format_formel)

[Valutabestimmung](./formate_der_zahlungsbedingungen.md#zabed_format_valutabestimmung)

<p class="just-emphasize">Typ</p>

Mit diesem Format wird der Typ einer Zahlungsbedingung festgelegt. *(Formatname „ZBEDTYP“)*

| Nr. | Bezeichnung | Beschreibung |
| --- | --- | --- |
| 1 | Fälligkeit in n Tagen | Fälligkeit der Zahlung beginnt nach der Zeitspanne von n Tagen abhängig vom Bezugsdatum |
| 2 | Fällig am Tag X des Folgemonats (Skto auf Bezug) | Fälligkeit der Zahlung zum genauen Stichtag im Folgemonat abhängig vom Bezugsdatum  
Skonto berechnet sich in Tagen nach dem Belegdatum |
| 3 | Fällig am Tag X dieses Monats (Skto auf Bezug) | Fälligkeit der Zahlung am Stichtag des laufenden Monats abhängig vom Bezugsdatum  
Skonto berechnet sich in Tagen nach dem Belegdatum |
| 4 | Fällig zum nächstmöglichen Tag X (Skto auf Bezug) | Wenn das Bezugsdatum der Fälligkeit **vor** dem Tag **X** des Monats liegt, wird der Betrag fällig am Tag **X** des **laufenden Monats**.  
Wenn das Bezugsdatum der Fälligkeit **nach** dem Tag **X** liegt, wird der Betrag fällig am Tag **X** des **nächsten Monats**.  
Skonto berechnet sich in Tagen nach dem Belegdatum |
| 5 | Fällig Monatsende Folgemonat | Fälligkeit der Zahlung zum Monatsende des Folgemonats abhängig vom Bezugsdatum |
| 6 | Fällig Monatsende aktueller Monat | Fälligkeit der Zahlung zum Monatsende des aktuellen Monats |
| 7 | Datum manuell eingebbar | Manuelles Fälligkeitsdatum |
| 8 | Fällig am Tag X der nächsten Woche (Skto auf Bezug) | Fälligkeit der Zahlung am Wochentag X in der nächsten Kalenderwoche  
Skonto berechnet sich in Tagen nach dem Belegdatum  
Als Wochentag wird hier Sonntag= 1, Montag = 2 usw. gerechnet |
| 9 | Fällig am Tag X des Folgemonats (Skto auf Fälligkeit) | Fälligkeit der Zahlung zum genauen Stichtag im Folgemonat abhängig vom Bezugsdatum  
Skonto berechnet sich in Tagen vor dem Fälligkeitsdatum |
| 10 | Fällig am Tag X dieses Monats (Skto auf Fälligkeit) | Fälligkeit der Zahlung am Stichtag des laufenden Monats abhängig vom Bezugsdatum  
Skonto berechnet sich in Tagen vor dem Fälligkeitsdatum |
| 11 | Fällig zum nächstmöglichen Tag X (Skto auf Fälligkeit) | Wenn das Bezugsdatum der Fälligkeit **vor** dem Tag **X** des Monats liegt, wird der Betrag fällig am Tag **X** des **laufenden Monats**.  
Wenn das Bezugsdatum der Fälligkeit **nach** dem Tag **X** liegt, wird der Betrag fällig am Tag **X** des **nächsten Monats**.  
Skonto berechnet sich in Tagen vor dem Fälligkeitsdatum |
| 12 | Fällig am Tag X der nächsten Woche (Skto auf Fälligkeit) | Fälligkeit der Zahlung am Wochentag X in der nächsten Kalenderwoche  
Als Wochentag wird hier Sonntag= 1, Montag = 2 usw. gerechnet  
Skonto berechnet sich in Tagen vor dem Fälligkeitsdatum |

<p class="just-emphasize">Bezug</p>

Mit diesem Format wird der Bezugstyp einer Zahlungsbedingung festgelegt. *(Formatname „ZBEDBEZUG“)*

| Nr. | Bezeichnung | Beschreibung |
| --- | --- | --- |
| 1 | Rechnungsdatum | Variable Zeitangaben beziehen sich auf das Rechnungsdatum |
| 2 | Lieferdatum | Variable Zeitangaben der Fälligkeit beziehen sich auf das Lieferdatum |
| 3 | Druckdatum | Variable Zeitangaben beziehen sich auf das Druckdatum |
| 4 | Manuelles Bezugsdatum | Variable Zeitangaben beziehen sich auf ein manuell festgelegtes Datum |

<p class="just-emphasize">Automatisch aufblenden</p>

Mit diesem Format wird festgelegt, wann die Maske automatisch aufblendet. *(Formatname „ZBEDAUTO“)*

| Nr. | Bezeichnung | Beschreibung |
| --- | --- | --- |
| 0 | nie | Maske soll nie automatisch angezeigt werden. |
| 1 | Neuerfassung | Maske soll nur bei Neuerfassung automatisch geöffnet werden. |
| 2 | Korrektur + Neuerfassung | Maske wird bei Korrektur und Neuerfassung automatisch angezeigt. |

<p class="just-emphasize">Formel</p>

Mit diesem Format wird die Formel der Zahlungsbedingung festgelegt. *(Formatname „ZBEDFORMEL“)*

| Nr. | Bezeichnung | Beschreibung |
| --- | --- | --- |
| 1 | voller Betrag | 100% des Rechnungsbetrages fällig entsprechend dieser Zahlungsbedingung, keine Folgezahlungsbedingung. |
| 2 | gleichmäßig verteilt | Aufteilung des Zahlungsbetrages gleichmäßig über die Zahlungsbedingungsfolgen. |

Der erste Fall ist die übliche Form. Für 2 gilt, dass maximal 2 Zahlungsbedingungen miteinander verknüpft sein können (unabhängig davon, ob 100 % erreicht sind) und dass die Berechnung abgebrochen wird, wenn eine Zahlungsbedingung auf sich selber verweist.

<p class="just-emphasize">Valutabestimmung</p>

Mit diesem Format wird die Valutabestimmung der Zahlungsbedingung festgelegt. *(Formatname „ZBEDKOVAL“)*

*Die Valutabestimmung bei den Zahlungsbedingungen wird nur bei Verwendung von Kontokorrent Rechnung KOKORE gezogen und angezeigt. Dazu muss der SPA **274 Valutadatum im Kontenblatt** mit **JA** aktiviert sein.*

| Nr. | Bezeichnung | Beschreibung |
| --- | --- | --- |
| 0 | Keine Valutaänderung | Das Valutadatum der Belege wird nicht angepasst. |
| 1 | Mittlere Valuta bei Bezug Lieferdatum | Es wird eine mittlere Valuta aus allen Valuten der Belege errechnet und dort vermerkt. |
| 2 | Valuta laut Kokoredatum | Das Datum der Erstellung wird als Valutadatum in die Belege übernommen. |
