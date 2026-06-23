# Faktor / THG-Wert / Anbauland

<!-- source: https://amic.de/hilfe/_nachhaltigkeit_stamm_faktorthg.htm -->

Hauptmenü > Wareneinkauf > Nachhaltigkeit > Nachhaltigkeitswerte

Direktsprung **[NAWER]**

<p class="just-emphasize">Faktor</p>

Für die THG-Werte können Faktoren hinterlegt werden, dafür stehen im Pfleger folgende Felder zu Verfügung. Der Faktor ist aktuell rein informativ und wird an keiner Stelle ausgewertet.

| Feld | Beschreibung |
| --- | --- |
| Nummer | Nummer des Faktors |
| Bezeichnung | Bezeichnung des Faktors |
| Einheit | Textliche Beschreibung der Einheit<br>*(Z.B. kg CO2-eq/kg N-Dünger)* |
| Quelle | Herkunft der Informationen<br>*(Z.B. IFEU / TREMOD)* |
| Wert | Wert des Faktors |

<p class="just-emphasize">THG-Werte</p>

In den THG-Werten werden für jede Fruchtart die Teilstandardwerte für Anbau, Verarbeitung und Lieferung verwaltet. Des Weiteren werden dort die Umrechnungsfaktoren von Standardwerten auf massebezogene Werte für Zwischenprodukte gepflegt.

Im Artikelstamm wird dann auf diese Werte Bezug genommen. Dadurch kann man für mehrere Artikelstämme die Informationen eines THG-Wertes verwenden.

| Feld | Beschreibung |
| --- | --- |
| Nummer | Nummer des THG-Wertes |
| Bezeichnung | Bezeichnung des THG-Wertes |
| Faktor | Zugrundeliegender Faktor des THG-Wertes |
| Allokationsfaktor | Fruchtartspezifische Konstante laut BLE |
| Konversionsfaktor | Fruchtartspezifische Konstante laut BLE |
| Teilstandardwert | Fruchtartspezifische Konstante laut BLE |
| Texte aktiv | Aktiviert die individuellen Texte für alle Artikel, die diesen THG-Wert hinterlegt haben. |
| Label | Individueller Text für den LABEL<br>(Ursprünglicher Text: „aus nachhaltigem Anbau.“) |
| Text1 | Individueller Text für den TEXT1<br>(Ursprünglicher Text: „Zertifizierungssystem für Nachhaltigkeit: &lt;ZERTIFIZIERUNGSMETHODETEXT>“) |
| Text2 | Individueller Text für den TEXT2<br>(Ursprünglicher Text: „&lt;ZERTIFIZIERUNGSMETHODETEXT>, Zertifikatsnummer: &lt;ZERTIFIKATBEMERKUNG>“) |
| Text3 | Individueller Text für den TEXT3<br>(Ursprünglicher Text: „Die Ware entspricht den Nachhaltigkeitsverordnungen (BioSt-NachV u. Biokraft-NachV)!“) |
| Text4 | Individueller Text für den TEXT4<br>(Ursprünglicher Text: „Für die Berechnung der Treibhausgasbilanzierung soll der Standardwert verwendet werden“) |
| Text5 | Individueller Text für den TEXT5<br>(Ursprünglicher Text: „(§ 8 u. Anlage 2 der Nachhaltigkeitsverordnungen)“) |

Für die individuellen Texte stehen Variablen zur Verfügung, die bei der Ermittlung der Texte ersetzt werden.

| Variable | Erläuterung |
| --- | --- |
| &lt;ZERTIFIZIERUNGSMETHODETEXT> | Text der Zertifizierungsmethode (Format: AF_ZERTMETH)<br>(Bsp.: „REDcert“) |
| &lt;ZERTIFIZIERUNGSMETHODE> | Nummer der Zertifizierungsmethode<br>(Bsp.: „11“) |
| &lt;ZERTIFIKAT_BLE> | Bemerkung des Zertifikats und Nummer der Warenbewegung hintereinander<br>(Bsp.: „DE-100-200-300-400-500-600-34888“) |
| &lt;ZERTIFIKATBEMERKUNG> | Bemerkung des Zertifikats<br>(Bsp.: „DE-100-200-300-400-500-600“) |
| &lt;WABEWLIEFNUMMER> | Belegnummer<br>(Bsp.: „34888“) |

<p class="just-emphasize">Anbauland</p>

Für jedes Anbauland lassen sich individuelle THG-Werte hinterlegen. Für die allgemeine Info eines Anbaulands lassen sich folgende Felder pflegen.

| Feld | Beschreibung |
| --- | --- |
| Nummer | Nummer des Anbaulands |
| NUTS Nummer | Die NUTS Nummer (*Nomenclature des unités territoriales statistiques*) ist eine eindeutige Identifizierung eines Gebietes in den Mitgliedsstaaten der Europäischen Union.<br>Nach Speichern des Anbaulands lässt sich die NUTS-Nummer nicht mehr ändern. |
| Land | Nummer des Landes aus den hinterlegten Staaten. |
| Region | Textliche Darstellung um welche Region es sich bei diesem Anbauland handelt. |

Zusätzlich zu den allgemeinen Infos, können THG-Werte hinterlegt werden. Die ab einem bestimmten Datum gültig sind. Dies sind globale THG-Werte des Anbaulandes

| Feld | Beschreibung |
| --- | --- |
| Ab Datum | Datum ab dem der Wert gültig ist. |
| Anbau | Wert für den THG-Wert Anbau |
| Lieferung | Wert für den THG-Wert Lieferung |
| Verarbeitung | Wert für den THG-Wert Verarbeitung |

 

Zusätzlich zu den Werten nach Datum, kann man die Werte auch abhängig vom Artikelstamm hinterlegen. Dies sind artikelspezifische THG-Werte des Anbaulandes.

| Feld | Beschreibung |
| --- | --- |
| Ab Datum | Datum ab dem der Wert abweichend vom globalen Wert für den eingerichteten Artikel gültig ist. |
| Artikelstamm | Die Nummer des Artikelstamms für den die Werte gültig sein sollen. |
| Anbau | Wert für den THG-Wert Anbau |
| Lieferung | Wert für den THG-Wert Lieferung |
| Verarbeitung | Wert für den THG-Wert Verarbeitung |

Zusätzlich zu den Werten befindet sich eine Artikelstammübersicht auf der Maske. Diese zeigt die Artikelstämme der letzten Zeiträume an. Da dies etwas Zeit in Anspruch nehmen kann, lässt sich im [Einrichterparameter](../../../firmenstamm/einrichterparameter/nachhaltigkeit_anbauland_epa_nhanbauland.md) einstellen, wie viele Zeiträume beachtet werden sollen.

<p class="just-emphasize">Import von deutschen NUTS2 Gebieten</p>

Über die Funktion „**Import NUTS2**“ wird eine vorgefertigte Liste mit den deutschen NUTS2 Gebieten importiert.
