# SET DELIMITER Statement

<!-- source: https://amic.de/hilfe/setdelimiterstatement.htm -->

<p class="just-emphasize">Syntax</p>

SET DELIMITER [?]

<p class="just-emphasize">Purpose</p>

Legt das Trennzeichen in Ausgabedateien fest;

<p class="just-emphasize">Anwendung</p>

Kommandodatei, Befehlszeile

<p class="just-emphasize">Berechtigung</p>

Alle Anwender

<p class="just-emphasize">Siehe auch</p>

[SET COMMAND_DELIMITER](./set_command_delimiter_statement.md)

<p class="just-emphasize">Beschreibung</p>

Im Normalfall ist das Trennsymbol zwischen den Datenspalten das Leerzeichen> &lt;. Will man dies umdefinieren so geschieht dies durch diesen Befehl. Es wird erst durch erneutes setzen zurückdefiniert;

<p class="just-emphasize">Beispiel</p>

SET DELIMITER ,;

Select \* from fibuforgklasse;

SET DELIMITER;
