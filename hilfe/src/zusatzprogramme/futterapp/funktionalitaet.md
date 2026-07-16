# Funktionalität

<!-- source: https://amic.de/hilfe/funktionalitt.htm -->

Mit Hilfe der FutterApp können:

Bestellungen erfasst werden. Diese kommen als XML-Datei im Verzeichnis (SPA 1047) an. Im Anschluss werden diese Dateien von der im Event hinterlegten Routine verarbeitet. Die XML-Datei wird nun in das Verzeichnis „\\Import\\Archiv“ oder bei einer Fehlermeldung in das Verzeichnis „\\Import\\Fehler“ verschoben. Wenn der Import funktioniert hat, wird im Verzeichnis „\\Export\\OrderConfirm“ eine Auftragsbestätigung geschrieben. In Abhängigkeit des Parameters „Buchen“ (eingerichtet im Event“ findet sich der Auftrag nun entweder im Vorgangsimport (**[VIMP]**) oder direkt bei den Aufträgen (**[AUB]**).

Silos bearbeitet oder gelöscht werden. Diese kommen als XML-Datei im Verzeichnis (SPA 1047) an. Im Anschluss werden diese Dateien von der im Event hinterlegten Routine verarbeitet. Die XML-Datei wird nun in das Verzeichnis „\\Import\\Archiv“ oder bei einer Fehlermeldung in das Verzeichnis „\\Import\\Fehler“ verschoben.

Nach der Einrichtung wird bei jeder Änderung von FutterApp-relevanten Daten im Kundenstamm oder beim Erstellen von Aufträgen (FutterApp-Kunde) ein Eintrag in der Tabelle „AMIC_AenderungsProtokoll“ mit dem Status „0“ erzeugt. Beim nächsten Aufruf der Batchdatei durch die Aufgabenplanung wird für jeden dieser Einträge (mit Status „0“) im zugehörigen Ordner im Export-Verzeichnis eine XML-Datei geschrieben. Der Status wird auf „1“ gesetzt.

Um nachträglich noch Aufträge in die App zu exportieren gibt es im Kundenstamm den Button „Historische Daten“. Es werden nun alle Aufträge der letzten 365 Tage ins „AMIC_AenderungsProtokoll“ geschrieben.

Bei jedem Storno eines Auftrags (echtes Storno und Gegenbeleg) wird im Verzeichnis „\\Export\\StornoAuftrag“ eine XML-Datei mit den Stornoinformationen erzeugt.
