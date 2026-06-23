# Abwicklung

<!-- source: https://amic.de/hilfe/_nachhaltigkeit_abwicklung.htm -->

Behandlung auf einem Artikelkonto

- Nachhaltige und nicht nachhaltige Ware wird auf einem Artikelkonto geführt
- Das Nachhaltigkeitskennzeichen j/n wird je Warenbewegung mitgeführt
- Je Warenbewegung werden die individuellen (Teil-) Standardwerte geführt, wenn sie über Stammdaten bzw. individuell je Bewegung erfasst, werden

Erfassungsunterstützung

- Ausgegangen wird davon, dass über die eingetragenen Stammdaten eine weitgehende Automatisierung durchgeführt werden kann.
- Für den Sonderfall sind individuelle Eingaben möglich
- Eine Nachbearbeitung ist möglich
- Abweichungen vom „Standard“ werden ausgewiesen
- Die Zuordnung des Merkmals „Nachhaltigkeit“ erfolgt mit der physischen Bewegung

Die Bearbeitung im Ein- und Verkauf (Funktionen ELE und LIE) erfolgt auf der Erfassungsmaske Eingangslieferschein und Lieferschein über den Tabreiter „Nachhaltig“ zur Übersteuerung der Defaultwerte.

Bei Rohwarevorgängen erfolgt die Bearbeitung Nachhaltigkeitsangaben direkt auf der Bearbeitungsmaske entsprechend der Einstellungen der zugehörigen Rohwarenparameter.

Interne Warenbewegungen

Lagerumbuchungen, Artikelumbuchungen und Produktionsumbuchungen werden ebenfalls über o.a. Tabreiter „Nachhaltig“ mit dem Kennzeichen versorgt. Vorbelegt werden die Umbuchungen über die Systematik „Eintragung im Mandantenstamm“ und „Artikelstamm“, also als „nachhaltig“. Nicht nachhaltige Umbuchungen sind also (wie auch oben) zu kennzeichnen.

Online Waage

Bei der Erfassung über die online Waage wird immer davon ausgegangen, dass die Standardvorbelegungen ziehen. Änderungen sind im Einzelfall entsprechend der Beschreibung in Abschnitt 4 vorzunehmen.

Abwicklung im Verkauf

Prinzipiell kann nachhaltige und nicht nachhaltige Ware gehandelt werden. Kunden, für die die Vorbelegung „nachhaltige Ware“ aktiviert werden soll, sind also wie unter „[Kunden / Mandant](../../../kunden_und_lieferanten/kunden_und_lieferantenstamm/zertifikate.md)“ beschrieben zu behandeln.

Berücksichtigung eigener Ware und Fremdware

Differenzierung auf dem Artikelkonto je Warenbewegung

Zuordnung und Festschreibungen von Massebilanzen

Es können nur Bewegungen zu eine Massebilanz zugeordnet werden, wenn eine NUTS-Nummer angegeben wurde und die Massebilanz für die jeweilige Bewegung eingerichtet wurde.(Lagernummer + Artikelnummer + NUTS-Nummer). Bei fehlender Einrichtung wird versucht die Einrichtung automatisch durchzuführen. Fall die Massebilanz von einem anderen Nutzer bearbeitet wird, oder die Bewegung eine leere NUTS-Nummer hat, wird das Zuordnen abgelehnt und es werden die Fehler ins Fehlerprotokoll geschrieben. Als Beispiel, dass Massebilanznummer XX von Benutzer XY gesperrt ist, dass Beleg XYZY für Massebilanz YY nicht eingerichtet werden konnte und das Beleg ZYZX keine leere NUTS-Nummer haben soll.

Nach dem Zuordnen von Bewegungen zu einer Massebilanz, werden die Zu – oder Abgänge in der Massebilanz selbst und den Massebilanzreporten berechnet und angezeigt. Beim Storno einer ,zu einer Massebilanz zugeordneten Rohwarebewegung, ist der vorherige Beleg(falls vorhanden) automatisch der Massebilanz zugeordnet.

Bei der Rohware ist nur der letzte Beleg einer Rohwarekette( Lieferschein - > Abschlag -> Folgeabschlag -> Finale) einer Massebilanz zuordbar.

Durch das Zuordnen einer Bewegung eines Rohwarebeleges, sind automatisch alle korrespondierenden Bewegungen in der Rohwarekette der Massebilanz zugeordnet.

Bei der Rohware wird durch das Zuordnen zu einer Bruttomassebilanz die Liefermenge der Bewegung beachtet und bei einer Nettomassebilanz die abgerechnete Menge der Bewegung.

Durch das Festschreiben der Massebilanz werden alle zugehörigen Bewegungen für andere Massebilanzen gesperrt und man kann die Bewegungen nicht mehr bearbeiten. Rohwarenbelege, die Bewegungen zu einer festgeschriebenen Massebilanz beinhalten, können durch die Funktion Storno mit Kopie noch weiterverarbeitet werden, allerdings werden die Massebilanzwerte dadurch nicht mehr geändert.

Zu beachten bei der Funktion Storno mit Kopie für Bewegungen von Rohwarebelegen, die noch keiner festgeschriebenen Massebilanz zugeordnet sind, ist der [Rohwareparameter 191](../../../rohware_modul/rohwareparameter_einrichten/rohwareparameter_uebersicht.md#RWPA_191) zu beachten.

<p class="siehe-auch">Siehe auch:</p>

- [Ermittlung des Nachhaltigkeitsstatus und der THG – Werte](./ermittlung_des_nachhaltigkeitsstatus_und_der_thg_werte.md)
- [Aktualisierung der Nachhaltigkeitswerte](./aktualisierung_der_nachhaltigkeitswerte.md)
