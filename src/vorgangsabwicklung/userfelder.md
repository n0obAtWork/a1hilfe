# Userfelder

<!-- source: https://amic.de/hilfe/_userfelder.htm -->

Hier können zu den Standardvorgangsfeldern noch weitere Felder in Abhängigkeit der Bedienerklasse, Vorgangsklasse und der Vorgangsunterklasse angezeigt werden.

<p class="just-emphasize">Positionierung</p>

Die Felder können dabei frei auf der Maske positioniert werden. Um die Einrichtung zu erleichtern, werden die Koordinaten beim Erstellen eines neuen Feldes automatisch vorgegeben.

Wird das erste Feld eingefügt, stehen die Bezeichnung und die Beschreibung in Zeile 13,79 und der Wert in Zeile 13,36. Die Spaltenwerte sind 0,5 für die Bezeichnung, 18,17 für den Wert und 38,00 für die Beschreibung. Wenn schon Felder existieren und der Cursor in die erste freie Zeile bewegt wird, wird das Feld bestimmt, das am weitesten unten steht. Auf die Zeilenwerte dieses Feldes wird 1,71 addiert, um die neuen Zeilenwerte zu erhalten. Die Spaltenwerte für das neue Feld übernommen.

Wird per **F8** zwischen zwei bestehenden Feldern ein neues Feld eingefügt, werden alle nachfolgenden Felder nach unten verschoben, indem 1,71 auf die Zeilenwerte addiert werden.

Das Verhalten beim Löschen eines Feldes kann mit einem Einrichterparameter bestimmt werden. Es besteht die Wahl zwischen:

- Alle nachfolgenden Felder rücken automatisch eine Zeile nach oben
- Die nachfolgenden Felder bleiben an ihrer alten Position
- Bei jedem Löschen wird nachgefragt, ob die Felder nachrücken sollen

<p class="just-emphasize">Maskenfelder</p>

| Feld | Bedeutung |
| --- | --- |
| Feldgruppe | Folgende Gruppen stehen zur Verfügung<br>1. Strecke<br>2. Text1<br>3. Umbuchung<br>4. Vorgang |
| Bed.Klasse | In diesem Feld wird die Bedienerklasse hinterlegt, welche die zugordneten Userfelder bei der Vorgangserfassung sehen darf. |
| Vorg.Klasse | Folgende UFLD Felder werden bei dieser Vorgangsklasse angezeigt |
| Unterklasse | Folgende UFLD Felder werden bei dieser Vorgangsunterklasse angezeigt |

<p class="just-emphasize">Gridbeschreibung</p>

| Bezeichnung | Bedeutung |
| --- | --- |
| Bezeichnung | In diesem Feld steht die Bezeichnung des UFLD Feldes |
| Feld | ID Nummer des UFLD Feldes |
| Schnellerfassung | Zur Auswahl stehen<br>1. Ja<br>2. Nein |
| Erfassungslevel | Hier kann angeben werden, wann UFLD Feld auf der Maske nagezeigt werden soll.<br>Zur Auswahl stehen<br>• Ersterfassung<br>• Neuerfassung mit Positionsteil<br>• Belegkorrektur<br>• Keine Änderung |
| Folgetaste | Hier kann eine Folgetaste hinterlegt werden, die nach der Eingabe des UFLD Feldes ausgeführt werden soll.<br>z.B. F5 in der SVMAIN für den Positionsteil |
| Reihenfolge | Für die Sortierung im Grid |
| Zeile-Bezeichnung. | In welcher Zeile soll die Bezeichnung stehen |
| Spalte-Bezeichnung | In welcher Spalte soll die Bezeichnung stehen |
| Schriftgröße | Schriftgröße für die Bezeichnung des Feldes |
| Länge-Bezeichnung | Die Länge des Feldes für die Bezeichnung |
| Zeile-Wert | In welcher Zeile soll das UFLD Feld stehen |
| Spalte- Wert | In welcher Spalte soll das UFLD Feld stehen |
| Schriftgröße | Schriftgröße des UFLD Feldes |
| Länge- Wert | Die Länge des UFLD Feldes |
| Zeile-Beschriftung | In welcher Zeile soll die Beschriftung stehen |
| Spalte-Beschriftung | In welcher Spalte soll die Beschriftung stehen |
| Schriftgröße | Schriftgröße für die Beschriftung des Feldes |
| Länge-Beschriftung | Die Länge des Feldes für die Beschriftung |
| Aktualisiere Maske | Wenn der Wert auf Ja steht wird nach Eingabe eines Wertes in dem Feld das AIS auf der Maske aktualisiert. |

<p class="just-emphasize">Vorgang</p>

| Nr. | ID | Name | Select-Statement für ein SQLK | Itembox auf dem Feld | Selected Text | Selected Variable |
| --- | --- | --- | --- | --- | --- | --- |
| 17 | ID_SPRACHE | Sprache | <code>select vs.Sprachnummer from amic_v_vorgaenge vs where v_id = :ID_V_ID</code> | IB_Sprache | cssc_sprachestamm | SprachBezeich |
| 108 | ID_VERSANDADRESSID | VersandAdr. | <code>select vs.V_VersAdressId from amic_v_vorgaenge vs where v_id = :ID_V_ID</code> | IB_VERS_ANSCHRIFT_ZUM_KUNDEN<br> | | |
| 110 | ID_V_WIEGENUMMER | Wiegenummer | <code>select vs.V_WiegeNummer from amic_v_vorgaenge vs where v_id = :ID_V_ID</code> | | | |
| 117 | ID_DAUER_ANFDAT | DA Anfang | <code>select vs.V_DauerAnfDat from amic_v_vorgaenge vs where v_id = :ID_V_ID</code> | | | |
| 118 | ID_DAUER_NAEDAT | DA nächtst.Termin | <code>select vs.V_DauerNaeDat from amic_v_vorgaenge vs where v_id = :ID_V_ID</code> | | | |
| 119 | ID_DAUER_PERIODE | DA Periode | <code>select vs.V_DauerPeriode from amic_v_vorgaenge vs where v_id = :ID_V_ID</code> | | | |
| 122 | ID_DAUER_ENDDAT | DA Ende | <code>Select vs.v_DauerEndDat from amic_v_vorgaenge vs where v_id = :ID_V_ID</code> | | | |
| 150 | ID_SPERRUMWANDLUNG | Umwandl.Sperre | <code>select vs.V_SperrUmwand from amic_v_vorgaenge vs where v_id = :ID_V_ID</code> | | | |
| 175 | ID_EDI_KU_AUFTRAGS_DATUM | KuAuftrDatum | <code>select vs.V_EDIKuAuftragsDatum from amic_v_vorgaenge vs where v_id = :ID_V_ID</code> | | | |
| 176 | ID_EDI_KU_AUFTRAGS_NUMMER | KuAuftrNummer | <code>select vs.V_EDIKuAuftragsNummer from amic_v_vorgaenge vs where v_id = :ID_V_ID</code> | | | |
| 404 | ID_KENNZ_DRUCK_KOKORE | Einzelz Kokore | <code>select vs.V_KennzDrRechKokore from amic_v_vorgaenge vs where v_id = :ID_V_ID</code> | | | |
| 427 | ID_ABW_ROHW_SORTE | AbwRohwSorte | <pre><code>Select vb.v_AbwRohwSorte from VorgBemerkung vb&#10;join amic_v_vorgaenge vs on (vb.V_BemId = vs.V_BemId)&#10;where vs.v_id = :ID_V_ID</code></pre> | ib_rohwaresorte<br> | ssc_rohwareSorteNummer | RohSorteBezeich |
| 428 | ID_ABW_ROHW_KUNDNUMMER | AbwRohwKunde | <pre><code>Select ku.KundNummer from Kundenstamm ku&#10;join VorgBemerkung vb on ( ku.KundId = vb.V_AbwRohKundId )&#10;join amic_v_vorgaenge vs on (vb.V_BemId = vs.V_BemId)&#10;where vs.v_id = :ID_V_ID</code></pre> | IB_KU<br> | SSC_KundenStamm | Kundbezeich |
| 430 | ID_ROHWAREVORERFASSUNG | RohwVorerfass | <pre><code>Select vb.v_RW_VorErfassung from VorgBemerkung vb&#10;join amic_v_vorgaenge vs on (vb.V_BemId = vs.V_BemId)&#10;where vs.v_id = :ID_V_ID</code></pre> | | | |
| 435 | ID_ARBEITSREGEL | Arbeitsregel | <code>select vs.ArbeitsRegel from amic_v_vorgaenge vs where v_id = :ID_V_ID</code> | IB_WORKFLOW<br> | ssc_workflowbezeich | wfs_name |
| 450 | ID_ABW_ROHW_LAGERKENNZID | Lager ändern | <pre><code>Select vb.v_AbwLagerKennz from VorgBemerkung vb&#10;join amic_v_vorgaenge vs on (vb.V_BemId = vs.V_BemId)&#10;where vs.v_id = :ID_V_ID</code></pre> | | | |
| 451 | ID_ABW_ROHW_LAGERID | Lagernr.neu | <pre><code>Select vb.v_AbwLagerNummer from VorgBemerkung vb&#10;join amic_v_vorgaenge vs on (vb.V_BemId = vs.V_BemId)&#10;where vs.v_id = :ID_V_ID</code></pre> | IB_LagerStamm | cssc_lagerstamm | LagerBezeich |
| 452 | ID_ABW_ROHW_LAGERPLZID | Lagerpl.neu | <pre><code>Select vb.v_AbwLagerPlatz from VorgBemerkung vb&#10;join amic_v_vorgaenge vs on (vb.V_BemId = vs.V_BemId)&#10;where vs.v_id = :ID_V_ID</code></pre> | IB_LagerPlatz | | |
| 456 | ID_TRANSPORTWEG | Transportweg | <code>select vs.V_TransportWeg from amic_v_vorgaenge vs where v_id = :ID_V_ID</code> | | | |
| 457 | ID_VERFAHRENSART | Verfahrensart | <code>select vs.V_VerfahrensArt from amic_v_vorgaenge vs where v_id = :ID_V_ID</code> | | | |
| 470 | ID_HERKUNFTZIELLAND | Vorbel. Ziel Herkunft Land | <code>select vs.HerkunftZielLand from amic_v_vorgaenge vs where v_id = :ID_V_ID</code> | IB_StaatStammBezeich | CSSC_HerkunftsLand | Staatbezeich |
| 471 | ID_HERKUNFTZIELREGION | Vorbel. Region | <code>select vs.HerkunftZielRegion from amic_v_vorgaenge vs where v_id = :ID_V_ID</code> | | | |
| 500 | ID_KLAMMERNR | Klammernr | <code>select vs.V_Klammernr from amic_v_vorgaenge vs where v_id = :ID_V_ID</code> | IB_Klammernummer | | |
| 502 | ID_KLAMMERTYP | Klammertyp | <code>select vs.V_Klammertyp from amic_v_vorgaenge vs where v_id = :ID_V_ID</code> | | | |
| 504 | ID_USTID_KUNDE | UST-ID Kunde | <code>select vs.UstId_Kunde from amic_v_vorgaenge vs where v_id = :ID_V_ID</code> | IB_USTID_KUNDE | | |
| 505 | ID_USTID_MANDANT | UST-ID Firma | <code>select vs.UstId_Mandant from amic_v_vorgaenge vs where v_id = :ID_V_ID</code> | IB_USTID_MANDANT | | |
| 547 | ID_INFOADRESSID | informelle Anschrift | <code>select vs.v_AdressIdInfo from amic_v_vorgaenge vs where v_id = :ID_V_ID</code> | IB_Anschrift | | |
| 577 | ID_SHIPFROM | ShipFrom | <code>select vs.v_ShipFrom from amic_v_vorgaenge vs where v_id = :ID_V_ID</code> | IB_STAATSTAMM | CSSC_STAATSTAMM | StaatBezeich |
| 578 | ID_SHIPTO | ShipTo | <code>select vs.v_ShipTo from amic_v_vorgaenge vs where v_id = :ID_V_ID</code> | IB_STAATSTAMM | CSSC_STAATSTAMM | StaatBezeich |
| 1034 | ID_VERSANDARTID | Versandart | <code>select vs.VersArtId from amic_v_vorgaenge vs where v_id = :ID_V_ID</code> | IB_VERSANDART | CSSC_VERSANDART | VersartBezeich |
| 1038 | ID_STEUERGRNUMMER | SteuerGr. | <code>select vs.SteuerGrNummer from amic_v_vorgaenge vs where v_id = :ID_V_ID</code> | IB_SteuerGruppe | cssc_steuergruppe | SteuerGrBezeich |
| 1059 | ID_ZUABKLASSNUMMERINDI | Ind_Zuab_Klasse | <code>select vs.ZuAbKlassNummerI from amic_v_vorgaenge vs where v_id = :ID_V_ID</code> | ib_ZuAbschKlasse | cssc_zuabschklasse | zuabklassbezeich |
| 1062 | ID_VERTGRNUMMER | Vertr.Gruppe | <code>select vs.VertGrNummer from amic_v_vorgaenge vs where v_id = :ID_V_ID</code> | IB_VERTRETERGRUPPE | CSSC_VERTRETERGRUPPE | VertGrBezeich |
| 1063 | ID_PREISKLNUMMER | LiPreiKl | <code>select vs.PreisKlNummer from amic_v_vorgaenge vs where v_id = :ID_V_ID</code> | ib_preisklassen | cssc_preisklassen | PreisKlBezeich |
| 1065 | ID_RABKLASSNUMMER | Rabattklasse | <code>select vs.RabKlassNummer from amic_v_vorgaenge vs where v_id = :ID_V_ID</code> | ib_RabattKlasse | CSSC_RabattKlasse | rabklassbezeich |
| 1066 | ID_ZUABKLASSNUMMER | ZuAB_Klasse | <code>select vs.ZuAbKlassNummer from amic_v_vorgaenge vs where v_id = :ID_V_ID</code> | ib_ZuAbschKlasse | cssc_zuabschklasse | zuabklassbezeich |
| 1067 | ID_FRAKLASSNUMMER | Frachtkl. | <code>select vs.FraKlassNummer from amic_v_vorgaenge vs where v_id = :ID_V_ID</code> | ib_FrachtKlasse | SSC_FraKlassBezeich | FraKlassBezeich |
| 1089 | ID_ABTEILUNG | Abteilung | <code>select vs.AbteilungId from amic_v_vorgaenge vs where v_id = :ID_V_ID</code> | IB_ABTEILUNG | CSSC_ABTEILUNG | Abteilungbezeich |
| 1095 | ID_FRACHTVARIANTE | Frachtvar. | <code>select vs.FraVariNummer from amic_v_vorgaenge vs where v_id = :ID_V_ID</code> | IB_FRACHTVARIANTE | cssc_FrachtVariante | FraVariBezeich |
| 1096 | ID_VERKAUFSGEBIET | VerkaufsGeb. | <code>select vs.VerkGebNummer from amic_v_vorgaenge vs where v_id = :ID_V_ID</code> | IB_VERKAUFSGEBIET | cssc_Verkaufsgebiet | VerkGebBezeich |
| 1097 | ID_GEBIET_VON | Gebiet von | <code>select vs.GebietNummerVon from amic_v_vorgaenge vs where v_id = :ID_V_ID</code> | IB_GEBIETSTAMM | cssc_Gebietstamm | GebietBezeich |
| 1098 | ID_GEBIET_NACH | Gebiet nach | <code>select vs.GebietNummerNach from amic_v_vorgaenge vs where v_id = :ID_V_ID</code> | IB_GEBIETSTAMM | cssc_Gebietstamm | GebietBezeich |
| 1099 | ID_LKW_NUMMER_MOTOR | LKW Nr.Motor | <code>select vs.LKW_NummerMotor from amic_v_vorgaenge vs where v_id = :ID_V_ID</code> | IB_LKW_MOTOR | CSSC_LKW | LKW_Bezeich |
| 1100 | ID_LKW_NUMMER_ANHAENGER | LKW Anhänger | <code>select vs.LKW_NummerAnhaen from amic_v_vorgaenge vs where v_id = :ID_V_ID</code> | IB_LKW_ANHAENGER | CSSC_LKW | LKW_BEZEICH |
| 1101 | ID_FAHRER | Fahrer | <code>select vs.FahrerNummer from amic_v_vorgaenge vs where v_id = :ID_V_ID</code> | IB_FAHRER | CSSC_FAHRER | FAHRERBEZEICH |
| 1155 | ID_FILIALE | Filiale | <code>select vs.FilialNummer from amic_v_vorgaenge vs where v_id = :ID_V_ID</code> | ib_filiale | CSSC_Filiale | FilialBezeich |
| 1156 | ID_ZENTRALE | Zentrale | <code>select vs.FilialNummerZen from amic_v_vorgaenge vs where v_id = :ID_V_ID</code> | ib_zentrale | CSSC_ZENTRALE | FilialBezeich |
| 1205 | ID_REFERENZNUMMER | Referenz.Nr | <code>select vs.V_ReferenzNummer from amic_v_vorgaenge vs where v_id = :ID_V_ID</code> | | | |
| 1207 | ID_FAKTURIERGRUPPE | Fakt.Grp. | <code>select vs.FaktGrupNummer from amic_v_vorgaenge vs where v_id = :ID_V_ID</code> | ib_faktgruppe | CSSC_FAKTURIERG | FaktGrupBezeich |
| 1231 | ID_V_DATUM_3 | Lieferdatum | <code>select vs.V_DatumStufe3 from amic_v_vorgaenge vs where v_id = :ID_V_ID</code> | | | |
| 1240 | ID_BONITAET | Bonitaet | <code>select ku.KundBonitaet from kundenstamm ku join amic_v_vorgaenge vs on (vs.KundIdZuOrd = ku.kundId) where vs.v_id = :ID_V_ID</code> | | | |
| 1335 | ID_V_DATUM_PREISE | Preisdatum | <code>select vs.V_DatumPreise from amic_v_vorgaenge vs where v_id = :ID_V_ID</code> | | | |
| 1345 | ID_V_DATUM_PLAN_3 | Plan/Liefdat | <pre><code>select &#10;( case&#10; when v_klassnummer in (700,790,800,890,1700,1790,1800,1890) &#10; then v_datum &#10; when v_klassnummer in (5100,5110,5120) then (select if&#10; vu.V_LGUBuchTyp = 2 then vs.v_datum else vs.v_plandatum&#10; endif as v_datum from VorgStammUmbuch vu where vu.v_id = &#10; vs.v_id)&#10; when v_klassnummer = 5220 then (select if V_ProdBuchTyp = 2&#10; then vs.v_datum else v_plandatum endif as v_datum from&#10; V_ProdVorgang vpv where vpv.v_id = vs.v_id)&#10; else v_PlanDatum&#10; END )&#10; as Datum from amic_v_vorgaenge vs</code></pre> | | | |
| 1354 | ID_USTKENNZEICHEN | USt-Ident. | <code>select vs.UstId_Kunde from amic_v_vorgaenge vs where v_id = :ID_V_ID</code> | | | |
| 1426 | ID_TOURNUMMER | Tournummer | <code>select vs.TourId from amic_v_vorgaenge vs where v_id = :ID_V_ID</code> | IB_TOURKUNDE | | |
| 1453 | ID_FW_NUMMER | Währung Nr | <code>Select vw.waehrnummer from V_WAEHRUNG as vw where vw.v_id = :ID_V_ID</code> | IB_Waehrung | cssc_waehrung | WaehrBezeich |
| 1456 | ID_FW_KURS | Kurs | <code>select vw.v_WaehrKurs from V_Waehrung vw where V_Id = :ID_V_ID</code> | | | |
| 1463 | ID_BRUTTOKENNZEICHEN | Bruttovorg. | <code>select vs.V_KennzBrutto from amic_v_vorgaenge vs where v_id = :ID_V_ID</code> | | | |
| 1510 | ID_LAGERNUMMER_FEHL | LagerNr. | <code>select vs.LagerNummerFehl from amic_v_vorgaenge vs where v_id = :ID_V_ID</code> | ib_lagerstamm | cssc_lagerstamm | LagerBezeich |
| 1531 | ID_LAGERPLATZ_FEHL | Lagerplatz | <code>select vs.LagerPlatzFehl from amic_v_vorgaenge vs where v_id = :ID_V_ID</code> | IB_LagerPlatz | | |
| 1766 | ID_PRUEFMENGE | Prüfmenge | <code>Wird nur bei der Erfassung von Vorgängen benötigt und kann mit keinem SQLK abgefragt werden.</code> | | | |
| 1767 | ID_LIEFERWOCHE | Lieferwoche | <code>select vs.V_LieferWoche from amic_v_vorgaenge vs where v_id = :ID_V_ID</code> | | | |
| 1771 | ID_V_BEMTEXT1 | Text1 | <pre><code>select vb.V_BemText1 from VorgBemerkung vb&#10;join amic_v_vorgaenge vs on (vb.V_BemId = vs.V_BemId)&#10;where vs.v_id = :ID_V_ID</code></pre> | | | |
| 1772 | ID_V_BEMTEXT2 | Text2 | <pre><code>select vb.V_BemText2 from VorgBemerkung vb&#10;join amic_v_vorgaenge vs on (vb.V_BemId = vs.V_BemId)&#10;where vs.v_id = :ID_V_ID</code></pre> | | | |
| 1773 | ID_V_BEMTEXT3 | Text3 | <pre><code>select vb.V_BemText3 from VorgBemerkung vb&#10;join amic_v_vorgaenge vs on (vb.V_BemId = vs.V_BemId)&#10;where vs.v_id = :ID_V_ID</code></pre> | | | |
| 1774 | ID_V_BEMTEXT4 | Text4 | <pre><code>select vb.V_BemText4 from VorgBemerkung vb&#10;join amic_v_vorgaenge vs on (vb.V_BemId = vs.V_BemId)&#10;where vs.v_id = :ID_V_ID</code></pre> | | | |
| 1775 | ID_V_BEMTEXT5 | Text5 | <pre><code>select vb.V_BemText5 from VorgBemerkung vb&#10;join amic_v_vorgaenge vs on (vb.V_BemId = vs.V_BemId)&#10;where vs.v_id = :ID_V_ID</code></pre> | | | |
| 1776 | ID_V_BEMLANGTEXT1 | Langtext1 | <pre><code>select vb.V_BemLangText1 from VorgBemerkung vb&#10;join amic_v_vorgaenge vs on (vb.V_BemId = vs.V_BemId)&#10;where vs.v_id = :ID_V_ID</code></pre> | | | |
| 1824 | ID_MENGE_FIKTIV | Fiktiv-Menge | <code>select vs.V_MengeFiktiv from amic_v_vorgaenge vs where v_id = :ID_V_ID</code> | | | |
| 1825 | ID_ME_NUMMER_FIKTIV | Mengeneinheit | <code>select vs.ME_NummerFiktMen from amic_v_vorgaenge vs where v_id = :ID_V_ID</code> | IB_MengeneinheitPassend_0 | | ME_Kurztext |
| 1828 | ID_PARITAETNUMMER | Parität | <code>select vs.ParitaetNummer from amic_v_vorgaenge vs where v_id = :ID_V_ID</code> | IB_ParitaetStamm | CSSC_ParitaetStamm | ParitaetBezeich |
| 1851 | ID_AKTOBJEKT_NUMMER | Objekte | <pre><code>Select bs.Baustnummer from Bauststamm bs&#10;join amic_v_vorgaenge vs on (vs.BaustId = bs.BaustId)&#10;where vs.v_id =:ID_V_ID</code></pre> | IB_BAUSTSTAMM_KU | cssc_baustelle | BaustBezeich |
| 1894 | ID_KUNDNUMMER_RECHNUNGSEMPF | Rechnungsempf | <code>select vs.KundIdRechn from amic_v_vorgaenge vs where v_id = :ID_V_ID</code> | IB_KU | SSC_KundenStamm_OberKunde | KundBezeich |
| 1895 | ID_KUNDNUMMER_ZAHLUNGSPFL | Zahlungspfl. | <code>select vs.KundIdFinanz from amic_v_vorgaenge vs where v_id = :ID_V_ID</code> | IB_KU | SSC_KundenStamm_OberKunde | KundBezeich |
| 1899 | ID_KUNDNUMMER_KONTRAKTKUNDE | Kontraktkunde | <code>select vs.KundIdKontrakt from amic_v_vorgaenge vs where v_id = :ID_V_ID</code> | IB_KU | SSC_KundenStamm_OberKunde | KundBezeich |
| 1914 | ID_FA_BELEGREFERENZ | Beleg-Referenz | <code>select vs.FA_BelegReferenz from amic_v_vorgaenge vs where v_id = :ID_V_ID</code> | | | |
| 1972 | ID_KENNZBARVERK | Barverkauf | <code>select vs.V_KennzBarverk from amic_v_vorgaenge vs where v_id = :ID_V_ID</code> | | | |
| 1974 | ID_KASSENNUMMER | Kasse | <code>Select vk.Kassennummer from v_kasseninfo where v_id = :ID_V_ID</code> | IB_Kassensysteme | | |
| 1975 | ID_KASSENKONTO | KassenKonto | <code>Select vk.V_KassKonto from v_kasseninfo where v_id = :ID_V_ID</code> | IB_SACHKONTO | | |
| 1976 | ID_SPERRFIBU | FibuSperre | <code>select vs.V_SperrFibu from amic_v_vorgaenge vs where v_id = :ID_V_ID</code> | | | |
| 1977 | ID_SPERRFILIA | FiliaSperre | <code>select vs.V_SperrFilia from amic_v_vorgaenge vs where v_id = :ID_V_ID</code> | | | |
| 1978 | ID_SPERRREBUCH | RebuchSperre | <code>select vs.V_SperrReBuch from amic_v_vorgaenge vs where v_id = :ID_V_ID</code> | | | |
| 1985 | ID_LIEFERZEITNUMMER | Lieferzeit | <pre><code>select vta.TransLizNummer from VorgTRansAuftrag vta&#10;join amic_v_vorgaenge vs on ( vta.V_TransId = vs.V_TransId )&#10;where vs.v_id = :ID_V_ID</code></pre> | IB_TRANSLIEFERZEIT | ssc_lieferzeit | translizbezeich |
| 1986 | ID_LADEZEITNUMMER | Ladezeit | <pre><code>select vta.TransLazNummer from VorgTRansAuftrag vta&#10;join amic_v_vorgaenge vs on ( vta.V_TransId = vs.V_TransId )&#10;where vs.v_id = :ID_V_ID</code></pre> | IB_TRANSLADEZEIT | ssc_Ladezeit | translazbezeich |
| 1991 | ID_PRUEFSUMME | Prüfsumme Vorgang | <code>Wird nur bei der Erfassung von Vorgängen benötigt und kann mit keinem SQLK abgefragt werden.</code> | | | |
| 1992 | ID_LADEDATUM | LadeDatum | <pre><code>select vta.V_TransLadeDatum from VorgTRansAuftrag vta&#10;join amic_v_vorgaenge vs on ( vta.V_TransId = vs.V_TransId )&#10;where vs.v_id = :ID_V_ID</code></pre> | | | |
| 1993 | ID_LADEORTNUMMER | Ladeort | <pre><code>select LagerNummerLade from VorgTRansAuftrag vta&#10;join amic_v_vorgaenge vs on ( vta.V_TransId = vs.V_TransId )&#10;where vs.v_id = :ID_V_ID</code></pre> | IB_LagerStamm | cssc_lagerstamm | lagerbezeich |
| 1996 | ID_GLZUAB_KLASSE1 | ZuAbschlKl.1 | <pre><code>select vgz.V_GlZuAbKlasse1 from VorgGlobalZuAb vgz&#10;join amic_v_vorgaenge vs on (vgz.V_GlZuAbId = vs.V_GlZuAbId )&#10;where vs.v_id :ID_V_ID</code></pre> | ib_ZuAbschKlasse | cssc_zuabschklasse | ZuAbKlassBezeich |
| 1997 | ID_GLZUAB_KLASSE2 | ZuAbschlKl.2 | <pre><code>select vgz.V_GlZuAbKlasse2 from VorgGlobalZuAb vgz&#10;join amic_v_vorgaenge vs on (vgz.V_GlZuAbId = vs.V_GlZuAbId )&#10;where vs.v_id :ID_V_ID</code></pre> | ib_ZuAbschKlasse | cssc_zuabschklasse | ZuAbKlassBezeich |
| 1998 | ID_GLZUAB_WERT1 | ZuAbWert 1 | <pre><code>select vgz.V_GlZuAbWert1 from VorgGlobalZuAb vgz&#10;join amic_v_vorgaenge vs on (vgz.V_GlZuAbId = vs.V_GlZuAbId )&#10;where vs.v_id :ID_V_ID</code></pre> | | | |
| 1999 | ID_GLZUAB_WERT2 | ZuAbWert 2 | <pre><code>select vgz.V_GlZuAbWert2 from VorgGlobalZuAb vgz&#10;join amic_v_vorgaenge vs on (vgz.V_GlZuAbId = vs.V_GlZuAbId )&#10;where vs.v_id :ID_V_ID</code></pre> | | | |
| 2216 | ID_GELANGENSBEST | Gelangensbestätigung | <code>select vs.V_GelangensBesrtJN from amic_v_vorgaenge vs where v_id = :ID_V_ID</code> | | | |
| 2300 | ID_ZAHLARTID | Zahl.Art | <code>select vs.ZahlArtId from amic_v_vorgaenge vs where v_id = :ID_V_ID</code> | ib_zahlungsart | CSSC_ZAHLUNGSART | ZahlArtBezeich |
| 4200 | ID_PROD_KUNDNUMMER | Prod.KuNr. | <code>select pv.V_ProdKundNummer from V_ProdVorgang pv where v_id = :ID_V_ID</code> | IB_KU | SSC_KundenStamm | Kundbezeich |
| 4204 | ID_PROD_BUCHTYP | Prod.Buchtyp | <code>select pv.V_ProdBuchTyp from V_ProdVorgang pv where v_id = :ID_V_ID</code> | | | |
| 8268 | ID_V_DATUMEINGANG | Eingangsdatum | | | | |
| 8269 | ID_ZAHLUNGSREFERENZ | Zahlungsreferenz | | | | |

<p class="just-emphasize">Umbuchung</p>

| Nr | ID | Name | Select-Statement für ein SQLK | Itembox auf dem Feld | Selected Text | Selected Variable |
| --- | --- | --- | --- | --- | --- | --- |
| 108 | ID_VERSANDADRESSID | VersandAdr. | <code>select vs.V_VersAdressId from amic_v_vorgaenge vs where v_id = :ID_V_ID</code> | IB_VERS_ANSCHRIFT_ZUM_KUNDEN | | |
| 150 | ID_SPERRUMWANDLUNG | Umwandl.Sperre | <code>select vs.V_SperrUmwand from amic_v_vorgaenge vs where v_id = :ID_V_ID</code> | | | |
| 435 | ID_ARBEITSREGEL | Arbeitsregel | <code>select vs.ArbeitsRegel from amic_v_vorgaenge vs where v_id = :ID_V_ID</code> | IB_WORKFLOW | ssc_workflowbezeich | wfs_name |
| 504 | ID_USTID_KUNDE | UST-ID Kunde | <code>select vs.UstId_Kunde from amic_v_vorgaenge vs where v_id = :ID_V_ID</code> | IB_USTID_KUNDE | | |
| 505 | ID_USTID_MANDANT | UST-ID Firma | <code>select vs.UstId_Mandant from amic_v_vorgaenge vs where v_id = :ID_V_ID</code> | IB_USTID_MANDANT | | |
| 1034 | ID_VERSANDARTID | Versandart | <code>select vs.VersArtId from amic_v_vorgaenge vs where v_id = :ID_V_ID</code> | IB_VERSANDART | CSSC_VERSANDART | VersartBezeich |
| 1089 | ID_ABTEILUNG | Abteilung | <code>select vs.AbteilungId from amic_v_vorgaenge vs where v_id = :ID_V_ID</code> | IB_ABTEILUNG | CSSC_ABTEILUNG | Abteilungbezeich |
| 1099 | ID_LKW_NUMMER_MOTOR | LKW Nr.Motor | <code>select vs.LKW_NummerMotor from amic_v_vorgaenge vs where v_id = :ID_V_ID</code> | IB_LKW_MOTOR | CSSC_LKW | LKW_Bezeich |
| 1100 | ID_LKW_NUMMER_ANHAENGER | LKW Anhänger | <code>select vs.LKW_NummerAnhaen from amic_v_vorgaenge vs where v_id = :ID_V_ID</code> | IB_LKW_ANHAENGER | CSSC_LKW | LKW_BEZEICH |
| 1101 | ID_FAHRER | Fahrer | <code>select vs.FahrerNummer from amic_v_vorgaenge vs where v_id = :ID_V_ID</code> | IB_FAHRER | CSSC_FAHRER | FAHRERBEZEICH |
| 1155 | ID_FILIALE | Filiale | <code>select vs.FilialNummer from amic_v_vorgaenge vs where v_id = :ID_V_ID</code> | ib_filiale | CSSC_Filiale | FilialBezeich |
| 1156 | ID_ZENTRALE | Zentrale | <code>select vs.FilialNummerZen from amic_v_vorgaenge vs where v_id = :ID_V_ID</code> | ib_zentrale | CSSC_ZENTRALE | FilialBezeich |
| 1205 | ID_REFERENZNUMMER | Referenz.Nr | <code>select vs.V_ReferenzNummer from amic_v_vorgaenge vs where v_id = :ID_V_ID</code> | | | |
| 1345 | ID_V_DATUM_PLAN_3 | Plan/Liefdat | <pre><code>select &#10;( case&#10; when v_klassnummer in (700,790,800,890,1700,1790,1800,1890) &#10; then v_datum &#10; when v_klassnummer in (5100,5110,5120) then (select if&#10; vu.V_LGUBuchTyp = 2 then vs.v_datum else vs.v_plandatum&#10; endif as v_datum from VorgStammUmbuch vu where vu.v_id = &#10; vs.v_id)&#10; when v_klassnummer = 5220 then (select if V_ProdBuchTyp = 2&#10; then vs.v_datum else v_plandatum endif as v_datum from&#10; V_ProdVorgang vpv where vpv.v_id = vs.v_id)&#10; else v_PlanDatum&#10; END )&#10; as Datum from amic_v_vorgaenge vs</code></pre> | | | |
| 1510 | ID_LAGERNUMMER_FEHL | LagerNr. | <code>select vs.LagerNummerFehl from amic_v_vorgaenge vs where v_id = :ID_V_ID</code> | ib_lagerstamm | cssc_lagerstamm | LagerBezeich |
| 1771 | ID_V_BEMTEXT1 | Text1 | <pre><code>select vb.V_BemText1 from VorgBemerkung vb&#10;join amic_v_vorgaenge vs on (vb.V_BemId = vs.V_BemId)&#10;where vs.v_id = :ID_V_ID</code></pre> | | | |
| 1772 | ID_V_BEMTEXT2 | Text2 | <pre><code>select vb.V_BemText2 from VorgBemerkung vb&#10;join amic_v_vorgaenge vs on (vb.V_BemId = vs.V_BemId)&#10;where vs.v_id = :ID_V_ID</code></pre> | | | |
| 1773 | ID_V_BEMTEXT3 | Text3 | <pre><code>select vb.V_BemText3 from VorgBemerkung vb&#10;join amic_v_vorgaenge vs on (vb.V_BemId = vs.V_BemId)&#10;where vs.v_id = :ID_V_ID</code></pre> | | | |
| 1774 | ID_V_BEMTEXT4 | Text4 | <pre><code>select vb.V_BemText4 from VorgBemerkung vb&#10;join amic_v_vorgaenge vs on (vb.V_BemId = vs.V_BemId)&#10;where vs.v_id = :ID_V_ID</code></pre> | | | |
| 1775 | ID_V_BEMTEXT5 | Text5 | <pre><code>select vb.V_BemText5 from VorgBemerkung vb&#10;join amic_v_vorgaenge vs on (vb.V_BemId = vs.V_BemId)&#10;where vs.v_id = :ID_V_ID</code></pre> | | | |
| 1824 | ID_MENGE_FIKTIV | Fiktiv-Menge | <code>select vs.V_MengeFiktiv from amic_v_vorgaenge vs where v_id = :ID_V_ID</code> | | | |
| 1825 | ID_ME_NUMMER_FIKTIV | Mengeneinheit | <code>select vs.ME_NummerFiktMen from amic_v_vorgaenge vs where v_id = :ID_V_ID</code> | IB_MengeneinheitPassend_0 | | ME_Kurztext |
| 1828 | ID_PARITAETNUMMER | Parität | <code>select vs.ParitaetNummer from amic_v_vorgaenge vs where v_id = :ID_V_ID</code> | IB_ParitaetStamm | CSSC_ParitaetStamm | ParitaetBezeich |
| 1914 | ID_FA_BELEGREFERENZ | Beleg-Referenz | <code>select vs.FA_BelegReferenz from amic_v_vorgaenge vs where v_id = :ID_V_ID</code> | | | |
| 1976 | ID_SPERRFIBU | FibuSperre | <code>select vs.V_SperrFibu from amic_v_vorgaenge vs where v_id = :ID_V_ID</code> | | | |
| 1977 | ID_SPERRFILIA | FiliaSperre | <code>select vs.V_SperrFilia from amic_v_vorgaenge vs where v_id = :ID_V_ID</code> | | | |
| 1978 | ID_SPERRREBUCH | RebuchSperre | <code>select vs.V_SperrReBuch from amic_v_vorgaenge vs where v_id = :ID_V_ID</code> | | | |
| 1985 | ID_LIEFERZEITNUMMER | Lieferzeit | <pre><code>select vta.TransLizNummer from VorgTRansAuftrag vta&#10;join amic_v_vorgaenge vs on ( vta.V_TransId = vs.V_TransId )&#10;where vs.v_id = :ID_V_ID</code></pre> | IB_TRANSLIEFERZEIT | ssc_lieferzeit | translizbezeich |
| 1986 | ID_LADEZEITNUMMER | Ladezeit | <pre><code>select vta.TransLazNummer from VorgTRansAuftrag vta&#10;join amic_v_vorgaenge vs on ( vta.V_TransId = vs.V_TransId )&#10;where vs.v_id = :ID_V_ID</code></pre> | IB_TRANSLADEZEIT | ssc_Ladezeit | translazbezeich |
| 1991 | ID_PRUEFSUMME | Prüfsumme Vorgang | <code>Wird nur bei der Erfassung von Vorgängen benötigt und kann mit keinem SQLK abgefragt werden.</code> | | | |
| 1992 | ID_LADEDATUM | LadeDatum | <pre><code>select vta.V_TransLadeDatum from VorgTRansAuftrag vta&#10;join amic_v_vorgaenge vs on ( vta.V_TransId = vs.V_TransId )&#10;where vs.v_id = :ID_V_ID</code></pre> | | | |
| 1993 | ID_LADEORTNUMMER | Ladeort | <pre><code>select LagerNummerLade from VorgTRansAuftrag vta&#10;join amic_v_vorgaenge vs on ( vta.V_TransId = vs.V_TransId )&#10;where vs.v_id = :ID_V_ID</code></pre> | IB_LagerStamm | cssc_lagerstamm | lagerbezeich |
| 4501 | ID_LGU_BUCHTYP_KZ | Umbuch.Typ | <code>select vsu.v_LGUBuchTyp from VorgStammUmbuch vsu where vsu.v_id = :ID_V_ID</code> | | | |

<p class="just-emphasize">Strecke</p>

| Nr | ID | Name | Select-Statement für ein SQLK | Itembox auf dem Feld | Selected Text | Selected Variable |
| --- | --- | --- | --- | --- | --- | --- |
| 1034 | ID_VERSANDARTID | Versandart | <code>select vs.VersArtId from amic_v_vorgaenge vs where v_id = :ID_V_ID</code> | IB_VERSANDART | CSSC_VERSANDART | VersArtBezeich |

<p class="just-emphasize">Text1</p>

| Nr | ID | Name | Select-Statement für ein SQLK | Itembox auf dem Feld | Selected Text | Selected Variable |
| --- | --- | --- | --- | --- | --- | --- |
| 1771 | ID_VERSANDARTID | Bemerktext1 | <pre><code>select vb.V_BemText1 from VorgBemerkung vb&#10;join amic_v_vorgaenge vs on (vb.V_BemId = vs.V_BemId)&#10;where vs.v_id = :ID_V_ID</code></pre> | IB_VERSANDART | CSSC_VERSANDART | VersArtBezeich |

<p class="just-emphasize">Produktion</p>

| Nr. | ID | Name | Select-Statement für ein SQLK | Itembox auf dem Feld | Selected Text | Selected Variable |
| --- | --- | --- | --- | --- | --- | --- |
| 17 | ID_SPRACHE | Sprache | <code>select vs.Sprachnummer from amic_v_vorgaenge vs where v_id = :ID_V_ID</code> | IB_Sprache | cssc_sprachestamm | SprachBezeich |
| 150 | ID_SPERRUMWANDLUNG | Umwandl.Sperre | <code>select vs.V_SperrUmwand from amic_v_vorgaenge vs where v_id = :ID_V_ID</code> | | | |
| 175 | ID_EDI_KU_AUFTRAGS_DATUM | KuAuftrDatum | <code>select vs.V_EDIKuAuftragsDatum from amic_v_vorgaenge vs where v_id = :ID_V_ID</code> | | | |
| 176 | ID_EDI_KU_AUFTRAGS_NUMMER | KuAuftrNummer | <code>select vs.V_EDIKuAuftragsNummer from amic_v_vorgaenge vs where v_id = :ID_V_ID</code> | | | |
| 435 | ID_ARBEITSREGEL | Arbeitsregel | <code>select vs.ArbeitsRegel from amic_v_vorgaenge vs where v_id = :ID_V_ID</code> | IB_WORKFLOW<br> | ssc_workflowbezeich | |
| 457 | ID_VERFAHRENSART | Verfahrensart | <code>select vs.V_VerfahrensArt from amic_v_vorgaenge vs where v_id = :ID_V_ID</code> | | | |
| 500 | ID_KLAMMERNR | Klammernr | <code>select vs.V_Klammernr from amic_v_vorgaenge vs where v_id = :ID_V_ID</code> | | | |
| 502 | ID_KLAMMERTYP | Klammertyp | <code>select vs.V_Klammertyp from amic_v_vorgaenge vs where v_id = :ID_V_ID</code> | | | |
| 1100 | ID_LKW_NUMMER_ANHAENGER | LKW Anhänger | <code>select vs.LKW_NummerAnhaen from amic_v_vorgaenge vs where v_id = :ID_V_ID</code> | | | |
| 1101 | ID_FAHRER | Fahrer | <code>select vs.FahrerNummer from amic_v_vorgaenge vs where v_id = :ID_V_ID</code> | | | |
| 1205 | ID_REFERENZNUMMER | Referenz.Nr | <code>select vs.V_ReferenzNummer from amic_v_vorgaenge vs where v_id = :ID_V_ID</code> | | | |
| 1335 | ID_V_DATUM_PREISE | Preisdatum | <code>select vs.V_DatumPreise from amic_v_vorgaenge vs where v_id = :ID_V_ID</code> | | | |
| 1345 | ID_V_DATUM_PLAN_3 | Plan/Liefdat | <pre><code>select &#10;( case&#10; when v_klassnummer in (700,790,800,890,1700,1790,1800,1890) &#10; then v_datum &#10; when v_klassnummer in (5100,5110,5120) then (select if&#10; vu.V_LGUBuchTyp = 2 then vs.v_datum else vs.v_plandatum&#10; endif as v_datum from VorgStammUmbuch vu where vu.v_id = &#10; vs.v_id)&#10; when v_klassnummer = 5220 then (select if V_ProdBuchTyp = 2&#10; then vs.v_datum else v_plandatum endif as v_datum from&#10; V_ProdVorgang vpv where vpv.v_id = vs.v_id)&#10; else v_PlanDatum&#10; END )&#10; as Datum from amic_v_vorgaenge vs</code></pre> | | | |
| 1510 | ID_LAGERNUMMER_FEHL | LagerNr. | <code>select vs.LagerNummerFehl from amic_v_vorgaenge vs where v_id = :ID_V_ID</code> | | | |
| 1531 | ID_LAGERPLATZ_FEHL | Lagerplatz | <code>select vs.LagerPlatzFehl from amic_v_vorgaenge vs where v_id = :ID_V_ID</code> | | | |
| 1766 | ID_PRUEFMENGE | Prüfmenge | <code>Wird nur bei der Erfassung von Vorgängen benötigt und kann mit keinem SQLK abgefragt werden.</code> | | | |
| 1767 | ID_LIEFERWOCHE | Lieferwoche | <code>select vs.V_LieferWoche from amic_v_vorgaenge vs where v_id = :ID_V_ID</code> | | | |
| 1771 | ID_V_BEMTEXT1 | Text1 | <pre><code>select vb.V_BemText1 from VorgBemerkung vb&#10;join amic_v_vorgaenge vs on (vb.V_BemId = vs.V_BemId)&#10;where vs.v_id = :ID_V_ID</code></pre> | | | |
| 1772 | ID_V_BEMTEXT2 | Text2 | <pre><code>select vb.V_BemText2 from VorgBemerkung vb&#10;join amic_v_vorgaenge vs on (vb.V_BemId = vs.V_BemId)&#10;where vs.v_id = :ID_V_ID</code></pre> | | | |
| 1773 | ID_V_BEMTEXT3 | Text3 | <pre><code>select vb.V_BemText3 from VorgBemerkung vb&#10;join amic_v_vorgaenge vs on (vb.V_BemId = vs.V_BemId)&#10;where vs.v_id = :ID_V_ID</code></pre> | | | |
| 1774 | ID_V_BEMTEXT4 | Text4 | <pre><code>select vb.V_BemText4 from VorgBemerkung vb&#10;join amic_v_vorgaenge vs on (vb.V_BemId = vs.V_BemId)&#10;where vs.v_id = :ID_V_ID</code></pre> | | | |
| 1775 | ID_V_BEMTEXT5 | Text5 | <pre><code>select vb.V_BemText5 from VorgBemerkung vb&#10;join amic_v_vorgaenge vs on (vb.V_BemId = vs.V_BemId)&#10;where vs.v_id = :ID_V_ID</code></pre> | | | |
| 1776 | ID_V_BEMLANGTEXT1 | Langtext1 | <pre><code>select vb.V_BemLangText1 from VorgBemerkung vb&#10;join amic_v_vorgaenge vs on (vb.V_BemId = vs.V_BemId)&#10;where vs.v_id = :ID_V_ID</code></pre> | | | |
| 1828 | ID_PARITAETNUMMER | Parität | <code>select vs.ParitaetNummer from amic_v_vorgaenge vs where v_id = :ID_V_ID</code> | | | |
| 1851 | ID_AKTOBJEKT_NUMMER | Objekte | <pre><code>Select bs.Baustnummer from Bauststamm bs&#10;join amic_v_vorgaenge vs on (vs.BaustId = bs.BaustId)&#10;where vs.v_id =:ID_V_ID</code></pre> | | | |
| 1914 | ID_FA_BELEGREFERENZ | Beleg-Referenz | <code>select vs.FA_BelegReferenz from amic_v_vorgaenge vs where v_id = :ID_V_ID</code> | | | |
| 1976 | ID_SPERRFIBU | FibuSperre | <code>select vs.V_SperrFibu from amic_v_vorgaenge vs where v_id = :ID_V_ID</code> | | | |
| 1991 | ID_PRUEFSUMME | Prüfsumme Vorgang | <code>Wird nur bei der Erfassung von Vorgängen benötigt und kann mit keinem SQLK abgefragt werden.</code> | | | |
| 4200 | ID_PROD_KUNDNUMMER | Prod.KuNr. | <code>select pv.V_ProdKundNummer from V_ProdVorgang pv where v_id = :ID_V_ID</code> | | | |
| 4204 | ID_PROD_BUCHTYP | Prod.Buchtyp | <code>select pv.V_ProdBuchTyp from V_ProdVorgang pv where v_id = :ID_V_ID</code> | | | |

<p class="just-emphasize">Felder des Vorgangs mit Weiterreichung an Unterblöcke/Warenpositionen</p>

Folgende Abfragefelder des Vorgangs werden sowohl im Kopfteil als auch in den Warenpositionen gespeichert:

| Nr. | ID | Name |
| --- | --- | --- |
| 1089 | ID_ABTEILUNG | Abteilung |
| 1096 | ID_VERKAUFSGEBIET | VerkaufsGeb. |
| 1097 | ID_GEBIET_VON | Gebiet von |
| 1098 | ID_GEBIET_NACH | Gebiet nach |
| 1099 | ID_LKW_NUMMER_MOTOR | LKW Nr.Motor |
| 1100 | ID_LKW_NUMMER_ANHAENGER | LKW Anhänger |
| 1101 | ID_FAHRER | Fahrer |
| 1155 | ID_FILIALE | Filiale |
| 1156 | ID_ZENTRALE | Zentrale |
| 2300 | ID_ZAHLARTID | Zahl.Art |
| 1062 | ID_VERTGRNUMMER | Vertr.Gruppe |
| 1038 | ID_STEUERGRNUMMER | SteuerGr. |
| 1207 | ID_FAKTURIERGRUPPE | Fakt.Grp. |
| 1034 | ID_VERSANDARTID | Versandart |
| 1828 | ID_PARITAETNUMMER | Parität |

Diese Felder aus dem Vorgangskopf werden als Vorbelegung bei der Neuerfassung einer Warenposition herangezogen.

Beim Ändern eines dieser Felder im Vorgangskopf werden alle Warenposition angepasst, deren Werte mit dem Wert im Vorgangskopf (vor der Änderung!) übereinstimmen.

Beispiel:

Im Vorgangskopf ist die Vertretergruppe 10 eingestellt, in der ersten Warenposition die Gruppe 20 zugeordnet. Die restlichen Warenpositionen sind auf Gruppe 10 gestellt. Wird die Vertretergruppe im Vorgangskopf von 10 auf 30 geändert, werden alle Warenpositionen mit Gruppe 10 ( identisch mit der Gruppe des Vorgangkopfes ) auf 30 geändert. Die Warenposition mit Gruppe 20 wird nicht verändert.

Bei einem Sammelbeleg (z.B. Sammelrechnung aus Lieferscheinen) werden auch alle untergeordneten Warenpositionen bei einer Änderung im Vorgangskopf angepasst.
