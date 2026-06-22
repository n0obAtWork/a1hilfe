# Rohware an der Waage

<!-- source: https://amic.de/hilfe/_waage_rohware.htm -->

Abrechnungsschema(Sorte)

In den Abrechnungsschemas einer Rohwarengruppe könne folgende Einstellungen für die Übergabe von Werten aus der Waage in den Rohwarenbeleg verändert werden. Es gibt zwei Arten von Waagenparameter einmal die Globalen Parameter und die Parameter, die pro Abrechnungsschema gelten.

Globale Waagenparameter

| **Feldname** | **Wert** |
| --- | --- |
| Liefernummer | 0. Aus Waage<br>1. Aus Nummernkreis(Standard) |
| Lieferdatum | 0. Aus Waage (Standard)<br>1. Tagesdatum |
| Sortennummer | 0. Aus Waage (Standard)<br>1. Default Waagensorte |
| Lagernummer | 0. Aus Waage (Standard)<br>1. Aus Vorgangskonstanten (Standardlager des Bedieners) |
| Lagerplatznummer | 0.Immer Lagerplatz 0 (Standard)<br>1. Aus Waage |
| Filialnummer | 0. Aus Waage (Standard)<br>1. Aus Vorgangskonstanten (Filiale des Bedieners) |
| Fakturiergruppe | 0. Entsprechend Sorte (Standard)<br>1. Aus Waage |
| Vertretergruppe | 0. Entsprechend Sorte (Standard)<br>1. Aus Waage |
| Versandart | 0. Entsprechend Sorte (Standard)<br>1. Aus Waage |
| Verkaufsgebiet | 0. Entsprechend Sorte (Standard)<br>1. Aus Waage |

Abrechnungsschema Waagenparameter

| **Parametername** | **Einstellungsmöglichkeiten** |
| --- | --- |
| Kontrakt | 0. Ohne Kontrakt<br>1. Aus Waage<br>2. Automatische Kontraktfindung<br>3. Aus Waage, bei 0 automatische Kontraktzuordnung |
| Partie | 0. Ohne Partie<br>1. Aus Waage<br>2. Automatische Partiezuordnung<br>3. Automatische Partieanlage |
| Einlagerungsabrechnungsschema | Abrechungsschema für die Einlagerung |
| Kontraktlaufzeit Einlagerung in Tagen | Hier wird die Laufzeit des Einlagerungskontraktes in Tagen eingetragen |
| Kontraktunterklasse Einlagerung | Kontraktunterklasse für die Anlage bei eines Einlagerungskontraktes |
| Einlagerungs-Abrechnungs Verknüpfung | Hier wird die Abrechnungsart für den Rohwarenbeleg hinterlegt, welcher bei der Einlagerung angelegt wird.<br>0 (-) wenn 0 dann wird die 1 gewählt<br>1 Referenznummer<br>2 Erfassungsnummer<br>3 Abrechnungsnummer<br>4 Auswertungsnummer<br> |
| Steuergruppe Einlagerung | Hier wird die Steuergruppe hinterlegt, welche bei der Erzeugung einer Rechnung in der Einlagerung gesetzt werden soll. |

Rohwarenvorgang gegen eine Bestellung / einen Auftrag buchen

Bei dieser Funktionalität wird beim Erzeugen eines Rohwarenbeleges aus der Waage, die gewogene Menge von einem [Auftrag](../../vorgangsabwicklung/vorgangsbearbeitung_allgemein/vorgangsklassen_in_a_eins/auftrag/index.md) oder von einer [Bestellung](../../vorgangsabwicklung/vorgangsbearbeitung_allgemein/vorgangsklassen_in_a_eins/bestellung.md) abgebucht.

Da der Artikelposition dem [Auftrag](../../vorgangsabwicklung/vorgangsbearbeitung_allgemein/vorgangsklassen_in_a_eins/auftrag/index.md) oder der [Bestellung](../../vorgangsabwicklung/vorgangsbearbeitung_allgemein/vorgangsklassen_in_a_eins/bestellung.md) kein [Abrechnungsschema](../../rohware_modul/vorgehensweise_bei_der_einrichtung_von_abrechnungsschemata_s.md) zugeordnet werden kann, so kann der Position ein [Rohwarekontrakt](../../kontrakt/index.md) zugeordnet werden. Dies ist aber kein muss. Wurde der Position kein [Kontrakt](../../kontrakt/index.md) zugeordnet, so wird das [Standardabrechnungsschema](../../rohware_modul/vorgehensweise_bei_der_einrichtung_von_abrechnungsschemata_s.md) der Waage beim Erzeugen des Rohwarenbeleges gezogen.

Einrichtung / Voraussetzungen

1. Als erstes muss im [Rohwarenprozess](./prozess_einrichten/index.md) auf der Registerkarte [F3-Auswahlen](./prozess_einrichten/registerkarte_f3_auswahlen.md) eine Itembox für die Vorgangskopie ausgewählt werden. Dadurch ist es dann möglich ein [Auftrag](../../vorgangsabwicklung/vorgangsbearbeitung_allgemein/vorgangsklassen_in_a_eins/auftrag/index.md) oder eine [Bestellung](../../vorgangsabwicklung/vorgangsbearbeitung_allgemein/vorgangsklassen_in_a_eins/bestellung.md) der Wiegung zuzuordnen.

2. Folgende [Einrichterparameter](../../firmenstamm/einrichterparameter/index.md) der Waage werden ausgewertet.

2.1. [Abfrage ob Auftrag storniert oder ausgebucht werden soll](../../firmenstamm/einrichterparameter/online_waage_epa_owaage.md)

2.2. [Sollen Aufträge komplett storniert werden (sonst ausbuchen)?](../../firmenstamm/einrichterparameter/online_waage_epa_owaage.md)

2.3. [Prozentzahl der Menge bei deren Unterschreitung Auftrag Storno/Ausbuchen](../../firmenstamm/einrichterparameter/online_waage_epa_owaage.md).

3. Es muss eine Rohwarenwiegung durchgeführt werden.

4. Es ist darauf zu achten, dass in dem [Auftrag](../../vorgangsabwicklung/vorgangsbearbeitung_allgemein/vorgangsklassen_in_a_eins/auftrag/index.md) oder in der [Bestellung](../../vorgangsabwicklung/vorgangsbearbeitung_allgemein/vorgangsklassen_in_a_eins/bestellung.md) nur eine Position mit einem Rohwarenartikel existiert.

Merkmale bei der Erzeugung

1. Ist einer Wiegung, aus der noch kein Rohwarenbeleg erzeugt wurde, eine [Bestellung](../../vorgangsabwicklung/vorgangsbearbeitung_allgemein/vorgangsklassen_in_a_eins/bestellung.md) oder ein [Auftrag](../../vorgangsabwicklung/vorgangsbearbeitung_allgemein/vorgangsklassen_in_a_eins/auftrag/index.md) zugeordnet und die [Bestellung](../../vorgangsabwicklung/vorgangsbearbeitung_allgemein/vorgangsklassen_in_a_eins/bestellung.md) oder der [Auftrag](../../vorgangsabwicklung/vorgangsbearbeitung_allgemein/vorgangsklassen_in_a_eins/auftrag/index.md) wurde in der Zwischenzeit storniert, so wird die Erzeugung des Rohwarenbeleges abgebrochen. In diesem Fall ist dann zu entscheiden, ob der ausgewählte [Auftrag](../../vorgangsabwicklung/vorgangsbearbeitung_allgemein/vorgangsklassen_in_a_eins/auftrag/index.md) oder die Ausgewählte [Bestellung](../../vorgangsabwicklung/vorgangsbearbeitung_allgemein/vorgangsklassen_in_a_eins/bestellung.md) in der Waage durch einen gültigen [Auftrag](../../vorgangsabwicklung/vorgangsbearbeitung_allgemein/vorgangsklassen_in_a_eins/auftrag/index.md) oder einer gültigen [Bestellung](../../vorgangsabwicklung/vorgangsbearbeitung_allgemein/vorgangsklassen_in_a_eins/bestellung.md) ersetzt werden soll. Des Weiteren kann der [Auftrag](../../vorgangsabwicklung/vorgangsbearbeitung_allgemein/vorgangsklassen_in_a_eins/auftrag/index.md) oder die [Bestellung](../../vorgangsabwicklung/vorgangsbearbeitung_allgemein/vorgangsklassen_in_a_eins/bestellung.md) in der Wiegung abgewählt werden.

2. Ist der ausgewählte [Auftrag](../../vorgangsabwicklung/vorgangsbearbeitung_allgemein/vorgangsklassen_in_a_eins/auftrag/index.md) oder die ausgewählte [Bestellung](../../vorgangsabwicklung/vorgangsbearbeitung_allgemein/vorgangsklassen_in_a_eins/bestellung.md) durch einen anderen Bediener gesperrt, so wird die Rohwarenbeleg Erzeugung abgebrochen.

3. Steht der Status an der Waage auf „mit Vorgang“ und es kann kein Rohwarenbeleg gefunden werden, so kann das daran liegen, dass die [Prozess](./prozess_einrichten/index.md) Einstellung „Rohwarenlieferscheine sofort erzeugen” auf der Registerkarte [Rohware](./prozess_einrichten/registerkarte_rohware.md) nicht auf “Ja” steht. In der Anwendung “[Rohwarenübernahme Waage](../waagenimport/index.md)” kann der Rohwarenbeleg erzeugt werden.
