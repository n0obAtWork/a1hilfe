# GOTO und :LABEL Statement

<!-- source: https://amic.de/hilfe/gotoundlabelstatement.htm -->

Syntax

GOTO label;

.

.

.

:label;

Purpose

Befehlsfolgen in Kommandodateien überspringen.

Anwendung

Kommandodatei

Berechtigung

Alle Anwender

Siehe auch

[EXIT](./exit_statement.md), [IF](./if_statement.md)

Beschreibung

Eigentlich sind Kommandodateien dafür vorgesehen, sequentiell abgearbeitet zu werden. Nun kann es aber sein, das man bestimmte Befehle nicht oder mehrfach abarbeiten muss. Dazu dient GOTO. Da die Kommandodateien nicht den Anspruch einer Programmiersprache erheben und es sonst keine Befehle für Schleifen gibt, kann man darüber hinwegsehen, dass dies ein unter Programmierern gemiedener Befehl ist.

Beispiel

:Nochmal

// Dies ist keine Endlosschleife, da

// ASK bei Escape die Kommandodatei beendet

ASK ID des zu reorganisierenden Datensatzes>ID;

JPL REARG :ID;

GOTO Nochmal;
