# CONNECT Statement

<!-- source: https://amic.de/hilfe/connectstatement.htm -->

#### Syntax

CONNECT userid IDENTIFIED BY password

#### Purpose

Verbindung zur aktiven Datenbank mit einem anderen Benutzer herstellen.

#### Anwendung

Kommandodatei

#### Berechtigung

AMIC

#### Siehe auch

[DISCONNECT](./disconnect_statement.md)

#### Beschreibung

Mit dem CONNECT Statement können sie sich unter einem anderen Benutzer an die Datenbank anmelden. Ist die Kommandodatei beendet, wird automatisch ein Connect auf den ursprünglichen Benutzer durchgeführt. Verwendung findet dieses Statement vor allem beim anlegen von Views, Triggern, Funktionen oder anderen Objekten, die einzelnen Benutzern zugeordnet werden sollen. Im folgenden Beispiel wird die VIEW unter der Hoheit von Admin angelegt und ist somit allgemein gültig. Ohne das vorangegangene CONNECT wäre es in diesem Beispiel nur für den Benutzer zugänglich, der es angelegt hat.

#### Beispiel

CONNECT admin IDENTIFIED BY \*\*\*\*\*\*\*;

Create view op as select \* from offenerposten;

DISCONNECT;

Create view admin.op1 as select \* from offenerposten;
