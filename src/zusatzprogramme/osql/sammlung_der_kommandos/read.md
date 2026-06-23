# READ

<!-- source: https://amic.de/hilfe/read.htm -->

<p class="just-emphasize">Syntax</p>

READ Dateiname [COUNT]

<p class="just-emphasize">Purpose</p>

Liest eine Datei im DDS Format in eine Datenbankrelation ein.

<p class="just-emphasize">Anwendung</p>

Kommandodatei, Befehlszeile

<p class="just-emphasize">Berechtigung</p>

Alle Anwender

<p class="just-emphasize">Siehe auch</p>

[IDENTLOAD](./identload_statement.md), [LOAD](./load.md), [DBFLOAD](./dbfload_statement.md)

<p class="just-emphasize">Beschreibung</p>

Es werden anhand der Datenbeschreibung und der Angabe der Dateinamen in der Datei die Daten für die Datenbankeinspielung Formatiert und in die Datenbank eingespielt. COUNT ist die Anzahl Datensätze nach der ein Vortschrittsmeldung ausgegeben werden soll.

<p class="just-emphasize">Beispiel</p>

READ c:\\Daten\\Beschreibung.DDS 50;
