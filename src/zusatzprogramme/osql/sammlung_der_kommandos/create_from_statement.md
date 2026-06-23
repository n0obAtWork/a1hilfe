# CREATE FROM Statement

<!-- source: https://amic.de/hilfe/createfromstatement.htm -->

<p class="just-emphasize">Syntax</p>

CREATE FROM Dateiname;

<p class="just-emphasize">Purpose</p>

Löscht und legt Prozeduren, Views, Trigger und Funktionen an.

<p class="just-emphasize">Anwendung</p>

Befehlszeile, Kommandodatei

<p class="just-emphasize">Berechtigung</p>

Alle Anwender

<p class="just-emphasize">Siehe auch</p>

[COMMAND_DELIMITER](./set_command_delimiter_statement.md)

<p class="just-emphasize">Beschreibung</p>

In Prozeduren, Funktionen und Triggern kann man davon ausgehen, dass als Standard COMMAND_DELIMITER das Semikolon >;&lt; gesetzt ist. Dies führt beim Abarbeiten von Kommandodateien zu Problemen, da standardmäßig auch hier jedes Statement mit >;&lt; endet. Daher ist es Sinnvoll in diesem Fall einen anderen COMMAND_DELIMTER zu setzen. Dies übernimmt diese Funktion, und man vermeidet dadurch unnötige Fehler, weil man vergisst diesen wieder zurückzusetzen. Sie übernimmt zusätzlich auch das löschen, falls diese Prozedur... schon vorhanden war.

<p class="just-emphasize">Beispiel</p>

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
