# Private Procedure

<!-- source: https://amic.de/hilfe/_cescannerprivproc.htm -->

```sql
// Priv. Procedure p_artikelanzeige
//
// Beschreibung: Diese Funktion übergibt den gescannten wert an die IB_Box
//
Create procedure p_artikelanzeige (  in in_Aktionstyp integer,
                                    in in_aktionswert char(255),
                                    in in_ident integer,
                                    in in_positionsIdent integer,
                                    in in_scannernummer char(40),
                                    in in_kommando_scanident integer,
                                    in in_AnzahlImBlock integer,
                                    in in_Blockzaehler integer,
                                    in in_letzte_aktion integer,
                                    in in_Aktionstext char(100),
                                    in in_Kopftext1 char(100),
                                    in in_Kopftext2 char(100),
                                    in in_reaktionstyp char(5),
                                    in in_lagernummer integer,
                                    in in_bedienerid integer,
                                    in in_protokoll char(100),
                                    in in_feldid integer,
                                    in in_scanident integer,
                                    in in_klassnummer integer,
                                    in in_nummer integer,
                                    in in_testflag integer,
                                    in in_diese_positionsnummer integer)
BEGIN
  declare dc_scrollbar integer;
  declare dc_neuzeilennummer integer;
  declare dc_statustext char(1024);
  declare dc_Aktionstext char(100);
  declare dc_status integer;
  set dc_neuzeilennummer = 1;
  set dc_statustext = '';
  set dc_Aktionstext  = '';
  set dc_status = 0;
  if ( isnull(dc_neuzeilennummer,0)) = 0 then
    set dc_neuzeilennummer = 1;
  end if;
  select ALSWert into dc_scrollbar from AeinsLastSetting where
ALSAnwendung='Tcpip_Scanner' and ALSEintrag ='scanner_scrollbar' and
BedienerKurz= '0';
if ( in_Aktionstyp = -4 or in_Aktionstyp = -6 or in_Aktionstyp = -7 or
     in_Aktionstyp = 1) then
    set dc_status = 4;
    if dc_scrollbar = 1 then
      set   dc_statustext= 'IB_SCANNER_ANZEIGE;TOP=9&SEKUNDS='
      ||in_aktionswert|| '&ZEILENNUMMER=' ||dc_neuzeilennummer;
    else
      set dc_statustext = 'IB_SCANNER_ANZEIGE;TOP=50&SEKUNDS=
      '||in_aktionswert|| '&ZEILENNUMMER=' ||dc_neuzeilennummer;
    end if;
  end if;
// Berechnen der Zeilennummer
  if dc_scrollbar = 1 then
    If ( in_Aktionstyp = -1 and in_aktionswert = 'KEYUP' ) then
      if  in_disp_zeilennummer >= 9 then
        set dc_neuzeilennummer = in_disp_zeilennummer - 9;
      end if;
      update datenstromscanner set
      statustext = 'IB_SCANNER_ANZEIGE;TOP=9&SEKUNDS='||in_aktionswert||
      '&ZEILENNUMMER=' ||dc_neuzeilennummer,
      status = 4,
      zeilennummer = dc_neuzeilennummer
      where ident=in_ident;
      update tcpip_scanner set letzte_zeilennummer = dc_neuzeilennummer
      where tcpip_adresse  = in_scannernummer and scanident =
      in_kommando_scanident;
      set dc_statustext = 'IB_SCANNER_ANZEIGE;TOP=50&SEKUNDS='
      ||in_aktionswert|| '&ZEILENNUMMER=' ||dc_neuzeilennummer;
    end if;
    If ( in_Aktionstyp = -1 and in_aktionswert = 'KEYDOWN' ) then
      set dc_neuzeilennummer = in_disp_zeilennummer + 9;
      update datenstromscanner set
      statustext = 'IB_SCANNER_ANZEIGE;TOP=9&SEKUNDS='||in_aktionswert||
      '&ZEILENNUMMER=' ||dc_neuzeilennummer,
      status = 4,
      zeilennummer = dc_neuzeilennummer
      where ident=in_ident;
      update tcpip_scanner set letzte_zeilennummer = dc_neuzeilennummer
      where tcpip_adresse  = in_scannernummer and scanident =
      in_kommando_scanident;
      set dc_statustext = 'IB_SCANNER_ANZEIGE;TOP=50&SEKUNDS='
      ||in_aktionswert|| '&ZEILENNUMMER=' ||dc_neuzeilennummer;
    end if
  end if;
  set dc_Aktionstext  = in_Aktionstext;
  //
  // Übergabe der Daten an den Scanner
  //
  update datenstromscanner set
    statustext = dc_statustext,
    Aktionstext = 'Private Anwendung',
    Kopftext1=dc_Aktionstext,
    Kopftext2=in_Kopftext2,
    status = dc_status,
    StatusMelodie = '',
    ReaktionsTyp = in_reaktionstyp
    where ident=in_ident;
END
```
