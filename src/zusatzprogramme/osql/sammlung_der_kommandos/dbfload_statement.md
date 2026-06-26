# DBFLOAD Statement

<!-- source: https://amic.de/hilfe/dbfloadstatement.htm -->

#### Syntax

DBFLOAD [NOANSI] Dateiname.dbf [INTO Relationsname]

#### Purpose

Einlesen einer DBASE Tabelle

#### Anwendung

Befehlszeile, Kommandodatei

#### Berechtigung

Alle Anwender

#### Siehe auch

[LOAD](./load.md), [READ](./read.md), [IDENTLOAD](./identload_statement.md), [DBFCREATE](./dbfcreate_statement_ab_version_5_0.md)

#### Beschreibung

Man kann mit dem Befehl DBFLOAD direkt Daten die im dBASE Format ( Version dBASE III+ und dBase IV )abgelegt sind, in Relationen abspeichern. Existiert die angegebene Relation nicht wird sie anhand der Datenbeschreibung innerhalb der Datei angelegt. Ist kein Relationsname angegeben wird der Dateiname hierfür herangezogen. Es werden folgende Feldtypen ausgewertete: Character, Numerisch, Logical, Datum, Gleitpunkt. Dbasedateien mit anderen Feldtypen können nicht bearbeitet werden.  
 Ist in der DBF-Datei nicht die Codepage 1252 (Windows ANSI-Code), so wird für die Umlautdarstellung eine Codepagekonvertierung von MSDos nach Windows vorgenommen. Will man diese Konvertierung nicht haben, so muss man NOANSI angeben. ACHTUNG : NOANSI muss direkt hinter DBFLOAD stehen!

#### Beispiel

DBFLOAD FOP1.DBF into AMIC_OPS;
