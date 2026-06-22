# Vorgangseinrichtung

<!-- source: https://amic.de/hilfe/_vorgangseinrichtung.htm -->

In der [Formularzuordnung/Vorgangsunterklasse](../../../vorgangsabwicklung/formularzuordnung/formular_formularzuordnungen_zum_vorgang_unterklasse.md) [FRZ] ist der entsprechende Vorgang für die Produktion (Vorgangsklasse 5220 und entsprechende Vorgangsunterklasse) zu definieren.  
Im Register Formulare sind die zu verwendenden Formulare anzugeben.

Die Registerkarte **Produktion** erlaubt diverse Angaben zur Behandlung von Produktionsvorgängen der jeweiligen Vorgangsunterklasse:

| Direktsprung Hauptmaske Positionsteil | Bei Aufruf der Erfassung und/oder Korrektur eines Produktionsbelegs kann entsprechend der Einstellung dieses Wertes direkt zum Positionsteil verzweigt werden.  
Nein: keine automatische Weiterschaltung  
Erfassung: automatische Weiterschaltung bei Erfassung  
Korrektur: automatische Weiterschaltung bei Korrektur  
Erfassung +Korrektur: immer automatische Weiterschaltung |
| --- | --- |
| Direktsprung Positionsteil Produktionserfassung | Bei Aufruf der Erfassung und/oder Korrektur eines Produktionsbelegs kann entsprechend der Einstellung dieses Wertes bei Aufruf des Positionsteils direkt in die Produktionserfassungsmaske verzweigt werden.  
Nein: keine automatische Weiterschaltung  
Erfassung: automatische Weiterschaltung bei Erfassung  
Korrektur: automatische Weiterschaltung bei Korrektur  
Erfassung +Korrektur: immer automatische Weiterschaltung |
| Produktionslager aus Bedienerkonstanten | Legt fest, wie das Produktionslager zu bestimmen ist.  
Ja = Lagernummer aus Bedienerkonstanten  
Nein = Lagernummer wie im nächsten Feld angegeben |
| Produktionslager | Wird die Produktionslagernummer nicht aus den Bedienerkonstanten ermittelt, so wird die zu verwendende Produktionslagernummer hier angegeben. |
| Erlösklasse Produktion | Die Erlösklasse für Produktionen wird an dieser Stelle festgelegt. |
| Erlösklasse Vermahlung | Die Erlösklasse für Vermahlungen wird an dieser Stelle festgelegt. |
| Partieanzeigeprozedur | Parameterlose Datenbank-Prozedur zur Anzeige aller der jeweils aktuellen Position zugeordneten Partien (über die Tabelle ProduktionsPartien) im dafür zur Verfügung gestellten (per GDS pflegbarem) Grid. Ist hier eine Prozedur angegeben, so wird diese anstelle der im Standard vorgesehenen Datenbank-Prozedur ‚ProduktionPartieAnzeige‘ verwendet. |
| Komponentenzeilenprozedur | Es lässt sich eine Prozedur definieren, die nach Eingabe der Gesamtmenge die Komponentenmengen neu berechnet. Diese Prozedur erlaubt ein Übersteuern der automatisch berechneten Mengen aus der Rezeptureingabe. Die Parameter der Prozedur lauten:  
procedure ProduktionsZeilenMenge( in in_status_vorgang integer, in in_produktartikelnummer char(40), in_Gesamtmenge numeric(15,4), in in_zeile integer default 1, in in_artikelId integer, in in_menge numeric(15,4) )  
Im Status_Vorgang wird bei einer Korrektur eine 2 geliefert. |
| Gridbeite in Pixel | Hier lässt sich die Anzeige-Breite der Grids auf der Produktionsmaske festlegen. Die Angabe wird in Pixel vorgenommen, der Wert Leer oder 0 liefert den Standard von 1024 |
| Start/Endzeitabfrage | Die ‚Ja/Nein‘-Einstellung dieses Feldes bewirkt die Aktivierung/Deaktivierung der entsprechenden Erfassungsfelder auf der Produktions-Erfassungs-/Bearbeitungs-Maske. |

Unter [Vorgangszuordnungen/Nummernkreise](../../../firmenstamm/nummernkreise_fuer_ware_und_fibu/einrichtung_von_nummernkreisen.md#NKV_Vorgangszuordnung) [NKV] muss der Vorgangsklasse/Vorgangsunterklasse ein Nummernkreis zugeordnet werden.
