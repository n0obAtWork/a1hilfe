# MSG Statement

<!-- source: https://amic.de/hilfe/msgstatement.htm -->

#### Syntax

MSG Text, der angezeigt werden soll;

#### Purpose

Öffnen einer Messagebox

#### Anwendung

Kommandodatei

#### Berechtigung

Alle Anwender

#### Siehe auch

[PAUSE](./pause_statement.md)

#### Beschreibung

Hier kann ein Text angezeigt werden. Während die Dialogbox offen ist, ist die Ausführung der Datei unterbrochen, bis sie mit OK bestätigt werden. Innerhalb des Textes können auch Parameter bzw. mit ASK abgefragte Variablen angezeigt werden.

Ist Identisch zum PAUSE Statement;

#### Beispiel

```text
ASK Welche V_ID>ID;
MSG Es wurde :ID eingegeben!;
```
