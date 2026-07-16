# EXIT Statement

<!-- source: https://amic.de/hilfe/exitstatement.htm -->

#### Syntax

EXIT;

#### Purpose

Beendet eine Kommandodatei;

#### Anwendung

Kommandodatei

#### Berechtigung

Alle Anwender

#### Siehe auch

[GOTO](./goto_und_label_statement.md), [IF](./if_statement.md)

#### Beschreibung

Es kann wünschenswert sein, eine Kommandodatei vor dem Dateiende zu verlassen. Hierzu dient der Befehl EXIT;

#### Beispiel

Select \* from fibuvorgstamm where fibuv_nummer is NULL;

IF (VAL(DBERR)!=0) // Keine Daten gefunden

{

 EXIT;

}
