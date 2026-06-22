# Finanzbuchhaltung

<!-- source: https://amic.de/hilfe/_90_37533.htm -->

In den Zahlungsarten kann für Zahlungsausgang eingestellt werden, dass die Überweisung als Echtzeitüberweisungen ausgeführt werden soll. Es werden dann alle Rechnungen, die vor dem nächsten Stichtag bzw. deren Skonto vor dem nächsten Stichtag fällig ist und zwar ohne Versatz von einem Tag wie es bei der Standardüberweisung der Fall ist. Um dies zu gewährleisten musste die Logik der Fälligkeitsbestimmung im automatischen Zahlungsverkehr komplett überarbeitet werden.Weiterhin ist dabei zu beachten, dass Standardüberweisungen und Echtzeitüberweisung beim DTA nicht gemischt übertragen werden können. Daher wurde die Anwendung "Zahlungen bearbeiten (Direktsprung [ZHB]) um ein Spalte "Echtzeitueberweisung" (Ja/Nein) erweitert.

<p class="just-emphasize">Releasenote Kategorie:</p>

Ticket: 747665[37533]

Version: 9.0.2502.5

Datum: 15.10.2025

Anwendung: ZHVE,ZHVB,ZHB

Variante: --

Funktion/Report: --

[Weitere Informationen](http://www.amic.de/hilfe/zahlungsart.htm)

Tags:

Releasenote, 9.0.2502.5, 37533, 747665
