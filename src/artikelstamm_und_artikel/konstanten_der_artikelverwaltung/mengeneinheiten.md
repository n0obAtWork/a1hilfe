# Mengeneinheiten

<!-- source: https://amic.de/hilfe/_mengeneinheitenme.htm -->

Hauptmenü > Stammdatenpflege > Konstanten Artikelstamm > Mengeneinheiten

oder Direktsprung [ME]

Mit der Mengeneinheit wird bestimmt, welche Mengengrundlage bei Einkauf, Verkauf, Lager, etc. zugrunde gelegt wird.

Hierbei kann es sich um einen einfache Einheit mit Text, z.B. **kg**, **Stück** oder auch um eine komplexe Rechenformel handeln, wenn zur Mengenermittlung eine Gebindeberechnung zugrunde gelegt werden soll.

Im häufigen Fall, wenn z.B. Reifen in Stück eingekauft und verkauft werden, die Bestände in Stück geführt werden sollen und der Preisbezug **Stück** ist, genügt die Eintragung **Stück** für die Mengenbezüge im Artikelstamm.

Aufwändiger ist jedoch folgendes Beispiel: Wenn z.B. Kartoffeln in **kg** eingekauft werden, sie in verschiedenen Verpackungsgrößen verkauft werden (z.B. in 25 kg und 50 kg Säcken), der Einkaufpreis sich auf **100 kg** bezieht, der Verkaufspreis sich auf die **Verpackungsgröße** bezieht und der Bestand in **kg** geführt wird, müssen Umrechnungsformeln zwischen den verschiedenen Mengenbezügen eingeführt werden.

Die hier vorliegende Form der Mengeneinheitendefinition ermöglicht die automatische Umrechnung der verschiedenen Größenklassen.

Im Programm wird dabei unterschieden zwischen **Mengeneinheitengruppen**, die im Artikelstamm eingetragen werden und steuern, auf welcher (Mengeneinheiten-) Grundlage die Mengenberechnung im Einkauf, Verkauf, der Bestandsführung und der Preisfindung erfolgt. Hier wird also nur Bezug genommen auf die im Bereich "Mengeneinheiten" festgelegten Umrechnungsschlüssel. In einer Mengeneinheitsgruppe werden also (möglicherweise) unterschiedliche Mengeneinheiten für die Abwicklung des Artikels in Ein- und Verkauf zusammengefasst.

**Mengeneinheiten**, in denen die Daten für die Ermittlung der jeweiligen Mengen festgelegt sind. Dies sind Formeln (z.B. Länge x Breite x Höhe) und Faktoren (z.B. Karton mit **6** Flaschen).

**Mengengrundeinheiten**, in denen bestimmt wird, auf welche Einheiten zurückgerechnet wird. Hierbei handelt es sich ebenfalls um eine Mengeneinheit, nur dass diese bei Gebindeberechnungen nicht weiter aufgelöst wird, also keine Umrechnung erfolgt.

Die Erfassungsreihenfolge ergibt sich also aus der Mengeneinheit und anschließend der Mengeneinheitengruppe, die im Artikelstamm einzutragen ist.

In A.eins sind die Mengeneinheiten in nachfolgend beschriebene unterschiedliche Varianten unterteilt. Vor dem Speichern der jeweiligen Mengeneinheiten- Variante wird vor dem Speichern geprüft, ob die jeweils nötigen anderen Mengeneinheiten (z.B. Grundmengeneinheiten) eingetragen sind.
