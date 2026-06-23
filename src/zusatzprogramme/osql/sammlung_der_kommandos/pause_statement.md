# PAUSE Statement

<!-- source: https://amic.de/hilfe/pausestatement.htm -->

<p class="just-emphasize">Syntax</p>

PAUSE Text, der angezeigt werden soll;

<p class="just-emphasize">Purpose</p>

Öffnen einer Messagebox

<p class="just-emphasize">Anwendung</p>

Kommandodatei

<p class="just-emphasize">Berechtigung</p>

Alle Anwender

<p class="just-emphasize">Siehe auch</p>

[MSG](./msg_statement.md)

<p class="just-emphasize">Beschreibung</p>

Hier kann ein Text angezeigt werden. Während die Dialogbox offen ist, ist die Ausführung der Datei unterbrochen, bis sie mit OK bestätigt werden. Innerhalb des Textes können auch Parameter bzw. mit ASK abgefragte Variablen angezeigt werden.

Ist identisch zum MSG Statement

<p class="just-emphasize">Beispiel</p>

ASK Welche V_ID>ID;

PAUSE Es wurde :ID eingegeben!;
