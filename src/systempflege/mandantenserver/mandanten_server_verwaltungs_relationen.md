# Mandanten Server – Verwaltungs-Relationen

<!-- source: https://amic.de/hilfe/mandantenserververwaltungsrela.htm -->

| Relation | |
| --- | --- |
| MandserProzesse | Hier werden die Prozesse mit ID abgelegt, welche vom Mandanten Server verarbeitet werden.  
    
 |
| MandserStatus | Diese Relation dient der Speicherung von statistischen Daten während eines Laufes des Mandantenservers (Fehler, Anzahl abgearbeiteter Vorgänge etc.).  
    
 |
| Locker | Diese Relation vermerkt alle gelockten Datensätze des Systems mittels des Relationsnamens und des Schlüsselfeldes.  
   
Der blockierende Eintrag wird über die Login-Identifikation identifiziert.  
    
 |
| Datenstrom | Im Datenstrom findet sich ALLES wieder, was Datenaustausch zur Folge hat, beispielsweise alle Buchungs- oder Wertstellungs-Mechanismen, Stammdaten-Änderungen, wenn sie auch "extern" (in anderen Mandanten oder in Fremdprogrammen) Auswirkungen haben sollen. etc.  
   
Der Datenstrom-Verteiler oder auch "Mandanten-Server" liest diese Sätze und verarbeitet sie mittels definierter Methoden.  
    
 |
| MandserProzessliste | Liste der Prozesse, welche vom Mandantenserver gehandhabt werden.  
    
 |
