# Grundsätzliches zur Verwendung von JPP-Objekten

<!-- source: https://amic.de/hilfe/grundstzlicheszurverwendungvon.htm -->

Jedes JPP-Objekt muss instanziiert werden, dabei wird ein Zugriffs-Handle „hdl“ festgelegt.

Alle Methoden und Funktionen werden nun über dieses Handle referenziert.

Nach Abarbeitung der Aktionen muss das Zugriffs-Handel wieder freigegeben werden.

Vor der Ausführung einer Funktion erfolgt meistens eine Parameterzuweisung, deren Inhalt und Anzahl von der auszuführenden Funktion abhängt.

Die JPP-Objektnamen sowie die der auszuführenden Funktionen/Methoden sind in Hochkommatar einzugeben.

Viele der Funktionen liefern einen Rückgabewert (true/false) der in der Programmlogik verwendet werden kann.

Die wichtigsten JPP-Befehle (anhand eines „JDBX“-Objektes)

 jpp_new hdl, "JDBX" neues „JDBX“ JPP-Objekt instanziieren

 jpp_in hdl, "sql" , sql Parameter eingeben 

 jpp_do hdl , "exec" Methode ausführen 

 jpp_delete hdl Handle freigeben
