# GOTO und :LABEL Statement

<!-- source: https://amic.de/hilfe/gotoundlabelstatement.htm -->

<p class="just-emphasize">Syntax</p>

GOTO label;

.

.

.

:label;

<p class="just-emphasize">Purpose</p>

Befehlsfolgen in Kommandodateien überspringen.

<p class="just-emphasize">Anwendung</p>

Kommandodatei

<p class="just-emphasize">Berechtigung</p>

Alle Anwender

<p class="just-emphasize">Siehe auch</p>

[EXIT](./exit_statement.md), [IF](./if_statement.md)

<p class="just-emphasize">Beschreibung</p>

Eigentlich sind Kommandodateien dafür vorgesehen, sequentiell abgearbeitet zu werden. Nun kann es aber sein, das man bestimmte Befehle nicht oder mehrfach abarbeiten muss. Dazu dient GOTO. Da die Kommandodateien nicht den Anspruch einer Programmiersprache erheben und es sonst keine Befehle für Schleifen gibt, kann man darüber hinwegsehen, dass dies ein unter Programmierern gemiedener Befehl ist.

<p class="just-emphasize">Beispiel</p>

:Nochmal

// Dies ist keine Endlosschleife, da

// ASK bei Escape die Kommandodatei beendet

ASK ID des zu reorganisierenden Datensatzes>ID;

JPL REARG :ID;

GOTO Nochmal;
