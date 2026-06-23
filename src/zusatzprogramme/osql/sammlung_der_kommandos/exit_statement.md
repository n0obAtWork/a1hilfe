# EXIT Statement

<!-- source: https://amic.de/hilfe/exitstatement.htm -->

<p class="just-emphasize">Syntax</p>

EXIT;

<p class="just-emphasize">Purpose</p>

Beendet eine Kommandodatei;

<p class="just-emphasize">Anwendung</p>

Kommandodatei

<p class="just-emphasize">Berechtigung</p>

Alle Anwender

<p class="just-emphasize">Siehe auch</p>

[GOTO](./goto_und_label_statement.md), [IF](./if_statement.md)

<p class="just-emphasize">Beschreibung</p>

Es kann wünschenswert sein, eine Kommandodatei vor dem Dateiende zu verlassen. Hierzu dient der Befehl EXIT;

<p class="just-emphasize">Beispiel</p>

Select \* from fibuvorgstamm where fibuv_nummer is NULL;

IF (VAL(DBERR)!=0) // Keine Daten gefunden

{

 EXIT;

}
