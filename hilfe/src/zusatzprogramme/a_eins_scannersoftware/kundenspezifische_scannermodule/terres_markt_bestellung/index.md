# Terres-Markt Bestellung

<!-- source: https://amic.de/hilfe/_scanner_markt_bestellung.htm -->

Die Markt Bestellung funktioniert zurzeit nur Online, dies bedeutet der Scanner braucht eine ständige WLAN Verbindung.

Mit diesem Modul wird eine Bestellung erfasst, die dann an einem oder mehreren Lieferanten zugeordnet werden kann. Die mit dem Scanner erfassten Daten können in der [Vorgangimportschnittstelle](../../../../vorgangsabwicklung/vorgangsimport/vorgangsimport_mit_opentrans/zeitgesteuerter_importprozess.md) noch [bearbeitet](../../../../vorgangsabwicklung/vorgangsimport/vorgangsimport_schnittstelle/vorgangsimport_anwendung/funktionen_des_vimp_pflegers.md#PositionBearbeiten) werden, bevor aus den Daten eine [Bestellung](../../../../vorgangsabwicklung/vorgangsbearbeitung_allgemein/vorgangsklassen_in_a_eins/bestellung.md) [erzeugt](../../../../vorgangsabwicklung/vorgangsimport/vorgangsimport_schnittstelle/vorgangsimport_anwendung/index.md#StandardVorgangErzeugen) wird.

Hauptmenü > Externe Kommunikation > Stammdatenimport > Vorgangsimport

Um eine Bestellung mit einem Scanner aufzunehmen muss wie folgt vorgegangen werden.

1. Der [Steuerparameter 801](../../../../firmenstamm/steuerparameter/scanner/private_scannerprozedur_spa_801.md) muss für den Scanner eingerichtet werden.

2. Einrichtung des [Moduls](../index.md)

3. Der Scancode [BSE](./scanncodes_fuer_die_bestellung.md#BestellungStart) muss im EAN 128 Verschlüsselt ausgedruckt werden.

4. Der Scancode [BSENDE](./scanncodes_fuer_die_bestellung.md#BesetellungEnde) muss in EAN 128 Verschlüsselt ausgedruckt werden.

5. Der Scancode [STORNO](./scanncodes_fuer_die_bestellung.md#BestellungStorno) muss im EAN 128 Verschlüsselt ausgedruckt werden.

6. Optional ILN Nummer des [Lieferanten](../../../../kunden_und_lieferanten/kunden_und_lieferantenstamm/hauptmaske/index.md).

7. Der Scanner muss auf das jeweilige [Lager](../index.md#LagerNummerSetzen) eingestellt sein.

8. Einrichten des [Steuerparameters 883](../../../../firmenstamm/steuerparameter/optionen_warenwirtschaft/automatische_artikellieferanten_zuordnung_beim_vorgangsimpor.md)

#### Ablauf

Um eine Bestellung zu starten muss als erstes der Scancode [**BSE**](./scanncodes_fuer_die_bestellung.md#BestellungStart) mit dem Scanner erfasst werden. Nach dem Starten der Bestellung besteht die Möglichkeit, der Bestellung einen Lieferanten zuzuweisen. Der Lieferant ist wie folgt einzugeben ***00+ILN Nummer des Lieferanten***. Die ILN Nummer wird im Lieferantenstamm gepflegt. Die [ILN Nummer](./scanncodes_fuer_die_bestellung.md#BestellungILNBeispiel) kann aber auch eingescannt werden, wenn diese als Scancode vorhanden ist.

Der Lieferant kann aber auch nach dem Erfassen der Bestellung in der [Vorgangimportschnittstelle](../../../../vorgangsabwicklung/vorgangsimport/vorgangsimport_mit_opentrans/zeitgesteuerter_importprozess.md) den erfassten Positionen manuell zugewiesen werden.

Als nächstes werden die einzelnen Positionen erfasst, dazu wird der Artikel mit dem Scanner erfasst. Dies geschieht über die EAN Nummer des Artikels. Kann die Artikel EAN von dem Scanner nicht gelesen werden, oder der EAN Code ist nicht vorhanden, so kann entweder die EAN Nummer oder die Artikelnummer des Artikels im Eingabefeld des Scanners eingegeben werden. Die Eingabe ist mit der Eingabetaste(Enter) abzuschließen.

Dabei ist zu beachten, dass eine eingegebene Artikelnummer nur dann als Artikelnummer erkannt wird, wenn die eingegebene Artikelnummer Numerisch ist und mehr als sechs Zeichen hat.

Wird ein Artikel nicht auf dem Lager gefunden, so erscheint eine Meldung auf dem Scanner „**Artikel nicht gefunden**“. Der Artikel wird trotz der Meldung, mit in die Bestellliste auf genommen. In der Positionsübersicht wird dann Anstelle der Artikelnummer die EAN Nummer des nicht gefundenen Artikels angezeigt.

Nach dem der Artikel erfasst worden ist, werden Informationen zu diesem Artikel dargestellt.

1. Der Artikel samt Nummer

2. Die Verkaufseinheit des Artikels

3. Der Verkaufspreis des Artikels

4. Die Verpackungseinheit

5. Der Einkaufspreis

6. Der Hauptlieferant

Nach der Erfassung des Artikels, wird die Menge eingegeben. Die Menge wird per Tastatur des Scanners erfasst und wird mit der Eingabetaste(Enter) bestätigt. Nach der Mengen Eingabe werden die erfassten Artikel in einer Übersichtstabelle dargestellt.

Werden gleiche Artikel Mehrfach mit dem Scanner erfasst, so werden diese Positionen nicht zusammengefasst. Dies Bedeutet, dass für jede Erfassung eine neue Positions angelegt wird.

Ist eine Bestellung zu Ende erfasst worden, so wird mit dem Scancode [**BSENDE**](./scanncodes_fuer_die_bestellung.md#BesetellungEnde) die Bestellung abgeschlossen. Die Bestellung kann dann in der [Vorgangimportschnittstelle](../../../../vorgangsabwicklung/vorgangsimport/vorgangsimport_mit_opentrans/zeitgesteuerter_importprozess.md) noch [bearbeitet](../../../../vorgangsabwicklung/vorgangsimport/vorgangsimport_schnittstelle/vorgangsimport_anwendung/funktionen_des_vimp_pflegers.md#PositionBearbeiten) werden, bevor aus den Daten eine Bestellung [erzeugt](../../../../vorgangsabwicklung/vorgangsimport/vorgangsimport_schnittstelle/vorgangsimport_anwendung/index.md#StandardVorgangErzeugen) wird.

Wurde eine Bestellung mit [BSENDE](./scanncodes_fuer_die_bestellung.md#BesetellungEnde) nicht geschlossen, so kann an dieser Bestellung immer weiter gearbeitet werden. Dies ist auch möglich, wenn das A.eins auf dem Scanner geschlossen worden ist oder der Scanner sich abgeschaltet hat, weil z.B. die Batterie leer ist. Dies gilt nur für den Scanner mit der gleichen IP-Adresse. Es ist nicht möglich eine Bestellung mit einem anderen Scanner zu beenden.

#### Stornieren von Falschen Eingaben

Es ist möglich per Storno Funktion einzelne Position zu löschen. Die Positionen werden rückwärts gelöscht. Dazu muss einmal [STORNO](./scanncodes_fuer_die_bestellung.md#BestellungStorno) gescannt werden, jetzt befindet sich der Scanner im Storno Modus. Mit dem erneuten Scannen des [Storno](./scanncodes_fuer_die_bestellung.md#BestellungStorno) Befehls wird die letzte Erfasste Position gelöscht. Wird jetzt noch einmal die Storno Funktion aufgerufen, so wird die nächste Position gelöscht usw. Mit dieser Möglichkeit kann die Komplette Bestellung storniert werden.

Um eine Bestellung komplett zu beenden, kann zwei Mal hintereinander [BSE](./scanncodes_fuer_die_bestellung.md#BestellungStart) gescannt werden. Dadurch wird eine neue Bestellung gestartet.

Durch erneutes erfassen eines Artikels wird der Storno Modus in beiden Fällen verlassen.

#### Einstiegspunkte in der Prozedur ScannerMarktBestellung

#### Folgende Einstiegspunkte sind realisiert worden

| Punkt | Typ | Block |
| --- | --- | --- |
| Bestellung Anfang Blockanfang | 1 | 0 |
| Bestellung Anfang Blockende | 1 | 1 |
| Bestellung Ende Blockanfang | 5 | 0 |
| Bestellung Ende Blockende | 5 | 1 |
| Menge Blockanfang | 3 | 0 |
| Menge Blockende | 3 | 1 |
| Artikel Blockanfang | 2 | 0 |
| Artikel Blockende | 2 | 1 |
| Partie Blockanfang | 4 | 0 |
| Partie Blockende | 4 | 1 |
| Vor der Lieferanten suche | 6 | 0 |

<p class="siehe-auch">Siehe auch:</p>

- [Scanncodes für die Bestellung](./scanncodes_fuer_die_bestellung.md)
