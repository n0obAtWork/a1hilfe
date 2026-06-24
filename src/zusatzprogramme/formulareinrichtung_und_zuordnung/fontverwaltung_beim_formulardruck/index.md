# Fontverwaltung beim Formulardruck

<!-- source: https://amic.de/hilfe/fontverwaltungbeimformulardruc.htm -->

Bisher standen beim Formulardruck im Windows-Modus nur zwei Fonts zur Verfügung, die im Druckerstamm hinterlegt werden. Aus diesen Fonts wurden die im Asciidruck möglichen Attribute (normal, compress, fett, gesperrt) nachgebildet. Es ist nun möglich, in einem Formular beliebig viele Fonts zu benutzen! Fonts werden in einer [Fonttabelle](../font_tabellen_windowsdruck_f9/index.md) hinterlegt und im [Formularstamm kann dann eine Fontabelle](../der_formular_pfleger/font_zuordnung_zum_formular.md) ausgewählt werden. In den Druckpositionen kann unter Details in einem neuen Feld ein Font aus der zugeordneten Fonttabelle gezogen werden. Da die bisherige Windowsdruckvariante weiterhin bestehen bleibt, wird die Druckvariante nach Folgendem Schema bestimmt.   
1\. Druckerstamm nicht auf Windowsdruck:  
 - es wird im ASCII Druckmodus gearbeitet  
2\. Druckerstamm auf Windowsdruck gestellt:  
- Formularstamm hat Fonttabellennummer 0 -> alter Windowsdruck  
- Formularstamm hat Fonttabellennummer ungleich 0 -> neuer Windowsdruck.   
Beim Druck mit einer Fonttabelle werden die im Druckerstamm hinterlegten Fonts NICHT ausgewertet; nur die Fontangaben aus der Fonttabelle sind maßgeblich!   
    

Komponenten der Fonttabelle 

Basisfont: 

Dieser Font wird zunächst zur Bestimmung des Positionierungsrasters bestimmt, d.h. die Größe dieses Fonts legt fest, wie viele Spalten und Zeilen auf dem Formular bedruckt werden können (großer Font = weniger Spalten / Zeilen, kleiner Font = mehr Spalten / Zeilen).  
Die Positionierung eine Druckposition (wie im Formulareinrichter unter Spalte / Zeile hinterlegt) richtet sich also nach diesem Raster, gleichgültig, mit welchem Font eine Druckposition tatsächlich gedruckt wird! Der Basisfont dient ferner auch als Ersatzfont für alle Druckpositionen, denen kein Font zugeordnet wurde. 

X-Skalierung / Y-Skalierung: 

Es hat sich gezeigt, dass das aus dem Basisfont bestimmte Raster manchmal zu eng oder zu groß ist. Mit diesen Faktoren kann das Raster in feineren Schritten gestaucht (kleiner 1) oder gestreckt (> 1) werden. 

Reduzierung der Fontgröße um...:

Im Formulareinrichter kann ein gesamter Druckbereich als 'COMPRESS' markiert werden. Die hier angegebene Zahl wird bei einem solchen Bereich von allen verwendeten Fonts von der Größe abgezogen - der Font wird also künstlich kleiner gemacht.  
ACHTUNG: die Zahl wird positiv eingegeben!  
Z.B. bedeutet hier eine 2: aus 'Arial-10-fett' wird 'Arial-8-fett’. 

Benutzbare Fonts: 

Hier trägt man die zu verwendenden Fonts ein.  
Unter Bezeichnung / Verwendung tragen sie bitte eine sinnvolle Beschreibung des Fonts ein, sie wird im Formulareinrichter hinter der Fontnummer angezeigt!  
In der Spalte FONT können Sie per Hand Eintragungen vornehmen oder per F3 aus den auf ihrem System hinterlegten Fonts auswählen.  
Es wird folgende Notation benutzt: Fontname - Größe [ - Attribute] Fontname und FGröße müssen angegeben werden. Optional jeweils durch '-' getrennt sind folgende Attribute möglich: 'fett', 'kursiv', 'unterstrichen'. Die Optionen 'durchgestrichen' und 'Farbe' (aus der Fontdialogbox von Windows (F3)) werden nicht unterstützt!   
    

Allgemeine Hinweise:  
Die neue Fontverwaltung bietet viele gestalterische Möglichkeiten. Man sollte jedoch immer folgendes beachten:  
Die derzeitige Archivierung des Formulardrucks wird immer im ASCII-Modus durchgeführt.  
Man achte also darauf, dass das gestaltete Formular auch im ASCII Modus 'aussagefähig' bleibt. Insbesondere ist darauf zu achten, dass keine Überlappungen von Druckpositionen auftreten, da diese im Windowsdruck zwar sichtbar bleiben, im ASCII-Modus aber echt überdeckend wirken und es somit zu einem Informationsverlust kommen kann. Fonts mit variabler Zeichenbreite führen zu einer unterschiedlichen Drucklänge im Vergleich zu Fonts mit fester Zeichenbreite. Unter Details einer Druckposition gibt es hierfür einen neuen Schalter 'rechtsbündig drucken'. Er vereinfacht die spaltengerechte Ausrichtung von Zahlen und Texten insbesondere beim Windowsdruck (funktioniert auch im ASCII Modus).   
Zur Darstellung von Linien wurden in allen Druckbereichen zwei neue Druckpositionen hinzugefügt: Einfache / doppelte Linie. Der Parameter 'Länge' gibt an, wie lang die Linie werden soll. Im ASCII-Modus werden die Zeichen '-' für einfache Linie, '=' für doppelte Linie entsprechend der Länge wiederholt. Im Windows-Modus werden richtige Linien 'gemalt'. Somit ergibt sich in beiden DruckModi ein etwa identischer optischer Eindruck bezüglich der Länge!  
Zum Experimentieren und nur unter Windows: Die Linien werden zu Kästen, wenn unter Details die Blocklänge > 0 ist. Die Blocklänge gibt an, wie viele Zeilen innerhalb des Kastens liegen!

<p class="siehe-auch">Siehe auch:</p>

- [Einrichtungen kopieren](./einrichtungen_kopieren.md)
- [Einrichtungen transportieren](./einrichtungen_transportieren.md)
