# ALTER STRUCT Statement

<!-- source: https://amic.de/hilfe/alterstructstatement.htm -->

<p class="just-emphasize">Syntax</p>

ALTER STRUCT table-name [INTO Dateiname]

<p class="just-emphasize">Purpose</p>

Erstellt für eine Tabelle die Beschreibung. Im Gegensatz zu [CREATE STRUCT](./create_struct_statement.md) wird die Tabelle nur mit den Primäschlüsselfeldern angelegt und alle anderen Felder werden mit ALTER TABLE hinzugefügt.

<p class="just-emphasize">Anwendung</p>

Befehlszeile, Kommandodatei

<p class="just-emphasize">Berechtigung</p>

Alle Anwender

<p class="just-emphasize">Siehe auch</p>

[DBFCREATE](./dbfcreate_statement_ab_version_5_0.md), [CREATE PRIMARY KEY](./create_primary_key_from_statement_ab_version_4_5.md), [XMLImport](./xmlimport.md), [XMLExport](./xmlexport.md) , [DBFCREATE](./dbfcreate_statement_ab_version_5_0.md) , [CREATE STRUCT](./create_struct_statement.md)

<p class="just-emphasize">Beschreibung</p>

Um die Beschreibung ( das create table Statement) für eine Tabelle zu erhalten, steht dieses Statement zur Verfügung. Es erstellt eine Datei ( Achtung immer „Overwrite“ ) in der das Create-Statement zuzüglich der Indexe enthalten ist. Wird „INTO Dateiname“ nicht angegeben, wird der table-name mit der Endung „.SQL“ als Dateiname verwendet.  
    

<p class="just-emphasize">Beispiel</p>

ALTER STRUCT FIBUVORGKLASSE INTO c:\\FIBUKL.SQL;
