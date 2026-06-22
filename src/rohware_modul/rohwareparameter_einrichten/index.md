# Rohwareparameter einrichten

<!-- source: https://amic.de/hilfe/_rwparameterpflege.htm -->

Hauptmenü > Administration > Steuerung > Steuerparameter zeigen

Direktsprung **[SPA]**

Direktsprung **[RWPA]**

Viele Funktionen innerhalb der Module zur Bearbeitung von Rohwarebelegen werden von Rohwareparametern beeinflusst. Alle Parameter können für unterschiedliche Zeiträume (Gültigkeiten) mit unterschiedlichen Werten belegt werden. Diese Zeiträume sind, für jeden Parameter einzeln, durch das jeweilige Datum gekennzeichnet, das den Beginn der Gültigkeit markiert. Die in A.eins bereits vorhandenen Einstellungen aller Parameter mit der Gültigkeit ‚ab 01.01.1901‘ können nicht verändert werden. Gewünschte abweichende Einstellungen können daher zunächst nur mit Eintrag eines neuen Gültigkeitsbeginns festgelegt werden.

Die Werte dieser Parameter werden grundsätzlich getrennt für die Bereiche Einkauf und Verkauf festgelegt. Innerhalb dieser Bereiche können für die meisten Parameter, die zunächst einmal mit Berücksichtigung Ihrer Gültigkeiten für alle Rohwarenbelege im Einkauf beziehungsweise Verkauf gelten (globaler Wert), spezielle von der globalen Bedeutung abweichende Einstellungen für einzelne Rohwarengruppen wie auch einzelner Abrechnungsschemata vorgenommen werden. Die einzelnen Programmfunktionen ermitteln den Wert eines benötigten Parameters immer, indem zunächst nach der abrechnungsschemaspezifischen Einstellung gesucht wird. Ist diese nicht vorhanden, so wird der rohwarengruppenspezifische Wert gesucht, ist dieser ebenfalls nicht verfügbar, so ist der globale Parameterwert maßgeblich.

Organisatorisch ist die Menge der Rohwareparameter in Parametergruppen eingeteilt. Eine besondere Bedeutung haben die Parameter der Gruppe ‚***globale Einstellungen***‘: Diese können keine speziellen Einstellungen für Rohwarengruppen und/oder Abrechnungsschemata erhalten.

In der Auswahlliste der Rohwareparametereinstellungen können die aktuellen Werte der ausgewählten Parameter inklusive der auf Rohwarengruppen und/oder Abrechnungsschemata abgeleiteten Einstellungen dargestellt werden. Das aus dieser Auswahlliste heraus aufgerufene Pflege-Modul *‚Rohwareparameter pflegen‘ / Rohwareparameter ansehen‘* stellt jedoch für den ausgewählten Parameter immer alle Instanzen zur Bearbeitung zur Verfügung. Wird zum Beispiel in der Auswahlliste eine Zeile zu einem Parameter mit dem dargestellten Wert zu einem Abrechnungsschema und eine Zeile mit der globalen Wertdarstellung markiert, so wird das Pflegemodul zu beiden Einträgen dieselbe Darstellung bieten.

<p class="siehe-auch">Siehe auch:</p>

- [Rohwareparameter pflegen](./rohwareparameter_pflegen.md)
- [Rohwareparameter ansehen](./rohwareparameter_ansehen.md)
- [Rohwareparameter-Übersicht](./rohwareparameter_uebersicht.md)
