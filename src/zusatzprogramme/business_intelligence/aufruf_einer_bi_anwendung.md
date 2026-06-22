# Aufruf einer BI Anwendung

<!-- source: https://amic.de/hilfe/aufrufeinerbianwendung.htm -->

Durch Anklicken des Menüpunktes in der Abteilung BI-Anwendungen im Bereich Information kann nun direkt die Anwendung gestartet werden, hierzu wird der Anwender zunächst gefragt, für welchen Bereich(Filter) die Daten im Excel-Blatt bereitgestellt werden sollen. Es können hierbei verschiedene Profile genutzt werden, nach Betätigung der Auswahl durch die F9 Taste wird die Excel Anwendung gestartet. Hierbei gilt nun folgende Regel:

Befindet sich im „TEMP“-Verzeichnis des Anwenders eine BI Datei mit dem entsprechenden Namen, mit einem vorangestellten Mandantenkurzbezeichnungs-Precode, und ist das Datum dieser Datei größer oder gleich dem in der Datenbank gemerkten Erstelldatum dieser BI Excel-Tabelle, so wird sofort diese Excel Mappe gestartet.

Ist das Datum im „TEMP“ Bereich &lt; oder existiert keine Datei, so wird aus der Datenbank die Vorlage in das TEMP-Verzeichnis kopiert und dann gestartet.

Die BI Anwendung reagiert jetzt wie eine eigenständige Excel Datei, wobei dann zu beachten ist, dass einerseits die Sicherheitsrelevanten Kriterien der Excel Verarbeitung korrekt gesetzt sein müssen und es muss sichergestellt sein, dass die [ODBC](./odbc_anschluss_zur_datenbank.md) Verbindung zum Datenbankserver nutzbar ist.
