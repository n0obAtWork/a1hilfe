# OP-Führung mit mehreren Fremdwährungen

<!-- source: https://amic.de/hilfe/opfhrungmitmehrerenfremdwhrung.htm -->

Hauptmenü > OP-Verwaltung > OP-Bearbeitung > OP-Verwaltung

Direktsprung **[OPV]**

Für Personenkonten besteht die Möglichkeit mehrere unterschiedliche Währunge miteinander zu verrechnen. Hier ergibt sich eine weitere Besonderheit, da die unterschiedlichen Fremdwährungen auf eine Fremdwährung zurückgerechnet werden müssen um die Kursdifferenzen bzw. den zuviel oder zuwenig gezahlten Betrag bestimmen zu können. Im folgendem einfachen Beispiel sind die Zahlen so gewählt, dass die Zahlung in Sloty der Rechnung in US-Dollar zum Tageskurs der Zahlung entspricht.

| | USD | PLN | Kurs | EUR |
| --- | --- | --- | --- | --- |
| Rechnung | 10.000,00 S | | 1,366300 | 7.319,14 S |
| Zahlung | | 30.713,25 H | 4,220000 | 7.278,02 H |
| Währungsumrechnung 1 | | \-30.713,25 H | 4,220000 | \-7.278,02 H |
| Währungsumrechnung 2 | 10.000,00 H | | 1,374000 | 7.278,02 H |
| Kursdifferenz | 0,00 H | 0,00 H | | 41,12 S |

Wird die Rechnung mit der Zahlung ausgeziffert, so geschieht intern eine Umrechnung die den gelb markierten Zeilen entspricht. Die Zahlung wird zum aktuellen Kurs in die Fremdwährung USD der Rechnung umgerechnet. Dieser Kurs hat sich in der Zwischenzeit von 1,3663 auf 1,374 geändert, was zu dann zu der Kursdifferenz führt. Um diese Umrechnung zu belegen, wird beim Ausziffern eine technischer Beleg erstellt. Dieser Beleg wird nicht gebucht. Wird die Auszifferung aufgehoben verschwindet er sofort wieder.

Geht die Buchung nicht auf 0 auf , so sieht die Tabelle etwas anders aus.

| | USD | PLN | Kurs | EUR |
| --- | --- | --- | --- | --- |
| Rechnung | 10.000,00 S | | 1,366300 | 7.319,14 S |
| Zahlung | | 30.500,00 H | 4,220000 | 7.227,49 H |
| Währungsumrechnung 1 | | \-30.500,00 H | 4,220000 | \-7.227,49 H |
| Währungsumrechnung 2 | 9.930,57 H | | 1,374000 | 7.227,49 H |
| Kursdifferenz | 69,43 H | 0,00 H | | 41,12 S |

Hier wurde zuwenig gezahlt und zwar genau 69,43 USD. Aus den zusätzlich gebildetene Währungsumrechnungszeilen lässt sich dann genau ablesen, wie die Zahlen zustande kommen.

**Hinweis:** *Es wird für jeden Beleg, der aus einer Fremdwährung in eine andere Fremdwährung umgerechnet wird, ein zusätzlicher Währungsumrechnungsbeleg gebildet. Es entsteht also kein Sammelbeleg pro Auszifferung. Der Währungsumrechnungsbeleg hat immer dieselbe Belegnummer wie der Beleg dem er zugeordnet wurde.*

#### Welche Währung ist die Währung, in die Belege umgerechnet werden?

Hat man in den Stammdaten des Personenkontos eine andere Währung als die Buchwährung hinterlegt, so wird - soweit sie innerhalb der Auszifferung vorkommt - dies die Währung in die umgerechnet wird.

Ist in den Stammdaten des Personenkontos keine Währung hinterlegt oder kommt diese Währung innerhalb der Auszifferung nicht vor, so ist es jeweils die zuletzt angeklickte Fremdwährung, die die Zielwährung angibt.
