# EDIT Statement

<!-- source: https://amic.de/hilfe/editstatement.htm -->

Syntax

EDIT [Dateiname];

Purpose

Öffnet mit notepad die angegebene Datei

Anwendung

Befehlszeile

Berechtigung

Alle Anwender

Beschreibung

Unter Osql werden desöfteren Daten oder Strukturen in Dateien geschrieben. Das EDIT Statement bietet eine einfache und praktikable Möglichkeit diese Daten direkt zu prüfen bzw. zu bearbeiten.

Beispiel

Create Struct Kontosummen into c:\\Kontosummen.sql;

EDIT c:\\kontosummen.sql;
