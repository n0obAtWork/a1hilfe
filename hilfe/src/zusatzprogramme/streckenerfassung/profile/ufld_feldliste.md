# UFLD-Feldliste

<!-- source: https://amic.de/hilfe/_vorgangsmappeProfile_UfldFeldliste.htm -->

Zum Laden der UFLD-Felder in der Streckenerfassungsmaske müssen die Felder nur im Profil eingetragen werden. Alle unter „[Verwendbare UFLD-Felder](./ufld_feldliste.md#verwendbareUFLDFelder)“ angegebenen Felder werden zurzeit automatisch mit geladen.

Sollen weitere UFLD-Felder in der Streckenerfassung angezeigt werden, so kann dies über eine private Prozedur geschehen. Die Felder müssen dabei nach folgendem Schema aufgebaut sein „ufld_“ + Nummer des UFLD-Feldes.

z.B. für das Feld 1101

```text
vs.FahrerNummer as ufld_1101
```

In diesem Fall hat das Feld 1101 auch eine Itembox, aus diesem Grund wird noch ein weiteres Feld auf der Streckenerfassungsmaske angelegt. Um dieses beim Laden der Daten auch zu befüllen muss die Ladeprozedur ein weiteres Feld zurückgeben, welches folgendermaßen aufgebaut ist „ufld_“ + Nummer des UFLD-Feldes + „_“ + Name des Returnfeldes.

z.B. für das Feld 1101

```text
fahrer.fahrerbezeich as ufld_1101_fahrerbezeich
```

#### Verwendbare UFLD-Felder

Hier ist eine Liste aller bisher verwendbaren UFLD-Felder. Der Alias „vs“ im Select-Statement wird für den Vorgangstamm verwendet. In der Spalte Itemboxfeld steht der Name des Returnfeldes der dazugehörigen Itembox.

| Nr. | Name | Select-Statement | Itemboxfeld |
| --- | --- | --- | --- |
| 17 | Sprache | vs.SprachNummer as ufld_17 | SprachBezeich |
| 108 | VersandAdr. | vs.V_VersAdressId as ufld_108 | |
| 110 | Wiegenummer | vs.V_WiegeNummer as ufld_110 | |
| 117 | DA Anfang | vs.V_DauerAnfDat as ufld_117 | |
| 118 | DA nächtst.Termin | vs.V_DauerNaeDat as ufld_118 | |
| 119 | DA Periode | vs.V_DauerPeriode as ufld_119 | |
| 150 | Umwandl.Sperre | vs.V_SperrUmwand as ufld_150 | |
| 175 | KuAuftrDatum | vs.V_EDIKuAuftragsDatum as ufld_175 | |
| 176 | KuAuftrNummer | vs.V_EDIKuAuftragsNummer as ufld_176 | |
| 404 | Einzelz Kokore | vs.V_KennzDrRechKokore as ufld_404 | |
| 435 | Arbeitsregel | vs.ArbeitsRegel as ufld_435 | wfs_name |
| 456 | Transportweg | vs.V_TransportWeg as ufld_456 | |
| 457 | Verfahrensart | vs.V_VerfahrensArt as ufld_457 | |
| 470 | Vorbel. Ziel Herkunft Land | vs.HerkunftZielLand as ufld_470 | Staatbezeich |
| 471 | Vorbel. Region | vs.HerkunftZielRegion as ufld_471 | |
| 500 | Klammernr | vs.V_Klammernr as ufld_500 | |
| 502 | Klammertyp | vs.V_Klammertyp as ufld_502 | |
| 1034 | Versandart | vs.VersArtId as ufld_1034 | VersartBezeich |
| 1038 | SteuerGr. | vs.SteuerGrNummer as ufld_1038 | SteuerGrBezeich |
| 1059 | Ind_Zuab_Klasse | vs.ZuAbKlassNummerI as ufld_1059 | zuabklassbezeich |
| 1062 | Vertr.Gruppe | vs.VertGrNummer as ufld_1062 | VertGrBezeich |
| 1063 | LiPreiKl | vs.PreisKlNummer as ufld_1063 | PreisKlBezeich |
| 1065 | Rabattklasse | vs.RabKlassNummer as ufld_1065 | Rabklassbezeich |
| 1066 | ZuAB_Klasse | vs.ZuAbKlassNummer as ufld_1066 | zuabklassbezeich |
| 1067 | Frachtkl. | vs.FraKlassNummer as ufld_1067 | FraKlassBezeich |
| 1089 | Abteilung | vs.AbteilungId as ufld_1089 | Abteilungbezeich |
| 1095 | Frachtvar. | vs.FraVariNummer as ufld_1095 | FraVariBezeich |
| 1096 | VerkaufsGeb. | vs.VerkGebNummer as ufld_1096 | VerkGebBezeich |
| 1097 | Gebiet von | vs.GebietNummerVon as ufld_1097 | GebietBezeich |
| 1098 | Gebiet nach | vs.GebietNummerNach as ufld_1098 | GebietBezeich |
| 1099 | LKW Nr.Motor | vs.LKW_NummerMotor as ufld_1099 | LKW_Bezeich |
| 1100 | LKW Anhänger | vs.LKW_NummerAnhaen as ufld_1100 | LKW_BEZEICH |
| 1101 | Fahrer | vs.FahrerNummer as ufld_1101 | FAHRERBEZEICH |
| 1155 | Filiale | vs.FilialNummer as ufld_1155 | FilialBezeich |
| 1156 | Zentrale | vs.FilialNummerZen as ufld_1156 | FilialBezeich |
| 1205 | Referenz.Nr | vs.V_ReferenzNummer as ufld_1205 | |
| 1207 | Fakt.Grp. | vs.FaktGrupNummer as ufld_1207 | FaktGrupBezeich |
| 1231 | Lieferdatum | vs.V_DatumStufe3 as ufld_1231 | |
| 1335 | Preisdatum | vs.V_DatumPreise as ufld_1335 | |
| 1453 | Währung Nr | (Select vw.waehrnummer from V_WAEHRUNG as vw where vw.v_id = vs.v_id) as ufld_1453 | WaehrBezeich |
| 1463 | Bruttovorg. | vs.V_KennzBrutto as ufld_1463 | |
| 1510 | LagerNr. | vs.LagerNummerFehl as ufld_1510 | LagerBezeich |
| 1531 | Lagerplatz | vs.LagerPlatzFehl as ufld_1531 | |
| 1767 | Lieferwoche | vs.V_LieferWoche as ufld_1767 | |
| 1771 | Text1 | (select V_BemText1 from VorgBemerkung where V_BemId = vs.V_BemId ) as ufld_1771 | |
| 1772 | Text2 | (select V_BemText2 from VorgBemerkung where V_BemId = vs.V_BemId ) as ufld_1772 | |
| 1773 | Text3 | (select V_BemText3 from VorgBemerkung where V_BemId = vs.V_BemId ) as ufld_1773 | |
| 1774 | Text4 | (select V_BemText4 from VorgBemerkung where V_BemId = vs.V_BemId ) as ufld_1774 | |
| 1775 | Text5 | (select V_BemText5 from VorgBemerkung where V_BemId = vs.V_BemId ) as ufld_1775 | |
| 1776 | Langtext1 | (select V_BemLangText1 from VorgBemerkung where V_BemId = vs.V_BemId ) as ufld_1776 | |
| 1824 | Fiktiv-Menge | vs.V_MengeFiktiv as ufld_1824 | |
| 1825 | Mengeneinheit | vs.ME_NummerFiktMen as ufld_1825 | ME_Kurztext |
| 1828 | Parität | vs.ParitaetNummer as ufld_1828 | ParitaetBezeich |
| 1914 | Beleg-Referenz | vs.FA_BelegReferenz as ufld_1914 | |
| 1972 | Barverkauf | vs.V_KennzBarverk as ufld_1972 | |
| 1976 | FibuSperre | vs.V_SperrFibu as ufld_1976 | |
| 1977 | FiliaSperre | vs.V_SperrFilia as ufld_1977 | |
| 1978 | RebuchSperre | vs.V_SperrReBuch as ufld_1978 | |
| 1985 | Lieferzeit | (select TransLizNummer from VorgTRansAuftrag where V_TransId = vs.V_TransId) as ufld_1985 | translizbezeich |
| 1986 | Ladezeit | (select TransLazNummer from VorgTRansAuftrag where V_TransId = vs.V_TransId) as ufld_1986 | translazbezeich |
| 1992 | LadeDatum | (select V_TransLadeDatum from VorgTRansAuftrag where V_TransId = vs.V_TransId) as ufld_1992 | |
| 1993 | Ladeort | (select LagerNummerLade from VorgTRansAuftrag where V_TransId = vs.V_TransId) as ufld_1993 | lagerbezeich |
| 1996 | ZuAbschlKl.1 | (select V_GlZuAbKlasse1 from VorgGlobalZuAb where V_GlZuAbId = vs.V_GlZuAbId) as ufld_1996 | ZuAbKlassBezeich |
| 1997 | ZuAbschlKl.2 | (select V_GlZuAbKlasse2 from VorgGlobalZuAb where V_GlZuAbId = vs.V_GlZuAbId) as ufld_1997 | ZuAbKlassBezeich |
| 1998 | ZuAbWert 1 | (select V_GlZuAbWert1 from VorgGlobalZuAb where V_GlZuAbId = vs.V_GlZuAbId) as ufld_1998 | |
| 1999 | ZuAbWert 2 | (select V_GlZuAbWert2 from VorgGlobalZuAb where V_GlZuAbId = vs.V_GlZuAbId) as ufld_1999 | |
| 2300 | Zahl.Art | vs.ZahlArtId as ufld_2300 | ZahlArtBezeich |
