# JPL Statement

<!-- source: https://amic.de/hilfe/jplstatement.htm -->

#### Syntax

JPL namederprozedur

#### Purpose

Ruft eine JPL – Prozedur auf.

#### Anwendung

Kommandodatei, Befehlszeile

#### Berechtigung

Alle Anwender

#### Siehe auch

[MAKRO](./makro_oder_makrof_statement.md), [^](./prototyped_funktion.md)

#### Beschreibung

Wenn die einfachen SQL-Befehle nicht mehr ausreichen um einen komplexen Sachverhalt abzubilden bzw. zu lösen, kann man auch auf selbstgeschriebene JPL-Prozeduren zurückgreifen. Parameter können wie unter JPL angegeben werden. Will man dann auf ein Ergebnis dieser Prozedur zurückgreifen, kann dies zurzeit nur über LDB_Variablen geschehen und zwar in der Form : IF ( VAL(TRANSFER)==1 ).....

#### Beispiel

JPL zinsrecalc :KTO;

IF (VAL(TRANSFER)==0)

{

 PAUSE Zinssaldo für Konto :KTO erfolgreich errechnet!;

 EXIT;

}

update fibuvorgposition ......
