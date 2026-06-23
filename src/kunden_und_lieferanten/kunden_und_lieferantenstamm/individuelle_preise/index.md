# Individuelle Preise

<!-- source: https://amic.de/hilfe/_IndividuellePreise.htm -->

Hauptmenü > Stammdatenpflege > Kunden / Lieferanten > Kundenstamm

Direktsprung **[KU]**

Hauptmenü > Stammdatenpflege > Kunden / Lieferanten > Lieferanten

Direktsprung **[LF]**

Hauptmenü > Stammdatenpflege > Artikelstamm > Artikel

Direktsprung **[AR]**

Ein [individueller Preis](../../../preise_konditionen/individuelle_preise/index.md), oder eine mengenabhängige Preisstaffel, werden am Kreuzungspunkt aus Kunde und Artikel gepflegt. Um nicht für jeden Kunden und jeden einzelnen Artikel eine solche Preispflege vornehmen zu müssen, werden Kunden in [Preisklassen](../../../preise_konditionen/individuelle_preise/individuelle_preisklasse.md) und Artikel in [Preisgruppen](../../../preise_konditionen/individuelle_preise/individuelle_preisgruppe.md) organisiert. Die Zuordnung erfolgt in den jeweiligen Anwendungen zur Stammdatenpflege, hierbei getrennt für eine Verkaufs- und eine Einkaufs-Seite. Ein Kontokorrentkunde der gleichzeitig Kunde und Lieferant sein kann, hat üblicherweise zwei Preisklassen (Einkauf und Verkauf). Ähnliches gilt für einen Artikel der aktiv gehandelt wird, er benötigt zwei Preisgruppen (auch hier Einkauf und Verkauf).

Individuelle Preise können auf zwei Arten gepflegt werden: über eine Einzelsatzanwendung, bei welcher Daten an einem der oben beschriebenen Kreuzungspunkte erfasst werden, also für **eine** Kombination aus Preisklasse **und** Preisgruppe. Wird hingegen eine der fixierten Dimensionen freigegeben (Preisgruppe frei à Artikel frei wählbar à Einstieg über den festen Kunden/Lieferanten oder Preisklasse frei à Kunde frei wählbar à Einstieg über den festen Artikel) benötigt man den im folgenden beschriebenen Preisstapelpfleger für individuelle Preise.

Da eine der Dimensionen variabel ist, werden die Daten folglich in einer Tabelle dargestellt: für einen gewählten Kunden/Lieferanten alle – über ihre Preisgruppe – zugeordneten Artikel, oder für einen gewählten Artikel alle – über ihre Preisklasse – zugeordneten Kunden. Die Laufvariable ist dann die Zeit, dargestellt über die Zeiträume [gültig ab, gültig bis]. Erfasst werden die anzuwendende Ab-Menge und die für diese Menge relevanten Preisinformationen.

Die Bereitstellung der Daten erfolgt über eine auf den Anwendungsfall zugeschnittene Ladeprozedur:

- **HoleIndividuellePreiseKunde** aus der Auswahlliste Kunden/Lieferanten
- **HoleIndividuellePreiseArtikel** aus der Auswahlliste Artikel

Je nach Herkunft (Kunden, Lieferanten oder Artikel) werden einzelne Felder ein oder ausgeblendet und mit Daten gefüllt. Der Umfang der angezeigten Daten kann über Profileinstellungen angepasst werden. Über eine in der Profilansicht angegebene Kalkulationsprozedur können die angegebenen Preise je nach Kundenwunsch kalkuliert werden. Um die Funktionsweise zu verdeutlichen, wird eine Kalkulationsprozedur **Beispiel_Einstieg_IndiPrKalk** mitgeliefert. Um eigene Kalkulationen durchführen zu können, empfehlen wir die Einrichtung privater Kalkulationsprozeduren.

<p class="siehe-auch">Siehe auch:</p>

- [Aufruf aus Kunden [KU] oder Lieferanten [LI]](./aufruf_aus_kunden_ku_oder_lieferanten_li/index.md)
- [Aufruf aus Artikel [AR]](./aufruf_aus_artikel_ar/index.md)
- [Preis Profile](./preis_profile.md)
