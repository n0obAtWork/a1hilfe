# PARAMETER beim Dateiaufruf (OSQL)

<!-- source: https://amic.de/hilfe/parameterbeimdateiaufrufosql.htm -->

Syntax

@Datei CONTINUE ON ERROR

oder

@Datei FORMAT BINARY

oder

@Datei PAR1=wert[,PAR2=wert,…]

Purpose

Übergabe von Parametern an die Kommandodatei

Anwendung

Befehlszeile

Berechtigung

Alle Anwender

Siehe auch

[@](./index.md), [ASK](./alter_struct_statement.md)

Beschreibung

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

Beispiel1

@C:\\SQL\\KUNDUPD.SQL KONTOVON=10000 KONTOBIS=20000

//Die Datei könnte folgendermaßen aussehen

select \* from kundenstamm where kontonummer

 between :KONTOVON and :KONTOBIS

Beispiel2

@C:\\SQL\\KUNDUPD.SQL &DROP

//Die Datei würde folgendermaßen aussehen

&DROP; //Begin

drop table temp_fibu; 

&DROP; // Endet mit selber Marke

&CREATE; // Nur bis hierher wird die Datei ausgfeführt

create table temp_fibu.....
