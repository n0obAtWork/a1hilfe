# Sammelerstdruck

<!-- source: https://amic.de/hilfe/_rwsammelErstDruck.htm -->

Hauptmenü > Rohwarenabrechnung > Rohwarenabrechnung > EK-Rohwarenbearbeitung

Direktsprung **[RWB]**

Hauptmenü > Rohwarenabrechnung > Rohwarenabrechnung > VK-Rohwarenbearbeitung

Direktsprung **[RWBV]**

Die Auswahlvariante ‚*Sammelerstdruck*‘ im Einkauf und im Verkauf dient der Erstellung von Rohware-Sammeldruck-Belegen. Unter Berücksichtigung der in der Bereichsauswahl anzugebenden Auswahlkriterien werden hier Einzelbelege aufgeführt, die die für die Verwendung in einem Sammeldruck erforderlichen Kriterien erfüllen:

Die Belege dürfen noch nicht gedruckt sein

Die Belege dürfen noch nicht an die Finanzbuchhaltung übertragen sein

Die Belege dürfen noch nicht weiterverarbeitet sein

Für die Abrechnungsstufe muss ein Sammelabrechnungsformular in den Belegen zugeordnet sein

Die Beleg-Trennung pro Sammeldruck erfolgt automatisch nach den Kriterien

Kunden-/Lieferantennummer

Rechnungsempfänger

Zahlungsempfänger/Zahlungspflichtiger

Währung

UmsatzsteuerID des Kunden/Lieferanten im Beleg

Eigene UmsatzsteuerID im Beleg

Wirtschaftsjahr

Warenwirtschaftsperiode

Sammelformularnummer

Zusätzlich können mit den entsprechenden Rohwareparametern [RWPA] weitere Trennkriterien festgelegt werden

[Vertretergruppe](../rohwareparameter_einrichten/rohwareparameter_uebersicht.md#RWPA_138)

[Kontraktnummer](../rohwareparameter_einrichten/rohwareparameter_uebersicht.md#RWPA_152)

[Versandadresse](../rohwareparameter_einrichten/rohwareparameter_uebersicht.md#RWPA_166)

[Lagernummer](../rohwareparameter_einrichten/rohwareparameter_uebersicht.md#RWPA_132)

[Rohwarengruppe](../rohwareparameter_einrichten/rohwareparameter_uebersicht.md#RWPA_102)

[Abrechnungsschema](../rohwareparameter_einrichten/rohwareparameter_uebersicht.md#RWPA_103)

[Artikel](../rohwareparameter_einrichten/rohwareparameter_uebersicht.md#RWPA_104)

[Liefermonat](../rohwareparameter_einrichten/rohwareparameter_uebersicht.md#RWPA_105)

[Lieferwoche](../rohwareparameter_einrichten/rohwareparameter_uebersicht.md#RWPA_106)

[Währungskurs](../rohwareparameter_einrichten/rohwareparameter_uebersicht.md#RWPA_188)

Einfluss auf die Trennung hat auch der Rohwareparameter [RWPA] [*Sammeldruck-Sortierung*](../rohwareparameter_einrichten/rohwareparameter_uebersicht.md#RWPA_139) mit den Einstellmöglichkeiten ‚*automatisch*‘ und ‚*nach Belegauswahl‘*. Erstere gewährleistet die Berücksichtigung der Einzelbelege in einer den internen und den eingestellten Trennkriterien entsprechenden Reihenfolge, so dass möglichst optimale Trennungen erfolgen. Zu beachten ist in diesem Falle auch die Einstellung des Parameters [Sammeldrucksortierung](../rohwareparameter_einrichten/rohwareparameter_uebersicht.md#RWPA_187) automatisch: mit Wiegenummer. Die Einstellung ‚*nach Belegauswahl‘* hingegen berücksichtigt die Einzelbelege entsprechend der Reihenfolge der getroffenen Auswahl. Dieses ist nur bei bestimmten Arbeitsweisen sinnvoll, da zum Beispiel bei einer primären Sortierung nach Lieferdatum bei Belegen mehrerer Kunden und mehreren Tagen das Trennkriterium ‚*Kundennummer*‘ bei fast jedem Einzelbelegwechsel zur Trennung führen wird.

Ist der Rohwareparameter [RWPA] [*Sammeldrucktrennungs-Hinweis generieren*](../rohwareparameter_einrichten/rohwareparameter_uebersicht.md#RWPA_164) mit dem Wert ‚*Ja*‘ belegt, so wird bei jedem Trennungsvorgang ein Hinweis bezüglich des verursachenden Trennkriteriums erzeugt und zusammen mit anderen auftretenden Meldungen und Hinweisen am Ende des Sammeldruck-Laufs ausgegeben.

Für den Sammeldruck von Rohwareabrechnungen der Abrechnungsstufen ‚*Folgeabschlag*‘ und ‚*Finale*‘ sind die Rohwareparameter [RWPA] [*Sammeldruck Folge-Abschlag mit Sammelabschlag-Prüfung*](../rohwareparameter_einrichten/rohwareparameter_uebersicht.md#RWPA_156) beziehungsweise [*Sammeldruck Finale mit Sammelabschlag-Prüfung*](../rohwareparameter_einrichten/rohwareparameter_uebersicht.md#RWPA_157) von Bedeutung, wenn diese nicht mit dem Wert ‚*ohne*‘ belegt sind. Die Einstellungen ‚*mit Sammeldruck-Abbruch*‘ und ‚*mit Warnung ohne Abbruch*‘ lösen nach der Zusammenstellung der Einzelbelege für den Druck eines Sammeldruck-Belegs eine Vollständigkeitsprüfung hinsichtlich der Berücksichtigung der Einzelbelege in Sammeldruck-Belegen der vorhergehenden Abrechnungsstufe aus. Ist der zugehörige Beleg der vorhergehenden Abrechnungsstufe eines Einzelbelegs der aktuellen Zusammenstellung ebenfalls Teil eines Sammeldrucks, so ist diese Prüfung nur dann erfolgreich, wenn auch die Nachfolger aller anderen Belege jenes Sammeldruckbelegs im aktuell zu druckenden Sammelbeleg enthalten sind. Je nach Einstellung des Rohwareparameters wird nur ein Warnhinweis generiert oder der Druck des aktuellen Sammeldrucks unterdrückt.

Einzelbelege für den Sammeldruck werden, sofern sie es noch nicht sind und über das entsprechende Freigabekennzeichen verfügen, bei der Sammeldruckerstellung automatisch abgerechnet. Ist der Rohwareparameter [RWPA] [*Abrechnung für Sammeldruck mit Druckdatum*](../rohwareparameter_einrichten/rohwareparameter_uebersicht.md#RWPA_107) mit dem Wert ‚*Ja*‘ eingestellt, so werden auch bereits abgerechnete Belege noch einmal abgerechnet und dabei das jeweilige Rechnungsdatum der Einzelbelege auf das angegebene Druckdatum gesetzt.

Damit eRechnungen beim Aufruf der Funktion **Sammel-Erstdruck** erzeugt werden können, müssen im Abschnitt [Abwicklung](../../vorgangsabwicklung/formularzuordnung/abwicklung.md#Abwicklung_Versand) beschriebene Einrichtungen durchgeführt werden.

Funktion: Sammel-Erstdruck

Nach Aufruf der Funktion **Sammel-Erstdruck** erscheint eine Steuerungsmaske, auf der Einstellungen für die Sammeldruckerstellung vorgenommen werden können:

Zunächst kann ein Druckdatum, vorbelegt mit dem Tagesdatum, angegeben werden.  
Dabei ist zu beachten, dass dieses Datum auch das Belegdatum des Sammeldruck-Belegs ist und daher ebenso wie die ggf. angegebene Periode im selben Geschäftsjahr wie das der zu verarbeitenden Einzelbelege liegen muss.

Ist entweder der Rohwareparameter [RWPA] [*Abrechnung für Sammeldruck mit Druckdatum*](../rohwareparameter_einrichten/rohwareparameter_uebersicht.md#RWPA_107) mit dem Wert ‚Ja‘ oder der Rohwareparameter [RWPA] [*Rechnungsdatum setzten auf*](../rohwareparameter_einrichten/rohwareparameter_uebersicht.md#RWPA_108) mit dem Wert ‚Datum neu bei Abrechnung‘ eingestellt, so kann in einem weiteren Maskenfeld angegeben werden, ob die warenwirtschaftlichen Buchungsperioden der Belege gegebenenfalls an das neue Datum anzupassen sind oder die bereits festgelegten Perioden beizubehalten sind. Dieses Feld ist auf der Maske jedoch nur aktiv, wenn der Rohwareparameter [RWPA] [*Periode bei Belegdatum=Abrechnungsdatum*](../rohwareparameter_einrichten/rohwareparameter_uebersicht.md#RWPA_173) mit der Einstellung ‚Laut Maskeneinstellung‘ versehen ist. Die beiden anderen Einstellungsmöglichkeiten jenes Parameters (‚Periode beibehalten‘ oder ‚Periodenanpassung an Belegdatum‘) werden automatisch berücksichtigt.

Für die Behandlung des Valutadatums (Zahlungsziel) für Belege, deren Zahlungsbedingung die Eingabe eines festen Datums als Zahlungsziel erfordern (manuelles Zieldatum), kann festgelegt werden, ob das auf der Maske anzugebende Valutadatum nur für solche Belege übernommen wird, deren manuelles Zieldatum noch nicht gesetzt wurde (= ‚*01.01.1901*‘), oder auch bereits mit gültigem Datum versehene manuelle Ziele überschrieben werden sollen.  
Dieses Feld ist auf der Maske jedoch nur aktiv, wenn der Rohwareparameter [RWPA] [*Abrechnen: ZB mit man. Valutadatum*](../rohwareparameter_einrichten/rohwareparameter_uebersicht.md#RWPA_128) mit der Einstellung ‚*Laut Maskeneinstellung*‘ versehen ist. Die beiden anderen Einstellungsmöglichkeiten jenes Parameters (‚*Tagesdatum, wenn noch nicht gesetzt*‘ oder ‚ *immer auf Tagesdatum*‘) werden automatisch berücksichtigt. Das Valutadatum wird aber nur behandelt, wenn ein Beleg während des Sammeldrucks abgerechnet wird.

Funktion: Ansicht

Die Funktion **Ansicht** öffnet die gewählten Einzelbelege im Ansicht-Modus.
