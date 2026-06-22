# Kompatibilität

<!-- source: https://amic.de/hilfe/kompatibilitt.htm -->

Die Angabe des Buchungstyps löst die alten Kontraktklassen 2 und 12 (Fremdlager VK, Fremdlager EK) ab. Aus Kompatibilitätsgründen können in der Übersicht auch alte Vorverkaufs- bzw. Voreinkaufskontrakte der Kontraktklassen 2 und 12 angezeigt werden. Neu erzeugt werden diese Kontraktklassen jedoch nicht mehr.

Bisher war die Definition eines Kontrakts über die Kontraktklasse möglich. Die Kontraktklassen 2 bzw. 12 zeigten an, dass es sich um Fremdware bzw. Fremdlagerkontrakte handelte.

Mit Beginn der Einlagerung laufen diese Kontrakte aus. Es wird ein zusätzliches Kennzeichen, der KtrBuchTyp eingeführt. Dieser Buchungstyp gibt bei Einkaufs- und Verkaufskontrakten künftig an, um welche Art von Kontrakt es sich handelt.

| Felder aus der Tabelle Kontraktstamm |
| --- |
| **Kontraktklasse**<br>**KtrKlasse** | **Buchungstyp**<br>**KtrBuchTyp** | **Alte**<br>**Kontraktklasse** | **Beschreibung** |
| 1 | 0 | 1 | Verkaufskontrakt |
| 1 | 1 | 2 | Vorverkaufskontrakt (Fremdware Verkauf) |
| 1 | 4 | \--- | Kommission Verkauf |
| 11 | 0 | 11 | Einkaufskontrakt |
| 11 | 2 | 12 | Voreinkaufskontrakt (Fremdlager Einkauf) |
| 11 | 3 | \--- | Einlagerungskontrakt |

Bei allen Reporten und Auswertungen muss also künftig diese neue Konstellation parallel zu der auslaufenden alten Konstellation berücksichtigt werden. Bestehende privater Reports müssen angepasst werden.

Die ausgelieferten Reports, Auswahllisten und Itemboxen sind bereits angepasst worden, zeigen jedoch zum Teil nur die bisher verfügbaren Informationen an. Für Wünsche zur Ergänzung oder Hinweise zu unbearbeiteten Listen sind wir dankbar. Kontaktieren Sie bitte den Support.
