# Belegversand Empfänger (SPA 888)

<!-- source: https://amic.de/hilfe/_SPA_888.htm -->

Beim Belegversand im Vorgang müssen für verschiedene Vorgangsklassen unterschiedliche Wege gegangen werden, den Empfänger zu ermitteln. So wird für Rechnungen die Mailadresse des Rechnungsempfängers ermittelt, bei Lieferscheinen die Mailadresse der Lieferadresse.

Eine privatisierte Version dieser Funktion kann hier eingetragen werden.

| Bereich | Bedeutung / Option |
| --- | --- |
| \- | Kein Wert |
| Standard | In dieser Option wird eine Datenbankprozedur hinterlegt, welche anstelle der Standarddatenbankprozedur „Amic_Belegversand_Mailempfaenger“ im Belegversand den Empfänger ermittelt. Dabei ist zu beachten, dass die private Datenbankprozedur die gleichen Eingangs- sowie Ausgangsparameter der Standarddatenbankprozedur besitzt. |
| RohwareSammeldruck | In dieser Option wird eine Datenbankprozedur hinterlegt, welche anstelle der Standarddatenbankprozedur „Amic_Belegversand_RW_Mailempfaenger“ bei Rohwaresammeldruck aufgerufen wird. Dabei ist zu beachten, dass die private Datenbankprozedur die gleichen Eingangs- sowie Ausgangsparameter der Standarddatenbankprozedur besitzt. |
