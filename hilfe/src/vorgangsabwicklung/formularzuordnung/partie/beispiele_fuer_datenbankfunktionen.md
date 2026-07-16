# Beispiele für Datenbankfunktionen

<!-- source: https://amic.de/hilfe/beispielefrdatenbankfunktionen.htm -->

### Beispiel für eine Datenbankfunktion für die Partieverteilung:

```sql
CREATE PROCEDURE "admin"."p_PartieAutoSuche"
(
     -- Die Parameter wurden automatisch mit der  Einrichtung unter FRZ/Partie/DB Prozedur für Verteilung
     -- zusammengestellt( Cursor im Feld DB Prozedur für Verteilung positionieren, SF8, Parameter auswählen / Testfunktion ausführen)
     in in_Vorgangsklasse   integer
   , in in_Unterklasse    integer
   , in in_VerteilMenge   numeric (15,6)
   , in in_Mengeneinheit    integer
   , in in_ArtikelId    integer
   , in in_KundId   integer
   , in in_EKVK_kennzeichen   integer
   , in in_AbgrenzDatum   date
   , in in_ArtikelStammId   integer
   , in in_NurDiesePartie_Id   integer
   , in in_LagerPLatzNummer   integer
)
-- Das Resultset muss mindestens die hier aufgeführten Felder haben. Die Reihenfolge
-- ist nicht wichtig - die Namensgebung aber um so mehr !
-- eine eine Zurückgegeben zeile entspricht einer Partiezuordnung
-- es können mehrere, eine oder auch keine Zeilen zurückgegeben werden
result
(
   PartieId integer,                    -- Die Partieid ist absolut wichtig
   PartieNummer integer,
   PartieBezeich char(40),
   PartieAbdatum date,
   PartieEKP_Kennz smallint,            -- wichtige Kennzeichen für die Partiepreisermittlung
   PartieVKP_Kennz smallint,            -- dito
   WaehrNummer smallint,                -- in welcher Währung wird die Partie geführt
   Me_NummerPartie integer,             -- die Mengeneinheit der Partiebestandsbuchführung
   PartieArtiPosit integer,             -- wichtig für die eindeutige Identifiziere des Artikels in der Partie
   LagerplatzNummer integer,            -- von welchem Lagerplatz soll abgebucht werden
   sort_lagerplatz integer,             -- Hilfsfeld zum Sortieren eigenen und fremden Lagerplätze ( zum Lagerplatz der Warenposition)
   RestMenge_VK numeric(15,4 )          -- Benennung dieses Feldes ist historisch gewachsen. Es gibt sowohl bei Einkauf als auch
                                        -- bei Verkauf die zugeordnete Menge dieser Partiezeile an !!!
)
BEGIN
-- lokale temporäre Tabelle für Zwischenergebnisse
declare local temporary table p_result
(
   PartieId integer,
   PartieNummer integer,
   PartieBezeich char(40),
   PartieAbdatum date,
   PartieEKP_Kennz smallint,
   PartieVKP_Kennz smallint,
   WaehrNummer smallint,
   Me_NummerPartie integer,
   PartieArtiPosit integer,
   LagerplatzNummer integer,
   sort_lagerplatz integer,
   RestMenge_VK numeric(15,4 )
) on commit preserve rows;
   declare dc_err_notfound exception for sqlstate value '02000';
   declare dc_PartieId integer;
   declare dc_PartieNummer integer;
   declare dc_PartieBezeich char(40);
   declare dc_PartieAbdatum date;
   declare dc_PartieEKP_Kennz smallint;
   declare dc_PartieVKP_Kennz smallint;
   declare dc_WaehrNummer smallint;
   declare dc_Me_NummerPartie integer;
   declare dc_PartieArtiPosit integer;
   declare dc_LagerplatzNummer integer;
   declare dc_sort_lagerplatz integer;
   declare dc_RestMenge_VK numeric(15,4) ;
   declare dc_buchmenge numeric(15,4);
   declare me_Nummer_Artikel integer;
   declare me_nummer_effektiv integer;
-- Ein Cursor für alle potentiell möglichen Partien
  declare partiedaten dynamic scroll cursor for
  -- Hinweis: der With-Anteile selektiert  alle  formal passenden Partien
  -- gemäß der   Bedeutung in Aeins
  with preselect( partieid,artikelid,partieartiposit) as
  (
    select s.partieid, a.artikelid,min(a.partieartiposit)
    from PartieArtikel a join Partiestamm s on (a.PartieId = s.PartieId)
    where
      ( ((a.ArtikelId = in_ArtikelStammId) and (a.PartieArtiTyp = 4))
      or((a.ArtikelId = in_ArtikelId) and (a.PartieArtiTyp = 5)))
    and (s.PartieErlKennz = 0)
    and (s.PartieLoeKennz = 0)
    and (s.PartieSperrKennz = 0)
    and (s.PartieTyp = 1)
    and (in_AbgrenzDatum    between s.PartieAbDatum and s.PartieBisDatum)
    and ( in_NurDiesePartie_Id  = 0 or in_NurDiesePartie_Id  = s.PartieId)
    and ( ( (in_EKVK_kennzeichen = 2) and ( (s.PartieKunLiKennz = 0)     //:OR_IS_LGU_ARU
        or (in_KundId in ( select KundId from PartieKundListe l
                         where (l.PartieId = s.PartieId) ) ) ) )
      or ( (in_EKVK_kennzeichen != 2) and ( (s.PartieLieLiKennz = 0)    // :OR_IS_LGU_ARU
        or (in_KundId in ( select KundId from PartieLiefListe l
                     where (l.PartieId = s.PartieId) ) ) ) ) )
    and ( (s.PartieWagrKennz = 0) )
    group by s.partieid, a.artikelid
  )
   -- Zusammenstellen der Ergebnisse im eigentlichen Select
  select
  s.PartieId,s.PartieNummer,s.PartieBezeich,s.PartieAbdatum,s.PartieEKP_Kennz,s.PartieVKP_Kennz,
  s.WaehrNummer,s.ME_Nummer as Me_NummerPartie,a.PartieArtiPosit,pbp.LagerplatzNummer,
  if ( pbp.LagerplatzNummer = in_LagerPLatzNummer )
    then cast(0 as integer)
    else cast(1 as integer)
  endif as sort_lagerplatz,   -- bewirkt in der Itembox Sortierung: zunächst Partien passend zum vorgegebenen LAgerplatz
  AMIC_FUNC_PARTIEBESTAND(ps.PartieId,in_ArtikelId,pbp.lagerplatznummer,0,1)
    as  RestMenge_VK
  from preselect ps
  join PartieArtikel a on a.partieid = ps.partieid and a.partieartiposit = ps.partieartiposit
  join Partiestamm s on (a.PartieId = s.PartieId)
  LEFT join partiebestandpur pbp on pbp.partieid = ps.partieid and pbp.artikelid = in_Artikelid
  left outer join partiestammaddon padd on(ps.partieid=padd.partieid)
  where 1 = 1
  and Restmenge_VK > 0
  //AND_ONLY_ID
  order by isnull(padd.mhd,s.PartieAbDatum) , s.PartieNummer, a.PartieArtiPosit;
  // Einige Vorselects
  select me_nummerLager into me_Nummer_Artikel   from artikelstamm ars join
  mengeinhgruppe mg on mg.me_grupnummer  =  ars.me_grupnummer and ars.Artistammid = in_ArtikelStammId;
  // Steupa für Mengenbuchführung in ME der Partie
  IF VAREXISTS('db_SPA606') = 0 THEN
    create variable db_SPA606 integer;
    select   AMIC_STEUPA_WERT_TODAY( 606 ) into db_SPA606 from dummy  ;
  END IF;
  open  partiedaten;
  workloop::
  loop
    fetch next partiedaten
    into
      dc_PartieId ,
      dc_PartieNummer,
      dc_PartieBezeich ,
      dc_PartieAbdatum ,
      dc_PartieEKP_Kennz ,
      dc_PartieVKP_Kennz ,
      dc_WaehrNummer ,
      dc_Me_NummerPartie ,
      dc_PartieArtiPosit,
      dc_LagerplatzNummer ,
      dc_sort_lagerplatz ,
      dc_RestMenge_VK ;
    if sqlstate = dc_err_notfound then
      leave workloop;
    end if;
    // schon genug Menge zurückgegeben
    if in_VerteilMenge <= 0 then
      leave workloop;
    end if;
    set dc_buchmenge = 0;
    // Umrechnung Mengeneinheit aus der Partie
    // in die Mengeneinheit der zu verteilenden Menge
    set me_nummer_effektiv = me_Nummer_Artikel;
    if ( db_SPA606 = 1  and isnull(dc_Me_NummerPartie,0)  != 0 )  then // Me Nummer der Partie soll wirken für Bestandsmenge
      set  me_nummer_effektiv = dc_Me_NummerPartie;
    end if;
    if ( me_Nummer_Artikel is not null ) then
      set dc_RestMenge_VK = Amic_Me_Umrechnung( dc_RestMenge_VK,me_nummer_effektiv,0,in_Mengeneinheit,0,in_Artikelid,4 );
    end if;
    if ( dc_RestMenge_VK >= in_VerteilMenge ) then
       set dc_buchmenge = in_VerteilMenge;
    else
     set dc_buchmenge = dc_RestMenge_VK;
    end if;
    if ( dc_buchmenge > 0 ) then
      insert into p_result values
      (
        dc_PartieId ,
        dc_PartieNummer,
        dc_PartieBezeich ,
        dc_PartieAbdatum ,
        dc_PartieEKP_Kennz ,
        dc_PartieVKP_Kennz ,
        dc_WaehrNummer ,
        dc_Me_NummerPartie ,
        dc_PartieArtiPosit,
        dc_LagerplatzNummer ,
        dc_sort_lagerplatz ,
        dc_buchmenge
      );
      // den noch zu verteilenden Rest anpassen
      set in_VerteilMenge = in_VerteilMenge - dc_buchmenge;
    end if ;
  end loop workloop;
  close partiedaten;
  commit;
  --Rückgabe der Ergebnisse
  select * from p_result;
END
```

### Beispiel für eine Datenbankfunktion für Gebindeparameter:

```sql
// FUNCTION AMIC_V_UKlassGebFak1Proc
//
//---------------------------------------------------------------------
CREATE FUNCTION AMIC_V_UKlassGebFak1Proc (
  IN ArtikelId INTEGER,
  IN MeNummer INTEGER,
  IN Lagerplatznummer INTEGER,
  IN Menge DECIMAL(15,4))
  RETURNS DECIMAL(15,4)
BEGIN
  DECLARE anzahl_partien INTEGER;
  DECLARE ergebnis DECIMAL(15,4);
  set anzahl_partien = (select count(*) from Temp_Partie_Uebergabe);
  if  anzahl_partien > 0 THEN
   // für 'mit Partien' temporäre Tabelle Temp_Partie_Uebergabe nutzen
   // für jeden einzelnen Datensatz die Menge anpassen
   // Menge möglichst auf zwei Nachkommastellen runden
   update Temp_Partie_Uebergabe set menge = round(menge * 1.276,2);
   commit;
   set ergebnis = (select sum(isnull(menge,0)) from Temp_Partie_Uebergabe);
  ELSE
    // neue Gesamtmenge zurückgeben
    set ergebnis = round (menge * 1.5,2);
  END IF ;
  return ergebnis;
END
```
