# Qualitäten in Waage

<!-- source: https://amic.de/hilfe/_reg_waage_qual.htm -->

<p class="just-emphasize">Erfassung der 20 Standard Qualitäten</p>

Die Qualitäten 1 bis 20 werden auf der Registerkarte Qualitäten gepflegt. Per [Einrichterparameter](../firmenstamm/einrichterparameter/online_waage_epa_owaage.md) „Soll die Registerkarte Qualitäten ausgeblendet werden“ kann die Registerkarte angeschaltet werden. Des Weiteren kann in den Einrichterparametern die Bezeichnung, das Format des Feldes sowie die Verbindung auf die Rohwarenqualitäten hinterlegt werden.

<p class="just-emphasize">Erfassung von Qualitäten per Tabelle</p>

Um die Erfassung per Tabellenform zu aktivieren, muss der [Steuerparameter 932](../firmenstamm/steuerparameter/waagensteuerung/qualitaetsverarbeitung_in_der_waage_spa_932.md) „Qualitätsverarbeitung in der Waage“ auf „1“ gestellt werden. Nach der Umstellung wird dann anstelle der zwanzig Standardfelder für die Qualitäten eine Tabelle angezeigt. Damit in der Tabelle die Qualitäten angezeigt werden, müssen die Qualitäten als Artikelbetsandteile ( [Bestandsteile](../artikelstamm_und_artikel/konstanten_der_artikelverwaltung/bestandteile.md) ) angelegt werden.

Hauptmenü > Stammdaten > Konstanten Artikelstamm > Bestandteil oder Direktsprung [**ABST**].

Nach dem die Bestandsteile zugeordnet worden sind, kann eine [Zuordnung](../artikelstamm_und_artikel/parameter_des_artikelstamms/zusammensetzung.md) der Bestandsteile im Artikelstamm erfolgen. Hier wird dann pro Artikelstamm festgelegt, welche Qualitäten abgefragt werden sollen (hierbei reicht es aus, wenn nur ein Repräsentant der Rohwarengruppe eine Zuordnung erhält). In den Prozessbeschreibungsparametern der Waage kann noch zusätzlich eine private Prozedur zur Anzeige angegeben werden. Die Zuordnung passiert mit der Funktion ***Zusammensetzung*** **F2.**

Hauptmenü > Stammdaten > Artikelstamm oder Direktsprung [**ARS**]

Die Qualitäten bei den Tabellen gestützten Erfassung werden in eine eigene Relation geschrieben „Owaage_Qualitäten“ von dort aus werden diese dann bei der Rohwarenbeleg Erzeugung in die Standard Tabelle übernommen.
