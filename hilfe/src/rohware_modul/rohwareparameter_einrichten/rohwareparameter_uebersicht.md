# Rohwareparameter-Übersicht

<!-- source: https://amic.de/hilfe/_rwparuebersicht.htm -->

Hauptmenü > Administration > Steuerung > Steuerparameter zeigen > Rohwareparameter pflegen

Hauptmenü > Administration > Steuerung > Steuerparameter zeigen > Rohwareparameter ansehen

Direktsprung **[SPA]**

Direktsprung **[RWPA]**

Die einzelnen Rohwareparameter und ihre Bedeutung werden nachfolgend beschrieben.

**Erfassungsstart mit Kontraktnummer**

Parameternummer: 177

Einstellungen für Rohwarengruppen und Abrechnungsschemata sind nicht möglich.

Optionen:

- **Ja**
- **Nein**

  Für die Erfassung von Rohwarebelegen wird mit diesem Parameter festgelegt, ob das erste zu bedienende Maskenfeld die Auswahl eines Kontrakts erlaubt, aus dem dann Artikel-, Abrechnungsschema und Kundendaten vorbelegt werden.

  **Lager**

  Parameternummer: 2

  Einstellungen für Rohwarengruppen und Abrechnungsschemata sind nicht möglich.

  Optionen:

- **keine Anzeige**
- **mit Anzeige**
- **Erfassung**

  Für die Erfassung, Anzeige und Korrektur von Rohwarebelegen wird mit diesem Parameter festgelegt, ob die Lagernummer auf der Bearbeitungsmaske unterdrückt, nur dargestellt oder änderbar sein soll.

  **Rohware manuelle Werte – Qualitätsergebnis änderbar**

  Parameternummer: 192

  Einstellungen für Rohwarengruppen und Abrechnungsschemata sind möglich.

  Optionen:

- **Nein**
- **Ja**

  Für die Erfassung und Korrektur von Rohwarebelegen wird mit diesem Parameter festgelegt, ob im Positionsteil die berechneten Werte der Qualitätspositionen durch manuelle Werte überschrieben werden dürfen.

  **Rohware manuelle Werte – Kostenergebnis änderbar**

  Parameternummer: 193

  Einstellungen für Rohwarengruppen und Abrechnungsschemata sind möglich.

  Optionen:

- **Nein**
- **Ja**

  Für die Erfassung und Korrektur von Rohwarebelegen wird mit diesem Parameter festgelegt, ob im Positionsteil die berechneten Beträge durch manuelle Werte überschrieben werden dürfen.

### Freigegebene Belege immer abrechnen

  Parameternummer: 174

  Einstellungen für Rohwarengruppen und Abrechnungsschemata sind nicht möglich.

  Optionen:

- **Nein**
- **Ja**

  Ist dieser Parameter mit dem Wert ‚*Ja*‘ belegt, so werden Rohwarebelege der Abrechnungsstufen ‚*Abschlag*‘, ‚Folgeabschlag‘ und <em>‚</em>Finale‘ nach Umwandlung und Korrekturen immer sofort abgerechnet, sofern sie für die entsprechende Stufe das Kennzeichen ‚*freigegeben‘* tragen. Die Funktion ***Abrechnen*** wird dann in den entsprechenden Auswahllisten auch nicht mehr zur Verfügung gestellt. Einstellungen des Parameters ‚[Abrechnung nach Belegkorrektur](./rohwareparameter_uebersicht.md#RWPA_172)’ werden mit der Einstellung ‚*Ja*‘ außer Kraft gesetzt.

### Abrechnung nach Belegkorrektur

  Parameternummer: 172

  Einstellungen für Rohwarengruppen und Abrechnungsschemata sind möglich.

  Optionen:

- **Nein**
- **Ja**

  Ist dieser Parameter mit dem Wert ‚*Ja*‘ belegt, so werden Rohwarebelege der Abrechnungsstufen ‚*Abschlag*‘, ‚Folgeabschlag‘ und <em>‚</em>Finale‘ nach der Korrektur im Rohware-Bearbeitungsmodul **[RWB]** immer sofort abgerechnet, sofern sie für die entsprechende Stufe das Kennzeichen ‚*freigegeben‘* tragen. Dieser Parameter ist allerdings bedeutungslos, wenn der Parameter ‚[Freigegebene Belege immer abrechnen](./rohwareparameter_uebersicht.md#RWPA_174)‘ mit dem Wert ‚*Ja*‘ belegt ist.

### Stornobelege: Vorbelegung: >mit Kopie&lt;

  Parameternummer: 171

  Einstellungen für Rohwarengruppen und Abrechnungsschemata sind nicht möglich.

  Optionen:

- **Nein**
- **Ja**

  Sofern dem Steuerparameter **[SPA]** ‚<em>Rohwarestorno mit Quellbeleg-Kopie‘</em> der Wert ‚*erlaubt*‘ zugewiesen ist, kann bei der Erzeugung eines Storno-Vorgangs aus einem Rohwarebeleg zusätzlich eine Kopie des Quellbelegs erzeugt werden. Dieser Rohwareparameter bewirkt in diesem Fall die Vorbelegung für das Feld **‚mit Kopie‘** auf der Steuerungsmaske der Stornofunktion.

### Massebilanz bei Storno mit Kopie

  Parameternummer: 191

  Einstellungen für Rohwarengruppen und Abrechnungsschemata sind nicht möglich.

  Optionen:

- **Einstellung auf Umwandelmaske**
- **Standard**
- **Storno und Kopie nicht in Massebilanz**

Sofern bei Ausführung der Funktionen zur Erzeugung von **Rohware-Stornoabrechnungen** und **Rohware-Sammelstornobelegen** auf der jeweiligen Umwandelmaske die Option **Kopie nach Storno** eingeschaltet ist, steuert dieser Parameter die Wirksamkeit der Positionen des Stornobelegs und der Positionen der erzeugten Kopie des Originalbelegs bezüglich der Berücksichtigung in der **Massebilanz**.

Der Wert **Einstellung auf Umwandelmaske** bewirkt die Aktivierung des Eingabefeldes **Kopie in Massebilanz**, in dem die Einstellung **Standard** oder **Storno und Kopie nicht in Massebilanz** gewählt werden kann.

Der Wert **Standard** (per Parameter voreingestellt oder auf Maske ausgewählt) bewirkt, dass bei Bewegungen, die keiner oder einer noch nicht abgeschlossenen Massebilanz zugeordnet sind, die Übernahme dieses Zustands in die Bewegung des Kopie-Belegs erfolgt. Damit ist der neue Beleg bezüglich dieser Bewegung der aktuelle massebilanzwirksame Beleg. Diese ist dann zum Beispiel in der **Bewegungsübersicht** der Anwendung **Nachhaltigkeit** sichtbar und bearbeitbar (Massebilanz zuordnen, entfernen, THG-Werte ändern).  
Bei Bewegungen, die einer bereits **festgeschriebenen** Massebilanz zugeordnet sind, werden sowohl die Bewegung des Stornobelegs wie auch die der Belegkopie sowie der durch Weiterverarbeitung aus diesem entstehenden Belegen nicht mehr in der Massebilanz berücksichtigt. Der Originalbeleg bleibt bezüglich dieser Bewegung der aktuelle massebilanzwirksame Beleg

Der Wert **Storno und Kopie nicht in Massebilanz** (per Parameter voreingestellt oder auf Maske ausgewählt) bewirkt, dass der bisherige bezüglich dieser Bewegung aktuelle massebilanzwirksame Beleg weiterhin der massebilanzwirksame Beleg bleibt und sowohl die Bewegung des Stornobelegs wie auch die der Belegkopie sowie der durch Weiterverarbeitung aus diesem entstehenden Belegen nicht in der Massebilanz berücksichtigt werden

### Perioden bei Stornobelegen

Parameternummer: 121

Einstellungen für Rohwarengruppen und Abrechnungsschemata sind nicht möglich.

Optionen:

- **aus Originalbelegen**
- **laut Maskeneinstellung**

  Bei der Erzeugung von Stornobelegen aus Rohwareeinzelbelegen wird mit diesem Parameter die Behandlung der Perioden der Stornobelege festgelegt. Neben der Möglichkeit, diese immer entsprechend der Originalbelege zu setzen, gibt es die Möglichkeit, dieses auf der Steuerungsmaske der Stornofunktion einzustellen oder aber dort die Perioden explizit zu bestimmen. Bei entsprechender Einstellung des Parameters [Storno-Fibuperiode=0 vorbelegen](./rohwareparameter_uebersicht.md#RWPA_175) ist eine gesonderte Einstellung der Periode für die Finanzbuchhaltung mit 0 realisierbar.

### Storno-Fibuperiode=0 vorbelegen

  Parameternummer: 175

  Einstellungen für Rohwarengruppen und Abrechnungsschemata sind nicht möglich.

  Optionen:

- **Nein**
- **Ja**

  Ist dieser Parameter mit dem Wert ‚*Ja*‘ eingestellt, so wird bei der Erzeugung von Stornobelegen aus Rohwareeinzelbelegen wie auch Sammelbelegen Buchungs-Periode und Buchungs-Jahr für die Finanzbuchhaltung grundsätzlich mit dem Wert ‚*0*‘ vorbelegt. Dieses gilt auch, wenn die Einstellungen der Parameter [Perioden bei Stornobelegen](./rohwareparameter_uebersicht.md#RWPA_121) oder [Perioden bei Sammel-Stornobelegen](./rohwareparameter_uebersicht.md#RWPA_144) mit der Einstellung ‚aus Originalbelegen‘ versehen sind. Diese wirken sich dann nur auf die Periodenbehandlung der Warenwirtschaft aus. Bei dort eingestellter manuell erfassbarer Fibu-Periode kann die gewünschte Fibu-Periode aber auf den Vorschalt-Masken der Storno-Funktionen angegeben werden. 

### Storno alter Belege nach Inventureinspielung

  Parameternummer: 151

  Einstellungen für Rohwarengruppen und Abrechnungsschemata sind nicht möglich.

  Optionen:

- **immer erlaubt**
- **nie erlaubt**
- **Hinweis mit Abfrage**

  Bei der Erzeugung von Stornobelegen zu Belegen aus vorhergehenden Geschäftsjahren wird hier festgelegt, wie im Falle des Vorliegens einer bereits eingespielten Inventur zu verfahren ist.

### Belegdatum bei Stornobelegen

  Parameternummer: 131

  Einstellungen für Rohwarengruppen und Abrechnungsschemata sind nicht möglich.

  Optionen:

- **aus Originalbelegen**
- **laut Maskeneinstellung**

  Bei der Erzeugung von Stornobelegen wird hier bestimmt, ob die neuen Belege das Belegdatum der Originalbelege erhalten sollen oder dieses auf der Steuerungsmaske der Stornofunktion eingestellt wird.

### Sammelnummer-Release bei Druckrücksetzen

  Parameternummer: 163

  Einstellungen für Rohwarengruppen und Abrechnungsschemata sind nicht möglich.

  Optionen:

- **Nein**
- **Ja**

  Dieser Parameter legt fest, ob bei Aufhebung eines Rohwaresammeldruck-Belegs die aus dem dafür herangezogenen Nummernkreis ermittelte Drucknummer diesem wieder zur erneuten Verwendung zur Verfügung gestellt wird.

### Sammel-Storno-Beleg-/Druck-Datum

  Parameternummer: 140

  Einstellungen für Rohwarengruppen und Abrechnungsschemata sind nicht möglich.

  Optionen:

- **aus Originalbelegen**
- **laut Maskeneinstellung**

  Das Druckdatum von Rohwaresammelbelegen, dass auch als Sammelbelegdatum fungiert, kann durch Einstellung dieses Parameter bei der Erzeugung von Sammelstornobelegen auch als Beleg- und Druckdatum des erzeugten Sammelstornobelegs herangezogen werden. Neben der Möglichkeit, diese immer entsprechend der Originalbelege zu setzen, gibt es die Alternative, dieses auf der Steuerungsmaske der Stornofunktion einzustellen oder aber dort das Datum explizit zu bestimmen.

### Sammel-Storno-Belegnummer

  Parameternummer: 141

  Einstellungen für Rohwarengruppen und Abrechnungsschemata sind nicht möglich.

  Optionen:

- **Originalnummer**
- **Neue Belegnummer**

  Die Drucknummer von Rohwaresammelbelegen, die auch als Sammelbelegnummer fungiert, kann durch Einstellung dieses Parameters bei der Erzeugung von Sammelstornobelegen auch als Beleg- und Drucknummer des erzeugten Sammelstornobelegs herangezogen werden. Ist diese Option eingestellt, so ist darauf zu achten, dass die zugehörigen Nummern- und Zählkreise auch kompatibel bezüglich der in Frage kommenden Nummernbereiche sind. Die zweite Option der Nummernvergabe ist die Ermittlung einer neuen Belegnummer für den Sammelstornobeleg aus dem zugehörigen Nummernkreis.

### Sammel-Storno-Einzelbeleg-Datum

  Parameternummer: 142

  Einstellungen für Rohwarengruppen und Abrechnungsschemata sind nicht möglich.

  Optionen:

- **Originaldatum**
- **Sammel-Storno-Belegdatum**

  Bei der Erzeugung eines Rohwaresammelstornobelegs können die Einzelbelege als Belegdatum entweder das Belegdatum des jeweiligen Quellbelegs oder aber das Sammeldruck-/Belegdatum des Sammeldruckstornobelegs erhalten.

### Perioden bei Sammel-Stornobelegen

  Parameternummer: 144

  Einstellungen für Rohwarengruppen und Abrechnungsschemata sind nicht möglich.

  Optionen:

- **aus Originalbelegen**
- **laut Maskeneinstellung**

  Bei der Erzeugung von Rohwaresammelstornobelegen wird mit diesem Parameter die Behandlung der Perioden der Sammelstornobelege festgelegt. Neben der Möglichkeit, diese immer entsprechend der Originalbelege zu setzen, gibt es die Option, dieses auf der Steuerungsmaske der Stornofunktion einzustellen oder aber dort die Perioden explizit zu bestimmen. Bei entsprechender Einstellung des Parameters [Storno-Fibuperiode=0 vorbelegen](./rohwareparameter_uebersicht.md#RWPA_175) ist eine gesonderte Einstellung der Periode für die Finanzbuchhaltung mit 0 realisierbar.

### Sammel-Storno nur nach Fibuübertrag

  Parameternummer: 145

  Einstellungen für Rohwarengruppen und Abrechnungsschemata sind nicht möglich.

  Optionen:

- **Nein**
- **Ja**

  Dieser Parameter legt fest, ob die Erzeugung eines Rohwaresammelstornobelegs erst erfolgen darf, wenn der zu stornierende Sammelbeleg an die Finanzbuchhaltung übertragen wurde. Handelt es sich jedoch um die Erzeugung eines Sammel-Stornos mit Kopie des Original-Sammeldruckbelegs in ein anderes Wirtschaftsjahr, so setzt die Einstellung ‚*Ja, ohne Fibu-Übertrag*‘ im Feld ‚*Finale nach Jahreswechsel*‘ in den Belegen diese Sperre außer Kraft.  
Dieser Parameter betrifft lediglich die Behandlung von Rohware-Sammeldruckbelegen. Für Einzel-Storno-Belege regelt dieses der Parameter [Stornobeleg nur nach Fibuübertrag](./rohwareparameter_uebersicht.md#RWPA_114).

### Sammel-Stornoformular-Offset

  Parameternummer: 146

  Einstellungen für Rohwarengruppen und Abrechnungsschemata sind nicht möglich.

  Optionen: Integer-Wert

  Dieser Parameter legt fest, wie, ausgehend von dem Rohwaresammeldruck-Formular eines Rohwaresammeldruckbelegs, das zu verwendende Sammeldruckformular des zu erzeugenden Sammelstornobelegs bestimmt wird. Die gesuchte Formularnummer ergibt sich aus der Nummer des Originalformulars zuzüglich des angegebenen Wertes dieses Parameters.

### Stornoformularnummer: Offset zum Originalformular

  Parameternummer: 84

  Einstellungen für Rohwarengruppen und Abrechnungsschemata sind nicht möglich.

  Optionen: Integer-Wert

  Dieser Parameter legt fest, wie, ausgehend von dem Einzeldruck-Formular eines Rohwarebelegs, das zu verwendende Druckformular des zu erzeugenden Stornobelegs bestimmt wird. Die gesuchte Formularnummer ergibt sich aus der Nummer des Originalformulars zuzüglich des angegebenen Wertes dieses Parameters.

### Artikel-/Abrechnungsschema-Auswahl

  Parameternummer: 14

  Einstellungen für Rohwarengruppen und Abrechnungsschemata sind nicht möglich.

  Optionen:

- **Artikel--->Schema**
- **Schema--->Artikel**

  Dieser Parameter legt fest, ob bei der Erfassung eines neuen Rohwarelieferscheins zunächst die Artikelnummer oder die Nummer des zu verwendenden Abrechnungsschemas anzugeben ist. Im Falle der Artikelnummer als erste Eingabe wird das Abrechnungsschema mit dem Standardschema der dem Artikel zugeordneten Rohwaregruppe vorbelegt. Wird zunächst das Schema erfasst, so stehen für die Artikelnummer nur Artikel der zum Abrechnungsschema gehörigen Rohwarengruppe zur Verfügung.

### Partie bei Artikel-/Lager-Wechsel

  Parameternummer: 176

  Einstellungen für Rohwarengruppen und Abrechnungsschemata sind nicht möglich.

  Optionen:

- **Partie nur bei vorhandenem Partieartikel beibehalten**
- **Artikel automatisch zur Partie hinzufügen**

  Bei der Erfassung und Korrektur von Rohwarebelegen bis zur ersten Rechnungsstufe kann die Lagernummer und/oder Artikelnummer des Lieferartikels geändert werden. Ist der Position vor der Korrektur bereits eine Partie zugeordnet, wird durch die Einstellung dieses Parameters geregelt, ob diese Partiezuordnung auch dann beizubehalten ist, wenn der neue Artikel noch kein zur Partie gehöriger Artikel ist. Ist die Einstellung des Parameters ‚*Partie nur bei vorhandenem Partieartikel beibehalten*‘, so bleibt die Partiezuordnung bei der Änderung nur erhalten, wenn der neue Artikel bereits Partieartikel der Partie ist. Sonst wird die Zuordnung aufgehoben.  
Die Einstellung ‚*Artikel automatisch zur Partie hinzufügen*‘ hingegen sorgt dafür, dass der neue Artikel gegebenenfalls automatisch zur Partie hinzugefügt wird.

### Ab Kundennummer bei Folgebeleg

  Parameternummer: 76

  Einstellungen für Rohwarengruppen und Abrechnungsschemata sind nicht möglich.

  Optionen:

- **Nein**
- **Ja**

  Bei der Erfassung mehrerer Rohwarelieferscheine nacheinander, ohne die Erfassungsmaske zu verlassen, werden grundlegende Daten wie Artikelnummer, Abrechnungsschema und die daraus resultierende Belegstruktur nach Abschluss eines Beleges zur Initialisierung des nächsten zu erfassenden Belegs genutzt. Ist dieser Parameter mit ‚*Ja*‘ eingestellt, so wird zu Beginn der Erfassung ab dem zweiten Beleg grundsätzlich mit dem Feld ‚Kunde/Lieferant‘ begonnen.

### Währung

  Parameternummer: 115

  Einstellungen für Rohwarengruppen und Abrechnungsschemata sind nicht möglich.

  Optionen:

- **keine Anzeige**
- **nur Anzeige**
- **Erfassung**

  An dieser Stelle wird für die Erfassung von Rohwarenbelegen festgelegt, dass die Belegwährung auf der Erfassungsmaske unterdrückt, nur dargestellt oder änderbar sein soll.

### Währung aus Kundenstamm

  Parameternummer: 116

  Einstellungen für Rohwarengruppen und Abrechnungsschemata sind nicht möglich.

  Optionen:

- **Nein**
- **Ja**

  Die Vorbelegung der Belegwährung bei der Erfassung eines Rohwarelieferscheins kann entweder aus dem Kundenstamm oder mit der Zentralwährung erfolgen.

### Währungskurs

  Parameternummer: 117

  Einstellungen für Rohwarengruppen und Abrechnungsschemata sind nicht möglich.

  Optionen:

- **keine Anzeige**
- **nur Anzeige**
- **Erfassung**

  An dieser Stelle wird für die Erfassung von Rohwarenbelegen festgelegt, dass der Währungskurs bei Fremdwährungsbelegen auf der Erfassungsmaske unterdrückt, nur dargestellt oder änderbar sein soll.

### Filiale

  Parameternummer: 1

  Einstellungen für Rohwarengruppen und Abrechnungsschemata sind nicht möglich.

  Optionen:

- **keine Anzeige**
- **nur Anzeige**
- **Erfassung**

  An dieser Stelle wird für die Erfassung von Rohwarenbelegen festgelegt, dass die Filiale auf der Erfassungsmaske unterdrückt, nur dargestellt oder änderbar sein soll.

### Filiale aus Kundenstamm

  Parameternummer: 77

  Einstellungen für Rohwarengruppen und Abrechnungsschemata sind nicht möglich.

  Optionen:

- **Nein**
- **Ja**

  Die Vorbelegung der Filialnummer für Rohwarebelege wird grundsätzlich bereits zu Beginn der Erfassung auf einer Vorschaltmaske festgelegt. Mit diesem Parameter kann jedoch festgelegt werden, dass die Vorbelegung der Filiale immer aus dem Kunden-/Lieferantenstamm übernommen wird.

### Zentrale

  Parameternummer: 21

  Einstellungen für Rohwarengruppen und Abrechnungsschemata sind nicht möglich.

  Optionen:

- **keine Anzeige**
- **nur Anzeige**
- **Erfassung**

  An dieser Stelle wird für die Erfassung, Anzeige und Korrektur von Rohwarenbelegen festgelegt, ob die Zentrale auf der Erfassungsmaske unterdrückt, nur dargestellt oder änderbar sein soll.

### Zentrale aus Filialstamm

  Parameternummer: 22

  Einstellungen für Rohwarengruppen und Abrechnungsschemata sind nicht möglich.

  Optionen:

- **Nein**
- **Ja**

  Die Vorbelegung der Nummer der Zentrale für Rohwarebelege wird grundsätzlich bereits zu Beginn der Erfassung auf einer Vorschaltmaske festgelegt. Mit diesem Parameter kann jedoch festgelegt werden, dass die Vorbelegung der Zentrale immer aus dem zugeordneten Filialstamm übernommen wird.

### Abteilung

  Parameternummer: 3

  Einstellungen für Rohwarengruppen und Abrechnungsschemata sind nicht möglich.

  Optionen:

- **keine Anzeige**
- **nur Anzeige**
- **Erfassung**

  An dieser Stelle wird für die Erfassung von Rohwarenbelegen festgelegt, ob die Abteilung auf der Erfassungsmaske unterdrückt, nur dargestellt oder änderbar sein soll.

### Unterabteilung

  Parameternummer: 5

  Einstellungen für Rohwarengruppen und Abrechnungsschemata sind nicht möglich.

  Optionen:

- **keine Anzeige**
- **nur Anzeige**
- **Erfassung**

  An dieser Stelle wird für die Erfassung von Rohwarenbelegen festgelegt, ob die Unterabteilung auf der Erfassungsmaske unterdrückt, nur dargestellt oder änderbar sein soll.

### Lieferdatum größer Erfassungsdatum erlaubt

  Parameternummer: 79

  Einstellungen für Rohwarengruppen und Abrechnungsschemata sind nicht möglich.

  Optionen:

- **Nein**
- **Ja**

  Ist dieser Parameter mit dem Wert ‚Ja‘ belegt, so kann bei der Erfassung von Rohwarelieferscheinen kein Datum eingegeben werden, dass größer als das aktuelle Tagesdatum ist.

### Liefermengensperre bei Korrektur

  Parameternummer: 80

  Einstellungen für Rohwarengruppen und Abrechnungsschemata sind nicht möglich.

  Optionen:

- **immer gesperrt**
- **ab Abschlag**
- **ab Folge-Abschlag**
- **ab Finale**
- **ab Nachvergütung**
- **immer korrigierbar**

  Für die Korrektur der Liefermenge von bereits erfassten Rohware-Belegen wird mit diesem Parameter festgelegt, ab welcher Belegstufe die ursprünglich erfasste Liefermenge nicht mehr korrigierbar sein soll. Unabhängig von diesem Parameter ist die Liefermenge in Belegen zu Fremdeinkäufen und Fremdverkäufen bei der Belegkorrektur nie änderbar.

### Kippwaagen-Kontrollrechnung

  Parameternummer: 82

  Einstellungen für Rohwarengruppen und Abrechnungsschemata sind nicht möglich.

  Optionen:

- **Nein**
- **Ja**

  Ist dieser Parameter mit dem Wert ‚Ja‘ belegt, so kann bei der Erfassung von Rohwarelieferscheinen im Erfassungsfeld der Liefermenge die Funktion ***Kippwaagendaten*** aufgerufen werden und dort anhand der anzugebenen Anzahl der Kippungen und der Restwiegung die daraus berechnete Menge in das Liefermengenerfassungsfeld übertragen werden.

### Kippwaage: Kippmenge

  Parameternummer: 83

  Einstellungen für Rohwarengruppen und Abrechnungsschemata sind möglich.

  Optionen: Integer-Wert

  Dieser Parameter legt die eine Kippung der Kippwaage auslösende Menge für die Funktion ***Kippwaagendaten*** bei der Erfassung von Rohwarebelegen fest. 

### Kundenanschrift Autoanzeige

  Parameternummer: 78

  Einstellungen für Rohwarengruppen und Abrechnungsschemata sind nicht möglich.

  Optionen:

- **Nein**
- **Ja**

  Ist dieser Parameter mit dem Wert ‚Ja‘ belegt, so wird nach Angebe der Kunden-/Lieferantennummer die zugehörige Anschrift auf einer Maske dargestellt und kann für den aktuellen Beleg geändert werden.

### Artikelauswahl-Randbedingung

  Parameternummer: 85

  Einstellungen für Rohwarengruppen und Abrechnungsschemata sind nicht möglich.

  Optionen:

- **geringer Anteil von Rohwareartikeln**
- **hoher Anteil von Rohwareartikeln**
- **alle Artikel auflisten**

  Dieser Parameter dient zur Optimierung des Aufbaus der Item-Box für die Artikelauswahl bei der Erfassung von Rohwarebelegen. In der Regel ist hier die erste oder zweite Variante zu wählen.

### Rechnungsdatum setzten auf

  Parameternummer: 108

  Einstellungen für Rohwarengruppen und Abrechnungsschemata sind nicht möglich.

  Optionen:

- **Datum neu bei Abrechnung**
- **Datum erhalten bei Abrechnung**

  Dieser Parameter bewirkt, dass bei Wahl der Funktion ***Abrechnen*** Felder für die Behandlung des Belegdatums mit ‚*Tagesdatum*‘ und damit verbunden die Eingabemöglichkeit eines hierfür heranzuziehenden Datums oder mit ‚*Belegdatum beibehalten*‘ vorbelegt wird. Im ersten Fall ist es zudem möglich, festzulegen, ob die Warenwirtschaftsperiode an das neue Belegdatum anzupassen ist oder beibehalten werden soll.

### Abrechnung für Sammeldruck mit Druckdatum

  Parameternummer: 107

  Einstellungen für Rohwarengruppen und Abrechnungsschemata sind nicht möglich.

  Optionen:

- **Nein**
- **Ja**

  Dieser Parameter bewirkt bei der Einstellung ‚*Ja*‘, dass auch bereits abgerechnete Einzelbelege bei der Zusammenstellung eines Rohwaresammeldruck-Belegs mittels der Funktion [Sammel-Erstdruck](../sammelabrechnung/sammelerstdruck.md) noch einmal abgerechnet werden. Dabei wird das Belegdatum auf das zuvor angegebene Druckdatum geändert und entsprechend der ebenfalls zuvor angegebenen Behandlungsweise die Warenwirtschaftsperiode gegebenenfalls angepasst.

### Periode bei Belegdatum=Abrechnungsdatum

  Parameternummer: 173

  Einstellungen für Rohwarengruppen und Abrechnungsschemata sind nicht möglich.

  Optionen:

- **Periode beibehalten**
- **Periodenanpassung an Belegdatum**
- **Laut Maskeneinstellung**

  Dieser Parameter bestimmt bei der Wahl einer der Funktionen ***Abrechnen*** oder [Sammel-Erstdruck](../sammelabrechnung/sammelerstdruck.md), sofern dort die Optionen des Abrechnens mit neuem Belegdatum gewählt wurde, die Behandlung der Warenwirtschaftsperiode der Belege. Die Option ‚*Laut Maskeneinstellung*‘ ermöglicht die Festlegung auf der Steuerungsmaske der jeweiligen Funktion.

  **Abrechnen: ZB mit manuellem Valutadatum**

  Parameternummer: 128

  Einstellungen für Rohwarengruppen und Abrechnungsschemata sind nicht möglich.

  Optionen:

- **Tagesdatum, wenn noch nicht gesetzt**
- **Immer auf Tagesdatum**
- **Laut Maskeneinstellung**

  Dieser Parameter bestimmt die Behandlung von Zahlungsbedingungen in Rohwareabrechnungen, deren Zahlungsziel per Fix-Datum festzulegen ist. Dieses Zahlungsbedingungsdatum kann bei der Abrechnung von Belegen bei Einstellung des Parameters mit dem Wert ‚*Tagesdatum, wenn noch nicht gesetzt*‘ mit dem Tagesdatum besetzt werden, wenn das Zahlungsbedingungsdatum des Belegs noch mit dem Initialisierungsdatum (01.01.1901) versehen ist. Mit dem Wert ‚*Immer auf Tagesdatum*‘ werden auch bereits gesetzte Zahlungsziele mit dem aktuellen Tagesdatum bei der Abrechnung überschrieben. Die Einstellung ‚*Laut Maskeneinstellung*‘ erlaubt die Wahl einer der ersten beiden Alternativen und die Eingabe des gewünschten Datums auf der Steuerungsmaske der Abrechnungs-Funktion.  
Dieses gilt auch für die Erstellung von Sammeldruckbelegen, sofern die dafür berücksichtigten Einzelbelege entweder noch nicht abgerechnet wurden oder aufgrund der Einstellung des Parameters ‚[Abrechnung für Sammeldruck mit Druckdatum](./rohwareparameter_uebersicht.md#RWPA_107)‘ mit dem Wert ‚*Ja*‘ beim [Sammel-Erstdruck](../sammelabrechnung/sammelerstdruck.md) immer neu abgerechnet werden.

  **Sammeldruck-Sortierung**

  Parameternummer: 139

  Einstellungen für Rohwarengruppen und Abrechnungsschemata sind nicht möglich.

  Optionen:

- **automatisch**
- **nach Belegauswahl**

  Die Einstellung ‚*automatisch*‘ dieses Parameters sorgt bei der Zusammenstellung von gewählten Rohware-Einzelbelegen zur Erzeugung von Rohware-Sammeldruckbelegen mittels der Funktion [Sammel-Erstdruck](../sammelabrechnung/sammelerstdruck.md) dafür, dass die Einzelbelege in einer Reihenfolge abgearbeitet werden und in den Sammelbelegen erscheinen, die die Anzahl der erzeugten Sammelbelege aufgrund der heranzuziehenden Trennkriterien möglichst gering hält. Zu beachten ist in diesem Falle auch die Einstellung des Parameters [Sammeldrucksortierung](./rohwareparameter_uebersicht.md#RWPA_187) automatisch: mit Wiegenummer. Hingegen werden die Einzelbelege bei dem Parameterwert ‚*nach Belegauswahl*‘ in der Reihenfolge der Belegauswahl berücksichtigt, was bei ungünstiger Auswahl aufgrund der heranzuziehenden Trennkriterien zur Erzeugung von mehr Sammelbelegen mit jeweils weniger Einzelbelegen führen kann.

  **Sammeldrucksortierung automatisch: mit Wiegenummer**

  Parameternummer: 187

  Einstellungen für Rohwarengruppen und Abrechnungsschemata sind nicht möglich.

  Optionen:

- **Ja**
- **Nein**

  Ist die Einstellung des zugehörigen Rohwareparameters [Sammeldruck-Sortierung](./rohwareparameter_uebersicht.md#RWPA_139) ‚*automatisch*‘, so wird bei Einstellung dieses Parameters mit ‚*Ja*‘ eine Untersortierung der Einzelbelege innerhalb eines Sammeldruckbelegs bei der Zusammenstellung eines Rohwaresammeldruck-Belegs mittels der Funktion [Sammel-Erstdruck](../sammelabrechnung/sammelerstdruck.md) nach der jeweiligen Wiegenummer bewirkt.

### Sammeldruck-Trennung: Rohwarengruppe

  Parameternummer: 102

  Einstellungen für Rohwarengruppen und Abrechnungsschemata sind nicht möglich.

  Optionen:

- **Nein**
- **Ja**

  Dieser Parameter bewirkt bei der Einstellung ‚*Ja*‘, dass bei der Zusammenstellung von Rohwaresammeldruck-Belegen mittels der Funktion [Sammel-Erstdruck](../sammelabrechnung/sammelerstdruck.md) die Sammelbelege bei unterschiedlichen Rohwarengruppen der Einzelbelege getrennt werden.

### Sammeldruck-Trennung: Schemanummer

  Parameternummer: 103

  Einstellungen für Rohwarengruppen und Abrechnungsschemata sind nicht möglich.

  Optionen:

- **Nein**
- **Ja**

  Dieser Parameter bewirkt bei der Einstellung ‚*Ja*‘, dass bei der Zusammenstellung von Rohwaresammeldruck-Belegen mittels der Funktion [Sammel-Erstdruck](../sammelabrechnung/sammelerstdruck.md) die Sammelbelege bei unterschiedlichen Abrechnungsschemata der Einzelbelege getrennt werden.

### Sammeldruck-Trennung: Liefer-Artikelnr.

  Parameternummer: 104

  Einstellungen für Rohwarengruppen und Abrechnungsschemata sind nicht möglich.

  Optionen:

- **Nein**
- **Ja**

  Dieser Parameter bewirkt bei der Einstellung ‚*Ja*‘, dass bei der Zusammenstellung von Rohwaresammeldruck-Belegen mittels der Funktion [Sammel-Erstdruck](../sammelabrechnung/sammelerstdruck.md) die Sammelbelege bei unterschiedlichen Artikelnummer der jeweiligen Liefer-Position der Einzelbelege getrennt werden.

### Sammeldruck-Trennung: Liefermonat

  Parameternummer: 105

  Einstellungen für Rohwarengruppen und Abrechnungsschemata sind nicht möglich.

  Optionen:

- **Nein**
- **Ja**

  Dieser Parameter bewirkt bei der Einstellung ‚*Ja*‘, dass bei der Zusammenstellung von Rohwaresammeldruck-Belegen mittels der Funktion [Sammel-Erstdruck](../sammelabrechnung/sammelerstdruck.md) die Sammelbelege getrennt werden, wenn das jeweilige Lieferdatum der Einzelbelege in unterschiedliche Monate fällt.

### Sammeldruck-Trennung: Lieferwoche

  Parameternummer: 106

  Einstellungen für Rohwarengruppen und Abrechnungsschemata sind nicht möglich.

  Optionen:

- **Nein**
- **Ja**

  Dieser Parameter bewirkt bei der Einstellung ‚*Ja*‘, dass bei der Zusammenstellung von Rohwaresammeldruck-Belegen mittels der Funktion [Sammel-Erstdruck](../sammelabrechnung/sammelerstdruck.md) die Sammelbelege getrennt werden, wenn das jeweilige Lieferdatum der Einzelbelege in unterschiedliche Wochen fällt.

### Sammeldruck-Trennung: Lieferdatum

  Parameternummer: 189

  Einstellungen für Rohwarengruppen und Abrechnungsschemata sind nicht möglich.

  Optionen:

- **Nein**
- **Ja**

  Dieser Parameter bewirkt bei der Einstellung ‚*Ja*‘, dass bei der Zusammenstellung von Rohwaresammeldruck-Belegen mittels der Funktion [Sammel-Erstdruck](../sammelabrechnung/sammelerstdruck.md) die Sammelbelege bei unterschiedlichen Lagernummern der Einzelbelege getrennt werden.

### Sammeldruck-Trennung: Lagernummer

  Parameternummer: 132

  Einstellungen für Rohwarengruppen und Abrechnungsschemata sind nicht möglich.

  Optionen:

- **Nein**
- **Ja**

  Dieser Parameter bewirkt bei der Einstellung ‚*Ja*‘, dass bei der Zusammenstellung von Rohwaresammeldruck-Belegen mittels der Funktion [Sammel-Erstdruck](../sammelabrechnung/sammelerstdruck.md) die Sammelbelege bei unterschiedlichen Lagernummern der Einzelbelege getrennt werden.

### Sammeldruck-Trennung: Vertretergruppe

  Parameternummer: 138

  Einstellungen für Rohwarengruppen und Abrechnungsschemata sind nicht möglich.

  Optionen:

- **Nein**
- **Ja**

  Dieser Parameter bewirkt bei der Einstellung ‚*Ja*‘, dass bei der Zusammenstellung von Rohwaresammeldruck-Belegen mittels der Funktion [Sammel-Erstdruck](../sammelabrechnung/sammelerstdruck.md) die Sammelbelege bei unterschiedlichen Vertretergruppen der Einzelbelege getrennt werden.

  **Sammeldruck-Trennung: Kontrakt**

  Parameternummer: 152

  Einstellungen für Rohwarengruppen und Abrechnungsschemata sind nicht möglich.

  Optionen:

- **Nein**
- **Ja**

  Dieser Parameter bewirkt bei der Einstellung ‚*Ja*‘, dass bei der Zusammenstellung von Rohwaresammeldruck-Belegen mittels der Funktion [Sammel-Erstdruck](../sammelabrechnung/sammelerstdruck.md) die Sammelbelege bei unterschiedlichen Kontraktzuordnungen der jeweiligen Liefer-Position der Einzelbelege getrennt werden.

### Sammeldruck-Trennung: Versandadresse

  Parameternummer: 166

  Einstellungen für Rohwarengruppen und Abrechnungsschemata sind nicht möglich.

  Optionen:

- **Nein**
- **Ja**

  Dieser Parameter bewirkt bei der Einstellung ‚*Ja*‘, dass bei der Zusammenstellung von Rohwaresammeldruck-Belegen mittels der Funktion [Sammel-Erstdruck](../sammelabrechnung/sammelerstdruck.md) die Sammelbelege bei unterschiedlicher ID der den Einzelbelegen zugeordneten Versandadressen getrennt werden.

### Sammeldruck-Trennung: Währungskurs

  Parameternummer: 188

  Einstellungen für Rohwarengruppen und Abrechnungsschemata sind nicht möglich.

  Optionen:

- **Nein**
- **Ja**

  Dieser Parameter bewirkt bei der Einstellung ‚*Ja*‘, dass bei der Zusammenstellung von Rohwaresammeldruck-Belegen mittels der Funktion [Sammel-Erstdruck](../sammelabrechnung/sammelerstdruck.md) die Sammelbelege von Fremdwährungsbelegen bei unterschiedlichem Währungskurs der Einzelbelege getrennt werden.

### Sammeldrucktrennungs-Hinweis generieren

  Parameternummer: 164

  Einstellungen für Rohwarengruppen und Abrechnungsschemata sind nicht möglich.

  Optionen:

- **Nein**
- **Ja**

  Ist der Parameter mit dem Wert ‚*Ja*‘ eingestellt, so wird bei der Zusammenstellung eines Rohwaresammeldruck-Belegs mittels der Funktion [Sammel-Erstdruck](../sammelabrechnung/sammelerstdruck.md) bei jeder Belegtrennung ein Hinweis erzeugt, der die aktuelle Liefernummer und den Grund der Trennung enthält. Diese Hinweise werden am Ende der Funktion zusammen mit anderen aufgetretenen Hinweisen, Warnungen und Fehlermeldungen aufgelistet.

### Sammelbuchungen bei Sammeldruck

  Parameternummer: 122

  Einstellungen für Rohwarengruppen und Abrechnungsschemata sind nicht möglich.

  Optionen:

- **Nein**
- **Ja**

  Rohwaresammeldruckbelege können mit der Drucknummer als Belegnummer und dem Druckdatum als Belegdatum als ein Fibu-Beleg an die Finanzbuchhaltung übertragen werden, wenn dieser Parameter mit dem Wert ‚Ja‘ eingestellt ist. In diesem Fall stehen hierfür in den Anwendungen zum Bearbeiten von Rohwarebelegen <strong>[RWB] [RWBV]</strong> und Fibu-Übertrag **[FIB]** gesonderte Anwendungs-Varianten zur Verfügung. Einzelne Rohwarebelege, die Teil eines Rohwaresammeldruckbelegs sind, können dann nicht einzeln übertragen werden.

### Sammeldruck- Folgeabschlag mit Sammelabschlag-Prüfung

  Parameternummer: 156

  Einstellungen für Rohwarengruppen und Abrechnungsschemata sind nicht möglich.

  Optionen:

- **ohne**
- **mit Sammeldruck-Abbruch**
- **mit Warnung ohne Abbruch**

  Mittels dieses Parameters kann bei der Erstellung von Rohwaresammeldruckbelegen der Abrechnungsstufe ‚*Folgeabschlag*‘ eine Routine aktiviert werden, die die Vollständigkeit von im Folgeabschlagsammelbeleg enthaltenen Abschlagsammelbelegen testet. Ist ein zu einem Folgeabschlag des aktuellen Sammelbelegs gehöriger Abschlagbeleg Teil eines Abschlagsammelbelegs, so wird geprüft, ob auch alle anderen Folgeabschlagbelege, deren Vorgängerabschlagbelege Teil desselben Abschlagsammelbelegs sind, im aktuellen Folgeabschlagsammelbeleg enthalten sind. Ist dieses nicht der Fall, so wird entsprechend der Einstellung dieses Parameters der Folgeabschlagsammelbeleg verworfen oder lediglich eine entsprechende Warnmeldung erzeugt.

### Sammeldruck- Finale mit Sammelabschlag-Prüfung

  Parameternummer: 157

  Einstellungen für Rohwarengruppen und Abrechnungsschemata sind nicht möglich.

  Optionen:

- **ohne**
- **mit Sammeldruck-Abbruch**
- **mit Warnung ohne Abbruch**

  Mittels dieses Parameters kann bei der Erstellung von Rohwaresammeldruckbelegen der Abrechnungsstufe ‚*Finale*‘ eine Routine aktiviert werden, die die Vollständigkeit von im Finalsammelbeleg enthaltenen Abschlag- und Folgeabschlagsammelbelegen testet. Ist ein zu einem Finalbeleg des aktuellen Sammelbelegs gehöriger Abschlag- oder Folgeabschlagbeleg Teil eines Sammelbelegs, so wird geprüft, ob auch alle anderen Finalbelege, deren Vorgängerbelege Teil desselben Sammelbelegs sind, im aktuellen Finalsammelbeleg enthalten sind. Ist dieses nicht der Fall, so wird entsprechend der Einstellung dieses Parameters der Finalsammelbeleg verworfen oder lediglich eine entsprechende Warnmeldung erzeugt.

### Fibuübertrag nach Steuersatzänderung

  Parameternummer: 190

  Einstellungen für Rohwarengruppen und Abrechnungsschemata sind nicht möglich.

  Optionen:

- **nur Warnung**
- **Fibuübertrag ablehnen**

  Dieser Parameter regelt das Verhalten des Fibuübertragmoduls für den Fall, dass sich in einer oder mehreren Belegpositionen der Steuersatz zur korrespondierenden Position eines bereits zuvor gebuchten Vorgängerbelegs der optionalen Kette *Abschlag->Folgeabschlag->Finalabrechnung* geändert hat.  
Bei der Einstellung **nur Warnung** wird eine Warnmeldung im Rahmen der Meldungsausgabe erzeugt, der Beleg aber dennoch gebucht. In diesem Fall sind unter Umständen Steuerumbuchungen in der Finanzbuchhaltung vorzunehmen.  
Bei der Einstellung **Fibuübertrag ablehnen** wird eine Warnmeldung im Rahmen der Meldungsausgabe erzeugt und der Fibuübertrag abgelehnt. In diesem Fall muss entweder die Steuergruppe des aktuellen Belegs angepasst oder die Vorgangskette rückabgewickelt und mit der gültigen Steuergruppe neu erzeugt werden.  
    

### Buchungstext bei Sammelbuchung

  Parameternummer: 135

  Einstellungen für Rohwarengruppen und Abrechnungsschemata sind nicht möglich.

  Optionen:

- **siehe SPA-Einstellung**
- **Kurzinfo**
- **Vorgangsklasse**
- **Vorgangs-Unterklasse**
- **Klasse + Unterklasse**
- **Sammeldrucknummer**
- **Vorgangsklasse + Sammeldrucknummer**
- **Unterklasse + Sammeldrucknummer**
- **Klasse + Unterklasse + Sammeldrucknummer**
- **erster Warenpositions-Text**
- **Abrechnungsschemabezeichnung**
- **Klasse + Abrechnungsschemabezeichnung**
- **Unterklasse + Abrechnungsschemabezeichnung**
- **Klasse + Unterklasse + Abrechnungsschemabezeichnung**
- **Abrechnungsschemabezeichnung + Sammeldrucknummer**
- **Klasse + Abrechnungsschemabezeichnung + Sammeldrucknummer**
- **Unterklasse + Abrechnungsschemabezeichnung + Sammeldrucknummer**
- **Klasse + Unterklasse + Abrechnungsschemabezeichnung + Sammeldrucknummer**
- **Klasse + Rohwarengruppenbezeichnung**
- **Unterklasse + Rohwarengruppenbezeichnung**
- **Klasse + Unterklasse + Rohwarengruppenbezeichnung**
- **Rohwarengruppenbezeichnung + Sammeldrucknummer**
- **Klasse + Rohwarengruppenbezeichnung + Sammeldrucknummer**
- **Unterklasse + Rohwarengruppenbezeichnung + Sammeldrucknummer**
- **Klasse + Unterklasse + Rohwarengruppenbezeichnung + Sammeldrucknummer**

  Mittels dieses Parameters kann bei der Übertragung Rohwaresammeldruckbelegen an die Finanzbuchhaltung die Variante des Hauptbuchungstextes eingestellt werden. Bei der Einstellung ‚*siehe SPA-Einstellung*‘ wird der Hauptbuchungstextes entsprechend der Einstellung des Steuerparameters **[SPA]** ‚*Variante Haupt-Buchungstext Einkauf(SPA 170)*‘ beziehungsweise ‚*Variante Haupt-Buchungstext Verkauf(SPA 171)*‘ erzeugt.

### Buchungstext bei Einzelbuchung

  Parameternummer: 136

  Einstellungen für Rohwarengruppen und Abrechnungsschemata sind nicht möglich.

  Optionen:

- **siehe SPA-Einstellung**
- **Kurzinfo**
- **Vorgangsklasse**
- **Vorgangs-Unterklasse**
- **Klasse + Unterklasse**
- **Referenznummer**
- **Vorgangsklasse + Referenznummer**
- **Unterklasse + Referenznummer**
- **Klasse + Unterklasse + Referenznummer**
- **erster Warenpositions-Text**
- **Abrechnungsschemabezeichnung**
- **Klasse + Abrechnungsschemabezeichnung**
- **Unterklasse + Abrechnungsschemabezeichnung**
- **Klasse + Unterklasse + Abrechnungsschemabezeichnung**
- **Abrechnungsschemabezeichnung + Referenznummer**
- **Klasse + Abrechnungsschemabezeichnung + Referenznummer**
- **Unterklasse + Abrechnungsschemabezeichnung + Referenznummer**
- **Klasse + Unterklasse + Abrechnungsschemabezeichnung + Referenznummer**
- **Klasse + Rohwarengruppenbezeichnung**
- **Unterklasse + Rohwarengruppenbezeichnung**
- **Klasse + Unterklasse + Rohwarengruppenbezeichnung**
- **Rohwarengruppenbezeichnung + Referenznummer**
- **Klasse + Rohwarengruppenbezeichnung + Referenznummer**
- **Unterklasse + Rohwarengruppenbezeichnung + Referenznummer**
- **Klasse + Unterklasse + Rohwarengruppenbezeichnung + Referenznummer**

  Mittels dieses Parameters kann bei der Übertragung Rohwaresammeldruckbelegen an die Finanzbuchhaltung die Variante des Hauptbuchungstextes eingestellt werden. Bei der Einstellung ‚*siehe SPA-Einstellung*‘ wird der Hauptbuchungstextes entsprechend der Einstellung des Steuerparameters **[SPA]** ‚*Variante Haupt-Buchungstext Einkauf(SPA 170)*‘ beziehungsweise ‚*Variante Haupt-Buchungstext Verkauf(SPA 171)*‘ erzeugt. Bei den Einstellungen mit Referenznummer, wird die Liefernummer als Referenznummer berücksichtigt.

### Steuergruppe bei Kundenänderung

  Parameternummer: 159

  Einstellungen für Rohwarengruppen und Abrechnungsschemata sind nicht möglich.

  Optionen:

- **aus Quellbeleg**
- **wie Beleg-Erfassung**

  Speziell für die Funktion ***Schema-/Kundenänderung*** im Rohwarebearbeitungsmodul **[RWB] RWBV]** steuert dieser Parameter die Behandlung der Steuergruppe bei der Änderung des Lieferanten/Kunden eines Rohwarelieferscheins. Es kann entweder die Steuergruppe des Quellbelegs beibehalten werden oder entsprechend der Rohwareparameter [Steuergruppenvorbelegung](./rohwareparameter_uebersicht.md#RWPA_40) und [Steuergruppenfestwert](./rohwareparameter_uebersicht.md#RWPA_41) bestimmt werden.

  **Steuergruppe**

  Parameternummer: 39

  Einstellungen für Rohwarengruppen und Abrechnungsschemata sind möglich.

  Optionen:

- **keine Anzeige**
- **nur Anzeige**
- **Erfassung**

  Mit diesem Parameter wird für die Erfassung, Anzeige und Korrektur von Rohwarebelegen festgelegt, ob die Steuergruppe auf der Erfassungsmaske unterdrückt, nur dargestellt oder änderbar ist.

  **Steuergruppenvorbelegung**

  Parameternummer: 40

  Einstellungen für Rohwarengruppen und Abrechnungsschemata sind möglich.

  Optionen:

- **aus Kundenstamm**
- **fester Wert**

  Für die Erfassung und Erzeugung von Rohwarelieferscheine steuert dieser Parameter die Vorbelegung der Steuergruppe. Diese kann aus dem Stammsatz des Lieferanten/Kunden entnommen werden. Die Einstellung ‚*fester Wert*‘ hingegen legt die Steuergruppe auf den dem Parameter [Steuergruppenfestwert](./rohwareparameter_uebersicht.md#RWPA_41) zugewiesenen Wert fest.

  **Steuergruppenfestwert**

  Parameternummer: 41

  Einstellungen für Rohwarengruppen und Abrechnungsschemata sind möglich.

  Optionen: Steuergruppennummer

  Für die Erfassung und Erzeugung von Rohwarelieferscheine beinhaltet dieser Parameter die Vorbelegung der Steuergruppe, wenn der zugehörige Parameter [Steuergruppenvorbelegung](./rohwareparameter_uebersicht.md#RWPA_40) mit der Einstellung ‚*fester Wert*‘ versehen ist.

  **Lagerplatz**

  Parameternummer: 11

  Einstellungen für Rohwarengruppen und Abrechnungsschemata sind möglich.

  Optionen:

- **keine Anzeige**
- **nur Anzeige**
- **Erfassung**

  Mit diesem Parameter wird für die Erfassung, Anzeige und Korrektur von Rohwarebelegen festgelegt, ob die Lagerplatznummer auf der Erfassungsmaske unterdrückt, nur dargestellt oder änderbar sein soll.

  **Periode Ware**

  Parameternummer: 91

  Einstellungen für Rohwarengruppen und Abrechnungsschemata sind nicht möglich.

  Optionen:

- **keine Anzeige**
- **nur Anzeige**
- **Erfassung**

  Mit diesem Parameter wird für die Erfassung, Anzeige und Korrektur von Rohwarebelegen festgelegt, ob die Warenwirtschafts-Periode auf der Erfassungsmaske unterdrückt, nur dargestellt oder änderbar ist.

  **UST-ID Kunde**

  Parameternummer: 161

  Einstellungen für Rohwarengruppen und Abrechnungsschemata sind möglich.

  Optionen:

- **keine Anzeige**
- **nur Anzeige**
- **Erfassung**

  Mit diesem Parameter wird für die Erfassung, Anzeige und Korrektur von Rohwarebelegen festgelegt, ob die Umsatzsteuer-ID des Lieferanten beziehungsweise Kunden auf der Erfassungsmaske unterdrückt, nur dargestellt oder änderbar sein soll.

  **UST-ID Firma**

  Parameternummer: 161

  Einstellungen für Rohwarengruppen und Abrechnungsschemata sind möglich.

  Optionen:

- **keine Anzeige**
- **nur Anzeige**
- **Erfassung**

  Mit diesem Parameter wird für die Erfassung, Anzeige und Korrektur von Rohwarebelegen festgelegt, ob die eigene Umsatzsteuer-ID des aktuellen Mandanten auf der Erfassungsmaske unterdrückt, nur dargestellt oder änderbar sein soll.

  **Liefer-Datum**

  Parameternummer: 6

  Einstellungen für Rohwarengruppen und Abrechnungsschemata sind möglich.

  Optionen:

- **keine Anzeige**
- **nur Anzeige**
- **Erfassung**

  Mit diesem Parameter wird für die Erfassung, Anzeige und Korrektur von Rohwarebelegen festgelegt, ob das Lieferdatum auf der Erfassungsmaske unterdrückt, nur dargestellt oder bei der Erfassung auch änderbar sein soll.

  **Lieferscheinnummer**

  Parameternummer: 7

  Einstellungen für Rohwarengruppen und Abrechnungsschemata sind möglich.

  Optionen:

- **keine Anzeige**
- **nur Anzeige**
- **Erfassung**

  Mit diesem Parameter wird für die Erfassung, Anzeige und Korrektur von Rohwarebelegen festgelegt, ob die Lieferscheinnummer auf der Erfassungsmaske unterdrückt, nur dargestellt oder bei der Erfassung auch änderbar sein soll. Dieser Parameter wirkt jedoch nicht, wenn entsprechend der Einstellung des Parameters [Vorbelegung Lieferscheinnummer](./rohwareparameter_uebersicht.md#RWPA_8) die Vorbelegung der Lieferscheinnummer nicht aus dem Nummernkreis vorbelegt wird.

  **Vorbelegung Lieferscheinnummer**

  Parameternummer: 8

  Einstellungen für Rohwarengruppen und Abrechnungsschemata sind möglich.

  Optionen:

- **Aus Nummernkreis**
- **Letzte Nummer + 1**
- **Vorbelegung mit 0**

  Mit diesem Parameter wird für die Erfassung von Rohwarenbelegen festgelegt, wie die Liefernummer vorbelegt wird. Neben der Vorbelegungsmöglichkeit aus dem zugeordneten Nummernkreis bei jedem neuen Beleg bei der Erfassung mehrerer Belege, ohne dass die Erfassungsmaske verlassen wird, kann auch die Option genutzt werden, dass zur Initialisierung eines neuen Belegs die zuvor genutzte Nummer um 1 erhöht wird. Die Vorbelegung mit 0 ist ebenfalls möglich, um zum Beispiel bei der manuellen Nacherfassung von Belegen sicherzustellen, dass das Maskenfeld auf jeden Fall bedient wird.

  **Wiegenoten-Nummer**

  Parameternummer: 9

  Einstellungen für Rohwarengruppen und Abrechnungsschemata sind möglich.

  Optionen:

- **keine Anzeige**
- **nur Anzeige**
- **Erfassung**

  Mit diesem Parameter wird für die Erfassung, Anzeige und Korrektur von Rohwarebelegen festgelegt, ob die Wiegenotennummer auf der Erfassungsmaske unterdrückt, nur dargestellt oder bei der Erfassung auch änderbar sein soll. Dieser Parameter wirkt jedoch nicht, wenn die Einstellung des Parameters [Vorbelegung Wiegenoten-Nummer](./rohwareparameter_uebersicht.md#RWPA_10) ‚*letzte Nummer + 1*‘ ist.

  **Vorbelegung Wiegenoten-Nummer**

  Parameternummer: 10

  Einstellungen für Rohwarengruppen und Abrechnungsschemata sind möglich.

  Optionen:

- **letzte Nummer + 1**
- **gleich Lieferscheinnummer**
- **Vorbelegung mit 0**

  Mit diesem Parameter wird für die Erfassung von Rohwarenbelegen festgelegt, wie die Wiegenotennummer vorbelegt wird. Bei der Erfassung mehrerer Belege, ohne dass die Erfassungsmaske verlassen wird, kann die Option genutzt werden, dass zur Initialisierung eines neuen Belegs die zuvor genutzte Nummer um 1 erhöht wird. Die Vorbelegung mit der Lieferscheinnummer des Belegs bewirkt auch die automatische Anpassung bei Änderung der Lieferscheinnummer. Die Vorbelegung mit 0 ist ebenfalls möglich und als gültige Wiegenotennummer an dieser Stelle auch zulässig.

  **Versandart**

  Parameternummer: 18

  Einstellungen für Rohwarengruppen und Abrechnungsschemata sind möglich.

  Optionen:

- **keine Anzeige**
- **nur Anzeige**
- **Erfassung**

  Mit diesem Parameter wird für die Erfassung, Anzeige und Korrektur von Rohwarebelegen festgelegt, ob die Versandart auf der Erfassungsmaske unterdrückt, nur dargestellt oder änderbar sein soll.

  **Vorbelegung Versandart**

  Parameternummer: 19

  Einstellungen für Rohwarengruppen und Abrechnungsschemata sind möglich.

  Optionen: Versandartnummer oder 0

  Für die Erfassung und Erzeugung von Rohwarelieferscheine beinhaltet dieser Parameter die Vorbelegung der Versandart. Ist der Parameterwert ‚0‘, so wird zur Vorbelegung die dem Kunden beziehungsweise Lieferanten zugeordnete Versandart genutzt.

  **Versandadresse**

  Parameternummer: 20

  Einstellungen für Rohwarengruppen und Abrechnungsschemata sind möglich.

  Optionen:

- **keine Anzeige**
- **nur Anzeige**
- **Erfassung**

  Mit diesem Parameter wird für die Erfassung, Anzeige und Korrektur von Rohwarebelegen festgelegt, ob die Versandadresse auf der Erfassungsmaske unterdrückt, nur dargestellt oder änderbar sein soll.

  **Vorbelegung Versandadresse aus Kunde**

  Parameternummer: 165

  Einstellungen für Rohwarengruppen und Abrechnungsschemata sind möglich.

  Optionen:

- **nein**
- **ja**

  Für die Erfassung und Erzeugung von Rohwarelieferscheine legt dieser Parameter fest, ob die Versandadresse aus dem Kunden-/Lieferantenstamm vorbelegt werden soll.

  **Preisansprung nach Menge**

  Parameternummer: 112

  Einstellungen für Rohwarengruppen und Abrechnungsschemata sind möglich.

  Optionen:

- **nein**
- **ja**

  Für die Erfassung und Korrektur von Rohwarebelegen kann mit diesem Parameter durch die Einstellung ‚*Ja*‘ festgelegt werden, dass nach der Eingabe der Liefermenge die Preis-Spalte des Erfassungs-Grids angesprungen wird. Andernfalls wird in der nächstmöglichen Positionszeile fortgefahren.

  **Arbeits-Regel**

  Parameternummer: 137

  Einstellungen für Rohwarengruppen und Abrechnungsschemata sind möglich.

  Optionen:

- **keine Anzeige**
- **nur Anzeige**
- **Erfassung**

  Mit diesem Parameter wird für die Erfassung, Anzeige und Korrektur von Rohwarebelegen festgelegt, ob das Attribut ‚*Arbeits-Regel*‘ auf der Erfassungsmaske unterdrückt, nur dargestellt oder änderbar sein soll.

  **Anzahl der Analyseproben**

  Parameternummer: 147

  Einstellungen für Rohwarengruppen und Abrechnungsschemata sind möglich.

  Optionen:

- **keine Anzeige**
- **nur Anzeige**
- **Erfassung**

  Mit diesem Parameter wird für die Erfassung, Anzeige und Korrektur von Rohwarebelegen festgelegt, ob das Attribut ‚*Anzahl der Analyseproben*‘ auf der Erfassungsmaske unterdrückt, nur dargestellt oder änderbar sein soll. Der Wert dieses Attributs wird bei der Ermittlung von Analysewerten per Formel mit dem Typ ‚Fix-Durchschnitt‘ benötigt.

  **Herkunft-/Ziel-Land**

  Parameternummer: 153

  Einstellungen für Rohwarengruppen und Abrechnungsschemata sind möglich.

  Optionen:

- **keine Anzeige**
- **nur Anzeige**
- **Erfassung**

  Mit diesem Parameter wird für die Erfassung, Anzeige und Korrektur von Rohwarebelegen festgelegt, ob das Feld ‚*Herkunft-/Ziel-Land*‘ auf der Erfassungsmaske unterdrückt, nur dargestellt oder änderbar sein soll.

  **Herkunft-/Ziel-Region**

  Parameternummer: 154

  Einstellungen für Rohwarengruppen und Abrechnungsschemata sind möglich.

  Optionen:

- **keine Anzeige**
- **nur Anzeige**
- **Erfassung**

  Mit diesem Parameter wird für die Erfassung, Anzeige und Korrektur von Rohwarebelegen festgelegt, ob das Feld ‚*Herkunft-/Ziel-Region*‘ auf der Erfassungsmaske unterdrückt, nur dargestellt oder änderbar sein soll.

  **Ship-From**

  Parameternummer: 179

  Einstellungen für Rohwarengruppen und Abrechnungsschemata sind möglich.

  Optionen:

- **keine Anzeige**
- **nur Anzeige**
- **Erfassung**

  Mit diesem Parameter wird für die Erfassung, Anzeige und Korrektur von Rohwarebelegen festgelegt, ob das Feld ‚*Ship-From*‘ auf der Erfassungsmaske unterdrückt, nur dargestellt oder änderbar sein soll.

  **Ship-To**

  Parameternummer: 180

  Einstellungen für Rohwarengruppen und Abrechnungsschemata sind möglich.

  Optionen:

- **keine Anzeige**
- **nur Anzeige**
- **Erfassung**

  Mit diesem Parameter wird für die Erfassung, Anzeige und Korrektur von Rohwarebelegen festgelegt, ob das Feld ‚*Ship-To*‘ auf der Erfassungsmaske unterdrückt, nur dargestellt oder änderbar sein soll.

  **FA-Beleg-Referenz**

  Parameternummer: 148

  Einstellungen für Rohwarengruppen und Abrechnungsschemata sind möglich.

  Optionen:

- **keine Anzeige**
- **nur Anzeige**
- **Erfassung**

  Mit diesem Parameter wird für die Erfassung, Anzeige und Korrektur von Rohwarebelegen festgelegt, ob das Feld ‚*FA-Beleg-Referenz*‘ auf der Erfassungsmaske unterdrückt, nur dargestellt oder änderbar sein soll.

  **Einlagerungskennzeichen**

  Parameternummer: 167

  Einstellungen für Rohwarengruppen und Abrechnungsschemata sind möglich.

  Optionen:

- **keine Anzeige**
- **nur Anzeige**
- **Erfassung**

  Mit diesem Parameter wird für die Erfassung, Anzeige und Korrektur von Rohwarebelegen festgelegt, ob Einlagerungskennzeichen auf der Erfassungsmaske unterdrückt, nur dargestellt oder änderbar sein soll.

  **Vorbelegung Einlagerung**

  Parameternummer: 168

  Einstellungen für Rohwarengruppen und Abrechnungsschemata sind möglich.

  Optionen:

- **nein**
- **ja**

  Für die Erfassung und Erzeugung von Rohwarelieferscheine legt dieser Parameter fest, wie das Einlagerungskennzeichen vorbelegt werden soll.

  **Einlagerungsvereinnahmungskennzeichen**

  Parameternummer: 169

  Einstellungen für Rohwarengruppen und Abrechnungsschemata sind möglich.

  Optionen:

- **keine Anzeige**
- **nur Anzeige**
- **Erfassung**

  Mit diesem Parameter wird für die Erfassung, Anzeige und Korrektur von Rohwarebelegen festgelegt, ob Einlagerungsvereinnahmungskennzeichen auf der Erfassungsmaske unterdrückt, nur dargestellt oder änderbar sein soll.

  **Vorbelegung Einlagerungsvereinnahmung**

  Parameternummer: 170

  Einstellungen für Rohwarengruppen und Abrechnungsschemata sind möglich.

  Optionen:

- **nein**
- **ja**

  Für die Erfassung und Erzeugung von Rohwarelieferscheine legt dieser Parameter fest, wie das Vereinnahmungskennzeichen von Einlagerungen vorbelegt werden soll. Wird jedoch bereits das Einlagerungskennzeichen mit ‚*Ja*‘ vorbelegt, so kann das Vereinnahmungskennzeichen nur auf ‚*Nein*‘ gesetzt werden.

  **Nachhaltigkeits-Status**

  Parameternummer: 181

  Einstellungen für Rohwarengruppen und Abrechnungsschemata sind möglich.

  Optionen:

- **keine Anzeige**
- **nur Anzeige**
- **Erfassung**

  Mit diesem Parameter wird für die Erfassung, Anzeige und Korrektur von Rohwarebelegen festgelegt, ob der Nachhaltigkeitsstatus auf der Erfassungsmaske unterdrückt, nur dargestellt oder änderbar sein soll.

  **Nachhaltigkeit-Anbauland/Region**

  Parameternummer: 182

  Einstellungen für Rohwarengruppen und Abrechnungsschemata sind möglich.

  Optionen:

- **keine Anzeige**
- **nur Anzeige**
- **Erfassung**

  Mit diesem Parameter wird für die Erfassung, Anzeige und Korrektur von Rohwarebelegen festgelegt, ob das Feld für Anbauland/-Region zur Nachhaltigkeit auf der Erfassungsmaske unterdrückt, nur dargestellt oder änderbar sein soll.

  **THG/TSW Anbau (Nachhaltigkeit)**

  Parameternummer: 183

  Einstellungen für Rohwarengruppen und Abrechnungsschemata sind möglich.

  Optionen:

- **keine Anzeige**
- **nur Anzeige**
- **Erfassung**

  Mit diesem Parameter wird für die Erfassung, Anzeige und Korrektur von Rohwarebelegen festgelegt, ob das Feld für den THG/TSW-Wert des Anbaus zur Nachhaltigkeit auf der Erfassungsmaske unterdrückt, nur dargestellt oder änderbar sein soll.

  **THG/TSW Lieferung (Nachhaltigkeit)**

  Parameternummer: 184

  Einstellungen für Rohwarengruppen und Abrechnungsschemata sind möglich.

  Optionen:

- **keine Anzeige**
- **nur Anzeige**
- **Erfassung**

  Mit diesem Parameter wird für die Erfassung, Anzeige und Korrektur von Rohwarebelegen festgelegt, ob das Feld für den THG/TSW-Wert der Lieferung zur Nachhaltigkeit auf der Erfassungsmaske unterdrückt, nur dargestellt oder änderbar sein soll.

  **THG/TSW Verarbeitung (Nachhaltigkeit)**

  Parameternummer: 185

  Einstellungen für Rohwarengruppen und Abrechnungsschemata sind möglich.

  Optionen:

- **keine Anzeige**
- **nur Anzeige**
- **Erfassung**

  Mit diesem Parameter wird für die Erfassung, Anzeige und Korrektur von Rohwarebelegen festgelegt, ob das Feld für den THG/TSW-Wert der Verarbeitung zur Nachhaltigkeit auf der Erfassungsmaske unterdrückt, nur dargestellt oder änderbar sein soll.

  **Nachhaltigkeits-Zertifikat**

  Parameternummer: 186

  Einstellungen für Rohwarengruppen und Abrechnungsschemata sind möglich.

  Optionen:

- **keine Anzeige**
- **nur Anzeige**
- **Erfassung**

  Mit diesem Parameter wird für die Erfassung, Anzeige und Korrektur von Rohwarebelegen festgelegt, ob das Feld für die Zertifikat-ID zur Nachhaltigkeit auf der Erfassungsmaske unterdrückt, nur dargestellt oder änderbar sein soll.

  **Zahlungsart Abschlag**

  Parameternummer: 23

  Einstellungen für Rohwarengruppen und Abrechnungsschemata sind möglich.

  Optionen:

- **keine Anzeige**
- **nur Anzeige**
- **Erfassung**

  Mit diesem Parameter wird für die Erfassung, Anzeige und Korrektur von Rohwarebelegen festgelegt, ob die Zahlungsart für Rohwarebelege der Stufe ‚*Abschlag*‘ auf der Erfassungsmaske unterdrückt, nur dargestellt oder änderbar sein soll.

  **Vorbelegung Zahlungsart Abschlag**

  Parameternummer: 24

  Einstellungen für Rohwarengruppen und Abrechnungsschemata sind möglich.

  Optionen: Versandartnummer oder 0

  Für die Erfassung und Erzeugung von Rohwarelieferscheine beinhaltet dieser Parameter die Vorbelegung der Zahlungsart der Stufe ‚*Abschlag*‘. Ist der Parameterwert ‚-1‘, so wird zur Vorbelegung die dem Kunden beziehungsweise Lieferanten zugeordnete Zahlungsart genutzt.

  **Zahlungsart Folgeabschlag**

  Parameternummer: 25

  Einstellungen für Rohwarengruppen und Abrechnungsschemata sind möglich.

  Optionen:

- **keine Anzeige**
- **nur Anzeige**
- **Erfassung**

  Mit diesem Parameter wird für die Erfassung, Anzeige und Korrektur von Rohwarebelegen festgelegt, ob die Zahlungsart für Rohwarebelege der Stufe ‚*Folgeabschlag*‘ auf der Erfassungsmaske unterdrückt, nur dargestellt oder änderbar sein soll.

  **Vorbelegung Zahlungsart Folgeabschlag**

  Parameternummer: 26

  Einstellungen für Rohwarengruppen und Abrechnungsschemata sind möglich.

  Optionen: Versandartnummer oder 0

  Für die Erfassung und Erzeugung von Rohwarelieferscheine beinhaltet dieser Parameter die Vorbelegung der Zahlungsart der Stufe ‚*Folgeabschlag*‘. Ist der Parameterwert ‚-1‘, so wird zur Vorbelegung die dem Kunden beziehungsweise Lieferanten zugeordnete Zahlungsart genutzt.

  **Zahlungsart Finale**

  Parameternummer: 27

  Einstellungen für Rohwarengruppen und Abrechnungsschemata sind möglich.

  Optionen:

- **keine Anzeige**
- **nur Anzeige**
- **Erfassung**

  Mit diesem Parameter wird für die Erfassung, Anzeige und Korrektur von Rohwarebelegen festgelegt, ob die Zahlungsart für Rohwarebelege der Stufe ‚*Finalabrechnung*‘ auf der Erfassungsmaske unterdrückt, nur dargestellt oder änderbar sein soll.

  **Vorbelegung Zahlungsart Finale**

  Parameternummer: 28

  Einstellungen für Rohwarengruppen und Abrechnungsschemata sind möglich.

  Optionen: Versandartnummer oder 0

  Für die Erfassung und Erzeugung von Rohwarelieferscheine beinhaltet dieser Parameter die Vorbelegung der Zahlungsart der Stufe ‚*Finalabrechnung*‘. Ist der Parameterwert ‚-1‘, so wird zur Vorbelegung die dem Kunden beziehungsweise Lieferanten zugeordnete Zahlungsart genutzt.

  **Zahlungsbedingung Abschlag**

  Parameternummer: 56

  Einstellungen für Rohwarengruppen und Abrechnungsschemata sind möglich.

  Optionen:

- **keine Anzeige**
- **nur Anzeige**
- **ZB-Nummer erfassen**
- **ZB-Nummer + Werte erfassbar**
- **ZB-Nummer + Werte immer erfassen**
- **Nur ZB-Werte erfassbar**
- **nur Anzeige mit ZB-Werten**
- **ZB-Nummer erfassen, Werte anzeigen**

  Mit diesem Parameter wird für die Erfassung, Anzeige und Korrektur von Rohwarebelegen festgelegt, wie die Zahlungsbedingungsfelder für Rohwarebelege der Stufe ‚*Abschlag*‘ auf der Bearbeitungsmaske zu behandeln sind. Das betrifft sowohl das Maskenfeld für die Zahlungsbedingungsnummer als auch den Bereich mit den zugehörigen Zahlungsbedingungswerten.

  **Feste ZB für Abschlag**

  Parameternummer: 57

  Einstellungen für Rohwarengruppen und Abrechnungsschemata sind möglich.

  Optionen:

- **nein**
- **ja**

  Für die Erfassung und Erzeugung von Rohwarelieferscheine legt dieser Parameter fest, wie die Zahlungsbedingung für Rohwarebelege der Stufe ‚*Abschlag*‘ vorbelegt werden. Die Einstellung ‚*Nein*‘ bewirkt die Vorbelegung mit der dem Kunden beziehungsweise Lieferanten zugeordneten Zahlungsbedingung. Ist hier der Wert ‚*Ja*‘ zugeordnet, so wird der Wert des Parameters [Abschlag-Zahlungsbedingungs-Nummer](./rohwareparameter_uebersicht.md#RWPA_58) zur Vorbelegung herangezogen.

  **Abschlag-Zahlungsbedingungs-Nummer**

  Parameternummer: 58

  Einstellungen für Rohwarengruppen und Abrechnungsschemata sind möglich.

  Optionen: Versandartnummer oder 0

  Für die Erfassung und Erzeugung von Rohwarelieferscheine beinhaltet dieser Parameter die Vorbelegung der Zahlungsbedingungsnummer der Stufe ‚*Abschlag*‘, wenn der Wert des Parameters [Feste ZB für Abschlag](./rohwareparameter_uebersicht.md#RWPA_57) mit dem Wert ‚*Ja‘* eingestellt ist.

  **Zahlungsbedingung Folgeabschlag**

  Parameternummer: 59

  Einstellungen für Rohwarengruppen und Abrechnungsschemata sind möglich.

  Optionen:

- **keine Anzeige**
- **nur Anzeige**
- **ZB-Nummer erfassen**
- **ZB-Nummer + Werte erfassbar**
- **ZB-Nummer + Werte immer erfassen**
- **Nur ZB-Werte erfassbar**
- **nur Anzeige mit ZB-Werten**
- **ZB-Nummer erfassen, Werte anzeigen**

  Mit diesem Parameter wird für die Erfassung, Anzeige und Korrektur von Rohwarebelegen festgelegt, wie die Zahlungsbedingungsfelder für Rohwarebelege der Stufe ‚*Folgeabschlag*‘ auf der Bearbeitungsmaske zu behandeln sind. Das betrifft sowohl das Maskenfeld für die Zahlungsbedingungsnummer als auch den Bereich mit den zugehörigen Zahlungsbedingungswerten.

  **Feste ZB für Folgeabschlag**

  Parameternummer: 60

  Einstellungen für Rohwarengruppen und Abrechnungsschemata sind möglich.

  Optionen:

- **nein**
- **ja**

  Für die Erfassung und Erzeugung von Rohwarelieferscheine legt dieser Parameter fest, wie die Zahlungsbedingung für Rohwarebelege der Stufe ‚*Folgeabschlag*‘ vorbelegt werden. Die Einstellung ‚*Nein*‘ bewirkt die Vorbelegung mit der dem Kunden beziehungsweise Lieferanten zugeordneten Zahlungsbedingung. Ist hier der Wert ‚*Ja*‘ zugeordnet, so wird der Wert des Parameters [Folgeabschlag-Zahlungsbedingungs-Nummer](./rohwareparameter_uebersicht.md#RWPA_61) zur Vorbelegung herangezogen.

  **Folgeabschlag-Zahlungsbedingungs-Nummer**

  Parameternummer: 61

  Einstellungen für Rohwarengruppen und Abrechnungsschemata sind möglich.

  Optionen: Versandartnummer oder 0

  Für die Erfassung und Erzeugung von Rohwarelieferscheine beinhaltet dieser Parameter die Vorbelegung der Zahlungsbedingungsnummer der Stufe ‚*Folgeabschlag*‘, wenn der Wert des Parameters [Feste ZB für Folgeabschlag](./rohwareparameter_uebersicht.md#RWPA_60) mit dem Wert ‚*Ja‘* eingestellt ist.

  **Zahlungsbedingung für Finale**

  Parameternummer: 62

  Einstellungen für Rohwarengruppen und Abrechnungsschemata sind möglich.

  Optionen:

- **keine Anzeige**
- **nur Anzeige**
- **ZB-Nummer erfassen**
- **ZB-Nummer + Werte erfassbar**
- **ZB-Nummer + Werte immer erfassen**
- **Nur ZB-Werte erfassbar**
- **nur Anzeige mit ZB-Werten**
- **ZB-Nummer erfassen, Werte anzeigen**

  Mit diesem Parameter wird für die Erfassung, Anzeige und Korrektur von Rohwarebelegen festgelegt, wie die Zahlungsbedingungsfelder für Rohwarebelege der Stufe ‚*Finalabrechnung*‘ auf der Bearbeitungsmaske zu behandeln sind. Das betrifft sowohl das Maskenfeld für die Zahlungsbedingungsnummer als auch den Bereich mit den zugehörigen Zahlungsbedingungswerten.

  **Feste ZB für Finale**

  Parameternummer: 63

  Einstellungen für Rohwarengruppen und Abrechnungsschemata sind möglich.

  Optionen:

- **nein**
- **ja**

  Für die Erfassung und Erzeugung von Rohwarelieferscheine legt dieser Parameter fest, wie die Zahlungsbedingung für Rohwarebelege der Stufe ‚*Finalabrechnung*‘ vorbelegt werden. Die Einstellung ‚*Nein*‘ bewirkt die Vorbelegung mit der dem Kunden beziehungsweise Lieferanten zugeordneten Zahlungsbedingung. Ist hier der Wert ‚*Ja*‘ zugeordnet, so wird der Wert des Parameters [Final-Zahlungsbedingungs-Nummer](./rohwareparameter_uebersicht.md#RWPA_64) zur Vorbelegung herangezogen.

  **Final-Zahlungsbedingungs-Nummer**

  Parameternummer: 64

  Einstellungen für Rohwarengruppen und Abrechnungsschemata sind möglich.

  Optionen: Versandartnummer oder 0

  Für die Erfassung und Erzeugung von Rohwarelieferscheine beinhaltet dieser Parameter die Vorbelegung der Zahlungsbedingungsnummer der Stufe ‚*Finalabrechnung*‘, wenn der Wert des Parameters [Feste ZB für Finale](./rohwareparameter_uebersicht.md#RWPA_63) mit dem Wert ‚*Ja‘* eingestellt ist.

  **Abschlagabrechnung**

  Parameternummer: 12

  Einstellungen für Rohwarengruppen und Abrechnungsschemata sind möglich.

  Optionen:

- **kein Abschlag**
- **Abschlag erlaubt**
- **Abschlag muss erfolgen**

  Für die Erfassung und Erzeugung von Rohwarelieferscheine legt dieser Parameter fest, ob Abschlagabrechnungen erstellt werden dürfen oder müssen. Bei der Umwandlung von Rohwarelieferscheinen in eine Abrechnung wird dieser Parameter entsprechend zur Prüfung der Umwandelerlaubnis herangezogen. Ist kein Abschlag erlaubt, so werden bei der Erfassung und Korrektur der Belege abschlagrelevante Daten in der Regel nicht dargestellt.

  **Abschlagermittlung**

  Parameternummer: 13

  Einstellungen für Rohwarengruppen und Abrechnungsschemata sind möglich.

  Optionen:

- **durch Prozentsatz**
- **durch Abschlagpreis**

  Für die Erfassung, Erzeugung und Abrechnung von Rohwarebelegen legt dieser Parameter fest, wie die Berechnung von Abschlagabrechnungen zu erfolgen hat. Ist der Wert ‚*durch %-Satz*‘, so kann im Beleg beziehungsweise im zugehörigen Abrechnungsschema ein Prozentsatz angegeben werden, der den Anteil des eigentlichen Beleggesamtwertes, gegebenenfalls mit Ausnahme von Kostenpositionen, als Abschlagbetrag bestimmt. Bei der Option ‚*durch Abschlagpreis*‘ hingegen wird in den Warenpositionen mit einem eventuell vom eigentlichen Preis abweichenden Abschlagpreis gerechnet.

  **Abschlagsatz**

  Parameternummer: 68

  Einstellungen für Rohwarengruppen und Abrechnungsschemata sind möglich.

  Optionen:

- **keine Anzeige**
- **nur Anzeige**
- **Erfassung**

  Mit diesem Parameter wird für die Erfassung, Anzeige und Korrektur von Rohwarebelegen festgelegt, ob der prozentuale Abschlagsatz auf der Erfassungsmaske unterdrückt, nur dargestellt oder änderbar sein soll. Bei der Erfassung wird der Abschlagsatz immer aus dem Abschlagsatz des zugehörigen Abrechnungsschemas vorbelegt. Ist der Wert des Parameters [Abschlagermittlung](./rohwareparameter_uebersicht.md#RWPA_13) ‚*durch Abschlagpreis*‘, so wird das Maskenfeld für den Abschlagsatz immer unterdrückt.

  **Abschlagstatus**

  Parameternummer: 73

  Einstellungen für Rohwarengruppen und Abrechnungsschemata sind möglich.

  Optionen:

- **keine Anzeige**
- **nur Anzeige**
- **Erfassung**

  Mit diesem Parameter wird für die Erfassung, Anzeige und Korrektur von Rohwarebelegen festgelegt, ob der Abschlagstatus auf der Erfassungsmaske unterdrückt, nur dargestellt oder änderbar ist.

  **Vorbelegung Abschlagstatus**

  Parameternummer: 70

  Einstellungen für Rohwarengruppen und Abrechnungsschemata sind möglich.

  Optionen:

- **ohne**
- **gesperrt**
- **freigegeben**

  Mit diesem Parameter wird für die Erfassung und Erzeugung von Rohwarelieferscheine festgelegt, mit welchem Wert der Abschlagstatus vorbelegt wird. Der Wert ‚*ohne*‘ bewirkt, dass der Lieferschein nicht in einen Abschlagbeleg sondern nur direkt in einen Finalbeleg gewandelt werden kann. Rohwarelieferscheine zu Vorfakturierungskontrakten können nur direkt finalisiert werden. Der Wert ‚*gesperrt*‘ erzwingt als nächste Stufe einen Beleg der Stufe ‚*Abschlag*‘, der aber noch nicht abgerechnet werden kann. Es können hingegen nach der Umwandlung noch fehlende Belegkorrekturen vorgenommen werden. Der Status ‚*freigegeben*‘ ermöglicht nach der Umwandlung in einen Abschlagbeleg die unmittelbare Abrechnung desselben.

  **Folgeabschlagstatus**

  Parameternummer: 74

  Einstellungen für Rohwarengruppen und Abrechnungsschemata sind möglich.

  Optionen:

- **keine Anzeige**
- **nur Anzeige**
- **Erfassung**

  Mit diesem Parameter wird für die Erfassung, Anzeige und Korrektur von Rohwarebelegen festgelegt, ob der Folgeabschlagstatus auf der Erfassungsmaske unterdrückt, nur dargestellt oder änderbar ist.

  **Vorbelegung Folgeabschlagstatus**

  Parameternummer: 71

  Einstellungen für Rohwarengruppen und Abrechnungsschemata sind möglich.

  Optionen:

- **ohne**
- **gesperrt**
- **freigegeben**

  Mit diesem Parameter wird für die Erfassung und Erzeugung von Rohwarelieferscheine festgelegt, mit welchem Wert der Folgeabschlagstatus vorbelegt wird. Der Wert ‚*ohne*‘ bewirkt, dass kein Folgeabschlagbeleg erzeugt werden kann. Folgeabschlagbelege können nur zu Abschlagbelegen erzeugt werden. Der Wert ‚*gesperrt*‘ erzwingt die Erzeugung eines Beleges der Stufe ‚*Folgeabschlag*‘, der aber noch nicht abgerechnet werden kann. Es können hingegen nach der Umwandlung noch fehlende Belegkorrekturen vorgenommen werden. Der Status ‚*freigegeben*‘ ermöglicht nach der Umwandlung in einen Folgeabschlagbeleg die unmittelbare Abrechnung desselben.

  **Finalstatus**

  Parameternummer: 75

  Einstellungen für Rohwarengruppen und Abrechnungsschemata sind möglich.

  Optionen:

- **keine Anzeige**
- **nur Anzeige**
- **Erfassung**

  Mit diesem Parameter wird für die Erfassung, Anzeige und Korrektur von Rohwarebelegen festgelegt, ob der Finalstatus auf der Erfassungsmaske unterdrückt, nur dargestellt oder änderbar ist.

  **Vorbelegung Finalstatus**

  Parameternummer: 72

  Einstellungen für Rohwarengruppen und Abrechnungsschemata sind möglich.

  Optionen:

- **gesperrt**
- **freigegeben**

  Mit diesem Parameter wird für die Erfassung und Erzeugung von Rohwarelieferscheine festgelegt, mit welchem Wert der Finalstatus vorbelegt wird. Der Wert ‚*gesperrt*‘ bewirkt, dass der Finalbeleg nach der Erzeugung noch nicht abgerechnet werden kann. Es können hingegen nach der Umwandlung noch fehlende Belegkorrekturen vorgenommen werden. Der Status ‚*freigegeben*‘ ermöglicht nach der Umwandlung in einen Finalbeleg die unmittelbare Abrechnung desselben.

  **Finale nach Jahreswechsel geplant**

  Parameternummer: 158

  Einstellungen für Rohwarengruppen und Abrechnungsschemata sind möglich.

  Optionen:

- **keine Anzeige**
- **nur Anzeige**
- **Erfassung**

  Mit diesem Parameter wird für die Erfassung, Anzeige und Korrektur von Rohwarebelegen festgelegt, ob das Kennzeichen ‚*Finale nach Jahreswechsel* ‘auf der Erfassungsmaske unterdrückt, nur dargestellt oder änderbar ist.

  **Vertretergruppe**

  Parameternummer: 31

  Einstellungen für Rohwarengruppen und Abrechnungsschemata sind möglich.

  Optionen:

- **keine Anzeige**
- **nur Anzeige**
- **Erfassung**

  Mit diesem Parameter wird für die Erfassung, Anzeige und Korrektur von Rohwarebelegen festgelegt, ob die Vertretergruppe auf der Erfassungsmaske unterdrückt, nur dargestellt oder änderbar ist.

  **Vorbelegung Vertretergruppe**

  Parameternummer: 32

  Einstellungen für Rohwarengruppen und Abrechnungsschemata sind möglich.

  Optionen:

- **aus Kundenstamm**
- **fester Wert**

  Mit diesem Parameter wird für die Erfassung und Erzeugung von Rohwarelieferscheine von Rohwarebelegen festgelegt, mit welchem Wert die Vertretergruppe vorbelegt wird. Dieser kann entweder aus dem Kundenstamm beziehungsweise Lieferantenstamm versorgt werden oder mit dem Fix-Wert des Parameters [Vertretergruppenfestwert](./rohwareparameter_uebersicht.md#RWPA_45) vorbelegt werden. Bei der Erfassung oder Erzeugung von Rohwarelieferscheinen mit Kontraktzuordnung, kann die Vertretergruppe entsprechend der Einstellung des Parameters [Vertretergruppe aus Kontrakt](./rohwareparameter_uebersicht.md#RWPA_87) auch aus der Zuordnung im Kontrakt übernommen werden.

  **Vertretergruppenfestwert**

  Parameternummer: 45

  Einstellungen für Rohwarengruppen und Abrechnungsschemata sind möglich.

  Optionen: Vertretergruppennummer oder 0

  Für die Erfassung und Erzeugung von Rohwarelieferscheine beinhaltet dieser Parameter die Nummer der Vertretergruppenvorbelegung, wenn der Parameter [Vorbelegung Vertretergruppe](./rohwareparameter_uebersicht.md#RWPA_32) mit ‚*fester Wert*‘ eingestellt ist.

  **Verkaufsgebiet**

  Parameternummer: 38

  Einstellungen für Rohwarengruppen und Abrechnungsschemata sind möglich.

  Optionen:

- **keine Anzeige**
- **nur Anzeige**
- **Erfassung**

  Mit diesem Parameter wird für die Erfassung, Anzeige und Korrektur von Rohwarebelegen festgelegt, ob das Verkaufsgebiet auf der Erfassungsmaske unterdrückt, nur dargestellt oder änderbar ist.

  **Fakturiergruppe**

  Parameternummer: 42

  Einstellungen für Rohwarengruppen und Abrechnungsschemata sind möglich.

  Optionen:

- **keine Anzeige**
- **nur Anzeige**
- **Erfassung**

  Mit diesem Parameter wird für die Erfassung, Anzeige und Korrektur von Rohwarebelegen festgelegt, ob die Fakturiergruppe auf der Erfassungsmaske unterdrückt, nur dargestellt oder änderbar ist.

  **Vorbelegung Fakturiergruppe**

  Parameternummer: 43

  Einstellungen für Rohwarengruppen und Abrechnungsschemata sind möglich.

  Optionen:

- **aus Kundenstamm**
- **fester Wert**

  Für die Erfassung und Erzeugung von Rohwarelieferscheine steuert dieser Parameter die Vorbelegung der Fakturiergruppe. Diese kann aus dem Stammsatz des Lieferanten/Kunden entnommen werden. Die Einstellung ‚*fester Wert*‘ hingegen legt die Fakturiergruppe auf den dem Parameter [Fakturiergruppenfestwert](./rohwareparameter_uebersicht.md#RWPA_44) zugewiesenen Wert fest.

  **Fakturiergruppenfestwert**

  Parameternummer: 44

  Einstellungen für Rohwarengruppen und Abrechnungsschemata sind möglich.

  Optionen: Steuergruppennummer

  Für die Erfassung und Erzeugung von Rohwarelieferscheine beinhaltet dieser Parameter die Vorbelegung der Fakturiergruppe, wenn der zugehörige Parameter [Vorbelegung Fakturiergruppe](./rohwareparameter_uebersicht.md#RWPA_43) mit der Einstellung ‚*fester Wert*‘ versehen ist.

  **LKW-Nummer Motorwagen**

  Parameternummer: 33

  Einstellungen für Rohwarengruppen und Abrechnungsschemata sind möglich.

  Optionen:

- **keine Anzeige**
- **nur Anzeige**
- **Erfassung**

  Mit diesem Parameter wird für die Erfassung, Anzeige und Korrektur von Rohwarebelegen festgelegt, ob die für den Motorwagen vorgesehene LKW-Nummer auf der Erfassungsmaske unterdrückt, nur dargestellt oder änderbar ist.

  **LKW-Nummer Anhänger**

  Parameternummer: 34

  Einstellungen für Rohwarengruppen und Abrechnungsschemata sind möglich.

  Optionen:

- **keine Anzeige**
- **nur Anzeige**
- **Erfassung**

  Mit diesem Parameter wird für die Erfassung, Anzeige und Korrektur von Rohwarebelegen festgelegt, ob die für den Anhänger vorgesehene LKW-Nummer auf der Erfassungsmaske unterdrückt, nur dargestellt oder änderbar ist.

  **Fahrernummer**

  Parameternummer: 35

  Einstellungen für Rohwarengruppen und Abrechnungsschemata sind möglich.

  Optionen:

- **keine Anzeige**
- **nur Anzeige**
- **Erfassung**

  Mit diesem Parameter wird für die Erfassung, Anzeige und Korrektur von Rohwarebelegen festgelegt, ob die Fahrernummer auf der Erfassungsmaske unterdrückt, nur dargestellt oder änderbar ist.

  **Startgebietsnummer**

  Parameternummer: 36

  Einstellungen für Rohwarengruppen und Abrechnungsschemata sind möglich.

  Optionen:

- **keine Anzeige**
- **nur Anzeige**
- **Erfassung**

  Mit diesem Parameter wird für die Erfassung, Anzeige und Korrektur von Rohwarebelegen festgelegt, ob die Nummer des Startgebiets auf der Erfassungsmaske unterdrückt, nur dargestellt oder änderbar ist.

  **Zielgebietsnummer**

  Parameternummer: 37

  Einstellungen für Rohwarengruppen und Abrechnungsschemata sind möglich.

  Optionen:

- **keine Anzeige**
- **nur Anzeige**
- **Erfassung**

  Mit diesem Parameter wird für die Erfassung, Anzeige und Korrektur von Rohwarebelegen festgelegt, ob die Nummer des Zielgebiets auf der Erfassungsmaske unterdrückt, nur dargestellt oder änderbar ist.

  **Druckformular Lieferschein**

  Parameternummer: 46

  Einstellungen für Rohwarengruppen und Abrechnungsschemata sind möglich.

  Optionen:

- **keine Anzeige**
- **nur Anzeige**
- **Erfassung**

  Mit diesem Parameter wird für die Erfassung, Anzeige und Korrektur von Rohwarebelegen festgelegt, ob die Nummer des Druckformulars für die Stufe ‚Lieferschein‘ auf der Erfassungsmaske unterdrückt, nur dargestellt oder änderbar ist.

  **Vorbelegung Lieferscheinformular**

  Parameternummer: 47

  Einstellungen für Rohwarengruppen und Abrechnungsschemata sind möglich.

  Optionen: Formularnummer oder 0

  Für die Erfassung und Erzeugung von Rohwarelieferscheine beinhaltet dieser Parameter die Vorbelegung der Formularnummer für den Lieferscheindruck. Ist der Parameterwert ‚0‘, so wird zur Vorbelegung die in der Formularzuordnung der Vorgangsunterklasse angegebene Formularnummer verwendet.

  **Druckformular Abschlag**

  Parameternummer: 48

  Einstellungen für Rohwarengruppen und Abrechnungsschemata sind möglich.

  Optionen:

- **keine Anzeige**
- **nur Anzeige**
- **Erfassung**

  Mit diesem Parameter wird für die Erfassung, Anzeige und Korrektur von Rohwarebelegen festgelegt, ob die Nummer des Einzelbeleg-Druckformulars für die Stufe ‚*Abschlag*‘ auf der Erfassungsmaske unterdrückt, nur dargestellt oder änderbar ist.

  **Vorbelegung Abschlagformular**

  Parameternummer: 49

  Einstellungen für Rohwarengruppen und Abrechnungsschemata sind möglich.

  Optionen: Formularnummer oder 0

  Für die Erfassung und Erzeugung von Rohwarelieferscheine beinhaltet dieser Parameter die Vorbelegung der Formularnummer für den Einzelbelegdruck der Stufe ‚*Abschlag*‘. Ist der Parameterwert ‚0‘, so wird zur Vorbelegung die in der Formularzuordnung der Vorgangsunterklasse angegebene Formularnummer verwendet.

  **Sammeldruckformular Abschlag**

  Parameternummer: 96

  Einstellungen für Rohwarengruppen und Abrechnungsschemata sind möglich.

  Optionen:

- **keine Anzeige**
- **nur Anzeige**
- **Erfassung**

  Mit diesem Parameter wird für die Erfassung, Anzeige und Korrektur von Rohwarebelegen festgelegt, ob die Nummer Sammeldruckformulars für die Stufe ‚*Abschlag*‘ auf der Erfassungsmaske unterdrückt, nur dargestellt oder änderbar ist.

  **Vorbelegung Abschlag-Sammeldruckformular**

  Parameternummer: 97

  Einstellungen für Rohwarengruppen und Abrechnungsschemata sind möglich.

  Optionen: Sammeldruck-Formularnummer oder 0

  Für die Erfassung und Erzeugung von Rohwarelieferscheine beinhaltet dieser Parameter die Vorbelegung der Formularnummer für den Sammelbelegdruck der Stufe ‚*Abschlag*‘. Generell bedeutet die Sammeldruckformularnummer ‚0‘, dass der Beleg nicht in einen Sammeldruckbeleg integriert werden kann.

  **Druckformular Folgeabschlag**

  Parameternummer: 50

  Einstellungen für Rohwarengruppen und Abrechnungsschemata sind möglich.

  Optionen:

- **keine Anzeige**
- **nur Anzeige**
- **Erfassung**

  Mit diesem Parameter wird für die Erfassung, Anzeige und Korrektur von Rohwarebelegen festgelegt, ob die Nummer des Einzelbeleg-Druckformulars für die Stufe ‚*Folgeabschlag*‘ auf der Erfassungsmaske unterdrückt, nur dargestellt oder änderbar ist.

  **Vorbelegung Folgeabschlagformular**

  Parameternummer: 51

  Einstellungen für Rohwarengruppen und Abrechnungsschemata sind möglich.

  Optionen: Formularnummer oder 0

  Für die Erfassung und Erzeugung von Rohwarelieferscheine beinhaltet dieser Parameter die Vorbelegung der Formularnummer für den Einzelbelegdruck der Stufe ‚*Folgeabschlag*‘. Ist der Parameterwert ‚0‘, so wird zur Vorbelegung die in der Formularzuordnung der Vorgangsunterklasse angegebene Formularnummer verwendet.

  **Sammeldruckformular Folgeabschlag**

  Parameternummer: 98

  Einstellungen für Rohwarengruppen und Abrechnungsschemata sind möglich.

  Optionen:

- **keine Anzeige**
- **nur Anzeige**
- **Erfassung**

  Mit diesem Parameter wird für die Erfassung, Anzeige und Korrektur von Rohwarebelegen festgelegt, ob die Nummer Sammeldruckformulars für die Stufe ‚*Folgeabschlag*‘ auf der Erfassungsmaske unterdrückt, nur dargestellt oder änderbar ist.

  **Vorbelegung Folgeabschlag-Sammeldruckformular**

  Parameternummer: 99

  Einstellungen für Rohwarengruppen und Abrechnungsschemata sind möglich.

  Optionen: Sammeldruck-Formularnummer oder 0

  Für die Erfassung und Erzeugung von Rohwarelieferscheine beinhaltet dieser Parameter die Vorbelegung der Formularnummer für den Sammelbelegdruck der Stufe ‚*Folgeabschlag*‘. Generell bedeutet die Sammeldruckformularnummer ‚0‘, dass der Beleg nicht in einen Sammeldruckbeleg integriert werden kann.

  **Druckformular Finale**

  Parameternummer: 52

  Einstellungen für Rohwarengruppen und Abrechnungsschemata sind möglich.

  Optionen:

- **keine Anzeige**
- **nur Anzeige**
- **Erfassung**

  Mit diesem Parameter wird für die Erfassung, Anzeige und Korrektur von Rohwarebelegen festgelegt, ob die Nummer des Einzelbeleg-Druckformulars für die Stufe ‚*Finalabrechnung*‘ auf der Erfassungsmaske unterdrückt, nur dargestellt oder änderbar ist.

  **Vorbelegung Finalformular**

  Parameternummer: 53

  Einstellungen für Rohwarengruppen und Abrechnungsschemata sind möglich.

  Optionen: Formularnummer oder 0

  Für die Erfassung und Erzeugung von Rohwarelieferscheine beinhaltet dieser Parameter die Vorbelegung der Formularnummer für den Einzelbelegdruck der Stufe ‚*Finalabrechnung*‘. Ist der Parameterwert ‚0‘, so wird zur Vorbelegung die in der Formularzuordnung der Vorgangsunterklasse angegebene Formularnummer verwendet.

  **Sammeldruckformular Finale**

  Parameternummer: 100

  Einstellungen für Rohwarengruppen und Abrechnungsschemata sind möglich.

  Optionen:

- **keine Anzeige**
- **nur Anzeige**
- **Erfassung**

  Mit diesem Parameter wird für die Erfassung, Anzeige und Korrektur von Rohwarebelegen festgelegt, ob die Nummer Sammeldruckformulars für die Stufe ‚*Finalabrechnung*‘ auf der Erfassungsmaske unterdrückt, nur dargestellt oder änderbar ist.

  **Vorbelegung Final-Sammeldruckformular**

  Parameternummer: 101

  Einstellungen für Rohwarengruppen und Abrechnungsschemata sind möglich.

  Optionen: Sammeldruck-Formularnummer oder 0

  Für die Erfassung und Erzeugung von Rohwarelieferscheine beinhaltet dieser Parameter die Vorbelegung der Formularnummer für den Sammelbelegdruck der Stufe ‚*Finalabrechnung*‘. Generell bedeutet die Sammeldruckformularnummer ‚0‘, dass der Beleg nicht in einen Sammeldruckbeleg integriert werden kann.

  **Abschlag-Vorschauformular**

  Parameternummer: 109

  Einstellungen für Rohwarengruppen und Abrechnungsschemata sind möglich.

  Optionen: Formularnummer oder 0

  Für die Vorschau-Funktion von abgerechneten Rohwarebelegen der Stufe ‚*Abschlag*‘ kann mit diesem Parameter ein Druckformular festgelegt werden. Ist der Parameterwert ‚0‘, so wird zur Vorschau die in der Formularzuordnung der Vorgangsunterklasse angegebene Vorschau-Formularnummer verwendet.

  **Folgeabschlag-Vorschauformular**

  Parameternummer: 110

  Einstellungen für Rohwarengruppen und Abrechnungsschemata sind möglich.

  Optionen: Formularnummer oder 0

  Für die Vorschau-Funktion von abgerechneten Rohwarebelegen der Stufe ‚*Folgeabschlag*‘ kann mit diesem Parameter ein Druckformular festgelegt werden. Ist der Parameterwert ‚0‘, so wird zur Vorschau die in der Formularzuordnung der Vorgangsunterklasse angegebene Vorschau-Formularnummer verwendet.

  **Final-Vorschauformular**

  Parameternummer: 111

  Einstellungen für Rohwarengruppen und Abrechnungsschemata sind möglich.

  Optionen: Formularnummer oder 0

  Für die Vorschau-Funktion von abgerechneten Rohwarebelegen der Stufe ‚*Finalabrechnung*‘ kann mit diesem Parameter ein Druckformular festgelegt werden. Ist der Parameterwert ‚0‘, so wird zur Vorschau die in der Formularzuordnung der Vorgangsunterklasse angegebene Vorschau-Formularnummer verwendet.

  **Nummer des Feuchte-Qualitätsmerkmals**

  Parameternummer: 81

  Einstellungen für Rohwarengruppen und Abrechnungsschemata sind möglich.

  Optionen: Referenznummer

  Zur korrekten Darstellung der Feuchtigkeitswerte in der Auswahllistenvariante ‚*Erfassungsprotokoll*‘ des Rohwarebearbeitungs-Moduls <strong>[RWB] [RWBV]</strong> muss in diesem Parameter die Referenznummer des auszuwertenden Qualitätsmerkmals (in der Regel ‚*Feuchte*‘) angegeben sein. Die relevanten statistischen Daten werden bereits bei Erfassung, Korrektur und Abrechnung von Rohwarebelegen ermittelt und in dafür reservierten Datenfeldern festgehalten.

  **Erfassung mit Kontrakt**

  Parameternummer: 86

  Einstellungen für Rohwarengruppen und Abrechnungsschemata sind möglich.

  Optionen:

- **nein**
- **ja**

  Dieser Parameter legt für die Erfassung, Erzeugung und Bearbeitung von Rohwarebelegen fest, ob die Kontraktverwaltung aktiviert ist. Insbesondere für Rohwarengruppen oder Abrechnungsschemata, für die grundsätzlich keine Kontrakte zu berücksichtigen sind, kann die Einstellung ‚*Nein*‘ aus programmlaufzeittechnischen Gründen sinnvoll sein.

  **Restmenge überprüfen**

  Parameternummer: 95

  Einstellungen für Rohwarengruppen und Abrechnungsschemata sind möglich.

  Optionen:

- **nein**
- **ja**

  Dieser Parameter legt für die Erfassung von Rohwarelieferscheinen mit Kontraktzuordnung der Lieferposition fest, ob bei Abschluss des Beleges eine Prüfung der Liefermenge mit der Restmenge des Kontrakts erfolgen soll. Ist dieses der Fall, so wird entsprechend der Einstellung des Kontrakts die erfasste Bruttomenge beziehungsweise die errechnete Nettomenge zur Prüfung herangezogen. Ist die Restmenge der Kontraktposition insgesamt oder die Zeitraum-Restmenge nicht ausreichend, so erfolgt ein entsprechender Hinweis und die Abfrage, ob der Beleg dennoch gespeichert werden soll oder zu korrigieren ist.

  **Beleg aus Waage bei Überziehung möglich**

  Parameternummer: 155

  Einstellungen für Rohwarengruppen und Abrechnungsschemata sind möglich.

  Optionen:

- **nein**
- **ja**

  Dieser Parameter legt für die Erzeugung von Rohwarelieferscheinen mit Kontraktzuordnung aus der Waagenschnittstelle **[RWWE] [RWWV]** fest, ob bei Überziehung des Kontrakts bezüglich der Restmenge der Rohwarebeleg dennoch zu erzeugen ist. Es wird entsprechend der Einstellung des Kontrakts die erfasste Bruttomenge beziehungsweise die errechnete Nettomenge zur Prüfung herangezogen. Ist die Restmenge der Kontraktposition insgesamt oder die Zeitraum-Restmenge nicht ausreichend, so erfolgt ein entsprechender Hinweis. Je nach Einstellung dieses Parameters wird der Beleg dann dennoch gespeichert oder verworfen.

  **Erfassungsbeleg teilen bei Übermenge**

  Parameternummer: 178

  Einstellungen für Rohwarengruppen und Abrechnungsschemata sind möglich.

  Optionen:

- **ja**
- **nein**

  Mit der Einstellung dieses Parameters kann bei Belegabschluss einer Rohwareerfassung mit Überschreitung der noch offenen Restmenge des zur Hauptposition angegebenen Kontrakts die [Bearbeitungsmaske zur Belegteilung](../rohwarenbearbeitung/rohware_erfassung_mit_belegsplitting_bei_kontraktmengenueber/index.md) aktiviert werden.

  **Vertretergruppe aus Kontrakt**

  Parameternummer: 87

  Einstellungen für Rohwarengruppen und Abrechnungsschemata sind möglich.

  Optionen:

- **wenn ungleich 0 im Kontrakt**
- **immer**
- **nein**

  Dieser Parameter legt für die Erfassung und Erzeugung von Rohwarelieferscheinen mit Kontraktzuordnung fest, ob die Vertretergruppe des Kontrakts zu übernehmen ist. Der Parameterwert ‚*wenn ungleich 0 im Kontrakt*‘ bewirkt, dass die Vertretergruppe des Rohwarebelegs unverändert bleibt, wenn im Kontrakt die Vertretergruppe ‚*0*‘ eingetragen ist. In diesem Fall, wie auch im Fall der Einstellung mit dem Wert ‚*nein*‘, bleibt der durch die Parameter [Vorbelegung Vertretergruppe](./rohwareparameter_uebersicht.md#RWPA_32) und [Vertretergruppenfestwert](./rohwareparameter_uebersicht.md#RWPA_45) vorbelegte und gegebenenfalls manuell geänderte Wert der Vertretergruppe im Rohwarebeleg unverändert.

  **ZB-Abschlag aus Kontrakt**

  Parameternummer: 88

  Einstellungen für Rohwarengruppen und Abrechnungsschemata sind möglich.

  Optionen:

- **wenn ungleich 0 im Kontrakt**
- **immer**
- **nein**

  Dieser Parameter legt für die Erfassung und Erzeugung von Rohwarelieferscheinen mit Kontraktzuordnung fest, ob die Zahlungsbedingung des Kontrakts als Zahlungsbedingung für den Rohwarebeleg der Stufe ‚*Abschlag*‘ zu übernehmen ist. Der Parameterwert ‚*wenn ungleich 0 im Kontrakt*‘ bewirkt, dass die Abschlag-Zahlungsbedingung des Rohwarebelegs unverändert bleibt, wenn im Kontrakt die Zahlungsbedingung ‚*0*‘ eingetragen ist. In diesem Fall, wie auch im Fall der Einstellung mit dem Wert ‚*nein*‘, bleibt der durch die Parameter [Feste ZB für Abschlag](./rohwareparameter_uebersicht.md#RWPA_57) und [Abschlag-Zahlungsbedingungs-Nummer](./rohwareparameter_uebersicht.md#RWPA_58) vorbelegte und gegebenenfalls manuell geänderte Wert im Rohwarebeleg unverändert.

  **ZB-Folgeabschlag aus Kontrakt**

  Parameternummer: 89

  Einstellungen für Rohwarengruppen und Abrechnungsschemata sind möglich.

  Optionen:

- **wenn ungleich 0 im Kontrakt**
- **immer**
- **nein**

  Dieser Parameter legt für die Erfassung und Erzeugung von Rohwarelieferscheinen mit Kontraktzuordnung fest, ob die Zahlungsbedingung des Kontrakts als Zahlungsbedingung für den Rohwarebeleg der Stufe ‚*Folgeabschlag*‘ zu übernehmen ist. Der Parameterwert ‚*wenn ungleich 0 im Kontrakt*‘ bewirkt, dass die Folgeabschlag-Zahlungsbedingung des Rohwarebelegs unverändert bleibt, wenn im Kontrakt die Zahlungsbedingung ‚*0*‘ eingetragen ist. In diesem Fall, wie auch im Fall der Einstellung mit dem Wert ‚*nein*‘, bleibt der durch die Parameter [Feste ZB für Folgeabschlag](./rohwareparameter_uebersicht.md#RWPA_60) und [Folgeabschlag-Zahlungsbedingungs-Nummer](./rohwareparameter_uebersicht.md#RWPA_61) vorbelegte und gegebenenfalls manuell geänderte Wert im Rohwarebeleg unverändert.

  **ZB-Finale aus Kontrakt**

  Parameternummer: 90

  Einstellungen für Rohwarengruppen und Abrechnungsschemata sind möglich.

  Optionen:

- **wenn ungleich 0 im Kontrakt**
- **immer**
- **nein**

  Dieser Parameter legt für die Erfassung und Erzeugung von Rohwarelieferscheinen mit Kontraktzuordnung fest, ob die Zahlungsbedingung des Kontrakts als Zahlungsbedingung für den Rohwarebeleg der Stufe ‚*Finalabrechnung*‘ zu übernehmen ist. Der Parameterwert ‚*wenn ungleich 0 im Kontrakt*‘ bewirkt, dass die Final-Zahlungsbedingung des Rohwarebelegs unverändert bleibt, wenn im Kontrakt die Zahlungsbedingung ‚*0*‘ eingetragen ist. In diesem Fall, wie auch im Fall der Einstellung mit dem Wert ‚*nein*‘, bleibt der durch die Parameter [Feste ZB für Finale](./rohwareparameter_uebersicht.md#RWPA_63) und [Final-Zahlungsbedingungs-Nummer](./rohwareparameter_uebersicht.md#RWPA_64) vorbelegte und gegebenenfalls manuell geänderte Wert im Rohwarebeleg unverändert.

  **Kontraktzu-/abschlag für Abschlagpreis**

  Parameternummer: 124

  Einstellungen für Rohwarengruppen und Abrechnungsschemata sind möglich.

  Optionen:

- **ohne Report, ohne Überziehungszuschlag**
- **mit Report, ohne Überziehungszuschlag**
- **ohne Report, mit Überziehungszuschlag**
- **mit Report, mit Überziehungszuschlag**

  Dieser Parameter legt für Rohwarebelege der Stufe ‚*Abschlag*‘ mit Kontraktzuordnung fest, ob im Kontrakt festgelegt Reports und /oder Überziehungszuschläge zu berücksichtigen sind. Als Bezugsdatum zur Ermittlung dieser Zuschläge dient immer das Lieferdatum des Rohwarebelegs. Zu berücksichtigende Zuschläge werden bei der Preisfindung immer dem entsprechenden Kontraktpreis zugeschlagen.

  **Kontraktzu-/abschlag für Folgeabschlagpreis**

  Parameternummer: 125

  Einstellungen für Rohwarengruppen und Abrechnungsschemata sind möglich.

  Optionen:

- **ohne Report, ohne Überziehungszuschlag**
- **mit Report, ohne Überziehungszuschlag**
- **ohne Report, mit Überziehungszuschlag**
- **mit Report, mit Überziehungszuschlag**

  Dieser Parameter legt für Rohwarebelege der Stufe ‚*Folgeabschlag*‘ mit Kontraktzuordnung fest, ob im Kontrakt festgelegt Reports und /oder Überziehungszuschläge zu berücksichtigen sind. Als Bezugsdatum zur Ermittlung dieser Zuschläge dient immer das Lieferdatum des Rohwarebelegs. Zu berücksichtigende Zuschläge werden bei der Preisfindung immer dem entsprechenden Kontraktpreis zugeschlagen.

  **Kontraktzu-/abschlag für Finalpreis**

  Parameternummer: 123

  Einstellungen für Rohwarengruppen und Abrechnungsschemata sind möglich.

  Optionen:

- **ohne Report, ohne Überziehungszuschlag**
- **mit Report, ohne Überziehungszuschlag**
- **ohne Report, mit Überziehungszuschlag**
- **mit Report, mit Überziehungszuschlag**

  Dieser Parameter legt für Rohwarebelege der Stufe ‚*Finalabrechnung*‘ mit Kontraktzuordnung fest, ob im Kontrakt festgelegt Reports und /oder Überziehungszuschläge zu berücksichtigen sind. Als Bezugsdatum zur Ermittlung dieser Zuschläge dient immer das Lieferdatum des Rohwarebelegs. Zu berücksichtigende Zuschläge werden bei der Preisfindung immer dem entsprechenden Kontraktpreis zugeschlagen.

  **Rechnungsempfänger=Hauptkunde**

  Parameternummer: 133

  Einstellungen für Rohwarengruppen und Abrechnungsschemata sind möglich.

  Optionen:

- **nicht erlaubt**
- **erlaubt**

  Dieser Parameter legt für Rohwarebelege mit Kontraktzuordnung fest, ob entsprechend der Einstellungen in der betroffenen Kontraktgruppe ‚*Rechnung an HK (nur Rohware)*‘ mit den Alternativen ‚*immer*‘ und ‚*siehe Kontrakt*‘ und somit im Kontrakt ‚*RW-Rechnung an HK (Ja/Nein)*‘ der Kontraktgruppen-Hauptkunde als Rechnungskunde der Rohwarenlabrechnungen zu gelten hat. 

  **Zahlungskunde=Hauptkunde**

  Parameternummer: 134

  Einstellungen für Rohwarengruppen und Abrechnungsschemata sind möglich.

  Optionen:

- **nicht erlaubt**
- **erlaubt**

  Dieser Parameter legt für Rohwarebelege mit Kontraktzuordnung fest, ob entsprechend der Einstellungen in der betroffenen Kontraktgruppe ‚Zahlungspflichtig *HK (nur Rohware)*‘ mit den Alternativen ‚*immer*‘ und ‚*siehe Kontrakt*‘ und somit im Kontrakt ‚*RW-Zahlungspflichtig = HK (Ja/Nein)*‘ der Kontraktgruppen-Hauptkunde als Zahlungspflichtiger der Rohwarenlabrechnungen zu gelten hat.

  **Rohwareabrechnung mit Partie**

  Parameternummer: 93

  Einstellungen für Rohwarengruppen und Abrechnungsschemata sind möglich.

  Optionen:

- **nein**
- **ja**

  Dieser Parameter legt für die Erfassung, Erzeugung und Bearbeitung von Rohwarebelegen fest, ob die Partieverwaltung aktiviert ist. Insbesondere für Rohwarengruppen oder Abrechnungsschemata, für die grundsätzlich keine Partien zu berücksichtigen sind, kann die Einstellung ‚*Nein*‘ aus programmlaufzeittechnischen Gründen sinnvoll sein.

  **Partieauswahl bei Neuerfassung**

  Parameternummer: 94

  Einstellungen für Rohwarengruppen und Abrechnungsschemata sind möglich.

  Optionen:

- **Auswahlmaske per OB-Klick**
- **immer neue Partie erzeugen**
- **immer Auswahlmaske**
- **Auswahlmaske ab 2 Partien**

  Dieser Parameter legt für die Erfassung von Rohwarelieferungen fest, wie die Partiezuordnung der Lieferposition zu erfolgen hat. Neben der Option, für jede Lieferung immer zunächst eine neue Partie zu anzulegen, deren Erzeugung aber bei der Partiebearbeitung nach der Mengeneingabe abgebrochen werden kann, wird festgelegt, ob bei bereits vorhandenen Partien die Partieauswahl nie, immer oder erst bei mehr als einer vorhandenen Partie automatisch nach der Mengeneingabe erscheint. 

  **nächste Stufe nur nach Fibuübertrag**

  Parameternummer: 113

  Einstellungen für Rohwarengruppen und Abrechnungsschemata sind möglich.

  Optionen:

- **nein**
- **ja**

  Dieser Parameter legt für die Umwandlung von Rohwarebelegen in die nächste Abrechnungsstufe fest, ob dieses erst nach erfolgtem Übertrag an die Finanzbuchhaltung gestattet ist.

  **Stornobeleg nur nach Fibuübertrag**

  Parameternummer: 114

  Einstellungen für Rohwarengruppen und Abrechnungsschemata sind möglich.

  Optionen:

- **nein**
- **ja**

  Dieser Parameter legt für die Erzeugung von Einzel-Stornobelegen zu Rohwarebelegen fest, ob dieses erst nach erfolgtem Übertrag an die Finanzbuchhaltung gestattet ist. Handelt es sich jedoch um die Erzeugung eines Stornobelegs mit Kopie des Originalbelegs in ein anderes Wirtschaftsjahr, so setzt die Einstellung ‚*Ja, ohne Fibu-Übertrag*‘ im Feld ‚*Finale nach Jahreswechsel*‘ im Originalbeleg diese Sperre außer Kraft.  
Dieser Parameter betrifft lediglich die Behandlung von Rohware-Einzelbelegen. Für Sammeldruck-Storno-Belege regelt dieses der Parameter [Sammel-Storno nur nach Fibuübetrag](./rohwareparameter_uebersicht.md#RWPA_145).

  **Währung in Nachvergütungen**

  Parameternummer: 126

  Einstellungen für Rohwarengruppen und Abrechnungsschemata sind möglich.

  Optionen:

- **wie Finale**
- **aus Kundenstamm**

  Dieser Parameter legt für die Erzeugung von Nachvergütungsbelegen zu Rohware-Finalabrechnungen fest, ob diese in der Währung der Finalabrechnung erstellt werden oder immer in der Währung, die dem Kunden zugeordnet ist.

  **Qualitätsnachtrag per**

  Parameternummer: 127

  Einstellungen für Rohwarengruppen und Abrechnungsschemata sind möglich.

  Optionen:

- **per Referenznummer**
- **per Waagen-Qualitätsnummer**

  Dieser Parameter legt für die automatische Qualitätsnachtrag-Funktion bei der Umwandlung und Abrechnung von Rohwarebelegen sowie den Pflege-Varianten ‚*Werte für Qualitätsnachtrag*‘ und ‚*Qualitätsnachtragliste*‘ des Rohwarebearbeitungsmoduls im Einkauf **[RWB]** fest, wie die Zuordnung der Qualitätsnachtragssätze zu den Abrechnungsschemapositionen zu interpretieren ist. Dieses entweder über die Referenznummer oder die Waagen-Qualitätsnummer der Qualitätsdefinition im zugehörigen Abrechnungsschema sein.

  **Folgetextzeilen bei Lieferartikel**

  Parameternummer: 118

  Einstellungen für Rohwarengruppen und Abrechnungsschemata sind möglich.

  Optionen:

- **nein**
- **ja**

  Dieser Parameter legt für den Druck von Rohwarebelegen fest, ob für den Artikel der Lieferposition vorhandene Folgetextzeilen mit auszugeben sind.

  **Folgetextzeilen bei Sekundärartikel**

  Parameternummer: 119

  Einstellungen für Rohwarengruppen und Abrechnungsschemata sind möglich.

  Optionen:

- **nein**
- **ja**

  Dieser Parameter legt für den Druck von Rohwarebelegen fest, ob für Artikel von Sekundär-Warenpositionen vorhandene Folgetextzeilen mit auszugeben sind.

  **Folgetextzeilen bei Kostenartikel**

  Parameternummer: 120

  Einstellungen für Rohwarengruppen und Abrechnungsschemata sind möglich.

  Optionen:

- **nein**
- **ja**

  Dieser Parameter legt für den Druck von Rohwarebelegen fest, ob für Artikel von Kosten- und Vergütungspositionen vorhandene Folgetextzeilen mit auszugeben sind.

  **Ziel für Ware-Zusatzinfo-Text**

  Parameternummer: 129

  Einstellungen für Rohwarengruppen und Abrechnungsschemata sind möglich.

  Optionen:

- **keine Übernahme**
- **Ergänzungstext 1**
- **Ergänzungstext 2**
- **Ergänzungstext 3**
- **Ergänzungstext 4**
- **Ergänzungstext 5**
- **Ergänzungstext 6**

  Dieser Parameter legt für die Wandlung von Ware-Lieferscheinen in Rohwarelieferscheine fest, ob ein im Feld ‚*Ware-Zusatzinfo*‘ erfasster Text in eines der Rohware-Ergänzungs-Textfelder zu übernehmen ist.

  **Ziel für Ware-Zusatzinfo2-Text**

  Parameternummer: 130

  Einstellungen für Rohwarengruppen und Abrechnungsschemata sind möglich.

  Optionen:

- **keine Übernahme**
- **Ergänzungstext 1**
- **Ergänzungstext 2**
- **Ergänzungstext 3**
- **Ergänzungstext 4**
- **Ergänzungstext 5**
- **Ergänzungstext 6**

  Dieser Parameter legt für die Wandlung von Ware-Lieferscheinen in Rohwarelieferscheine fest, ob ein im Feld ‚*Ware-Zusatzinfo 2*‘ erfasster Text in eines der Rohware-Ergänzungs-Textfelder zu übernehmen ist.

  **Abschlagpreis aus Lieferschein wenn != 0**

  Parameternummer: 149

  Einstellungen für Rohwarengruppen und Abrechnungsschemata sind möglich.

  Optionen:

- **nein**
- **ja**

  Dieser Parameter legt für die Wandlung von Ware-Lieferscheinen in Rohwarelieferscheine fest, ob ein im Warelieferschein erfasster Preis des Lieferartikels als Abschlagpreis in den Rohwarebeleg zu übernehmen ist.

  **Finalpreis aus Lieferschein wenn != 0**

  Parameternummer: 150

  Einstellungen für Rohwarengruppen und Abrechnungsschemata sind möglich.

  Optionen:

- **nein**
- **ja**

  Dieser Parameter legt für die Wandlung von Ware-Lieferscheinen in Rohwarelieferscheine fest, ob ein im Warelieferschein erfasster Preis des Lieferartikels als Finalabrechnungspreis in den Rohwarebeleg zu übernehmen ist.

  **Abbruch wenn Kontrakt nicht zuzuordnen**

  Parameternummer: 160

  Einstellungen für Rohwarengruppen und Abrechnungsschemata sind möglich.

  Optionen:

- **nein**
- **ja**

  Dieser Parameter legt für die Wandlung von Ware-Lieferscheinen in Rohwarelieferscheine fest, ob die Erstellung des Rohwarebelegs abgebrochen werden soll, wenn der im Ware-Lieferschein angegebene Kontrakt unter Rohwarebedingungen nicht zugeordnet werden kann. Dieses ist zum Beispiel der Fall, wenn es sich nicht um einen Rohware-Kontrakt handelt.
