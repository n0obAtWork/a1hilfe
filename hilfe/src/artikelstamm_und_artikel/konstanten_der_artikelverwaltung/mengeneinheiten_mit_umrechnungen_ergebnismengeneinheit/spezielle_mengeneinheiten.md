# Spezielle Mengeneinheiten

<!-- source: https://amic.de/hilfe/_speziellemengeneinhe.htm -->

**Anbruchgebinde**

Sie sind als Mengeneinheiten mit dem Gebindetypen

**Anbruch Gebinde, aufgerundet**

**Anbruch Gebinde, abgerundet**

einrichtbar.

**Anbruchgebinde aufgerundet:**

Es wird eine bestimmte Anzahl von Einheiten eingeben, die Gebindeanzahl.

Die resultierende Menge ist nur ein Vielfaches des ersten Gebindefaktors

Es wird hierbei davon ausgegangen, dass nur Mengen fakturiert werden dürfen, die voll in einer Packungsgröße aufgehen, sozusagen untrennbar sind.

Beispiel:

Es sind 15 Fliesen in einem Karton.

Es sollen nur volle Kartons in Rechnung gestellt werden

Bei Eingabe von 1490 Stück kann wird dann automatisch auf 1500 aufgerundet

Dabei wird nicht im kaufmännischen Verfahren auf- bzw. abgerundet, sondern immer auf volle Packungsgrößen aufgerundet, sobald eine neue Packung angefangen wird.

Darüber hinaus ist es dann möglich, die Einheiten in Bezug zu weiteren Größen zu stellen. Bei diesem Beispiel wäre es interessant, wie viele Quadratmeter, wie viel Kisten oder wie viel Paletten die gewünschte Menge ergeben werden. (Siehe dazu Packungsgröße. weiter unten) Denkbar wäre bei der Fakturierung auch eine Mengeneingabe in Quadratmetern, wobei aber auch nur immer volle Kartons bewegt werden sollen.

**Anbruchgebinde aufrunden Stufe 2:**

Bei dieser Einstellung wird nicht auf volle Gebinde (z.B. Paletten) aufgerundet, sondern eine Stufe weiter unten, also z.B. Kartons. Das eigentliche Gebinde darf also angebrochen werden, die nächstniedrigere Ebene nicht. Natürlich ist diese Einstellung nur bei Verwendung mehrstufiger Gebinde sinnvoll 

**Anbruchgebinde, abgerundet** 

Dieses Berechnungsverfahren läuft analog zum aufgerundeten Anbruchgebinde, wobei jedoch die Eingabemenge im Anbruchfall immer auf die nächste kleinere Packungsgröße zurückgerechnet wird.

**Packungsgröße:**

In dem Feld Gebindemaß 1 wird die Anzahl bzw. der Faktor hinterlegt, der als fester Rundungswert gelten soll. Dieses Maß kann in der Mengeneinheit selbst hinterlegt werden (Faktorherkunft = aus Mengeneinheit); sinnvollerweise wird es aber im Artikelstamm hinterlegt, da die Rechengrößen ja oft unterschiedlich sind (Faktorherkunft = aus Artikelstamm, bzw. aus Artikel). Ist der Faktor positiv, so wird die Eingabe durch den Faktor geteilt, gerundet und dann wieder mit dem Faktor multipliziert.

Beispiel: Eingabe in qm bei fester Packungsgröße

In einer Packung Fliesen sind immer 1,09 qm, die nur komplett fakturiert werden soll, d.h. Teilmengen sind nicht zulässig

Die Quadratmeteranzahl je Packung wird in dem Gebindemaß 1 hinterlegt.

Die Mengeneingabe erfolgt in Quadratmeter, so wie sie z.B. der Kunde angibt.

Bei Eingabe von 2 qm wird dann das Ergebnis = {runde auf ( 2 qm / 1,09 qm )} \* 1,09 qm = 2,18 qm sein.

Da der Kunde 2 qm haben wollte, nur volle Verpackungen verkauft werden erhält er 2,18 qm

(Folgendermaßen wird gerechnet:

Eingabe 2 qm / 1,09 qm = 1,83

Ergebnis aufrunden: = 2

Gerundeter Betrag: 2 \* 1,09 qm = 2,18 qm)

Ist der Faktor negativ, wird der Faktor als Kehrwert interpretiert:

Die Eingabe wird mit dem Faktor multipliziert, gerundet und dann wieder durch den Faktor geteilt

Dies deckt den Sonderfall ab, falls die Größenangabe umgekehrt ist:

1 qm sind 9,5 Fliesen, es werden nur ganze Fliesen verkauft.

Ergebnis = {runde auf (3 qm \* 9,5 Fliesen)} / 9,5 Fliesen = 3,05 qm

Rechenweg: {runde auf (28,5)} = 29

29 / 9,5 = 3,05

**Bezugsgrößen:**

Zusätzlich zu dem festen Wert zur Berechnung der Fakturiermenge, möchte man eventuell die ermittelte Anzahl in Bezug zu anderen Größen setzen. Man möchte eventuell wissen, auf wie vielen Paletten wie viele Lagen von Fliesenkartons sind. Die Umrechnungsfaktoren zu der Basisgröße (hier Karton) werden in den Gebindemaßen 2 bis 4 optional hinterlegt. Der Andruck erfolgt nur, wenn ein Wert ungleich Null eingetragen ist.

Ein positiver Wert bewirkt eine Multiplikation, ein Negativer eine Division

Beispiel:

Angabe Gesamtstückzahl bei Eingabe von qm Werten

Ansatz: In Gebindemaß 2 wird die (positive=Multiplikation) Anzahl Fliesen je Paket hinterlegt. Das Ergebnis kann dann (im Druckformular, etc.) über die Bezugsgröße 1 abgerufen werden.

Ergebnis:

 = Anzahl Karton \* Anzahl Fliesen je Karton =

 = qm-Ergebnis / qm je Packung \* Anzahl Fliesen je Karton

 = qm-Ergebnis / Gebindemaß 1 \* Gebindemaß 2

 = Gesamtstückzahl

Beispiel:

Angabe Lagen bei Eingabe von qm Werten

Ansatz: In Gebindemaß 3 wird die (negative=Division) Anzahl Packungen je Lage hinterlegt

Ergebnis:

 = Anzahl Karton / Anzahl Karton je Lage

\= qm-Ergebnis / qm je Packung / Anzahl Karton je Lage

\= qm-Ergebnis / Gebindemaß1 \* Gebindemaß 3

Die Bezugsgröße 1 ist immer die Anzahl der nicht trennbaren Grundeinheiten. In diesem Beispiel wären das:

 qm-Ergebnis / qm je Packung = Anzahl Kartons.
