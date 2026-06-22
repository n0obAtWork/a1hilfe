# Nützliche Controlstrings für Funktionen in einer F3-Auswahl (Itembox)

<!-- source: https://amic.de/hilfe/ntzlichecontrolstringsfrfunkti.htm -->

Zu jeder F3-Auswahl kann eine eigene Optionbox mit angegeben sein oder eine private Optionbox mit angehängt werden. An diese können private Funktionen angehängt werden.

| Funktion | Controlstring |
| --- | --- |
| Aufruf einer AIS-Maske mit Übergabe einer Ident. | ^jpl ais_itemcall RETURNWERT AUFRUFART AISGRUPPE  
   
Wobei:  
• RETURNWERT: Dies ist der Name des Datenbankfeldes, aus dem die an AIS übergebene Ident versorgt wird. Dieses Feld muss in der F3-Auswahl enthalten sein.  
• AUFRUFART: 5 für Ändern, 6 für Ansehen.  
• AISGRUPPE: Name der Gruppe  
 |
| Aufruf eine Makros mit Übergabe einer Ident | ^jpl ais_itemmakrocall MAKRO RETURNWERT PARAMETER1 PARAMETER2  
   
Mit Hilfe des RETURNWERTS wird die an AIS zu übergebende Ident aus der F3.-Auswahl bestimmt. Dann wird das Makro mit folgenden Parametern aufgerufen:  
   
call makro( ":MAKRO" , ":Ident aus RETURNWERT" , ":PARAMETER1", ":PARAMETER2")  
 |

Wenn man aus einer F3-Auswahl(Itembox) heraus eine AIS-Maske aufrufen will und dort gleich auf Werte der aktiven Zeile zugreifen will
