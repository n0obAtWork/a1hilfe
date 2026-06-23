# Aufträge zusammenführen

<!-- source: https://amic.de/hilfe/_auftraegezusammenfuehren.htm -->

Hauptmenü \> Warenverkauf > Auftrag > Auftragsbearbeitung

oder Direktsprung ****[AUB]****

Die Freischaltung dieser Spezialfunktion unterliegt dem [Steuerparameter „723 – Aufträge zusammenführen erlauben?“](../../firmenstamm/steuerparameter/vorgangsbearbeitung_warenposition/auftragszusammenfuehrungen_erlauben_spa_723.md).

Mit dieser Spezialfunktion lassen sich ausgewählte Aufträge in denen Kunden mehrfach gleiche Artikel mit unterschiedlichen Mengen erstellt haben (nach Artikelnummern) zusammenfassen. Hierbei werden folgende Kriterien berücksichtigt:

- Kunde
- Artikel
- Lager und Lagerplatz
- Versandadresse
- Rechnungsempfänger
- Plan-/Lieferdatum

So werden Aufträge, in denen der gleiche Kunde den gleichen Artikel bestellt, aber beispielsweise unterschiedliche Läger verwendet zusammengefasst. Aufträge in denen der gleiche Kunde den gleichen Artikel unter Angabe verschiedener Plan-/Lieferdaten aufgibt werden zusammengefasst und zum spätesten Datum zur Lieferung geplant.

Zusammengefasste Aufträge werden auf unterschiedliche Arten behandelt. So ist es nicht immer notwendig Aufträge neu zu erstellen sondern nur zu ändern. Weiterhin kann über einen Einrichtungsparameter entschieden werden, ob die Originalaufträge storniert oder in Auftrags-Storno-Belege umgewandelt werden sollen.

Der Aufbau gliedert sich wie folgt und wird anschließend beschrieben:

Im oberen Teil wird das Abgangslager bestimmt. Weiter rechts befinden sich die einzelnen Funktionen, über die die Anwendung gesteuert wird.

Anschließend folgen die Reiter „Aufträge“, „Artikelübersicht“ und „Zusammengeführt“.

Das Abgangslager legt fest, aus welchem Lager die bestellten Artikel kommen sollen. Die Nummer des Lagers wird angegeben und bestätigt. Anschließend kann der Name des Lagers hinter der Nummer zur Prüfung eingesehen werden.

Die Funktionen in der Übersicht:

- ***Starten*** **F9** startet den Vorgang der Zusammenführung der ausgewählten Vorgänge
- ***Einrichtungsparameter*** (folg)
- ***Dieses Menü*** – wird unter diesem [Link](../../allgemeine_programmfunktionen/generelle_programmbedienung/dieses_menue/index.md) genauer beschrieben

Auf dem Reiter „Aufträge“ ist eine Übersicht der ausgewählten Aufträge zu sehen und gliedert sich wie folgt:

| Reiter „Aufträge“ | |
| --- | --- |
| Kundennummer | Angabe der Kundennummer des Auftrag gebenden Kunden |
| Kunde | Name des Auftraggebers |
| Lieferadresse | Kurzbezeichnung der Kundenlieferadresse |
| Lieferort | Ort an den geliefert werden soll |
| Artikeln | Die Nummer des in Auftrag gegebenen Artikels |
| Artikel | Artikelbezeichnung |
| Lagerplatz | Lagerplatz des Artikels im Lager |
| Menge | Die beauftragte Menge des Artikels |
| Plan-/Lieferdatum | Plandatum der Lieferung |
| Rechnungsadresse | Adresse des Rechnungsempfängers |
| Rechnungsort | Ort des Rechnungsempfängers |
| Belegnummer | Nummer des Originalbelegs |
| Neue Belegnummer | Bei Änderung der Belegnummer wird diese hier angezeigt |

Auf dem Reiter „Artikelübersicht“ werden die in den Aufträgen angegebenen Artikel zusammengefasst. Hierbei spielt allein der Artikel eine Rolle und nicht der Auftraggeber:

| Reiter „Artikelübersicht“ | |
| --- | --- |
| Artikeln | Die Nummer des in Auftrag gegebenen Artikels |
| Artikel | Artikelbezeichnung |
| Lagerplatz | Lagerplatz des Artikels im Lager |
| Menge | Die Gesamtmenge des Artikels aus ALLEN Aufträgen |

Auf dem Reiter „Zusammengeführt“ werden die markierten Aufträge bereits zusammengefasst dargestellt. Er zeigt somit das Ergebnis einer Zusammenführung der Aufträge des Reiters „Aufträge“. Hierbei werden Artikelmengen nach Kunden sortiert angezeigt.

| Reiter „Zusammengeführt“ | |
| --- | --- |
| Belegnr | ID des Beleges |
| Kundennummer | Angabe der Kundennummer des Auftrag gebenden Kunden |
| Lieferadresse | Kurzbezeichnung der Kundenlieferadresse |
| Lieferort | Ort an den geliefert werden soll |
| Artikeln | Die Nummer des in Auftrag gegebenen Artikels |
| Artikel | Artikelbezeichnung |
| Lagerplatz | Lagerplatz des Artikels im Lager |
| Anzahl | |
| Menge | Die Gesamtmenge des Artikels aus den Aufträgen des Kunden |
| Kundid | Kunden ID |

Nach Prüfung der ausgewählten Aufträge über die o.g. Reiter können die Aufträge nun über die Funktion ***Starten*** **F9** zusammengefasst werden.

Hierbei werden neue Vorgänge angelegt. Diese erhalten eine neue Vorgangs-ID. Die alten Aufträge werden aus dem System entfernt und sind somit nicht mehr vorhanden, sodass es nicht zu Doppelvorgängen kommt. Zu beachten ist, dass Vorgänge, welche nicht zusammengefasst werden natürlich keiner Veränderung bedürfen und keine neue Vorgangs-ID erhalten.

Ein einfach dargestelltes Beispiel:

Kunde 1 bestellt zweimal Artikel1 in den Mengen 50 und 25.

Kunde 2 bestellt Artikel1 in einer Menge von 75 und dazu Artikel2 in der Menge 50.

Die Aufträge werden in der Auftragsbearbeitung ausgewählt und werden unter dem Reiter „Aufträge“ dargestellt. Hieraus ergibt sich eine Summe von 4 Vorgängen mit den entsprechenden Auftragsnummern.

| Aufträge | | | | |
| --- | --- | --- | --- | --- |
| **Auftragsnummer** | **Kunde** | **Artikel** | **Artikelnummer** | **Menge** |
| 1 | Kunde 1 | Artikel 1 | 100 | 50 |
| 2 | Kunde 1 | Artikel 1 | 100 | 75 |
| 3 | Kunde 2 | Artikel 2 | 200 | 50 |
| 4 | Kunde 2 | Artikel 1 | 100 | 75 |

Unter dem Reiter „Artikelübersicht“ werden nun die beauftragten Artikel in der Gesamtmenge dargestellt. Hier erhalten wir somit je einen Eintrag zu Artikel 1 mit einer Gesamtmenge von 150 und zu Artikel 2 mit einer Menge von 50.

| Artikelübersicht | | |
| --- | --- | --- |
| **Artikel** | **Artikelnummer** | **Menge** |
| Artikel 1 | 100 | 150 |
| Artikel 2 | 200 | 50 |

Bei der Betrachtung des Reiters „Zusammengeführt“ ergibt sich folgendes Bild. Hier sind drei Einträge zu sehen. Diese sind nach Kunde und nach beauftragtem Artikel sortiert und stellen sich wie folgt dar:

| Zusammengeführt | | | | |
| --- | --- | --- | --- | --- |
| **Auftragsnummer** | **Kunde** | **Artikel** | **Artikelnummer** | **Menge** |
| 5 | Kunde 1 | Artikel 1 | 100 | 75 |
| 3 | Kunde 2 | Artikel 2 | 200 | 50 |
| 4 | Kunde 2 | Artikel 1 | 100 | 75 |

Durch starten der Zusammenfassung der Aufträge werden aus den beiden Vorgängen des Kunde 1 nun ein Vorgang mit neuer Auftragsnummer. Die beiden alten werden entfernt und sind nicht mehr auffindbar. Die Vorgänge für Kunde 2 können nicht zusammengefasst werden und bleiben unverändert vorhanden.
