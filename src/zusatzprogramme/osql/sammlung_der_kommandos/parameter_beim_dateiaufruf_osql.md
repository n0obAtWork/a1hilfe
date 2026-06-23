# PARAMETER beim Dateiaufruf (OSQL)

<!-- source: https://amic.de/hilfe/parameterbeimdateiaufrufosql.htm -->

<p class="just-emphasize">Syntax</p>

@Datei CONTINUE ON ERROR

oder

@Datei FORMAT BINARY

oder

@Datei PAR1=wert[,PAR2=wert,…]

<p class="just-emphasize">Purpose</p>

Übergabe von Parametern an die Kommandodatei

<p class="just-emphasize">Anwendung</p>

Befehlszeile

<p class="just-emphasize">Berechtigung</p>

Alle Anwender

<p class="just-emphasize">Siehe auch</p>

[@](./index.md), [ASK](./alter_struct_statement.md)

<p class="just-emphasize">Beschreibung</p>

CONTINUE ON ERROR

bewirkt, dass die Kommandodatei bei SQL-Felern nicht abbricht. Siehe [SET ERROR](./set_error_statement.md).

FORMAT BINARY

öffnet die Datei im Binärmodus. Im Binärmodus werden Sonderzeichen nicht als Steuerzeichen interpretiert.

PAR1=….

Um nicht für jede Situation eine neue Kommandodatei schreiben zu müssen, kann man über Parameter diese Variabler gestallten.

Es gibt noch eine zusätzliche Art von Parameter, mit deren Hilfe man die Kommandodatei in mehrere Bereiche unterteilen kann. Diese Parameter beginnen mit &, > oder &lt; . Man kann in der Kommandodatei Marken setzen die mit & beginnen, und über die Parameter diese Bereche dann ausführen lassen:

&NAME ==> Nur das was zwischen &NAME und &NAME steht.

\>NUMMER ==> Nummer sollten Numerisch sein. Alle ab der ersten Marke die kleiner als der Parameter sind.

&lt; NUMMER ==> Nummer sollten Numerisch sein. Alle ab der ersten Marke die größer als der Parameter sind.

<p class="just-emphasize">Beispiel1</p>

@C:\\SQL\\KUNDUPD.SQL KONTOVON=10000 KONTOBIS=20000

//Die Datei könnte folgendermaßen aussehen

select \* from kundenstamm where kontonummer

 between :KONTOVON and :KONTOBIS

<p class="just-emphasize">Beispiel2</p>

@C:\\SQL\\KUNDUPD.SQL &DROP

//Die Datei würde folgendermaßen aussehen

&DROP; //Begin

drop table temp_fibu; 

&DROP; // Endet mit selber Marke

&CREATE; // Nur bis hierher wird die Datei ausgfeführt

create table temp_fibu.....
