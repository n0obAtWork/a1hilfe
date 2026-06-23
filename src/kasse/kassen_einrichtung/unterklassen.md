# Unterklassen

<!-- source: https://amic.de/hilfe/unterklassen.htm -->

In den folgenden Vorgangsklassen können Kasseneinrichtungen vorgenommen werden:

| Vorgangsklassen |
| --- |
| Klasse | Beschreibung |
| 700 | Verkaufsrechnung |
| 790 | Verkaufsstornierung |
| 800 | Verkaufsgutschrift |
| 1700 | Einkaufsrechnung |
| 1790 | Einkaufsstornierung |
| 1800 | Einkaufsgutschrift |

<p class="just-emphasize">Definition der Unterklasse</p>

Die Standard-Unterklasse ist 9900. Es können jedoch seit Version 8 auch andere Unterklassen für Kasse genutzt werden. Diese werden in der [Formularzuordnung[FRZ] auf der Registerkarte Allgemein](../../firmenstamm/firmenkonstanten/bedienerwesen_bediener_bedienerklassen_und_erfasser/bedienerstamm/bedienerstamm_pfleger.md#Allgemein) im Feld „Kassen-Vorgang“ als Kassen-Unterklassen definiert.

<p class="just-emphasize">Zuordnung Nummernkreise und Zählkreise</p>

In den Unterklassen der Klassen 700,1700, 800 und 1800 müssen Nummernkreise und Zählkreise zugeordnet sein

Dabei ist insbesondere darauf zu achten, dass für jede Bedienerklasse, die Kassenvorgänge durchführen soll, eigene Einträge gemacht werden müssen (**[FRZ]** bzw. **[NKF]**)  
    

<p class="just-emphasize">Formulare</p>

In der [Formularzuordnung](../../vorgangsabwicklung/formularzuordnung/index.md) müssen für Barverkauf / Bareinkauf / Barverkauf Gutschrift die Zuordnungen der Formulare für Druck / Vorschau / Bildschirm gemacht werden.

Ebenso wird festgelegt, ob es sich um Brutto / Nettoerfassung handelt und welche Preisliste herangezogen werden soll.

Wenn bei der Preisliste 0 eingetragen ist, werden Preislisten gemäß Kunde / Artikel gezogen.

Als Kasse für Barverkauf zieht das für diesen Arbeitsplatz hinterlegte Konto in der Kassenverwaltung.

<p class="just-emphasize">Storno-Formulare</p>

Um die Belege stornieren zu können, sind die entsprechenden Storno Formulare zuzuordnen.

Diese sind: VK (790,9900), Gutschrift (880,9900), EK(1790,9900).
