# Environmentvariable

<!-- source: https://amic.de/hilfe/environmentvariable.htm -->

<p class="just-emphasize">Syntax</p>

%NAMEDERVARIABLEN%

<p class="just-emphasize">Purpose</p>

Zur Verwendung in Skripten als Variable

<p class="just-emphasize">Anwendung</p>

Befehlszeile, Kommandodatei

<p class="just-emphasize">Berechtigung</p>

Alle Anwender

<p class="just-emphasize">Siehe auch</p>

[Parameter](./parameter_beim_dateiaufruf_osql.md), Variablen

<p class="just-emphasize">Beschreibung</p>

Man kann in den Kommandodateien auf Envirenmentvariablen zugreifen. Diese werden genau wie Parameter und Variablen gegen deren Inhalt ausgetauscht.

<p class="just-emphasize">Beispiel</p>

SET OUTFILE %TEMP%\\outfile.txt

Select \*....
