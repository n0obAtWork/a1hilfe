# MAKRO oder MAKROF Statement

<!-- source: https://amic.de/hilfe/makroodermakrofstatement.htm -->

<p class="just-emphasize">Syntax</p>

MAKRO macroname [PAR1 [PAR2 [PAR3 [PAR4]]]];

<p class="just-emphasize">Purpose</p>

Ausführen eine Pascalskripts

<p class="just-emphasize">Anwendung</p>

Kommandodatei

<p class="just-emphasize">Berechtigung</p>

Alle Anwender

<p class="just-emphasize">Siehe auch</p>

[JPL](./jpl_statement.md)

<p class="just-emphasize">Beschreibung</p>

Wenn die einfachen SQL-Befehle nicht mehr ausreichen, um einen komplexen Sachverhalt abzubilden bzw. zu lösen, kann man auch auf selbstgeschriebene PASCAL–Skripte zurückgreifen. Parameter können, wie unter JPL bzw. dem Pascalinterpreter angegeben werden. MAKRO liest ein in der Datenbank existierendes Skript, MAKROF liest aus einer Datei, die durch den macronamen identifiziert wird.

<p class="just-emphasize">Beispiel</p>

MAKROF c:\\copy.pas 100 200
