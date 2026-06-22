# Materialbedarf Produktion

<!-- source: https://amic.de/hilfe/_lvs20_ProdBedarf.htm -->

Es gibt drei Möglichkeiten, den Bedarf der Produktionslinie zu decken:

**1. Der Bediener schreibt manuell einen Materialbedarf für die Linie**

(die für den Bediener aufwändigste Lösung)

Der Bediener erstellt mit dem Materialorder-Pfleger [[LVSMO]](./materialorder_lvsmo.md) eine Materialorder und legt dabei Artikel, Partie, Menge und Mengeneinheiten fest.

**2. Der Linienbedarf wird errechnet**

(die wohl technisch aufwändigste Lösung)

Die Linie sendet ein „BEGIN“, worauf hin die Produktion als aktiviert gilt. Bis zur Beendigung einer Produktion mit „END“ aus der Produktionsschnittstelle gilt diese Produktion als aktiv. Ein regelmäßig laufender Prozess rechnet den bedarf lauf Rezept durch, summiert diesen über alle aktiven Produktionen und zieht davon Materialien ab, die bereits in der Bereitstellungszone stehen bzw. dorthin unterwegs sind. Die Differenz wird in eine Materialorder für die Linie geschrieben und allokiert. Diese Implementation stellt hohe Anforderungen an die Produktionsschnittstelle und muss in der Regel individuell abgestimmt werden.

**3. Die Produktionslinie fordert Materialien an.**

(Die technisch einfachste Lösung)

Dabei werden Artikelnummern aus A.eins verwendet. Die Mengen werden in Kg bzw. Stück angegeben. In diesem Fall wird die Materialorder über die Produktionsanbindung geschrieben. Diese Lösung ist in der [Produktionsanbindung](./produktionsanbindung.md) implementiert.
