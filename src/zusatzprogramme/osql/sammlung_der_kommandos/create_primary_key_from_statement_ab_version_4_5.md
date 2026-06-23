# CREATE PRIMARY KEY FROM Statement (ab Version 4.5 )

<!-- source: https://amic.de/hilfe/createprimarykeyfromstatementa.htm -->

<p class="just-emphasize">Syntax</p>

CREATE PRIMARY KEY FROM Indexname;

<p class="just-emphasize">Purpose</p>

Legt einen Primary key an.

<p class="just-emphasize">Anwendung</p>

Befehlszeile, Kommandodatei

<p class="just-emphasize">Berechtigung</p>

AMIC

<p class="just-emphasize">Siehe auch</p>

[CREATE STRUCT](./create_struct_statement.md), [DBFCREATE](./dbfcreate_statement_ab_version_5_0.md), [ALTER STRUCT](./alter_struct_statement.md)

<p class="just-emphasize">Beschreibung</p>

Beim erstmaligen anlegen der Datenbank wurden anstatt PRIMARY KEYS zu verwenden Unique Indexe verwendet. PRIMARY KEYS werden jedoch benötigt um die referenzielle Integrität hinzubekommen ( FOREIGN-KEYS ). Um nun nachträglich die PRIMARY KEYS verwenden zu können, sind mehrere Schritte nötig. Die diese Funktion übernimmt:

Es wird geprüft, ob für diese Relation schon ein PRIMARY KEY existiert. Ist dies der Fall wird die Funktion beendet.

Es wird versucht die Felder zu NOT NULL Feldern zu machen, falls Sie noch keine sind. Funktioniert dies nicht, wird versucht alle Datensätze zu löschen, die eines der Indexfelder NULL haben und danach noch mal die Felder zu NOT NULL zu machen.

Es wird der PRIMARY KEY angelegt.

Der Index wird gedropt.

<p class="just-emphasize">Beispiel</p>

CREATE PRIMARY KEY FROM u0__Fibuv_id_offenerposten
