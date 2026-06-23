# CONTINUE Statement

<!-- source: https://amic.de/hilfe/continuestatement.htm -->

<p class="just-emphasize">Syntax</p>

CONTINUE;

<p class="just-emphasize">Purpose</p>

Beeinflussung des Abbruchs bei Fehlern.

<p class="just-emphasize">Anwendung</p>

Kommandodatei

<p class="just-emphasize">Berechtigung</p>

Alle Anwender

<p class="just-emphasize">Siehe auch</p>

[SET ERROR](./set_error_statement.md)

<p class="just-emphasize">Beschreibung</p>

Mit CONTINUE könne Sie dafür sorgen, dass das folgende Statement nicht zum beenden der Ausführung ihrer Kommandodatei führt. Dies ist zum Beispiel praktisch beim Anlegen von Feldern oder Tabellen, bei denen es möglich ist, das diese schon existieren. Nach der Ausführung des Folgestatements wird wieder der alte Zustand hergestellt, der standardmäßig auf „Abbruch bei Fehler“ steht.

<p class="just-emphasize">Beispiel</p>

CONTINUE;

Select \* from DIESERELATIONGIBTSNICH;

MSG Oh, hier geht’s ja weiter;

Select \* from DIESERELATIONGIBTSNICH;

MSG Hier komme ich nicht an;

CONTINUE ON ERROR;
