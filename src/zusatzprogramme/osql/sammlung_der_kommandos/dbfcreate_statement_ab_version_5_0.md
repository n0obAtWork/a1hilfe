# DBFCREATE Statement (Ab Version 5.0)

<!-- source: https://amic.de/hilfe/dbfcreatestatementabversion50.htm -->

<p class="just-emphasize">Syntax</p>

DBFCREATE Dateiname.dbf [INTO Relationsname]

<p class="just-emphasize">Purpose</p>

Anlegen einer Tabelle anhand der Feldbeschreibung in der DBASE Datei

<p class="just-emphasize">Anwendung</p>

Befehlszeile, Kommandodatei

<p class="just-emphasize">Berechtigung</p>

Alle Anwender

<p class="just-emphasize">Siehe auch</p>

[DBFLOAD](./dbfload_statement.md), [LOAD](./load.md), [READ](./read.md), [IDENTLOAD](./identload_statement.md), [CREATE STRUCT](./create_struct_statement.md), [ALTER STRUCT](./alter_struct_statement.md)

<p class="just-emphasize">Beschreibung</p>

Man kann mit dem Befehl DBFCREATE eine Tabelle anlegen. Die Struktur der Tabelle wird aus eine Datei im dBASE Format ( Version dBASE III+ und dBase IV )ermittelt. Ist kein Relationsname angegeben wird der Dateiname hierfür herangezogen. Es werden folgende Feldtypen ausgewertete: Character, Numerisch, Logical, Datum, Gleitpunkt. Dbasedateien mit anderen Feldtypen können nicht bearbeitet werden.

<p class="just-emphasize">Beispiel</p>

DBFCREATE FOP1.DBF into AMIC_OPS;
