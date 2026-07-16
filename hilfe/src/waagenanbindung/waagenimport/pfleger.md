# Pfleger

<!-- source: https://amic.de/hilfe/pfleger.htm -->

Der Pfleger wird durch folgende JPL-Prozeduren und JAM-Masken realisiert:

SKRIPTPA.JPL, SCRIPTDE.JPL

SKRIPTPA.JAM, SCRIPTDE.JAM (erstellt von NW 29.7.98)

Aufruf in Aeins-Funktionen:

Die JPL-Funktionen werden als Anwendfunktionen in unten dargestellter Weise eingerichtet.

![](../../ImagesExt/image8_534.jpg)  

Bearbeitungsrechte

Der Systemadministrator darf alles.

Die Bearbeitungsrechte an Script-Parameter werden restriktiv gehandhabt.

Folgende Rechte werden ausgewertet:

ENTWICKLER darf:

Alles

Otto Normalbenutzer mit OPT\* darf:

ScriptPBesitzer=0: Ändern ScriptPPWert1-3, ScriptPPAktiv

ScriptPBesitzer=1: alles

Allgemein gilt:

Bei ScriptPId beginnend mit "p_" wird automatisch ScriptPBesitzer=1 (privat) und ScriptSystem=0,

Bei ScriptPId nicht beginnend mit "p_" wird automatisch ScriptPBesitzer=0.

Bei Berechtigungsstufe *Otto Normalbenutzer* wird automatisch ScriptPBesitzer=1 und ScriptPId nur beginnend mit"p_" zugelassen.

Bei ScriptPId beginnend mit"p_" ist ScriptSystem=1 verboten.

**Anmerkung:** ScriptSystem=1 wird aus dem gewähltem Kopf geerbt und ist im Detailbereich nur bearbeitbar, wenn es im Kopfsatz nicht gesetzt ist. Wenn man keine Bearbeitungsrechte besitzt, besteht mindestens die Möglichkeit, die Daten anzusehen.

\*„Normale“ Bediener, die das Recht erhalten sollen, private Script-Parameter zu bearbeiten, müssen unter Direktsprung [OPT] einen Eintrag der nachstehenden Art erhalten.

![](../../ImagesExt/image8_535.jpg)  

Auslesen von Parametern in Pascal-Scripten

Das Einlesen der Parameter geschieht mit folgender Funktion:

**ReadScriptParam** (ScriptPPId, ScriptPId, ScriptPPWert1, ScriptPPWert2, ScriptPPWert3 : string) : integer;

Die Stringvariablen sollten mit mindestens 51 Stellen deklariert und mit Leerstrings initialisiert werden.
