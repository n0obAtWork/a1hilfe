# Einrichtung

<!-- source: https://amic.de/hilfe/_nachhaltigkeit_einrichtung.htm -->

Neben den Einrichtungen der Stammdaten sind noch spezielle Einrichtungsmaßnahmen erforderlich.

- Nachhaltigkeitsstatus (Format [AF_NACHHSTAT](../stammdaten/formate.md))
- Zertifizierungsmethode (Format [AF_ZERTMETH](../stammdaten/formate.md))
- Zertifizierungstyps (Format [AF_NAHA_ZERT](../stammdaten/formate.md))
- Kategorie des Zertifikats (Format [AF_ZERTKATEG](../stammdaten/formate.md))
- SQLK Texte für Nachhaltigkeitsausweise im Formulardruck
- Formularzuordnung

Die hier aufgelisteten Einzelmaßnahmen werden in den folgenden Abschnitten erläutert.

<p class="just-emphasize">SQLK Nachweisvorlage ![](../../../ImagesExt/image8_148.png)</p>

Eine Vorlage zum Nachweis nachhaltiger Ware auf einem Vorgangsformular liefern wir unter SQLK_Nachhaltig eine Musterlösung mit. Dabei wird eine Zulieferfunktion „ist_nachhaltig“ in Form einer Datenbankprozedur mit Resultset verwendet. Dieser Nachweis ist in jedem Fall für **Verkäufe** relevant, einzelne Anforderungen beziehen sich jedoch auch auf Einkäufe bzw. Getreidegutschriften. (siehe SQLK Nachweisvorlage)

<p class="just-emphasize">Einrichtung Vorgangswesen FRZ ![](../../../ImagesExt/image8_148.png)</p>

U.U. kann es gewollt sein, dass man für Lieferungen steuern möchte, ob nachhaltige oder nicht nachhaltige Ware geliefert werden soll. Für solche Kunden trägt man auf dem Zertifikate-Register die Nachhaltigkeit ein.

In der [Formularzuordnung](../../formularzuordnung/index.md) (FRZ) trägt man im Register Abwicklung für die betreffenden Vorgangsunterklassen ein, wie die Vorgangserfassung zu reagieren hat, wenn ein als nachhaltig geführter Artikel an einen als nicht nachhaltig geführten Kunden geliefert werden soll. (Feld „Kunde ungültige Nachhaltigkeit“)

<p class="siehe-auch">Siehe auch:</p>

- [SQLK Nachweisvorlage](./sqlk_nachweisvorlage.md)
- [Checkliste](./checkliste.md)
