# Vorgangs- und Rohwarendaten

<!-- source: https://amic.de/hilfe/vorgangsundrohwarendaten.htm -->

Die Konfiguration der Waagenschnittstelle sollte weitestgehend auf einer Referenzdatenbank (z. B. einer Datensicherung der Kundendatenbank) vorbereitet werden. Die so vorgefertigte Konfiguration kann dann problemlos auf der Kundendatenbank eingespielt werden.

Unter Direktsprung [SCPA] wird eine Liste von Parametergruppen für Makros angezeigt. Für die Waagenschnittstelle ist das Makro mit der ScpriptPId „WaagenImport“ zuständig. Existiert kein solcher Eintrag, so kann er über Die Option-Box **\*\* WaagenImport Standard** automatisch erzeugt werden. Die so erzeugten Parameter entsprechen dem Agricom-Standard, müssen jedoch trotzdem überarbeitet werden.

In der Dokumentation ***Parametrisierung von Pascal-Skripten – Bedienungsanleitung*** ist erläutert, welche weiteren Funktionen in der Option-Box bereitstehen. So gibt es Funktionen zum Löschen und zum Duplizieren von ganzen Parametergruppen. Dies macht Sinn, wenn für verschiedene Kunden oder verschiedene Waagen- / Vorgangsimporte die jeweiligen Parameter gesichert werden sollen.
