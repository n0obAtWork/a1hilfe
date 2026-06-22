# Mimetypen in A.eins

<!-- source: https://amic.de/hilfe/mimetypeninaeins.htm -->

Hauptmenü > Administration > Archiv > Mime

Direktsprung **[MIME]**

In dieser Variante werden die verwendbaren Mimetypen aufgelistet. Diese Mimetypen werden mit dem Update von A.eins ausgeliefert.

| Felder | |
| --- | --- |
| Mime | [Multipurpose Internet Mail Extensions](http://de.wikipedia.org/wiki/Multipurpose_Internet_Mail_Extensions) |
| Extension | [Dateinamenserweiterung](http://de.wikipedia.org/wiki/Dateinamenserweiterung)  
(Schlüssel der Relation AMIC_MIME) |
| Archiv-Volltext | Kennzeichen ob ein interner Filter existiert, der aus dem zugehörigen Blob Archivtext generiert |
| Archiv-Vorschau | Kennzeichen ob im Archiv(Strg+F12) eine Vorschau implementiert ist |
| Blob | Kennzeichen ob es sich programm-technisch um einen ["Blob"](http://de.wikipedia.org/wiki/Binary_Large_Object) handelt. |
| Beschreibung | Ebendies. |
| PDF | Kennzeichen ob es sich um [PDF](http://de.wikipedia.org/wiki/PDF) handelt. |
| Link | Kennzeichen (reserviert) |
| Signatur | Kennzeichen ob der Mimetyp im Sinne von A.eins „signaturfähig“ ist. |
| Kennung | Interne A.eins-Kennzeichnung und Kriterium für A.eins-Auslieferung.  
Mimetypen mit einer Kennung kleiner 1000 werden mit „skip on existing“ ausgeliefert.  
   
Die Auslieferungsmethode „On Existing Skip“ bedeutet Kunden haben nach der erstmaligen Auslieferung freie Hand; Entwickler müssen (notwendige) Änderungen per ATF ausliefern! |
| Archiv-Icon | Die Archiv-Anzeigen-Funktionen ermitteln die Icon-Zuordnungen einmalig zur Laufzeit aus der Windows-Systemregistrierung. Dies geschieht deshalb, damit der Anwender „seine“ Programme dort visuell wiederfinden kann.  
Da aber das lesen aus der Windows-Registrierung von Administratoren unterbunden sein kann bzw. diese Windows-Registrierung gar nicht mehr den aktuellen Plattenzustand widerspiegelt (z.B. fehlerhafte Deinstallation von Programmen) musste ein Weg gefunden werden der zumindest ein Default-Icon anzeigt, das im Falle der Nicht-Verfügbarkeit angezeigt werden soll.  
Den Namen der entsprechenden Aeins-Ressource kann hier hinterlegt werden.  
Der Name muss mit der entsprechenden Aeins-Ressource korrelieren. Es besteht keine Möglichkeit ein „eigenes“ Icon zu favorisieren. |
