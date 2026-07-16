# CREATE FROM Statement

<!-- source: https://amic.de/hilfe/createfromstatement.htm -->

#### Syntax

CREATE FROM Dateiname;

#### Purpose

Löscht und legt Prozeduren, Views, Trigger und Funktionen an.

#### Anwendung

Befehlszeile, Kommandodatei

#### Berechtigung

Alle Anwender

#### Siehe auch

[COMMAND_DELIMITER](./set_command_delimiter_statement.md)

#### Beschreibung

In Prozeduren, Funktionen und Triggern kann man davon ausgehen, dass als Standard COMMAND_DELIMITER das Semikolon >;&lt; gesetzt ist. Dies führt beim Abarbeiten von Kommandodateien zu Problemen, da standardmäßig auch hier jedes Statement mit >;&lt; endet. Daher ist es Sinnvoll in diesem Fall einen anderen COMMAND_DELIMTER zu setzen. Dies übernimmt diese Funktion, und man vermeidet dadurch unnötige Fehler, weil man vergisst diesen wieder zurückzusetzen. Sie übernimmt zusätzlich auch das löschen, falls diese Prozedur... schon vorhanden war.

#### Beispiel

// Datei C: \\FIBUVORGSTAMM_AFTDEL.SQL;

CREATE TRIGGER FiBuVorgStamm_aftdel

AFTER DELETE ON FiBuVorgStamm

REFERENCING OLD AS alt

FOR EACH ROW

WHEN ( alt.FiBuV_BUCHSTAT!=3 )

BEGIN

 delete from FiBuVorgUngebu

 where FibuV_id = alt.FibuV_id;

END;

//Aufruf aus einer Kommandodatei

CREATE FROM C:\\FIBUVORGSTAMM_AFTDEL.SQL;
