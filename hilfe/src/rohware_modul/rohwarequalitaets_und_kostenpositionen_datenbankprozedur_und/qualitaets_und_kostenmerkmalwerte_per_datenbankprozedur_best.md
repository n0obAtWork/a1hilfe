# Qualitäts- und Kostenmerkmalwerte per Datenbankprozedur bestimmen

<!-- source: https://amic.de/hilfe/qualittsundkostenmerkmalwertep.htm -->

Hauptmenü > Rohwarenabrechnung \> Rohwaren-Verwaltung > Bearbeiten > Abrechnungsschema > Merkmal-Definition

Direktsprung **[RWG]**

Hauptmenü > Administration \> Werkzeuge > SQL Textmanager

Direktsprung **[SQLM]**

Für die Abrechnung von Qualitäts- und Kostenpositionen können nun auch Datenbankprozeduren zur Versorgung oder Berechnung bestimmter Werte eingesetzt werden.

**Hinweis:** Es ist bei der Verwendung von Datenbankprozeduren unbedingt auf die Performance bei der Prozedurausführung zu achten, da diese während der Erfassung oder Korrektur eines Rohware bei allen Eingaben durchgeführt wird, die ergebnisrelevant sein könnten.

Bei Qualitätsmerkmalen können sowohl die Werte für

- Analysewert
- korrigierter Analysewert
- oberer Basiswert
- unterer Basiswert

  per Datenbankprozedur ermittelt werden, wie auch die

- Abrechnungsmethoden

  bei Unterschreitung des unteren Basiswertes und/oder bei Überschreitung des oberen Basiswertes durch den korrigierten Analysewert als Datenbankprozedur zur Ermittlung des Preis- oder Mengenzu- oder –abschlags angegeben werden.

  Bei Kosten-/Vergütungsmerkmalen können die Werte für

- Kosten-/Vergütungssatz
- Kosten-/Vergütungspauschale

  per Datenbankprozedur ermittelt werden.

  Mit der Einstellung ‚DB-Prozedur‘ für die Analysebestimmung und die Bestimmung des korrigierten Analysewertes bzw. ‚Basiswertbestimmung per DB-Prozedur‘ im Feld ‚Basis in Beleg‘ für Basiswertfestlegungen auf der zugehörigen Qualitätsdefinitionsmaske des  
Rohwarengruppen-/ Abrechnungsschemadefinitionsmoduls kann im zugehörigen Textfeld der Name der heranzuziehenden Datenbankprozedur angegeben werden. Es wird hier nur der jeweilige Prozedurname, nicht aber die Parameter angegeben.

  Zur Bestimmung einer Datenbankprozedur als Abrechnungsmethode wird im Feld ‚Typ‘ auf der gewünschten Seite (‚Abrechnung bei Analysewert unter Basis‘ bzw. ‚Abrechnung bei Analysewert über Basis‘) je nach gewünschtem Ergebnistyp der Wert ‚ Preiszu-/ab-DB-Prozedur‘ bzw. ‚Mengenzu-/ab-DB-Prozedur‘ eingestellt.

  Im nun zur Verfügung stehenden Maskenfeld unterhalb dieses Eintrags wird der jeweilige Prozedurname (ohne Parameter) eingetragen.

  Wird innerhalb der Prozedur eine Bezugsgröße benötigt (Parameter ‚**PAR_ABR_BEZUG‘**), so ist darauf zu achten, dass bei einer Preisänderungsprozedur eine Preisbezugsgröße, bei einer Prozedur zur Generierung eines Mengenzu- oder –abschlags eine Mengebezugsgröße im Feld ‚Bezugsgröße‘ angegeben wird.

  Zur Bestimmung von Kosten-/Vergütungssatz bzw. Kosten-/Vergütungspauschale per Datenbankprozedur wird auf der zugehörigen Kostendefinitionsmaske des Rohwarengruppen-/Abrechnungsschemadefinitionsmoduls im Feld ‚Kostenart‘ der Typ ‚Satz per DB_Prozedur‘ bzw. ‚Pauschale per DB_Prozedur‘ und im Feld ‚DBProc‘ der Name der Prozedur angegeben.

  Die verwendeten Datenbankprozeduren müssen ein RESULT mit einem Attribut zurückliefern, dass praktischerweise vom Typ ‚numeric‘, ‚decimal‘, ‚integer‘ oder ‚smallint‘ sein sollte. Der Name des Ergebnisfeldes ist beliebig wählbar. Bei der Erfassung, Korrektur und/oder Abrechnung eines entsprechenden Beleges wird der Ergebniswert entsprechend der in der Qualitätsdefinition eingestellten Nachkommastellenangabe konvertiert.

  Die Parameter der DB-Prozedur, die zur Analyse- oder Basiswertermittlung bzw. zur Qualitätsabrechnung herangezogen werden sollen, werden mittels festgelegter Parameternamen bestimmt. Diese sind mit DEFAULT-Werten in der Parameterliste zu versehen. Aus der [*Liste der möglichen Parameter*](../rohwareparameter_einrichten/rohwareparameter_uebersicht.md#RwDbProc_Parameter) müssen nur die tatsächlich benötigten deklariert werden.
