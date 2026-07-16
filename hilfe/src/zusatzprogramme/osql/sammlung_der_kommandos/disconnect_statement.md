# DISCONNECT Statement

<!-- source: https://amic.de/hilfe/disconnectstatement.htm -->

#### Syntax

DISCONNECT;

#### Purpose

Stellt die ursprüngliche Verbindung zur Datenbank wieder her.

#### Anwendung

Kommandodatei

#### Berechtigung

AMIC

#### Siehe auch

[CONNECT](./connect_statement.md)

#### Beschreibung

Hat man sich per CONNECT als anderer Benutzer an die Datenbank angemeldet, hebt DISCONNECT diese Verbindung wieder auf und meldet den ursprünglichen Benutzer wieder an.

#### Beispiel

CONNECT admin IDENTIFIED BY \*\*\*\*\*\*\*;

CREATE VIEW FIBUBELEG as select \* from fibuvorgstamm;

DISCONNECT;
