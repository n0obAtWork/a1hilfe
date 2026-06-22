# Zuordnung der Qualitätsbezeichnungen

<!-- source: https://amic.de/hilfe/zuordnungderqualittsbezeichnun.htm -->

Der [Steuerparameter 932](../../steuerparameter/waagensteuerung/qualitaetsverarbeitung_in_der_waage_spa_932.md) bestimmt, woher die Bezeichnungen der Qualitätsmerkmale stammen.

Einstellung „Bis max 20 in der Waage ausschließlich“

Hier werden die Bezeichnungen aus den [Einrichterparametern der Waage](../../einrichterparameter/online_waage_epa_owaage.md) geholt. Dadurch sind maximal 20 Qualitätsmerkmale möglich.

Einstellung „Über den Bereich Artbestandteil, beliebig viele Qualitäten“

In dieser Einstellung werden die Bezeichnungen in der Anwendung [Bestandteile](../../../artikelstamm_und_artikel/konstanten_der_artikelverwaltung/bestandteile.md) [ABST] gepflegt. Dabei werden nur die Datensätze beachtet, deren Qualitätsnummer größer als Null ist.

Die Reihenfolge der Felder in der Auswahlliste wird durch die Bestandteilnummer bestimmt. Soll bei der manuellen Erfassung der Qualitätswerte eine andere Reihenfolge verwendet werden, kann sie über das Feld Sortierung der Bestandteile eingestellt werden. Auf der Erfassungsmaske werden dann als erstes die Qualitätsmerkmale mit einer Sortierung ungleich Null aufsteigend und dahinter die Qualitätsmerkmale mit Sortierung gleich Null, aufsteigend nach Bestandteilnummer, angezeigt.

Aufgrund dieser Systematik muss man vorsichtig mit dem Erstellen und Löschen von Qualitätsmerkmalen sein, die aufgrund ihrer Bestandteilnummer nicht am Ende sind. Denn dadurch würden sich die Bezeichnungen der Qualitätsmerkmale verschieben, die Werte jedoch nicht.

Die Anzahl der verfügbaren Qualitätsmerkmale steigt mit dieser Methode auf 30. Sollten mehr als 30 Bestandteile mit Qualitätsnummern größer Null existieren, werden nur die ersten 30 Datensätze bei der Sortierung nach Bestandteilnummer genutzt.
