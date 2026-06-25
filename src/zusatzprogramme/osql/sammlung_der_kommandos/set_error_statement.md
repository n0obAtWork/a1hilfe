# SET ERROR Statement

<!-- source: https://amic.de/hilfe/seterrorstatement.htm -->

<p class="just-emphasize">Syntax</p>

SET ERROR &#124; CONTINUE &#124;

 &#124; NOCONTINUE&#124;

 &#124; DISPLAY &#124;

 &#124; NODISPLAY &#124;

<p class="just-emphasize">Purpose</p>

Beeinflussung des Verhaltens bei Fehlern.

<p class="just-emphasize">Anwendung</p>

Kommandodatei, Befehlszeile

<p class="just-emphasize">Berechtigung</p>

Alle Anwender

<p class="just-emphasize">Siehe auch</p>

[CONTINUE](./continue_statement.md), [SET OUTERR](./set_outerr_statement.md)

<p class="just-emphasize">Beschreibung</p>

Mit SET ERROR kann das allgemeine Verhalten bei SQL – Fehlern eingestellt werden.

SET ERROR CONTINUE ==> Datei hält nicht bei Fehlern an.

SET ERROR NOCONTINUE ==> Beendung der Kommandodatei bei  
 Fehlern. Dies ist die Standarteinstellung

SET ERROR DISPLAY ==> Fehlerbildschirm wird angezeigt. Dies ist   
 die Standardeinstellung.

SET ERROR NODISPLAY ==> Keine Anzeige von Fehlermeldungen.

<p class="just-emphasize">Beispiel</p>

SET ERROR CONTINUE;

SET ERROR NODISPLAY

Select \* from DIESERELATIONGIBTSNICH;

MSG Hier kam keine Fehlermeldung;

SET ERROR DISPLAY

Select \* from DIESERELATIONGIBTSNICH;

SET ERROR NOCONTINUE;
