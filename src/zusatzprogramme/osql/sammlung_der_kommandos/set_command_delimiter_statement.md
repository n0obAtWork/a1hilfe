# SET COMMAND_DELIMITER Statement

<!-- source: https://amic.de/hilfe/setcommanddelimiterstatement.htm -->

<p class="just-emphasize">Syntax</p>

SET COMMAND_DELIMMTER [?]

<p class="just-emphasize">Purpose</p>

Legt das Zeilenendekennzeichen fest.

<p class="just-emphasize">Anwendung</p>

Kommandodatei, Befehlszeile

<p class="just-emphasize">Berechtigung</p>

Alle Anwender

<p class="just-emphasize">Siehe auch</p>

[SET DELIMITER](./set_delimiter_statement.md)

<p class="just-emphasize">Beschreibung</p>

Im Normalfall ist der COMMAND_DELIMITER das Semikolon >;&lt;. Es kann aber Fälle geben, in denen es Sinnvoll ist, dieses umzudefinieren ( z.B.: beim Anlegen von Prozeduren). Dies erfolgt durch diesen Befehl. Im unten angegebenen Beispiel gilt nach dem ändern des COMMAND_DELIMITER das gesamte Create - Statement bis zum nächsten # als ein Statement. Ohne dies wäre nach dem Semikolon Ende und Sybase würde einen Fehler zurückliefern, da das „END“ fehlt.

 Gibt man kein neues Zeichen an, wird wieder das Ursprüngliche >;&lt; genommen.

<p class="just-emphasize">Beispiel</p>

SET COMMAND_DELIMITER #;

CREATE TRIGGER FiBuVorgStamm_aftdel

AFTER DELETE ON FiBuVorgStamm

REFERENCING OLD AS alt

FOR EACH ROW

WHEN ( alt.FiBuV_BUCHSTAT!=3 )

BEGIN

 delete from FiBuVorgUngebu where FibuV_id=alt.FibuV_id;

END#

SET COMMAND_DELIMITER#

EXIT;
