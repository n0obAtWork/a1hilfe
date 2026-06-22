# Darstellungsart Kalender

<!-- source: https://amic.de/hilfe/kachelkalender.htm -->

Administration > Menü > Dashboard > Variante Kachel

oder

Direktsprung **[DASH]** \> Variante Kachel

Neben den hier beschriebenen Feldern stehen zusätzlich alle Felder aus dem [Basisdesign](./basisdesign.md) zur Verfügung.

| | |
| --- | --- |
| ![](../../../../ImagesExt/image8_1525.jpg) | Kalender  
Der Kalender ist ein Control, welches zur Auswahl eines Stichtages verwendet werden kann. Das Design ist über folgende Felder in der View/Prozedur zu steuern:  
   
• **SelectedDate:** Das Datum, das in der Anzeige als ausgewählt erscheint. Das ausgewählte Datum bestimmt den Monat, der angezeigt wird. Standard ist das Tagesdatum.  
• **Fontname**: Name der Schriftart. Standard ist „Verdana“.  
• **Fontsize**: Größe der Schriftart. Die Größe des Kalenders wird durch die Größe der Schriftart gesteuert. Standard ist 9.  
• **TitleBackColor**: Hintergrundfarbe der Überschrift mit Monat und Jahr.  
• **TitleForeColor**: Vordergrundfarbe der Überschrift mit Monat und Jahr.  
• **TrailingForeColor:** Die Farbe für die Tage, die nicht zum Monat gehören. Standard ist Transparent.  
• **DimensionX** und  
• **DimensionY:** Es besteht die Möglichkeit mehrere Monate nebeneinander und/oder untereinander darzustellen. Standardeinstellung ist 1 für X und 1 für Y. Setzt man z.B. für DimensionX auf 4 und DimensionY auf 3 sieht das Ergebnis folgendermaßen aus:  
![](../../../../ImagesExt/image8_1526.jpg)  
   
Beispielview:  
   
Um eine Datenbankvariable mit dem Stichtag setzen zu können, muss diese dann in der Refresh-Prozedur gesetzt werden. In dem Feld in_Ident1 wird der ausgewählte Tag übergeben.  
Beispiel Refresh-Prozedur:  
   
 |
