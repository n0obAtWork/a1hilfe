# ALTER STRUCT Statement

<!-- source: https://amic.de/hilfe/alterstructstatement.htm -->

Syntax

ALTER STRUCT table-name [INTO Dateiname]

Purpose

Erstellt für eine Tabelle die Beschreibung. Im Gegensatz zu [CREATE STRUCT](./create_struct_statement.md) wird die Tabelle nur mit den Primäschlüsselfeldern angelegt und alle anderen Felder werden mit ALTER TABLE hinzugefügt.

Anwendung

Befehlszeile, Kommandodatei

Berechtigung

Alle Anwender

Siehe auch

[DBFCREATE](./dbfcreate_statement_ab_version_5_0.md), [CREATE PRIMARY KEY](./create_primary_key_from_statement_ab_version_4_5.md), [XMLImport](./xmlimport.md), [XMLExport](./xmlexport.md) , [DBFCREATE](./dbfcreate_statement_ab_version_5_0.md) , [CREATE STRUCT](./create_struct_statement.md)

Beschreibung

Um die Beschreibung ( das create table Statement) für eine Tabelle zu erhalten, steht dieses Statement zur Verfügung. Es erstellt eine Datei ( Achtung immer „Overwrite“ ) in der das Create-Statement zuzüglich der Indexe enthalten ist. Wird „INTO Dateiname“ nicht angegeben, wird der table-name mit der Endung „.SQL“ als Dateiname verwendet.  
    

Beispiel

ALTER STRUCT FIBUVORGKLASSE INTO c:\\FIBUKL.SQL;
