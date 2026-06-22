# Environmentvariable

<!-- source: https://amic.de/hilfe/environmentvariable.htm -->

Syntax

%NAMEDERVARIABLEN%

Purpose

Zur Verwendung in Skripten als Variable

Anwendung

Befehlszeile, Kommandodatei

Berechtigung

Alle Anwender

Siehe auch

[Parameter](./parameter_beim_dateiaufruf_osql.md), Variablen

Beschreibung

Man kann in den Kommandodateien auf Envirenmentvariablen zugreifen. Diese werden genau wie Parameter und Variablen gegen deren Inhalt ausgetauscht.

Beispiel

SET OUTFILE %TEMP%\\outfile.txt

Select \*....
