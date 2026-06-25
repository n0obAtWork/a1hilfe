# Informationen über Buchungen

<!-- source: https://amic.de/hilfe/informationenberbuchungen.htm -->

Buchungen in der Tabelle Warenbewegung werden an verschiedenen Stellen dazu verwendet, Summen zu bilden. Kennzeichen signalisieren, um welche Art von Buchung es sich handelt.

| Vorgang | [VorFakKennz](./vorfakturierungskennzeichen_wabewvorfakkennz.md) | [BestTyp](./bestandstyp_wabewbesttyp.md) | [BestTyp Reverse](./bestandstyp_fuer_ruecknahmen_wabewbesttypreverse.md) | [Typ EKVK](./einkauf_verkauf_typ_wbc_typ_ekvk.md) | [Bew Art](./bewegungsart_wbc_bewart.md) | [Bew Code](./bewegungscode_wbc_bewcode.md) | Inv |
| --- | --- | --- | --- | --- | --- | --- | --- |
| Verkauf | 0 | 0 | 0 | 2 | 0 | 20 | 1 |
| Vorverkauf | 0 | 0 | 0 | 2 | 0 | 1 | 1 |
| Vorverkauf Abholung | 0 | 1 | 0 | 0 | 1 | 11 | 1 |
| Vorverkauf Rücknahme | 0 | 0 | 1 | 2 | 0 | 21 | 1 |
| Kommission | 4 | 0 | 0 | 0 | 4 | 4 | 1 |
| Kommission Verkauf | 0 | 4 | 0 | 2 | 0 | 14 | 0 |
| Kommission Rücknahme | 0 | 0 | 4 | 0 | 4 | 24 | 1 |
| Einkauf | 0 | 0 | 0 | 1 | 0 | 10 | 1 |
| Voreinkauf | 1 | 0 | 0 | 1 | 0 | 2 | 0 |
| Voreinkauf Anlieferung | 0 | 2 | 0 | 0 | 2 | 12 | 0 |
| Voreinkauf Rückgabe | 0 | 0 | 2 | 1 | 0 | 22 | 1 |
| Einlagerung | 3 | 0 | 0 | 0 | 3 | 3 | 1 |
| Einlagerung Vereinnahmung | 0 | 3 | 0 | 1 | 0 | 13 | 1 |
| Einlagerung Abholung | 0 | 0 | 3 | 0 | 3 | 23 | |

<p class="siehe-auch">Siehe auch:</p>

- [Buchungstyp (KtrBuchTyp)](./buchungstyp_ktrbuchtyp.md)
- [Vorfakturierungskennzeichen (WaBewVorFakKennz)](./vorfakturierungskennzeichen_wabewvorfakkennz.md)
- [Bestandstyp (WaBewBestTyp)](./bestandstyp_wabewbesttyp.md)
- [Bestandstyp für Rücknahmen (WaBewBestTypReverse)](./bestandstyp_fuer_ruecknahmen_wabewbesttypreverse.md)
- [Einkauf/Verkauf Typ (wbc_Typ_EKVK)](./einkauf_verkauf_typ_wbc_typ_ekvk.md)
- [Bewegungsart (wbc_BewArt)](./bewegungsart_wbc_bewart.md)
- [Bewegungscode (wbc_BewCode)](./bewegungscode_wbc_bewcode.md)
- [View AMIC_V_Warenbewegung_Info](./view_amic_v_warenbewegung_info.md)
