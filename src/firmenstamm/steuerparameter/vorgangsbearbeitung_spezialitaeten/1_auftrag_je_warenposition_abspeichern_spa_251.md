# 1 Auftrag je Warenposition abspeichern(SPA 251)

<!-- source: https://amic.de/hilfe/_SPA_251.htm -->

Dieser Steuerparameter entscheidet, ob ein Auftrag mit mehreren Warenpositionen beim Speichern automatisch in weitere Aufträge aufgesplittet wird. Hierbei wird pro Warenposition ein Auftrag erzeugt. Die Einstellung kann über [**FRZ**] für die einzelnen Unterklassen überschrieben werden.

Wurden einzelne Warepositionen bereits teildisponiert, so ist ein Auftragssplitting nicht mehr möglich!

Folgende Werte können im Steuerparameter gespeichert werden:

| Wert | Beschreibung |
| --- | --- |
| Nein | Es wird kein Auftragssplitting durchgeführt. |
| Ja | Auftragssplitting wird durchgeführt. |
| Abfrage | Es wird beim Beenden des Vorgangs abgefragt, ob ein Auftragssplitting durchgeführt werden soll. |

Hinweis: Diese SPA-Einstellung kann durch eine Angabe in der [Vorgangsunterkasse](../../../vorgangsabwicklung/formularzuordnung/spa.md) überschrieben werden!
