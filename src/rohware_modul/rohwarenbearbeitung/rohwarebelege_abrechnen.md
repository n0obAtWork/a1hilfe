# Rohwarebelege abrechnen

<!-- source: https://amic.de/hilfe/_rwbelegeAbrechnen.htm -->

Hauptmenü > Rohwarenabrechnung > Rohwarenabrechnung > EK-Rohwarenbearbeitung > Abrechnen

Direktsprung **[RWB]**

Hauptmenü > Rohwarenabrechnung > Rohwarenabrechnung > VK-Rohwarenbearbeitung > Abrechnen

Direktsprung **[RWBV]**

Mit dieser Funktion können die ausgewählten Rohwarebelege abgerechnet werden, sofern diese für ihre Belegstufe (Abschlag, Folgeabschlag beziehungsweise Finale) in dem zugehörigen Statusattribut über den Wert ‚Freigegeben‘ verfügen. Nur abgerechnete Belege können gedruckt, an die Finanzbuchhaltung übergeben, in die nächste Stufe umgewandelt und/oder storniert werden.  
Ist der Rohwareparameter [RWPA] [*Freigegebene Belege immer abrechnen*](../rohwareparameter_einrichten/rohwareparameter_uebersicht.md#RWPA_174) mit dem Wert ‚*Ja*‘ eingestellt, so steht diese dann überflüssige Funktion nicht zur Verfügung.  
Nach Aufruf der Funktion **Abrechnen** erscheint eine Steuerungsmaske, auf der Einstellungen für den Abrechnungslauf vorgenommen werden können:

Zunächst kann festgelegt werden, ob das Originaldatum des jeweils abzurechnenden Belegs erhalten bleibt, oder ob es durch ein dann anzugebendes neues Beleg-Datum (vorbelegt mit dem aktuellen Tagesdatum) zu ersetzen ist. Die Vorbelegung dieser Auswahl ist mit dem Rohwareparameter [RWPA] [*Rechnungsdatum setzten auf*](../rohwareparameter_einrichten/rohwareparameter_uebersicht.md#RWPA_108) festgelegt.  
Wird hier die Variante mit Belegdatumangabe gewählt, so ist gegebenenfalls in einem weiteren Feld anzugeben, ob die warenwirtschaftliche Buchungsperiode der Belege gegebenenfalls an das neue Datum anzupassen ist oder die bereits festgelegten Perioden beizubehalten sind. Dieses Feld ist auf der Maske jedoch nur aktiv, wenn der Rohwareparameter [RWPA] [*Periode bei Belegdatum=Abrechnungsdatum*](../rohwareparameter_einrichten/rohwareparameter_uebersicht.md#RWPA_173) mit der Einstellung ‚*Laut Maskeneinstellung*‘ versehen ist. Die beiden anderen Einstellungsmöglichkeiten jenes Parameters (‚*Periode beibehalten*‘ oder ‚*Periodenanpassung an Belegdatum*‘) werden automatisch berücksichtigt.

Für die Behandlung des Valutadatums (Zahlungsziel) für Belege, deren Zahlungsbedingung die Eingabe eines festen Datums als Zahlungsziel erfordern (manuelles Zieldatum), kann festgelegt werden, ob das auf der Maske anzugebende Valutadatum nur für solche Belege übernommen wird, deren manuelles Zieldatum noch nicht gesetzt wurde (= ‚*01.01.1901*‘), oder auch bereits mit gültigem Datum versehene manuelle Ziele überschrieben werden sollen.  
Dieses Feld ist auf der Maske jedoch nur aktiv, wenn der Rohwareparameter [RWPA] [*Abrechnen: ZB mit man. Valutadatum*](../rohwareparameter_einrichten/rohwareparameter_uebersicht.md#RWPA_128) mit der Einstellung ‚*Laut Maskeneinstellung*‘ versehen ist. Die beiden anderen Einstellungsmöglichkeiten jenes Parameters (‚*Tagesdatum, wenn noch nicht gesetzt*‘ oder ‚ *immer auf Tagesdatum*‘) werden automatisch berücksichtigt.

Zusätzlich ist es an dieser Stelle möglich, mittels der Auswahl von Zusatzfunktionen festzulegen, ob die erfolgreich abgerechneten Belege im Anschluss automatisch gedruckt und/oder an die Finanzbuchhaltung zu übertragen sind.
