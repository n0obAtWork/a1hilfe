# Anmerkung:

<!-- source: https://amic.de/hilfe/anmerkung.htm -->

Für die Weiterverarbeitung ist auf Vorgangsebene das Pascal-Script **VorgangEinspielung** zuständig, das von der Funktion VorgangUebergabeBelErz aufgerufen wird.

Auf Rohwarenebene rufen die Funktionen CWLU_EK (für Einkauf) und CWLU_VK (für Verkauf) die JPL-Prozedur **cwegvorb** auf.

Dabei werden sowohl Vorgangsdaten als auch Rohwarendaten importiert. Je nach Bestimmung werden unterschiedliche Zwischenrelationen für die Speicherung der importierten Daten verwendet: Vorgangsdaten werden in die Relation **VorgangUebergabe** importiert. Rohwarendaten stehen nach dem Import in der Relation **RohwareHauptsatz_Waage** und die zugehörigen Analysewerte in der Relation **RohwareZusatzQualitet_Waage**.

Die Waagenschnittstelle in Form des Pascal-Scriptes WaagenImport ist einheitlich. Beim Start konfiguriert sie sich aus den Daten der Relationen ScriptParam und ScriptParamPar. Die Standard-Waage benutzt dazu die Datensätze mit der ScriptPId = „WaagenImport“. Bei Verwendung mehrerer Waagen oder Importschnittstellen, die unterschiedlich zu konfigurieren sind, kann immer wieder das gleiche Pascal-Script verwendet werden. Die unterschiedliche Konfiguration wird durch weitere Gruppen von Scriptparametern erreicht. Der Aufruf des Pascalscriptes muss dann ebenfalls angepasst werden (s. Spezielle Fragestellungen).
