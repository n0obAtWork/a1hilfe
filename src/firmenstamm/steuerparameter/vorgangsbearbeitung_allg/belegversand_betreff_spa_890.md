# Belegversand Betreff (SPA 890)

<!-- source: https://amic.de/hilfe/_SPA_890.htm -->

Beim Belegversand im Vorgang müssen für verschiedene Vorgangsklassen unterschiedliche Wege gegangen werden, den Betreff zu ermitteln. So wird in Abhängigkeit von der Vorgangsklasse und der Belegnummer ein Betreff erstellt.

Eine privatisierte Version dieser Funktion kann hier sowohl für den Versand von Einzelbelegen als auch für den Versand von Rohware-Sammeldruck-Belegen eingetragen werden.

| Bereich | Bedeutung / Option |
| --- | --- |
| \- | Kein Wert |
| Standard | In dieser Option wird eine Datenbankprozedur hinterlegt, welche anstelle der Standarddatenbankprozedur „Amic_Belegversand_Betreff“ im Belegversand den Betreff für den Belegversand ermittelt. Dieser Eintrag betrifft nicht den Versand von Rohware-Sammeldruck-Belegen. Für diese kann eine entsprechende Prozedur im Bereich ‚RohwareSammeldruck‘ angegeben werden (s.u.). Dabei ist zu beachten, dass die private Datenbankprozedur die gleichen Eingangs- sowie Ausgangsparameter der Standarddatenbankprozedur besitzt. |
| RohwareSammeldruck | In dieser Option kann für Rohware-Sammeldruck-Belege eine Datenbankprozedur hinterlegt, welche anstelle der Standarddatenbankprozedur „Amic_Belegversand_RW_Betreff“ den Betreff für den Belegversand von Rohware-Sammeldruck-Belegen aufgerufen wird. Dabei ist zu beachten, dass die private Datenbankprozedur die gleichen Eingangs- sowie Ausgangsparameter der Standarddatenbankprozedur besitzt. |
