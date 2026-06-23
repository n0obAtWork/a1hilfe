# EDIT Statement

<!-- source: https://amic.de/hilfe/editstatement.htm -->

<p class="just-emphasize">Syntax</p>

EDIT [Dateiname];

<p class="just-emphasize">Purpose</p>

Öffnet mit notepad die angegebene Datei

<p class="just-emphasize">Anwendung</p>

Befehlszeile

<p class="just-emphasize">Berechtigung</p>

Alle Anwender

<p class="just-emphasize">Beschreibung</p>

Unter Osql werden desöfteren Daten oder Strukturen in Dateien geschrieben. Das EDIT Statement bietet eine einfache und praktikable Möglichkeit diese Daten direkt zu prüfen bzw. zu bearbeiten.

<p class="just-emphasize">Beispiel</p>

Create Struct Kontosummen into c:\\Kontosummen.sql;

EDIT c:\\kontosummen.sql;
