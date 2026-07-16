# Tresenkasse

<!-- source: https://amic.de/hilfe/tresenkasse.htm -->

Hier stehen die Möglichkeiten des Bareinkaufs **[BVEE]**, des Barverkaufs **[BVVE]** und der Barverkaufsgutschrift **[BVG]** unter " Barvorgänge " zur Verfügung.  
Generell werden alle Funktionalitäten des Kreditverkaufs auch bei Barvorgängen zur Verfügung gestellt. Von Bedeutung sind hier insbesondere die Preisfindungsfunktionen.  
Die Erfassung beginnt mit der Bestimmung des Barverkaufskunden.  
Dies ist ein Kunde/Lieferant des Kunden-/Lieferantenstamms, es werden die für den Kunden/Lieferanten hinterlegten Konditionen angewandt. (Dieser ist vorbelegt mit dem Eintrag in den Kasseneinstellungen)

Für EK- bzw. VK können zwei verschieden Standardkunden/-lieferanten eingestellt werden. Außerdem kann für den Tresenverkauf und den Abverkauf an der POS-Kasse ebenfalls verschiedene Standardkunden hinterlegt werden.

Für den anonymen Barverkauf wird jedem Barverkauf bzw. Einkauf oder Gutschrift automatisch ein Standardkunde vorbelegt, der aus der entsprechenden Eintragung in den Kasseneinstellungen stammt, hier ist auch die Änderung des vorbelegten Kunden möglich. Die Konditionen dieses Standardkunden müssen im Kundenstamm hinterlegt sein. Diese Nummernvorbelegung ist nachträglich änderbar, solange noch keine Artikel erfasst wurden.

In Abhängigkeit von der Struktur des Barverkaufs werden zwei unterschiedliche Positionserfassungsmasken zur Verfügung gestellt. Unternehmen, die bei Barvorgängen auf Nebenbuchhaltungen wie Kontrakte, Partien, Baustellen zugreifen müssen, werden die bekannte Erfassungsmaske des Kreditverkaufs einsetzen.  
Von der Kopfinformation wird auf sie mit **F5**, **F4** verzweigt.  
    

Die Artikel werden über Artikelnummer / EAN - Nummer per Hand oder mit Hilfe eines Scanners eingelesen.

Bei der Artikelerfassung ist es möglich, mit **F3** die Artikelmaske aufzurufen. Unten links werden die zugelassenen Suchkriterien angezeigt. Das Hauptkriterium kann nun dadurch festgelegt werden, dass zuerst das Kriterium bestimmt wird und danach aus der Box unten links mit **F5** dieses Kriterium als Einstiegsvariante festgelegt wird. Diese Einstiegsvariante bleibt bis zur nächsten Änderung bestehen.

Generell beschränkt sich die Eingabe in der Schnellerfassung auf Artikel, Menge und ggf. den Einzelpreis. Innerhalb dieser Positionen wird der aufgelaufene Bruttobetrag angezeigt. Über entsprechende Parameter in den Einrichtungsparametern der Maske können Erfassungsreihenfolgen und -stopps definiert werden sowie Sicherheitsabfragen zwischengeschaltet werden.

Es besteht natürlich weiterhin die Möglichkeit, die Zusatzfunktionen (Text, Korrektur, etc.) mittels der Funktionstasten aufzurufen und bei der Artikelerfassung zu nutzen. Die erfassten Artikel werden oberhalb des Erfassungsfensters angezeigt. Zwischen der Schnellerfassung und der Standardmaske kann innerhalb eines Vorgangs gewechselt werden.

Bei Ausstieg / Beenden der Artikelerfassung wird die eigentliche Zahlungsmaske automatisch aufgerufen. Bei korrektem Vorgangsabschluss wird die getätigte Zahlung Buchungsweg.

Hierzu werden die Daten im Mandantenserver verbucht, dann wird der Vorgang an die FiBu übertragen und nach dem erneuten Mandantenserverlauf verbucht. Die Rechnung gilt beim Barverkauf dann als bezahlt.

Dabei wird beim Bareinkauf die Zahlung auf das in der Kassenverwaltung dieser Kasse zugeordnete Kassenkonto im Haben verbucht und gleichzeitig auf das Kostenkonto des Standardlieferanten im Soll verbucht.

Analog wird im Barverkauf auf das in der Kassenverwaltung dieser Kasse zugeordnete Kassenkonto im Soll verbucht und gleichzeitig auf dem Kostenkonto des Standardkunden im Haben gebucht.

<p class="siehe-auch">Siehe auch:</p>

- [weitere Funktionen der Tresenkasse](./weitere_funktionen_der_tresenkasse.md)
