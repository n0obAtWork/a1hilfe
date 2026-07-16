# Rohwarebelege ansehen

<!-- source: https://amic.de/hilfe/rohwarebelegeansehen.htm -->

Hauptmenü > Rohwarenabrechnung > Rohwarenabrechnung > EK-Rohwarenbearbeitung > Ansicht

Direktsprung **[RWB]**

Hauptmenü > Rohwarenabrechnung > Rohwarenabrechnung > VK-Rohwarenbearbeitung > Ansicht

Direktsprung **[RWBV]**

Mit dieser Funktion können die Daten der ausgewählten Rohwarebelege zur Ansicht gebracht werden. Im Gegensatz zur Korrektur können auch bereits weiterverarbeitete Belege sowie Einzelbelege angezeigt werden, die Teil eines Sammeldruckbelegs sind.

Zur Orientierung befindet sich in der ersten Maskenzeile ein Informationsfeld, dass im Korrektur- oder Ansicht-Modus Angaben über den Beleg-Status ausweist. Hier wird neben der aktuellen Belegstufe (Lieferung, Abschlag, Folgeabschlag, Finale) und zugehörigem Belegdatum gegebenenfalls auch die Sammeldrucknummer nebst Sammeldruckdatum ausgewiesen, wenn der Beleg Teil eines Sammeldrucks ist. Bei Stornobelegen wird das Wort ‚Storno‘ vorangestellt.  
Mit einer im Einrichterparameter ‚*Prozedurname für die freie Anzeige*‘ festlegbaren privaten Datenbankfunktion kann diese Anzeige durch einen durch die Datenbankfunktion zurückgelieferten Text ersetzt werden. Einer solchen privaten Datenbankfunktion wird als Parameter die V_Id des Belegs übergeben. Die Definition muss demnach etwa wie folgt vorgenommen werden:

Create function meineFunktion (in in_v_id integer default 0)  
returns char(256)  
BEGIN  
 declare dc_infotext char(256);

 .

 .

 .

 return dc_infotext; 

END
