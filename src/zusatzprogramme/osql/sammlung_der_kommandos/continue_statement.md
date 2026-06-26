# CONTINUE Statement

<!-- source: https://amic.de/hilfe/continuestatement.htm -->

#### Syntax

CONTINUE;

#### Purpose

Beeinflussung des Abbruchs bei Fehlern.

#### Anwendung

Kommandodatei

#### Berechtigung

Alle Anwender

#### Siehe auch

[SET ERROR](./set_error_statement.md)

#### Beschreibung

Mit CONTINUE könne Sie dafür sorgen, dass das folgende Statement nicht zum beenden der Ausführung ihrer Kommandodatei führt. Dies ist zum Beispiel praktisch beim Anlegen von Feldern oder Tabellen, bei denen es möglich ist, das diese schon existieren. Nach der Ausführung des Folgestatements wird wieder der alte Zustand hergestellt, der standardmäßig auf „Abbruch bei Fehler“ steht.

#### Beispiel

CONTINUE;

Select \* from DIESERELATIONGIBTSNICH;

MSG Oh, hier geht’s ja weiter;

Select \* from DIESERELATIONGIBTSNICH;

MSG Hier komme ich nicht an;

CONTINUE ON ERROR;
