# Weiterverarbeitung der eingelesenen Daten

<!-- source: https://amic.de/hilfe/weiterverarbeitungdereingelese.htm -->

Es folgen einige Schritte zur Validierung der Daten.

Schlägt die Validierung des Lieferscheindatums (bei der nur die kalendarische Gültigkeit und Lesbarkeit des Datums geprüft wird) fehl, wird der Datensatz mit folgender Meldung im Fehlerprotokoll abgewiesen: „LiefDat ungült. [...], Datei [...], Übern, #..., Zl. #...“

Ansonsten wird auch das Geschäftsjahr dahingehend überprüft, ob es sich im Geschäftsjahrstamm befindet. Ein Fehler bewirkt folgenden Eintrag ins Fehlerprotokoll: „Geschäftsjahr falsch [...], Datei [...], Übern. #..., Zl. #...“ und die Abweisung des Datensatzes.

Anschließend wird die Menge auf 0 getestet. Trifft dies zu, wird folgender Satz ins Fehlerprotokoll geschrieben: „Menge 0 [...], Datei [...], Übern. #..., Zl. #...“ und der Datensatz abgewiesen.

Hat bis hierhin alles geklappt werden bei Zielansprache ungleich UMLAGERUNG die Zugangslagernummer und der Zugangslagerplatz auf 0 gesetzt.

Nun wird eine eindeutige SatzId vergeben.

Bei Zielansprache &lt;> CEREA wird ein Datensatz in die Relation VorgangUebergabe eingefügt (Status: SKRIPT_LAEUFT (0)).

Bei Zielansprache = CEREA werden zusätzlich die Qualitäten eingelesen und in die Relation RohwareZusatzQualitaet_Waage eingefügt.

Ist in einem der Parameter QUALxx der Wert1 oder Wert2 =0 oder der Parameter inaktiv geschaltet, so wird die entsprechende Qualität nicht ausgewertet. In Wert3 steht die laufende Nummer der Qualität. Es darf keine 2 aktiven QUALxx-Parameter geben, bei denen die Wert3-Inhalte übereinstimmen! Sonst erfolgt folgende Fehlermeldung: DB-Error #...: PutQualitaeten, [...], Übern. #..., SatzId #..., Zl. #...

(Positionsparameter QUAL01 ... QUAL15)

Dann wird ein Datensatz in die Relation RohwareHauptsatz_Waage eingefügt (Status: SKRIPT_LAEUFT (0)).

Schleifenende – zurück zum Schleifenanfang – weiter mit dem nächsten Datensatz

Hat bis hierhin alles geklappt, wird der Status umgesetzt. Dies ist abhängig vom Parameter UEB_NUR_KOMLETT. Ist UEB_NUR_KOMLETT=1, wird jeder Satz einer Übergabe als defekt markiert, sobald nur ein einziger Satz wegen eines Fehlers abgewiesen wurde. Ansonsten werden alle übertragenen Sätze als korrekt markiert (Status: SKRIPT_OK (2).
