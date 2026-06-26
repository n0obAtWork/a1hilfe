# MAKRO oder MAKROF Statement

<!-- source: https://amic.de/hilfe/makroodermakrofstatement.htm -->

#### Syntax

MAKRO macroname [PAR1 [PAR2 [PAR3 [PAR4]]]];

#### Purpose

Ausführen eine Pascalskripts

#### Anwendung

Kommandodatei

#### Berechtigung

Alle Anwender

#### Siehe auch

[JPL](./jpl_statement.md)

#### Beschreibung

Wenn die einfachen SQL-Befehle nicht mehr ausreichen, um einen komplexen Sachverhalt abzubilden bzw. zu lösen, kann man auch auf selbstgeschriebene PASCAL–Skripte zurückgreifen. Parameter können, wie unter JPL bzw. dem Pascalinterpreter angegeben werden. MAKRO liest ein in der Datenbank existierendes Skript, MAKROF liest aus einer Datei, die durch den macronamen identifiziert wird.

#### Beispiel

MAKROF c:\\copy.pas 100 200
