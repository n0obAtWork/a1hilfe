# Hauptmenü - Menüpunkte

<!-- source: https://amic.de/hilfe/hauptmenmenpunkte.htm -->

Nach Anklicken eines Menüpunktes werden entsprechende A.eins-Aktivitäten ausgelöst.

Hinweis: Gewisse A.eins-Aktivitäten ziehen keine sofortige User-Interaktion nach sich. Gründe hierfür können langwierige Berechnungen bzw. Datenerhebungen sein. Das bedeutet das unter Umständen der Eindruck entstehen mag, dass A.eins nicht reagiert. In späteren A.eins-Releases wird das Hauptauswahlmenü diesen Umstand berücksichtigen und vorbeugend dem A.eins-Anwender eine visuelle Rückmeldung geben das nicht-visuelle A.eins-Aktivitäten durchgeführt werden.

Je nach Aktivität(\*) gibt es folgende Icon-Zuordnungen

| Funktionsart | Privat | Icon-Zuordnung | Standard-Zuordnung (\*\*) |
| --- | --- | --- | --- |
| Dialog | Nein | menuicon_anwendung | window_dialog  
![window\_dialog](../../../../ImagesExt/image8_1474.png "window_dialog") |
| Dialog | Ja | menuicon_privatanwendung | window_dialog_user  
![window\_dialog\_user](../../../../ImagesExt/image8_1475.png "window_dialog_user") |
| Liste, Crystallreport | Nein | menuicon_druck | printer2  
![printer2](../../../../ImagesExt/image8_1476.png "printer2") |
| Liste, Crystallreport | Ja | menuicon_privatdruck | printer2_user  
![printer2\_user](../../../../ImagesExt/image8_1477.png "printer2_user") |
| Anwendung | Nein | menuicon_auswahlliste | table2_selection-row  
![table2\_selection\_row](../../../../ImagesExt/image8_1478.png "table2_selection_row") |
| Anwendung | Ja | menuicon_privatauswahlliste | table2_selection_row_user  
![table2\_selection\_row\_user](../../../../ImagesExt/image8_1479.png "table2_selection_row_user") |
| BusinessIntelligence | Für Beide | menuicon_businessintelligence | excel_2013  
![excel\_2013](../../../../ImagesExt/image8_1480.png "excel_2013") |
| | | | |
| Alle übrigen wie folgt, entweder | Nein | menuicon_keinicon | placeholder  
![placeholder](../../../../ImagesExt/image8_1481.png "placeholder") |
| oder | Ja | menuicon_privat | user  
![user](../../../../ImagesExt/image8_1482.png "user") |

(\*) Die Funktionsarten werden über den Controlstring einer Funktion ermittelt. Maßgeblich ist dafür die Datenbank-Prozedure [amic_get_funktionsart](http://www.amic.de/ihilfe/index.html?turl=XMLDocuments%2FiAeins%2Fhtml%2FM_SQL_FUNCTION_amic_get_funktionsart_1_bb3a7a4f.htm).

(\*\*) Das resultierende Icon wird dann über die Relation Iconzuordnung ermittelt. Es ist angedacht in späteren Aeins-Releases hier private Iconzuordnungen durchführen zu können. Somit besteht dann die Möglichkeit private Funktionen mit „eigenen“ Icons auszustatten.

Jeder Menü-Punkt ist mit einem Menü-Punkt-Tooltip versehen und besteht aus folgenden Bestandteilen:

| Komponente | Daten-Herkunft im Anwendungsfunktion-Pfleger |
| --- | --- |
| Überschrift | Tiptext Titel |
| Text | Tiptext |
| (optional) Direktsprung  
![execute](../../../../ImagesExt/image8_1483.png "execute") | Direktsprung |

Die Anzeige der Tooltips ist über Steuerparameter 931 pro Bediener abschaltbar.

<p class="siehe-auch">Siehe auch:</p>

- [Kontextmenüs der Menüpunkte](./kontextmenues_der_menuepunkte.md)
