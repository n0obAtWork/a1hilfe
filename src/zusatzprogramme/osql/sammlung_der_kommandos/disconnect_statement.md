# DISCONNECT Statement

<!-- source: https://amic.de/hilfe/disconnectstatement.htm -->

<p class="just-emphasize">Syntax</p>

DISCONNECT;

<p class="just-emphasize">Purpose</p>

Stellt die ursprüngliche Verbindung zur Datenbank wieder her.

<p class="just-emphasize">Anwendung</p>

Kommandodatei

<p class="just-emphasize">Berechtigung</p>

AMIC

<p class="just-emphasize">Siehe auch</p>

[CONNECT](./connect_statement.md)

<p class="just-emphasize">Beschreibung</p>

Hat man sich per CONNECT als anderer Benutzer an die Datenbank angemeldet, hebt DISCONNECT diese Verbindung wieder auf und meldet den ursprünglichen Benutzer wieder an.

<p class="just-emphasize">Beispiel</p>

CONNECT admin IDENTIFIED BY \*\*\*\*\*\*\*;

CREATE VIEW FIBUBELEG as select \* from fibuvorgstamm;

DISCONNECT;
