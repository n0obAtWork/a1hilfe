# DBFCREATE Statement (Ab Version 5.0)

<!-- source: https://amic.de/hilfe/dbfcreatestatementabversion50.htm -->

#### Syntax

DBFCREATE Dateiname.dbf [INTO Relationsname]

#### Purpose

Anlegen einer Tabelle anhand der Feldbeschreibung in der DBASE Datei

#### Anwendung

Befehlszeile, Kommandodatei

#### Berechtigung

Alle Anwender

#### Siehe auch

[DBFLOAD](./dbfload_statement.md), [LOAD](./load.md), [READ](./read.md), [IDENTLOAD](./identload_statement.md), [CREATE STRUCT](./create_struct_statement.md), [ALTER STRUCT](./alter_struct_statement.md)

#### Beschreibung

Man kann mit dem Befehl DBFCREATE eine Tabelle anlegen. Die Struktur der Tabelle wird aus eine Datei im dBASE Format ( Version dBASE III+ und dBase IV )ermittelt. Ist kein Relationsname angegeben wird der Dateiname hierfür herangezogen. Es werden folgende Feldtypen ausgewertete: Character, Numerisch, Logical, Datum, Gleitpunkt. Dbasedateien mit anderen Feldtypen können nicht bearbeitet werden.

#### Beispiel

DBFCREATE FOP1.DBF into AMIC_OPS;
