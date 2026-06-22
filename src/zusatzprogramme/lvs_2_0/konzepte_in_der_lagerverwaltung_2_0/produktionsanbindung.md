# Produktionsanbindung

<!-- source: https://amic.de/hilfe/_lvs20_prodanbindung.htm -->

Es gibt für die Kommunikation mit Produktionssystemen eine XML-Dateiaustausch-basierende Schnittstelle.

Diese Schnittstelle erledigt verschiedene Aufgaben:

1. Übertragung der Produktionsdaten an das Produktionssystem

Hier wird die Komponentenliste mit den Mengen an die Produktion übertragen.

2. Empfang von Materialbedarf

In A.eins wird eine Materialorder [[LVSMO](./materialorder_lvsmo.md)] erstellt.

3. Empfang von Ware-Fertig-Meldungen

Hier werden in A.eins Ladeträger an der Fertigstellungslokalität der Linie erstellt und beladen. (siehe auch [[PRODL]](../../../firmenstamm/produktionslinien.md))

4. Empfang von Verbrauchsmeldungen

Hier wird die verbrauchte Menge vom Ladeträger in der Bereitstellungszone abgebucht. (siehe auch [[PRODL]](../../../firmenstamm/produktionslinien.md))

5. Empfang von Fertigmeldung einer Produktion. Hier werden die Verbräuche und die Produktmenge in der Produktion korrigiert.

Näheres dazu in [Produktionsinterface](../../../externe_kommunikation/produktions_interface/index.md#Produktionsinterface).
