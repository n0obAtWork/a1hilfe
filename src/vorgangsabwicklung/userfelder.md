# Userfelder

<!-- source: https://amic.de/hilfe/_userfelder.htm -->

Hier können zu den Standardvorgangsfeldern noch weitere Felder in Abhängigkeit der Bedienerklasse, Vorgangsklasse und der Vorgangsunterklasse angezeigt werden.

Positionierung

Die Felder können dabei frei auf der Maske positioniert werden. Um die Einrichtung zu erleichtern, werden die Koordinaten beim Erstellen eines neuen Feldes automatisch vorgegeben.

Wird das erste Feld eingefügt, stehen die Bezeichnung und die Beschreibung in Zeile 13,79 und der Wert in Zeile 13,36. Die Spaltenwerte sind 0,5 für die Bezeichnung, 18,17 für den Wert und 38,00 für die Beschreibung. Wenn schon Felder existieren und der Cursor in die erste freie Zeile bewegt wird, wird das Feld bestimmt, das am weitesten unten steht. Auf die Zeilenwerte dieses Feldes wird 1,71 addiert, um die neuen Zeilenwerte zu erhalten. Die Spaltenwerte für das neue Feld übernommen.

Wird per **F8** zwischen zwei bestehenden Feldern ein neues Feld eingefügt, werden alle nachfolgenden Felder nach unten verschoben, indem 1,71 auf die Zeilenwerte addiert werden.

Das Verhalten beim Löschen eines Feldes kann mit einem Einrichterparameter bestimmt werden. Es besteht die Wahl zwischen:

\- Alle nachfolgenden Felder rücken automatisch eine Zeile nach oben

\- Die nachfolgenden Felder bleiben an ihrer alten Position

\- Bei jedem Löschen wird nachgefragt, ob die Felder nachrücken sollen

Maskenfelder

| Feld | Bedeutung |
| --- | --- |
| Feldgruppe | Folgende Gruppen stehen zur Verfügung  
1. Strecke  
2. Text1  
3. Umbuchung  
4. Vorgang |
| Bed.Klasse | In diesem Feld wird die Bedienerklasse hinterlegt, welche die zugordneten Userfelder bei der Vorgangserfassung sehen darf. |
| Vorg.Klasse | Folgende UFLD Felder werden bei dieser Vorgangsklasse angezeigt |
| Unterklasse | Folgende UFLD Felder werden bei dieser Vorgangsunterklasse angezeigt |

Gridbeschreibung

| Bezeichnung | Bedeutung |
| --- | --- |
| Bezeichnung | In diesem Feld steht die Bezeichnung des UFLD Feldes |
| Feld | ID Nummer des UFLD Feldes |
| Schnellerfassung | Zur Auswahl stehen  
1. Ja  
2. Nein |
| Erfassungslevel | Hier kann angeben werden, wann UFLD Feld auf der Maske nagezeigt werden soll.  
Zur Auswahl stehen  
• Ersterfassung  
• Neuerfassung mit Positionsteil  
• Belegkorrektur  
• Keine Änderung |
| Folgetaste | Hier kann eine Folgetaste hinterlegt werden, die nach der Eingabe des UFLD Feldes ausgeführt werden soll.  
z.B. F5 in der SVMAIN für den Positionsteil |
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

Vorgang

| Nr. | ID | Name | Select-Statement für ein SQLK | Itembox auf dem Feld | Selected Text | Selected Variable |
| --- | --- | --- | --- | --- | --- | --- |
| 17 | ID_SPRACHE | Sprache | 
```sql
select vs.Sprachnummer from amic_v_vorgaenge vs where
      v_id = :ID_V_ID
```

 | IB_Sprache | cssc_sprachestamm | SprachBezeich |
| 108 | ID_VERSANDADRESSID | VersandAdr. | 

```sql
select vs.V_VersAdressId from amic_v_vorgaenge vs
      where v_id = :ID_V_ID
```

 | IB_VERS_ANSCHRIFT_ZUM_KUNDEN  
 | | |
| 110 | ID_V_WIEGENUMMER | Wiegenummer | 

```sql
select vs.V_WiegeNummer from amic_v_vorgaenge vs
      where v_id = :ID_V_ID
```

 | | | |
| 117 | ID_DAUER_ANFDAT | DA Anfang | 

```sql
select vs.V_DauerAnfDat from amic_v_vorgaenge vs
      where v_id = :ID_V_ID
```

 | | | |
| 118 | ID_DAUER_NAEDAT | DA nächtst.Termin | 

```sql
select vs.V_DauerNaeDat from amic_v_vorgaenge vs
      where v_id = :ID_V_ID
```

 | | | |
| 119 | ID_DAUER_PERIODE | DA Periode | 

```sql
select vs.V_DauerPeriode from amic_v_vorgaenge vs
      where v_id = :ID_V_ID
```

 | | | |
| 122 | ID_DAUER_ENDDAT | DA Ende | 

```sql
Select vs.v_DauerEndDat from amic_v_vorgaenge vs
      where v_id = :ID_V_ID
```

 | | | |
| 150 | ID_SPERRUMWANDLUNG | Umwandl.Sperre | 

```sql
select vs.V_SperrUmwand from amic_v_vorgaenge vs
      where v_id = :ID_V_ID
```

 | | | |
| 175 | ID_EDI_KU_AUFTRAGS_DATUM | KuAuftrDatum | 

```sql
select vs.V_EDIKuAuftragsDatum from amic_v_vorgaenge
      vs where v_id = :ID_V_ID
```

 | | | |
| 176 | ID_EDI_KU_AUFTRAGS_NUMMER | KuAuftrNummer | 

```sql
select
      vs.V_EDIKuAuftragsNummer from
      amic_v_vorgaenge vs where v_id = :ID_V_ID
```

 | | | |
| 404 | ID_KENNZ_DRUCK_KOKORE | Einzelz Kokore | 

```sql
select vs.V_KennzDrRechKokore from amic_v_vorgaenge
      vs where v_id = :ID_V_ID
```

 | | | |
| 427 | ID_ABW_ROHW_SORTE | AbwRohwSorte | 

```sql
Select vb.v_AbwRohwSorte from VorgBemerkung vb

join amic_v_vorgaenge
      vs on (vb.V_BemId = vs.V_BemId)
where vs.v_id = :ID_V_ID
```

 | ib_rohwaresorte  
 | ssc_rohwareSorteNummer | RohSorteBezeich |
| 428 | ID_ABW_ROHW_KUNDNUMMER | AbwRohwKunde | 

```sql
Select ku.KundNummer from Kundenstamm ku
join VorgBemerkung vb
      on ( ku.KundId = vb.V_AbwRohKundId )
join amic_v_vorgaenge
      vs on (vb.V_BemId = vs.V_BemId)
where vs.v_id = :ID_V_ID
```

 | IB_KU  
 | SSC_KundenStamm | Kundbezeich |
| 430 | ID_ROHWAREVORERFASSUNG | RohwVorerfass | 

```sql
Select vb.v_RW_VorErfassung from VorgBemerkung vb

join amic_v_vorgaenge
      vs on (vb.V_BemId = vs.V_BemId)
where vs.v_id = :ID_V_ID
```

 | | | |
| 435 | ID_ARBEITSREGEL | Arbeitsregel | 

```sql
select vs.ArbeitsRegel from amic_v_vorgaenge vs where v_id =
      :ID_V_ID
```

 | IB_WORKFLOW  
 | ssc_workflowbezeich | wfs_name |
| 450 | ID_ABW_ROHW_LAGERKENNZID | Lager ändern | 

```sql
Select vb.v_AbwLagerKennz from VorgBemerkung vb

join amic_v_vorgaenge
      vs on (vb.V_BemId = vs.V_BemId)
where vs.v_id = :ID_V_ID
```

 | | | |
| 451 | ID_ABW_ROHW_LAGERID | Lagernr.neu | 

```sql
Select vb.v_AbwLagerNummer from VorgBemerkung vb

join amic_v_vorgaenge
      vs on (vb.V_BemId = vs.V_BemId)
where vs.v_id = :ID_V_ID
```

 | IB_LagerStamm | cssc_lagerstamm | LagerBezeich |
| 452 | ID_ABW_ROHW_LAGERPLZID | Lagerpl.neu | 

```sql
Select vb.v_AbwLagerPlatz from VorgBemerkung vb

join amic_v_vorgaenge
      vs on (vb.V_BemId = vs.V_BemId)
where vs.v_id = :ID_V_ID
```

 | IB_LagerPlatz | | |
| 456 | ID_TRANSPORTWEG | Transportweg | 

```sql
select vs.V_TransportWeg from amic_v_vorgaenge vs
      where v_id = :ID_V_ID
```

 | | | |
| 457 | ID_VERFAHRENSART | Verfahrensart | 

```sql
select vs.V_VerfahrensArt from amic_v_vorgaenge vs where v_id =
      :ID_V_ID
```

 | | | |
| 470 | ID_HERKUNFTZIELLAND | Vorbel. Ziel Herkunft Land | 

```sql
select vs.HerkunftZielLand from amic_v_vorgaenge vs
      where v_id = :ID_V_ID
```

 | IB_StaatStammBezeich | CSSC_HerkunftsLand | Staatbezeich |
| 471 | ID_HERKUNFTZIELREGION | Vorbel. Region | 

```sql
select vs.HerkunftZielRegion from amic_v_vorgaenge vs
      where v_id = :ID_V_ID
```

 | | | |
| 500 | ID_KLAMMERNR | Klammernr | 

```sql
select vs.V_Klammernr from amic_v_vorgaenge vs where
      v_id = :ID_V_ID
```

 | IB_Klammernummer | | |
| 502 | ID_KLAMMERTYP | Klammertyp | 

```sql
select vs.V_Klammertyp from amic_v_vorgaenge vs where
      v_id = :ID_V_ID
```

 | | | |
| 504 | ID_USTID_KUNDE | UST-ID Kunde | 

```sql
select vs.UstId_Kunde from amic_v_vorgaenge vs where
      v_id = :ID_V_ID
```

 | IB_USTID_KUNDE | | |
| 505 | ID_USTID_MANDANT | UST-ID Firma | 

```sql
select vs.UstId_Mandant from amic_v_vorgaenge vs
      where v_id = :ID_V_ID
```

 | IB_USTID_MANDANT | | |
| 547 | ID_INFOADRESSID | informelle Anschrift | 

```sql
select vs.v_AdressIdInfo from amic_v_vorgaenge vs
      where v_id = :ID_V_ID
```

 | IB_Anschrift | | |
| 577 | ID_SHIPFROM | ShipFrom | 

```sql
select vs.v_ShipFrom from amic_v_vorgaenge vs where
      v_id = :ID_V_ID
```

 | IB_STAATSTAMM | CSSC_STAATSTAMM | StaatBezeich |
| 578 | ID_SHIPTO | ShipTo | 

```sql
select vs.v_ShipTo from amic_v_vorgaenge vs where
      v_id = :ID_V_ID
```

 | IB_STAATSTAMM | CSSC_STAATSTAMM | StaatBezeich |
| 1034 | ID_VERSANDARTID | Versandart | 

```sql
select vs.VersArtId from amic_v_vorgaenge vs where
      v_id = :ID_V_ID
```

 | IB_VERSANDART | CSSC_VERSANDART | VersartBezeich |
| 1038 | ID_STEUERGRNUMMER | SteuerGr. | 

```sql
select vs.SteuerGrNummer from amic_v_vorgaenge vs
      where v_id = :ID_V_ID
```

 | IB_SteuerGruppe | cssc_steuergruppe | SteuerGrBezeich |
| 1059 | ID_ZUABKLASSNUMMERINDI | Ind_Zuab_Klasse | 

```sql
select vs.ZuAbKlassNummerI from amic_v_vorgaenge vs
      where v_id = :ID_V_ID
```

 | ib_ZuAbschKlasse | cssc_zuabschklasse | zuabklassbezeich |
| 1062 | ID_VERTGRNUMMER | Vertr.Gruppe | 

```sql
select vs.VertGrNummer from amic_v_vorgaenge vs where
      v_id = :ID_V_ID
```

 | IB_VERTRETERGRUPPE | CSSC_VERTRETERGRUPPE | VertGrBezeich |
| 1063 | ID_PREISKLNUMMER | LiPreiKl | 

```sql
select vs.PreisKlNummer from amic_v_vorgaenge vs
      where v_id = :ID_V_ID
```

 | ib_preisklassen | cssc_preisklassen | PreisKlBezeich |
| 1065 | ID_RABKLASSNUMMER | Rabattklasse | 

```sql
select vs.RabKlassNummer from amic_v_vorgaenge vs
      where v_id = :ID_V_ID
```

 | ib_RabattKlasse | CSSC_RabattKlasse | rabklassbezeich |
| 1066 | ID_ZUABKLASSNUMMER | ZuAB_Klasse | 

```sql
select vs.ZuAbKlassNummer from amic_v_vorgaenge vs
      where v_id = :ID_V_ID
```

 | ib_ZuAbschKlasse | cssc_zuabschklasse | zuabklassbezeich |
| 1067 | ID_FRAKLASSNUMMER | Frachtkl. | 

```sql
select vs.FraKlassNummer from amic_v_vorgaenge vs
      where v_id = :ID_V_ID
```

 | ib_FrachtKlasse | SSC_FraKlassBezeich | FraKlassBezeich |
| 1089 | ID_ABTEILUNG | Abteilung | 

```sql
select vs.AbteilungId from amic_v_vorgaenge vs where
      v_id = :ID_V_ID
```

 | IB_ABTEILUNG | CSSC_ABTEILUNG | Abteilungbezeich |
| 1095 | ID_FRACHTVARIANTE | Frachtvar. | 

```sql
select vs.FraVariNummer from amic_v_vorgaenge vs
      where v_id = :ID_V_ID
```

 | IB_FRACHTVARIANTE | cssc_FrachtVariante | FraVariBezeich |
| 1096 | ID_VERKAUFSGEBIET | VerkaufsGeb. | 

```sql
select vs.VerkGebNummer from amic_v_vorgaenge vs
      where v_id = :ID_V_ID
```

 | IB_VERKAUFSGEBIET | cssc_Verkaufsgebiet | VerkGebBezeich |
| 1097 | ID_GEBIET_VON | Gebiet von | 

```sql
select vs.GebietNummerVon from amic_v_vorgaenge vs
      where v_id = :ID_V_ID
```

 | IB_GEBIETSTAMM | cssc_Gebietstamm | GebietBezeich |
| 1098 | ID_GEBIET_NACH | Gebiet nach | 

```sql
select vs.GebietNummerNach from amic_v_vorgaenge vs
      where v_id = :ID_V_ID
```

 | IB_GEBIETSTAMM | cssc_Gebietstamm | GebietBezeich |
| 1099 | ID_LKW_NUMMER_MOTOR | LKW Nr.Motor | 

```sql
select vs.LKW_NummerMotor from amic_v_vorgaenge vs
      where v_id = :ID_V_ID
```

 | IB_LKW_MOTOR | CSSC_LKW | LKW_Bezeich |
| 1100 | ID_LKW_NUMMER_ANHAENGER | LKW Anhänger | 

```sql
select vs.LKW_NummerAnhaen from amic_v_vorgaenge vs
      where v_id = :ID_V_ID
```

 | IB_LKW_ANHAENGER | CSSC_LKW | LKW_BEZEICH |
| 1101 | ID_FAHRER | Fahrer | 

```sql
select vs.FahrerNummer from amic_v_vorgaenge vs where
      v_id = :ID_V_ID
```

 | IB_FAHRER | CSSC_FAHRER | FAHRERBEZEICH |
| 1155 | ID_FILIALE | Filiale | 

```sql
select vs.FilialNummer from amic_v_vorgaenge vs where
      v_id = :ID_V_ID
```

 | ib_filiale | CSSC_Filiale | FilialBezeich |
| 1156 | ID_ZENTRALE | Zentrale | 

```sql
select vs.FilialNummerZen from amic_v_vorgaenge vs
      where v_id = :ID_V_ID
```

 | ib_zentrale | CSSC_ZENTRALE | FilialBezeich |
| 1205 | ID_REFERENZNUMMER | Referenz.Nr | 

```sql
select vs.V_ReferenzNummer from amic_v_vorgaenge vs
      where v_id = :ID_V_ID
```

 | | | |
| 1207 | ID_FAKTURIERGRUPPE | Fakt.Grp. | 

```sql
select vs.FaktGrupNummer from amic_v_vorgaenge vs
      where v_id = :ID_V_ID
```

 | ib_faktgruppe | CSSC_FAKTURIERG | FaktGrupBezeich |
| 1231 | ID_V_DATUM_3 | Lieferdatum | 

```sql
select vs.V_DatumStufe3 from amic_v_vorgaenge vs
      where v_id = :ID_V_ID
```

 | | | |
| 1240 | ID_BONITAET | Bonitaet | 

```sql
select ku.KundBonitaet from kundenstamm ku join
      amic_v_vorgaenge vs on (vs.KundIdZuOrd = ku.kundId)  where vs.v_id =
      :ID_V_ID
```

 | | | |
| 1335 | ID_V_DATUM_PREISE | Preisdatum | 

```sql
select vs.V_DatumPreise from amic_v_vorgaenge vs
      where v_id = :ID_V_ID
```

 | | | |
| 1345 | ID_V_DATUM_PLAN_3 | Plan/Liefdat | 

```sql
select
( case

      when v_klassnummer in
      (700,790,800,890,1700,1790,1800,1890)

      then
      v_datum

      when v_klassnummer in (5100,5110,5120) then (select if
       vu.V_LGUBuchTyp
      = 2 then vs.v_datum else vs.v_plandatum
       endif as v_datum
      from VorgStammUmbuch vu where vu.v_id =

      vs.v_id)

      when v_klassnummer = 5220 then (select if V_ProdBuchTyp = 2
      then vs.v_datum else
      v_plandatum endif as v_datum from
       V_ProdVorgang
      vpv where vpv.v_id = vs.v_id)
     else v_PlanDatum
    END
      )
  as Datum from amic_v_vorgaenge
      vs
```

 | | | |
| 1354 | ID_USTKENNZEICHEN | USt-Ident. | 

```sql
select vs.UstId_Kunde from amic_v_vorgaenge vs where
      v_id = :ID_V_ID
```

 | | | |
| 1426 | ID_TOURNUMMER | Tournummer | 

```sql
select vs.TourId from amic_v_vorgaenge vs where v_id
      = :ID_V_ID
```

 | IB_TOURKUNDE | | |
| 1453 | ID_FW_NUMMER | Währung Nr | 

```sql
Select vw.waehrnummer from V_WAEHRUNG as vw where
      vw.v_id = :ID_V_ID
```

 | IB_Waehrung | cssc_waehrung | WaehrBezeich |
| 1456 | ID_FW_KURS | Kurs | 

```sql
select vw.v_WaehrKurs from V_Waehrung vw  where
      V_Id =  :ID_V_ID
```

 | | | |
| 1463 | ID_BRUTTOKENNZEICHEN | Bruttovorg. | 

```sql
select vs.V_KennzBrutto from amic_v_vorgaenge vs
      where v_id = :ID_V_ID
```

 | | | |
| 1510 | ID_LAGERNUMMER_FEHL | LagerNr. | 

```sql
select vs.LagerNummerFehl from amic_v_vorgaenge vs
      where v_id = :ID_V_ID
```

 | ib_lagerstamm | cssc_lagerstamm | LagerBezeich |
| 1531 | ID_LAGERPLATZ_FEHL | Lagerplatz | 

```sql
select vs.LagerPlatzFehl from amic_v_vorgaenge vs
      where v_id = :ID_V_ID
```

 | IB_LagerPlatz | | |
| 1766 | ID_PRUEFMENGE | Prüfmenge | 

```text
Wird nur bei der Erfassung von Vorgängen benötigt und
      kann mit keinem SQLK abgefragt werden.
```

 | | | |
| 1767 | ID_LIEFERWOCHE | Lieferwoche | 

```sql
select vs.V_LieferWoche from amic_v_vorgaenge vs
      where v_id = :ID_V_ID
```

 | | | |
| 1771 | ID_V_BEMTEXT1 | Text1 | 

```sql
select vb.V_BemText1 from VorgBemerkung vb

join amic_v_vorgaenge
      vs on (vb.V_BemId = vs.V_BemId)
where vs.v_id = :ID_V_ID
```

 | | | |
| 1772 | ID_V_BEMTEXT2 | Text2 | 

```sql
select vb.V_BemText2 from VorgBemerkung vb

join amic_v_vorgaenge
      vs on (vb.V_BemId = vs.V_BemId)
where vs.v_id = :ID_V_ID
```

 | | | |
| 1773 | ID_V_BEMTEXT3 | Text3 | 

```sql
select vb.V_BemText3 from VorgBemerkung vb

join amic_v_vorgaenge
      vs on (vb.V_BemId = vs.V_BemId)
where vs.v_id = :ID_V_ID
```

 | | | |
| 1774 | ID_V_BEMTEXT4 | Text4 | 

```sql
select vb.V_BemText4 from VorgBemerkung vb

join amic_v_vorgaenge
      vs on (vb.V_BemId = vs.V_BemId)
where vs.v_id = :ID_V_ID
```

 | | | |
| 1775 | ID_V_BEMTEXT5 | Text5 | 

```sql
select vb.V_BemText5 from VorgBemerkung vb

join amic_v_vorgaenge
      vs on (vb.V_BemId = vs.V_BemId)
where vs.v_id = :ID_V_ID
```

 | | | |
| 1776 | ID_V_BEMLANGTEXT1 | Langtext1 | 

```sql
select vb.V_BemLangText1 from VorgBemerkung vb

join amic_v_vorgaenge
      vs on (vb.V_BemId = vs.V_BemId)
where vs.v_id = :ID_V_ID
```

 | | | |
| 1824 | ID_MENGE_FIKTIV | Fiktiv-Menge | 

```sql
select vs.V_MengeFiktiv from amic_v_vorgaenge vs
      where v_id = :ID_V_ID
```

 | | | |
| 1825 | ID_ME_NUMMER_FIKTIV | Mengeneinheit | 

```sql
select vs.ME_NummerFiktMen from amic_v_vorgaenge vs
      where v_id = :ID_V_ID
```

 | IB_MengeneinheitPassend_0 | | ME_Kurztext |
| 1828 | ID_PARITAETNUMMER | Parität | 

```sql
select vs.ParitaetNummer from amic_v_vorgaenge vs
      where v_id = :ID_V_ID
```

 | IB_ParitaetStamm | CSSC_ParitaetStamm | ParitaetBezeich |
| 1851 | ID_AKTOBJEKT_NUMMER | Objekte | 

```sql
Select bs.Baustnummer from Bauststamm bs
join amic_v_vorgaenge
      vs on (vs.BaustId = bs.BaustId)
where vs.v_id =:ID_V_ID
```

 | IB_BAUSTSTAMM_KU | cssc_baustelle | BaustBezeich |
| 1894 | ID_KUNDNUMMER_RECHNUNGSEMPF | Rechnungsempf | 

```sql
select vs.KundIdRechn from amic_v_vorgaenge vs where
      v_id = :ID_V_ID
```

 | IB_KU | SSC_KundenStamm_OberKunde | KundBezeich |
| 1895 | ID_KUNDNUMMER_ZAHLUNGSPFL | Zahlungspfl. | 

```sql
select vs.KundIdFinanz from amic_v_vorgaenge vs where
      v_id = :ID_V_ID
```

 | IB_KU | SSC_KundenStamm_OberKunde | KundBezeich |
| 1899 | ID_KUNDNUMMER_KONTRAKTKUNDE | Kontraktkunde | 

```sql
select vs.KundIdKontrakt from amic_v_vorgaenge vs
      where v_id = :ID_V_ID
```

 | IB_KU | SSC_KundenStamm_OberKunde | KundBezeich |
| 1914 | ID_FA_BELEGREFERENZ | Beleg-Referenz | 

```sql
select vs.FA_BelegReferenz from amic_v_vorgaenge vs
      where v_id = :ID_V_ID
```

 | | | |
| 1972 | ID_KENNZBARVERK | Barverkauf | 

```sql
select vs.V_KennzBarverk from amic_v_vorgaenge vs
      where v_id = :ID_V_ID
```

 | | | |
| 1974 | ID_KASSENNUMMER | Kasse | 

```sql
Select vk.Kassennummer from v_kasseninfo where v_id =
      :ID_V_ID
```

 | IB_Kassensysteme | | |
| 1975 | ID_KASSENKONTO | KassenKonto | 

```sql
Select vk.V_KassKonto from v_kasseninfo where v_id =
      :ID_V_ID
```

 | IB_SACHKONTO | | |
| 1976 | ID_SPERRFIBU | FibuSperre | 

```sql
select vs.V_SperrFibu from amic_v_vorgaenge vs where
      v_id = :ID_V_ID
```

 | | | |
| 1977 | ID_SPERRFILIA | FiliaSperre | 

```sql
select vs.V_SperrFilia from amic_v_vorgaenge vs where
      v_id = :ID_V_ID
```

 | | | |
| 1978 | ID_SPERRREBUCH | RebuchSperre | 

```sql
select vs.V_SperrReBuch from amic_v_vorgaenge vs
      where v_id = :ID_V_ID
```

 | | | |
| 1985 | ID_LIEFERZEITNUMMER | Lieferzeit | 

```sql
select vta.TransLizNummer from VorgTRansAuftrag vta

join amic_v_vorgaenge
      vs on ( vta.V_TransId = vs.V_TransId )
where vs.v_id = :ID_V_ID
```

 | IB_TRANSLIEFERZEIT | ssc_lieferzeit | translizbezeich |
| 1986 | ID_LADEZEITNUMMER | Ladezeit | 

```sql
select vta.TransLazNummer from VorgTRansAuftrag vta

join amic_v_vorgaenge
      vs on ( vta.V_TransId = vs.V_TransId )
where vs.v_id = :ID_V_ID
```

 | IB_TRANSLADEZEIT | ssc_Ladezeit | translazbezeich |
| 1991 | ID_PRUEFSUMME | Prüfsumme Vorgang | 

```text
Wird nur bei der Erfassung von Vorgängen benötigt und
      kann mit keinem SQLK abgefragt werden.
```

 | | | |
| 1992 | ID_LADEDATUM | LadeDatum | 

```sql
select vta.V_TransLadeDatum from VorgTRansAuftrag vta

join amic_v_vorgaenge
      vs on ( vta.V_TransId = vs.V_TransId )
where vs.v_id = :ID_V_ID
```

 | | | |
| 1993 | ID_LADEORTNUMMER | Ladeort | 

```sql
select LagerNummerLade from VorgTRansAuftrag vta

join amic_v_vorgaenge
      vs on ( vta.V_TransId = vs.V_TransId )
where vs.v_id  = :ID_V_ID
```

 | IB_LagerStamm | cssc_lagerstamm | lagerbezeich |
| 1996 | ID_GLZUAB_KLASSE1 | ZuAbschlKl.1 | 

```sql
select vgz.V_GlZuAbKlasse1 from VorgGlobalZuAb
      vgz
join amic_v_vorgaenge
      vs on (vgz.V_GlZuAbId = vs.V_GlZuAbId )
where vs.v_id :ID_V_ID
```

 | ib_ZuAbschKlasse | cssc_zuabschklasse | ZuAbKlassBezeich |
| 1997 | ID_GLZUAB_KLASSE2 | ZuAbschlKl.2 | 

```sql
select vgz.V_GlZuAbKlasse2 from VorgGlobalZuAb
      vgz
join amic_v_vorgaenge
      vs on (vgz.V_GlZuAbId = vs.V_GlZuAbId )
where vs.v_id :ID_V_ID
```

 | ib_ZuAbschKlasse | cssc_zuabschklasse | ZuAbKlassBezeich |
| 1998 | ID_GLZUAB_WERT1 | ZuAbWert 1 | 

```sql
select vgz.V_GlZuAbWert1 from VorgGlobalZuAb
      vgz
join amic_v_vorgaenge
      vs on (vgz.V_GlZuAbId = vs.V_GlZuAbId )
where vs.v_id :ID_V_ID
```

 | | | |
| 1999 | ID_GLZUAB_WERT2 | ZuAbWert 2 | 

```sql
select vgz.V_GlZuAbWert2 from VorgGlobalZuAb
      vgz
join amic_v_vorgaenge
      vs on (vgz.V_GlZuAbId = vs.V_GlZuAbId )
where vs.v_id :ID_V_ID
```

 | | | |
| 2216 | ID_GELANGENSBEST | Gelangensbestätigung | 

```sql
select vs.V_GelangensBesrtJN from amic_v_vorgaenge vs
      where v_id = :ID_V_ID
```

 | | | |
| 2300 | ID_ZAHLARTID | Zahl.Art | 

```sql
select vs.ZahlArtId from amic_v_vorgaenge vs where
      v_id = :ID_V_ID
```

 | ib_zahlungsart | CSSC_ZAHLUNGSART | ZahlArtBezeich |
| 4200 | ID_PROD_KUNDNUMMER | Prod.KuNr. | 

```sql
select pv.V_ProdKundNummer from V_ProdVorgang pv
      where v_id = :ID_V_ID
```

 | IB_KU | SSC_KundenStamm | Kundbezeich |
| 4204 | ID_PROD_BUCHTYP | Prod.Buchtyp | 

```sql
select pv.V_ProdBuchTyp from V_ProdVorgang pv where
      v_id = :ID_V_ID
```

 | | | |
| 8268 | ID_V_DATUMEINGANG | Eingangsdatum | 

 | | | |
| 8269 | ID_ZAHLUNGSREFERENZ | Zahlungsreferenz | 

 | | | |

Umbuchung

| Nr | ID | Name | Select-Statement für ein SQLK | Itembox auf dem Feld | Selected Text | Selected Variable |
| --- | --- | --- | --- | --- | --- | --- |
| 108 | ID_VERSANDADRESSID | VersandAdr. | 
```sql
select vs.V_VersAdressId from amic_v_vorgaenge vs
      where v_id = :ID_V_ID
```

 | IB_VERS_ANSCHRIFT_ZUM_KUNDEN | | |
| 150 | ID_SPERRUMWANDLUNG | Umwandl.Sperre | 

```sql
select vs.V_SperrUmwand from amic_v_vorgaenge vs
      where v_id = :ID_V_ID
```

 | | | |
| 435 | ID_ARBEITSREGEL | Arbeitsregel | 

```sql
select vs.ArbeitsRegel from amic_v_vorgaenge vs where v_id =
      :ID_V_ID
```

 | IB_WORKFLOW | ssc_workflowbezeich | wfs_name |
| 504 | ID_USTID_KUNDE | UST-ID Kunde | 

```sql
select vs.UstId_Kunde from amic_v_vorgaenge vs where
      v_id = :ID_V_ID
```

 | IB_USTID_KUNDE | | |
| 505 | ID_USTID_MANDANT | UST-ID Firma | 

```sql
select vs.UstId_Mandant from amic_v_vorgaenge vs
      where v_id = :ID_V_ID
```

 | IB_USTID_MANDANT | | |
| 1034 | ID_VERSANDARTID | Versandart | 

```sql
select vs.VersArtId from amic_v_vorgaenge vs where
      v_id = :ID_V_ID
```

 | IB_VERSANDART | CSSC_VERSANDART | VersartBezeich |
| 1089 | ID_ABTEILUNG | Abteilung | 

```sql
select vs.AbteilungId from amic_v_vorgaenge vs where
      v_id = :ID_V_ID
```

 | IB_ABTEILUNG | CSSC_ABTEILUNG | Abteilungbezeich |
| 1099 | ID_LKW_NUMMER_MOTOR | LKW Nr.Motor | 

```sql
select vs.LKW_NummerMotor from amic_v_vorgaenge vs
      where v_id = :ID_V_ID
```

 | IB_LKW_MOTOR | CSSC_LKW | LKW_Bezeich |
| 1100 | ID_LKW_NUMMER_ANHAENGER | LKW Anhänger | 

```sql
select vs.LKW_NummerAnhaen from amic_v_vorgaenge vs
      where v_id = :ID_V_ID
```

 | IB_LKW_ANHAENGER | CSSC_LKW | LKW_BEZEICH |
| 1101 | ID_FAHRER | Fahrer | 

```sql
select vs.FahrerNummer from amic_v_vorgaenge vs where
      v_id = :ID_V_ID
```

 | IB_FAHRER | CSSC_FAHRER | FAHRERBEZEICH |
| 1155 | ID_FILIALE | Filiale | 

```sql
select vs.FilialNummer from amic_v_vorgaenge vs where
      v_id = :ID_V_ID
```

 | ib_filiale | CSSC_Filiale | FilialBezeich |
| 1156 | ID_ZENTRALE | Zentrale | 

```sql
select vs.FilialNummerZen from amic_v_vorgaenge vs
      where v_id = :ID_V_ID
```

 | ib_zentrale | CSSC_ZENTRALE | FilialBezeich |
| 1205 | ID_REFERENZNUMMER | Referenz.Nr | 

```sql
select vs.V_ReferenzNummer from amic_v_vorgaenge vs
      where v_id = :ID_V_ID
```

 | | | |
| 1345 | ID_V_DATUM_PLAN_3 | Plan/Liefdat | 

```sql
select
( case

      when v_klassnummer in
      (700,790,800,890,1700,1790,1800,1890)

      then
      v_datum

      when v_klassnummer in (5100,5110,5120) then (select if
       vu.V_LGUBuchTyp
      = 2 then vs.v_datum else vs.v_plandatum
       endif as v_datum
      from VorgStammUmbuch vu where vu.v_id =

      vs.v_id)

      when v_klassnummer = 5220 then (select if V_ProdBuchTyp = 2
      then vs.v_datum else
      v_plandatum endif as v_datum from
       V_ProdVorgang
      vpv where vpv.v_id = vs.v_id)
     else v_PlanDatum
    END
      )
  as Datum from amic_v_vorgaenge
      vs
```

 | | | |
| 1510 | ID_LAGERNUMMER_FEHL | LagerNr. | 

```sql
select vs.LagerNummerFehl from amic_v_vorgaenge vs
      where v_id = :ID_V_ID
```

 | ib_lagerstamm | cssc_lagerstamm | LagerBezeich |
| 1771 | ID_V_BEMTEXT1 | Text1 | 

```sql
select vb.V_BemText1 from VorgBemerkung vb

join amic_v_vorgaenge
      vs on (vb.V_BemId = vs.V_BemId)
where vs.v_id = :ID_V_ID
```

 | | | |
| 1772 | ID_V_BEMTEXT2 | Text2 | 

```sql
select vb.V_BemText2 from VorgBemerkung vb

join amic_v_vorgaenge
      vs on (vb.V_BemId = vs.V_BemId)
where vs.v_id = :ID_V_ID
```

 | | | |
| 1773 | ID_V_BEMTEXT3 | Text3 | 

```sql
select vb.V_BemText3 from VorgBemerkung vb

join amic_v_vorgaenge
      vs on (vb.V_BemId = vs.V_BemId)
where vs.v_id = :ID_V_ID
```

 | | | |
| 1774 | ID_V_BEMTEXT4 | Text4 | 

```sql
select vb.V_BemText4 from VorgBemerkung vb

join amic_v_vorgaenge
      vs on (vb.V_BemId = vs.V_BemId)
where vs.v_id = :ID_V_ID
```

 | | | |
| 1775 | ID_V_BEMTEXT5 | Text5 | 

```sql
select vb.V_BemText5 from VorgBemerkung vb

join amic_v_vorgaenge
      vs on (vb.V_BemId = vs.V_BemId)
where vs.v_id = :ID_V_ID
```

 | | | |
| 1824 | ID_MENGE_FIKTIV | Fiktiv-Menge | 

```sql
select vs.V_MengeFiktiv from amic_v_vorgaenge vs
      where v_id = :ID_V_ID
```

 | | | |
| 1825 | ID_ME_NUMMER_FIKTIV | Mengeneinheit | 

```sql
select vs.ME_NummerFiktMen from amic_v_vorgaenge vs
      where v_id = :ID_V_ID
```

 | IB_MengeneinheitPassend_0 | | ME_Kurztext |
| 1828 | ID_PARITAETNUMMER | Parität | 

```sql
select vs.ParitaetNummer from amic_v_vorgaenge vs
      where v_id = :ID_V_ID
```

 | IB_ParitaetStamm | CSSC_ParitaetStamm | ParitaetBezeich |
| 1914 | ID_FA_BELEGREFERENZ | Beleg-Referenz | 

```sql
select vs.FA_BelegReferenz from amic_v_vorgaenge vs
      where v_id = :ID_V_ID
```

 | | | |
| 1976 | ID_SPERRFIBU | FibuSperre | 

```sql
select vs.V_SperrFibu from amic_v_vorgaenge vs where
      v_id = :ID_V_ID
```

 | | | |
| 1977 | ID_SPERRFILIA | FiliaSperre | 

```sql
select vs.V_SperrFilia from amic_v_vorgaenge vs where
      v_id = :ID_V_ID
```

 | | | |
| 1978 | ID_SPERRREBUCH | RebuchSperre | 

```sql
select vs.V_SperrReBuch from amic_v_vorgaenge vs
      where v_id = :ID_V_ID
```

 | | | |
| 1985 | ID_LIEFERZEITNUMMER | Lieferzeit | 

```sql
select vta.TransLizNummer from VorgTRansAuftrag vta

join amic_v_vorgaenge
      vs on ( vta.V_TransId = vs.V_TransId )
where vs.v_id = :ID_V_ID
```

 | IB_TRANSLIEFERZEIT | ssc_lieferzeit | translizbezeich |
| 1986 | ID_LADEZEITNUMMER | Ladezeit | 

```sql
select vta.TransLazNummer from VorgTRansAuftrag vta

join amic_v_vorgaenge
      vs on ( vta.V_TransId = vs.V_TransId )
where vs.v_id = :ID_V_ID
```

 | IB_TRANSLADEZEIT | ssc_Ladezeit | translazbezeich |
| 1991 | ID_PRUEFSUMME | Prüfsumme Vorgang | 

```text
Wird nur bei der Erfassung von Vorgängen benötigt und
      kann mit keinem SQLK abgefragt werden.
```

 | | | |
| 1992 | ID_LADEDATUM | LadeDatum | 

```sql
select vta.V_TransLadeDatum from VorgTRansAuftrag vta

join amic_v_vorgaenge
      vs on ( vta.V_TransId = vs.V_TransId )
where vs.v_id = :ID_V_ID
```

 | | | |
| 1993 | ID_LADEORTNUMMER | Ladeort | 

```sql
select LagerNummerLade from VorgTRansAuftrag vta

join amic_v_vorgaenge
      vs on ( vta.V_TransId = vs.V_TransId )
where vs.v_id  = :ID_V_ID
```

 | IB_LagerStamm | cssc_lagerstamm | lagerbezeich |
| 4501 | ID_LGU_BUCHTYP_KZ | Umbuch.Typ | 

```sql
select vsu.v_LGUBuchTyp from VorgStammUmbuch vsu
      where vsu.v_id = :ID_V_ID
```

 | | | |

Strecke

| Nr | ID | Name | Select-Statement für ein SQLK | Itembox auf dem Feld | Selected Text | Selected Variable |
| --- | --- | --- | --- | --- | --- | --- |
| 1034 | ID_VERSANDARTID | Versandart | 
```sql
select vs.VersArtId from amic_v_vorgaenge vs where
      v_id = :ID_V_ID
```

 | IB_VERSANDART | CSSC_VERSANDART | VersArtBezeich |

Text1

| Nr | ID | Name | Select-Statement für ein SQLK | Itembox auf dem Feld | Selected Text | Selected Variable |
| --- | --- | --- | --- | --- | --- | --- |
| 1771 | ID_VERSANDARTID | Bemerktext1 | 
```sql
select vb.V_BemText1 from VorgBemerkung vb

join amic_v_vorgaenge
      vs on (vb.V_BemId = vs.V_BemId)
where vs.v_id = :ID_V_ID
```

 | IB_VERSANDART | CSSC_VERSANDART | VersArtBezeich |

Produktion

| Nr. | ID | Name | Select-Statement für ein SQLK | Itembox auf dem Feld | Selected Text | Selected Variable |
| --- | --- | --- | --- | --- | --- | --- |
| 17 | ID_SPRACHE | Sprache | 
```sql
select vs.Sprachnummer from amic_v_vorgaenge vs where
      v_id = :ID_V_ID
```

 | IB_Sprache | cssc_sprachestamm | SprachBezeich |
| 150 | ID_SPERRUMWANDLUNG | Umwandl.Sperre | 

```sql
select vs.V_SperrUmwand from amic_v_vorgaenge vs
      where v_id = :ID_V_ID
```

 | | | |
| 175 | ID_EDI_KU_AUFTRAGS_DATUM | KuAuftrDatum | 

```sql
select vs.V_EDIKuAuftragsDatum from amic_v_vorgaenge
      vs where v_id = :ID_V_ID
```

 | | | |
| 176 | ID_EDI_KU_AUFTRAGS_NUMMER | KuAuftrNummer | 

```sql
select
      vs.V_EDIKuAuftragsNummer from
      amic_v_vorgaenge vs where v_id = :ID_V_ID
```

 | | | |
| 435 | ID_ARBEITSREGEL | Arbeitsregel | 

```sql
select vs.ArbeitsRegel from amic_v_vorgaenge vs where v_id =
      :ID_V_ID
```

 | IB_WORKFLOW  
 | ssc_workflowbezeich | |
| 457 | ID_VERFAHRENSART | Verfahrensart | 

```sql
select vs.V_VerfahrensArt from amic_v_vorgaenge vs where v_id =
      :ID_V_ID
```

 | | | |
| 500 | ID_KLAMMERNR | Klammernr | 

```sql
select vs.V_Klammernr from amic_v_vorgaenge vs where
      v_id = :ID_V_ID
```

 | | | |
| 502 | ID_KLAMMERTYP | Klammertyp | 

```sql
select vs.V_Klammertyp from amic_v_vorgaenge vs where
      v_id = :ID_V_ID
```

 | | | |
| 1100 | ID_LKW_NUMMER_ANHAENGER | LKW Anhänger | 

```sql
select vs.LKW_NummerAnhaen from amic_v_vorgaenge vs
      where v_id = :ID_V_ID
```

 | | | |
| 1101 | ID_FAHRER | Fahrer | 

```sql
select vs.FahrerNummer from amic_v_vorgaenge vs where
      v_id = :ID_V_ID
```

 | | | |
| 1205 | ID_REFERENZNUMMER | Referenz.Nr | 

```sql
select vs.V_ReferenzNummer from amic_v_vorgaenge vs
      where v_id = :ID_V_ID
```

 | | | |
| 1335 | ID_V_DATUM_PREISE | Preisdatum | 

```sql
select vs.V_DatumPreise from amic_v_vorgaenge vs
      where v_id = :ID_V_ID
```

 | | | |
| 1345 | ID_V_DATUM_PLAN_3 | Plan/Liefdat | 

```sql
select
( case

      when v_klassnummer in
      (700,790,800,890,1700,1790,1800,1890)

      then
      v_datum

      when v_klassnummer in (5100,5110,5120) then (select if
       vu.V_LGUBuchTyp
      = 2 then vs.v_datum else vs.v_plandatum
       endif as v_datum
      from VorgStammUmbuch vu where vu.v_id =

      vs.v_id)

      when v_klassnummer = 5220 then (select if V_ProdBuchTyp = 2
      then vs.v_datum else
      v_plandatum endif as v_datum from
       V_ProdVorgang
      vpv where vpv.v_id = vs.v_id)
     else v_PlanDatum
    END
      )
  as Datum from amic_v_vorgaenge
      vs
```

 | | | |
| 1510 | ID_LAGERNUMMER_FEHL | LagerNr. | 

```sql
select vs.LagerNummerFehl from amic_v_vorgaenge vs
      where v_id = :ID_V_ID
```

 | | | |
| 1531 | ID_LAGERPLATZ_FEHL | Lagerplatz | 

```sql
select vs.LagerPlatzFehl from amic_v_vorgaenge vs
      where v_id = :ID_V_ID
```

 | | | |
| 1766 | ID_PRUEFMENGE | Prüfmenge | 

```text
Wird nur bei der Erfassung von Vorgängen benötigt und
      kann mit keinem SQLK abgefragt werden.
```

 | | | |
| 1767 | ID_LIEFERWOCHE | Lieferwoche | 

```sql
select vs.V_LieferWoche from amic_v_vorgaenge vs
      where v_id = :ID_V_ID
```

 | | | |
| 1771 | ID_V_BEMTEXT1 | Text1 | 

```sql
select vb.V_BemText1 from VorgBemerkung vb

join amic_v_vorgaenge
      vs on (vb.V_BemId = vs.V_BemId)
where vs.v_id = :ID_V_ID
```

 | | | |
| 1772 | ID_V_BEMTEXT2 | Text2 | 

```sql
select vb.V_BemText2 from VorgBemerkung vb

join amic_v_vorgaenge
      vs on (vb.V_BemId = vs.V_BemId)
where vs.v_id = :ID_V_ID
```

 | | | |
| 1773 | ID_V_BEMTEXT3 | Text3 | 

```sql
select vb.V_BemText3 from VorgBemerkung vb

join amic_v_vorgaenge
      vs on (vb.V_BemId = vs.V_BemId)
where vs.v_id = :ID_V_ID
```

 | | | |
| 1774 | ID_V_BEMTEXT4 | Text4 | 

```sql
select vb.V_BemText4 from VorgBemerkung vb

join amic_v_vorgaenge
      vs on (vb.V_BemId = vs.V_BemId)
where vs.v_id = :ID_V_ID
```

 | | | |
| 1775 | ID_V_BEMTEXT5 | Text5 | 

```sql
select vb.V_BemText5 from VorgBemerkung vb

join amic_v_vorgaenge
      vs on (vb.V_BemId = vs.V_BemId)
where vs.v_id = :ID_V_ID
```

 | | | |
| 1776 | ID_V_BEMLANGTEXT1 | Langtext1 | 

```sql
select vb.V_BemLangText1 from VorgBemerkung vb

join amic_v_vorgaenge
      vs on (vb.V_BemId = vs.V_BemId)
where vs.v_id = :ID_V_ID
```

 | | | |
| 1828 | ID_PARITAETNUMMER | Parität | 

```sql
select vs.ParitaetNummer from amic_v_vorgaenge vs
      where v_id = :ID_V_ID
```

 | | | |
| 1851 | ID_AKTOBJEKT_NUMMER | Objekte | 

```sql
Select bs.Baustnummer from Bauststamm bs
join amic_v_vorgaenge
      vs on (vs.BaustId = bs.BaustId)
where vs.v_id =:ID_V_ID
```

 | | | |
| 1914 | ID_FA_BELEGREFERENZ | Beleg-Referenz | 

```sql
select vs.FA_BelegReferenz from amic_v_vorgaenge vs
      where v_id = :ID_V_ID
```

 | | | |
| 1976 | ID_SPERRFIBU | FibuSperre | 

```sql
select vs.V_SperrFibu from amic_v_vorgaenge vs where
      v_id = :ID_V_ID
```

 | | | |
| 1991 | ID_PRUEFSUMME | Prüfsumme Vorgang | 

```text
Wird nur bei der Erfassung von Vorgängen benötigt und
      kann mit keinem SQLK abgefragt werden.
```

 | | | |
| 4200 | ID_PROD_KUNDNUMMER | Prod.KuNr. | 

```sql
select pv.V_ProdKundNummer from V_ProdVorgang pv
      where v_id = :ID_V_ID
```

 | | | |
| 4204 | ID_PROD_BUCHTYP | Prod.Buchtyp | 

```sql
select pv.V_ProdBuchTyp from V_ProdVorgang pv where
      v_id = :ID_V_ID
```

 | | | |

Felder des Vorgangs mit Weiterreichung an Unterblöcke/Warenpositionen

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
