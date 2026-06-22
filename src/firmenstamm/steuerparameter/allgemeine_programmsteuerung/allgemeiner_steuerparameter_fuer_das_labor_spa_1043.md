# Allgemeiner Steuerparameter für das Labor (SPA 1043)

<!-- source: https://amic.de/hilfe/_SPA_1043.htm -->

In diesem Steuerparameter können Einstellungen aus dem FS Format FS_SPALABOR ausgewählt werden.

| Wert | Bedeutung / Option |
| --- | --- |
| \- | Kein Wert |
| ProzedurProbeTeilen | In dieser Option wird eine Datenbankprozedur hinterlegt, welche anstelle der Standarddatenbankprozedur „LaborProbeTeilen „bei der Funktionalität „Probeteilen“ und „ProbeteilenundDruck“ im Labor aufgerufen wird. Dabei ist zu beachten, dass die private Datenbankprozedur die gleichen Eingangs- sowie Ausgangsparameter der Standarddatenbankprozedur besitzt. |
| ProzedurKontrollanbauAuspraegung | In dieser Option wird eine Datenbankprozedur hinterlegt, welche anstelle der Standarddatenbankprozedur „KontrollanbauAuspraegung“ bei dem Verfahren Kontrollanbau aufgerufen wird. Dabei ist zu beachten, dass die private Datenbankprozedur die gleichen Eingangs- sowie Ausgangsparameter der Standarddatenbankprozedur besitzt. |
