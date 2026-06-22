# SET DELIMITER Statement

<!-- source: https://amic.de/hilfe/setdelimiterstatement.htm -->

Syntax

SET DELIMITER [?]

Purpose

Legt das Trennzeichen in Ausgabedateien fest;

Anwendung

Kommandodatei, Befehlszeile

Berechtigung

Alle Anwender

Siehe auch

[SET COMMAND_DELIMITER](./set_command_delimiter_statement.md)

Beschreibung

Im Normalfall ist das Trennsymbol zwischen den Datenspalten das Leerzeichen> &lt;. Will man dies umdefinieren so geschieht dies durch diesen Befehl. Es wird erst durch erneutes setzen zurückdefiniert;

Beispiel

SET DELIMITER ,;

Select \* from fibuforgklasse;

SET DELIMITER;
