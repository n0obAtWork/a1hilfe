# Kreditlimit-Prüfung(SPA 233)

<!-- source: https://amic.de/hilfe/_SPA_233.htm -->

Abhängig vom Wert dieses Steuerparameters wird innerhalb eines Vorganges folgendes überprüft:

**Nein**: es erfolgt keine Überprüfung

**Warnung**: nach Eingabe der Kundennummer wird bei Ladescheinen, Ausgangslieferscheinen, Ausgangsrechnungen, Ausgangsgutschriften, Eingangsrechnungen und Aufträgen (*Achtung: Steuerparameter 22 in dieser Gruppe*) und deren Stornos eine Meldung ausgegeben, wenn das Kreditlimit des Kunden überschritten ist.

**Sperrung**: Neben der Warnmeldung zu Beginn, wird der Beleg gegen Abschluss gesperrt, wenn das Kreditlimit überschritten ist; man besitzt jedoch noch die sofortige Möglichkeit zur Korrektur des Beleges.

**Abweisung**: Neben der Warnmeldung bei Überschreitung zu Beginn, wird ein Abschluss grundsätzlich verhindert, wenn das Kreditlimit überschritten ist.
