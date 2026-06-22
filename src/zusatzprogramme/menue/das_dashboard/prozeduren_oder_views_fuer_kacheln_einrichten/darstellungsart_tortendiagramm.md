# Darstellungsart Tortendiagramm

<!-- source: https://amic.de/hilfe/kacheltortendiagramm.htm -->

Administration > Menü > Dashboard > Variante Kachel

oder

Direktsprung **[DASH]** \> Variante Kachel

Neben den hier beschriebenen Feldern stehen zusätzlich alle Felder aus dem [Basisdesign](./basisdesign.md) zur Verfügung.

| | |
| --- | --- |
| ![Ein Bild, das Diagramm, Tortendiagramm enthält. Automatisch generierte Beschreibung](../../../../ImagesExt/image8_1499.png "Ein Bild, das Diagramm, Tortendiagramm enthält. Automatisch generierte Beschreibung") | Tortendiagramm  
In einem Tortendiagramm können bis zu zehn Datensätze („Tortenstücke“) angezeigt werden. Der Wert und die Bezeichnung des Datensatzes werden in der View/Prozedur mit den Feldern **Wert** und **Label** angegeben.  
Im Tortendiagramm besteht die Möglichkeit kleinere Tortenstücke in einem einzelnen Tortenstück („Sonstige“) zusammenzufassen. Dazu wird in der View/Prozedur dem Feld **OthersCategoryInPercent** ein Wert größer 0 zugewiesen. Mit diesem Wert gibt man eine Schwelle an, unter der alle Tortenstücke zusammengefasst werden. Beispiel:  
In der View wird für das Feld OthersCategoryInPercent eine 2 angegeben. Dann werden alle Datensätze, die weniger als 2% ausmachen, in dem Tortenstück „Sonstige“ zusammengefasst.  
Hinweis:  
*Auf dem Tortenstück „Sonstige“ kann keine Klick-Funktion ausgeführt werden. Des Weiteren wird im Tooltip nur der Text „Sonstige“ angezeigt.*  
   
Legende  
Mithilfe des Feldes **LegendVisible** kann eingestellt werden, ob die Legende standardmäßig ein- oder ausgeblendet ist. Unabhängig von dieser Option kann die Legende über die Funktion ***Legende ein-/ausblenden*** (rechte Maustaste auf der Kachel) aktiviert bzw. deaktiviert werden. Des Weiteren ist die Position (**LegendPosition**) und die Ausrichtung (**LegendOrientation**) der Legende über die View/Prozedur einstellbar. Mögliche Werte sind:  
LegendPosition  
• Right  
• Left  
• Bottom  
• Top  
LegendOrientation  
• Vertical  
• Horizontal  
   
Hinweis:  
*Im Tortendiagramm besteht die Möglichkeit die Klick-Funktion über die Legende auszuführen.*  
   
Tooltipp  
Mit dem Feld **SliceTooltip** kann der Tooltip über HTML formatiert werden. Der Tooltip erscheint, wenn der Mauszeiger über einen Datenpunkt des Diagramms bewegt wird. Existiert das Feld **SliceTooltip** nicht in der View/Prozedur, so wird der Tooltip nicht angezeigt.  
   
Beispielview:  
   
 |
