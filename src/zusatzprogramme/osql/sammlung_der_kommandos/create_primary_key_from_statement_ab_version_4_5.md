# CREATE PRIMARY KEY FROM Statement (ab Version 4.5 )

<!-- source: https://amic.de/hilfe/createprimarykeyfromstatementa.htm -->

Syntax

CREATE PRIMARY KEY FROM Indexname;

Purpose

Legt einen Primary key an.

Anwendung

Befehlszeile, Kommandodatei

Berechtigung

AMIC

Siehe auch

[CREATE STRUCT](./create_struct_statement.md), [DBFCREATE](./dbfcreate_statement_ab_version_5_0.md), [ALTER STRUCT](./alter_struct_statement.md)

Beschreibung

Beim erstmaligen anlegen der Datenbank wurden anstatt PRIMARY KEYS zu verwenden Unique Indexe verwendet. PRIMARY KEYS werden jedoch benötigt um die referenzielle Integrität hinzubekommen ( FOREIGN-KEYS ). Um nun nachträglich die PRIMARY KEYS verwenden zu können, sind mehrere Schritte nötig. Die diese Funktion übernimmt:

Es wird geprüft, ob für diese Relation schon ein PRIMARY KEY existiert. Ist dies der Fall wird die Funktion beendet.

Es wird versucht die Felder zu NOT NULL Feldern zu machen, falls Sie noch keine sind. Funktioniert dies nicht, wird versucht alle Datensätze zu löschen, die eines der Indexfelder NULL haben und danach noch mal die Felder zu NOT NULL zu machen.

Es wird der PRIMARY KEY angelegt.

Der Index wird gedropt.

Beispiel

CREATE PRIMARY KEY FROM u0__Fibuv_id_offenerposten
