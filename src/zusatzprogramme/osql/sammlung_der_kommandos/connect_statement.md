# CONNECT Statement

<!-- source: https://amic.de/hilfe/connectstatement.htm -->

<p class="just-emphasize">Syntax</p>

CONNECT userid IDENTIFIED BY password

<p class="just-emphasize">Purpose</p>

Verbindung zur aktiven Datenbank mit einem anderen Benutzer herstellen.

<p class="just-emphasize">Anwendung</p>

Kommandodatei

<p class="just-emphasize">Berechtigung</p>

AMIC

<p class="just-emphasize">Siehe auch</p>

[DISCONNECT](./disconnect_statement.md)

<p class="just-emphasize">Beschreibung</p>

Mit dem CONNECT Statement können sie sich unter einem anderen Benutzer an die Datenbank anmelden. Ist die Kommandodatei beendet, wird automatisch ein Connect auf den ursprünglichen Benutzer durchgeführt. Verwendung findet dieses Statement vor allem beim anlegen von Views, Triggern, Funktionen oder anderen Objekten, die einzelnen Benutzern zugeordnet werden sollen. Im folgenden Beispiel wird die VIEW unter der Hoheit von Admin angelegt und ist somit allgemein gültig. Ohne das vorangegangene CONNECT wäre es in diesem Beispiel nur für den Benutzer zugänglich, der es angelegt hat.

<p class="just-emphasize">Beispiel</p>

CONNECT admin IDENTIFIED BY \*\*\*\*\*\*\*;

Create view op as select \* from offenerposten;

DISCONNECT;

Create view admin.op1 as select \* from offenerposten;
