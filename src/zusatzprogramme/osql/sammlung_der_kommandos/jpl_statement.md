# JPL Statement

<!-- source: https://amic.de/hilfe/jplstatement.htm -->

<p class="just-emphasize">Syntax</p>

JPL namederprozedur

<p class="just-emphasize">Purpose</p>

Ruft eine JPL – Prozedur auf.

<p class="just-emphasize">Anwendung</p>

Kommandodatei, Befehlszeile

<p class="just-emphasize">Berechtigung</p>

Alle Anwender

<p class="just-emphasize">Siehe auch</p>

[MAKRO](./makro_oder_makrof_statement.md), [^](./prototyped_funktion.md)

<p class="just-emphasize">Beschreibung</p>

Wenn die einfachen SQL-Befehle nicht mehr ausreichen um einen komplexen Sachverhalt abzubilden bzw. zu lösen, kann man auch auf selbstgeschriebene JPL-Prozeduren zurückgreifen. Parameter können wie unter JPL angegeben werden. Will man dann auf ein Ergebnis dieser Prozedur zurückgreifen, kann dies zurzeit nur über LDB_Variablen geschehen und zwar in der Form : IF ( VAL(TRANSFER)==1 ).....

<p class="just-emphasize">Beispiel</p>

JPL zinsrecalc :KTO;

IF (VAL(TRANSFER)==0)

{

 PAUSE Zinssaldo für Konto :KTO erfolgreich errechnet!;

 EXIT;

}

update fibuvorgposition ......
