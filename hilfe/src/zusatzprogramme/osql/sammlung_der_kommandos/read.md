# READ

<!-- source: https://amic.de/hilfe/read.htm -->

#### Syntax

READ Dateiname [COUNT]

#### Purpose

Liest eine Datei im DDS Format in eine Datenbankrelation ein.

#### Anwendung

Kommandodatei, Befehlszeile

#### Berechtigung

Alle Anwender

#### Siehe auch

[IDENTLOAD](./identload_statement.md), [LOAD](./load.md), [DBFLOAD](./dbfload_statement.md)

#### Beschreibung

Es werden anhand der Datenbeschreibung und der Angabe der Dateinamen in der Datei die Daten für die Datenbankeinspielung Formatiert und in die Datenbank eingespielt. COUNT ist die Anzahl Datensätze nach der ein Vortschrittsmeldung ausgegeben werden soll.

#### Beispiel

READ c:\\Daten\\Beschreibung.DDS 50;
