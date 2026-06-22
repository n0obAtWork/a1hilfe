# Mandantenserver – Prozesse

<!-- source: https://amic.de/hilfe/_mandserprozesse.htm -->

Hauptmenü > Systempflege > Mandantenserver > Mandantenserver Protokoll > Variante „Prozesse im Mandantenserver“

Direktsprung **[MSP]**

Diese Maske dient zur Einrichtung der Mandantenserverprozesse.

| Bezeichnung | Inhalt |
| --- | --- |
| Name | Name des Prozesses  
    
 |
| Control | Controlstring, der ausgeführt wird (bei Typ Synchron und Asynchron (einfach))  
    
 |
| Async-Control | Controlstring, der im Fall eines einfachen Asynchronen Prozesses ausgeführt wird  
    
 |
| Versteckt | Wenn hier „Ja“ steht, wird die Ausführung im Protokoll nicht angezeigt  
    
 |
| Sekunden | Intervall in Sekunden  
    
 |
| Aktiv | JA bedeutet aktiv. Ein Mandantenserverprozess, der auf aktiv geschaltet wird, wird erstmals nach einem Mandantenserverneustart ausgeführt.  
    
 |
| Nur im Wartemodus | Ist hier „Ja“ eingetragen, hat der Prozess niedrige Priorität.  
Er wird nur während einer Ruhephase des Mandantenservers in regelmäßigen Abständen ausgeführt.  
    
 |
| Synchronität | 1 – Synchron – Der Mandantenserver selbst führt den Prozess aus und blockiert für die Dauer des Prozesses weitere Buchungen. Diese Prozesse sollten nie lange dauern!  
   
2- Asynchron (einfach) – Der Mandantenserver führt ein A.eins aus, das den Controlstring abarbeitet. Dieser Prozess wird nicht beaufsichtigt. Ist das Intervall kürzer als die Abarbeitungszeit können eine Reihe von parallelen Prozessen entstehen, die auch die Lebenszeit des Mandantenservers überschreiten.  
   
3\. Asynchron (managed) – Der Mandantenserver startet ein A.eins mit einer gegebenen Maske und den gegebenen Parametern.  
**ACHTUNG! Hier ist der Eintrag im Feld „Controlstring“ kein solcher!**  
Dieser Maskenprozess muss selbst in der Tabelle Mandserprozessliste seine letzte Aktivität nachtragen und prüfen, ob das Stop-Zeichen vom Mandantenserver gesetzt wurde.  
Diese Prozesse sind Dauerläufer und werden nur einmalig gestartet. Im gegebenen Intervall wird überprüft, ob der Prozess noch aktiv ist und sich seit der letzten Intervallzeit zurückgemeldet hat. Falls nicht, wird ein hängender Prozess gekillt, und neu gestartet. |
