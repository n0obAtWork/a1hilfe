# Stoffstrom-Bilanz-Daten

<!-- source: https://amic.de/hilfe/_stoffstrombilanzdaten.htm -->

Übersicht

Das Lizenz-Modul zur Ermittlung von Stoffstrom-Bilanz-Daten ermöglicht die Erfassung, Verwaltung und Ausweisung von Daten zur Unterstützung bei der Informationsbeschaffung stoffstrombilanzpflichtiger Betriebe.  
Das grundsätzliche Verfahren beruht darauf, zunächst jedem betroffenen Artikelstamm über seine [Zusammensetzung](../../artikelstamm_und_artikel/parameter_des_artikelstamms/zusammensetzung.md) die individuellen Stoffstrom-Komponenten zuzuordnen und dabei die jeweiligen Anteilwerte festzulegen. Alle zu verwendenden Stoffstrom-Komponenten sind dafür zunächst in die allgemeine Artikelbestandteil-Liste mit Angabe der Stoffstromart aufzunehmen (siehe [Bestandteile](../../artikelstamm_und_artikel/konstanten_der_artikelverwaltung/bestandteile.md)).  
Die Stoffstromarten sind im [Anwendungsformat](../../firmenstamm/formate.md#Anwendungsformate) *‚af_stofstart‘* aufgeführt und können bei Bedarf ergänzt werden. 

Bei der Erfassung, Erzeugung und Bearbeitung von Vorgängen werden für Positionen mit Artikeln, denen Stoffstrom-Komponenten zugeordnet sind, beim Speichern des Vorgangs aus der Positionsmenge und den Anteilen aus der Zusammensetzung die Stoffstrommengen berechnet und gespeichert. Bei Standardvorgängen im **Verkauf** kann auf der Warenpositionsmaske in der Registerkarte **Stoffstromwerte** der Lieferant aus dem Lieferantenstamm angegeben werden. Sind diesem Lieferanten im Artikelstamm der Position individuelle Stoffstromparameter zugeordnet, so ersetzen diese diejenigen aus der Artikelzusammensetzung. Im **Einkauf** werden diese Werte bei der Erzeugung oder Erfassung einer Belegposition bezüglich der Kunden-/Lieferantennummer des Vorgangs herangezogen.  
**Zu beachten ist, dass bei Änderung der Anteil-Angabe eines Stoffstrombestandteils in der Artikelstamm-Zusammensetzung nachfolgende Änderungen an Vorgängen automatisch die entsprechenden Stoffstrommengen neu berechnen, sofern der im Vorgang bereits festgehaltene Wert nicht per Stoffstrom-Editor oder bei Standardvorgängen in der Warenerfassungsmaske manuell geändert wurde.**   
Positionsorientierte Auswahlvarianten der diversen Vorgangsbearbeitungs-Module ermöglichen die manuelle Korrektur der Stoffstrom-Daten mittels eines speziellen [Stoffstrom-Editors](./editieren_von_stoffstromdaten.md). Hier können die Stoffstrom-Anteile oder die berechneten Stoffstrom-Mengen überschrieben werden und ein entsprechendes Manuell-Kennzeichen gesetzt werden.  
Zur automatischen [Nachberechnung/Neuberechnung](./editieren_von_stoffstromdaten.md) von Stoffstrom-Mengen zu Positionen von zum Beispiel älteren Vorgängen steht in diesen Auswahlvarianten eine stapelorientierte Funktion zur Verfügung. Dabei werden als manuell gekennzeichnete Anteil-Werte beziehungsweise manuelle Stoffstrom-Mengen jedoch nicht geändert.

Jedem Stoffstrom-Bestandteil kann optional eine [private Datenbankprozedur](./stoffstromdatenberechnung_per_datenbankprozedur.md) zugeordnet werden (siehe [Bestandteile](../../artikelstamm_und_artikel/konstanten_der_artikelverwaltung/bestandteile.md)), die zur Realisierung individueller Berechnungsmethoden herangezogen wird.  
Ist einem Stoffstrom-Bestandteil keine private Datenbankprozedur zugeordnet, so wird, falls vorhanden, eine globale private Datenbankprozedur namens ***‚p_StoffStromBerechnung‘*** herangezogen.

**Besonders zu beachten ist die Stoffstromdaten-Behandlung bei** [Rohwarevorgängen](./stoffstromdaten_in_rohwarebelegen.md) **und** [Produktionsvorgängen](./stoffstromdaten_in_produktionsbelegen.md)**.  
    
**Eine wichtige positionsorientierte Auswahlvariante mit der Bezeichnung **Stoffstrom_Positionen** befindet sich im Modul **Vorgangsübersicht**. In dieser Variante werden nur Vorgangs-Positionen zu Artikelstämmen mit Stoffstrom-Bestandteil-Zuordnungen gelistet. Dabei werden unter anderem für die definierten Stoffstromarten die jeweiligen Positionswerte dargestellt. Im Selektionsbereich können neben diversen weiteren Merkmalen bis zu zwei **Stoffstromarten** angegeben werden die, wenn sie aktiviert sind, die Ergebnismenge auf vorhandene Zuordnungen der Stoffstromart(en) in der jeweiligen Artikelstamm-Zusammensetzung einschränken. Durch die Vielzahl der möglichen Selektionskriterien kann diese Auswahlvariante als Basis für die Anbindung und Erzeugung von Berichten, Excel-Auswertungen etc. dienen. Ein weiteres spezielles Merkmal ist mit dem Label **nur nie berechnete** gekennzeichnet. Wird hier der Wert **Ja** angegeben, so werden nur diejenigen Positionen selektiert, die über weniger berechnete Stoffstromdaten verfügen als nach der zugehörigen aktuellen Artikelstamm-Zusammensetzung bei Neuberechnung vorhanden wären. Dieses ist dann der Fall, wenn Bestandteile mit Stoffstrom-Kennzeichnung nachträglich zu Artikelstamm-Zusammensetzungen hinzugefügt werden, zugehörige bereits existierende Vorgänge aber danach nicht korrigiert wurden.  
    

<p class="siehe-auch">Siehe auch:</p>

- [Editieren von Stoffstromdaten](./editieren_von_stoffstromdaten.md)
- [Stoffstromdatenbearbeitung auf der Warenpositions-Bearbeitungsmaske](./stoffstromdatenbearbeitung_auf_der_warenpositions_bearbeitun.md)
- [Stapel-Berechnung von Stoffstromdaten](./stapel_berechnung_von_stoffstromdaten.md)
- [Stoffstromdaten in Rohwarebelegen](./stoffstromdaten_in_rohwarebelegen.md)
- [Stoffstromdaten in Produktionsbelegen](./stoffstromdaten_in_produktionsbelegen.md)
- [Stoffstromdatenberechnung mit prozentualem Anteil](./stoffstromdatenberechnung_mit_prozentualem_anteil.md)
- [Stoffstromdatenberechnung per Datenbankprozedur](./stoffstromdatenberechnung_per_datenbankprozedur.md)
- [Stoffstromdatenberechnung mit Mengeneinheiten](./stoffstromdatenberechnung_mit_mengeneinheiten.md)
- [Stoffstromdatenberechnung bei Teildisposition](./stoffstromdatenberechnung_bei_teildisposition.md)
- [Stoffstromdaten-Druckpositionen](./stoffstromdaten_druckpositionen.md)
- [Artikelstamm-Stoffstromdaten-Anpassung](./artikelstamm_stoffstromdaten_anpassung.md)
- [Stoffstrom Kundenreport](./stoffstrom_kundenreport.md)
