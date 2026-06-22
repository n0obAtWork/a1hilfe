# Darstellungsart Fortschrittsbalken

<!-- source: https://amic.de/hilfe/kachelfortschrittsbalken.htm -->

Administration > Menü > Dashboard > Variante Kachel

oder

Direktsprung **[DASH]** \> Variante Kachel

Neben den hier beschriebenen Feldern stehen zusätzlich alle Felder aus dem [Basisdesign](./basisdesign.md) zur Verfügung.

| | |
| --- | --- |
| ![](../../../../ImagesExt/image8_1494.png) | Fortschrittsbalken  
Der Fortschrittsbalken benötigt zusätzlich zu den Feldern, die auch die Darstellungsart Text haben, noch die Felder, die den ihn beschreiben:  
ProgressbarMinimum, muss den Datenbanktypen integer liefern. Standard ist 0.  
ProgressbarMaximum, muss den Datenbanktypen integer liefern. Standard ist 100.  
**ProgressbarValue**, muss den Datenbanktypen integer liefern. Der Wert sollte zwischen Minimum und Maximum liegen.  
ProgressbarText (Optional). Wenn nicht angegeben, so wird „{nnn}% vom {ProgressbarMaximum}“ ausgegeben  
   
Beispielview:  
   
 |
