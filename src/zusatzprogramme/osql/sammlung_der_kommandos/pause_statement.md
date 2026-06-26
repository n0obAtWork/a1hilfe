# PAUSE Statement

<!-- source: https://amic.de/hilfe/pausestatement.htm -->

#### Syntax

PAUSE Text, der angezeigt werden soll;

#### Purpose

Öffnen einer Messagebox

#### Anwendung

Kommandodatei

#### Berechtigung

Alle Anwender

#### Siehe auch

[MSG](./msg_statement.md)

#### Beschreibung

Hier kann ein Text angezeigt werden. Während die Dialogbox offen ist, ist die Ausführung der Datei unterbrochen, bis sie mit OK bestätigt werden. Innerhalb des Textes können auch Parameter bzw. mit ASK abgefragte Variablen angezeigt werden.

Ist identisch zum MSG Statement

#### Beispiel

ASK Welche V_ID>ID;

PAUSE Es wurde :ID eingegeben!;
