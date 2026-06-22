# Permanente Inventur mit Hilfe des Lagerverwaltungssystems

<!-- source: https://amic.de/hilfe/permanenteinventurmithilfedesl.htm -->

Für die Erfassung soll die [Vorgangsimport-Schnittstelle LVS](../../../vorgangsabwicklung/vorgangsimport/vorgangsimport_schnittstelle/importierbare_vorgaenge/lvs.md) verwendet werden. Dort finden sich mit den Unterklassen 21 und 61 zwei Möglichkeiten einer inventurfähigen Zählung.

Durch geeignete organisatorische Maßnahmen (Inventurlisten) wird der Bestand eines Artikels und aller seiner Partien sukzessiv aufgenommen.

Nach der Erfassung einer Ladeträgermenge wird die Tatsache, dass dieser Ladeträger gezählt wurde, in den Ladeträgerbewegungen mittels Kennzeichen geplante Inventur festgehalten.

Zugleich wird jeweils geprüft, ob zu diesem Zeitpunkt alle Ladeträger im System, die diesen Artikel beinhalten, innerhalb des Zählzeitraums (SPA 1045) bzw. im Wirtschaftsjahr gezählt wurden.

Ist dies der Fall, so wird dieser Artikel für die Erstellung eines Bestandsbelegs vorgemerkt. Weitere bestandsverändernde Zu- oder Abgänge sind möglich. Die Änderung der Buchbestände erfolgt erst bei der Erstellung eines Inventurbestandsbeleges (5055).

Die Erstellung des Bestandsbuchungsbeleges (5055) kann manuell in der Variante [Inventur > Permanente Inventur > Permanente Inventur Prüfungen](../vollstaendigkeitspruefung_der_permanenten_inventur/index.md) \> LVS ungezählte Artikel erstellt werden oder [automatisch über ein Event](../bestandsbuchungen_vorgangsklasse_5055/automatisierung_der_erstellung_von_bestandsbuchungen.md).
