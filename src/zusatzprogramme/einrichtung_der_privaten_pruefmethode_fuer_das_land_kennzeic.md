# Einrichtung der Privaten Prüfmethode für das Land Kennzeichen im Swift und in der IBAN

<!-- source: https://amic.de/hilfe/_individuale_pruefungbank.htm -->

Mit dieser Prüfung soll verhindert werden, dass sich das Landkennzeichen in einer IBAN von dem Landkennzeichen im Swift Code von der Bank unterscheidet. Des Weiteren soll beim Speichern des Kunden geprüft werden, ob das Landkennzeichen des Kunden zum Landkennzeichen der Bank passt.

Aber es gibt Ausnahmen die trotz negativer Prüfung das Speichern der Daten zulassen.

Folgende Punkte können am Kunden nach der AIS Einrichtung ausgewählt werden.

1. Vendor

2. Ocean Freight vendor

3. Goods/Services delivered from same country as bank country

4. Must have Beneficial ownership from a record

Des Weiteren kann es vorkommen, dass der Swift Code der Bank ein anderes Landkennzeichen enthält als das Landkennzeichen in der IBAN z.B. der IBAN Country Code ist GB und der Swift Country Code ist IM für die Isle of Man. Diese Liste ist im AEZ zu pflegen. Diese Liste wird beim Speichern mit den zu speichernden Daten verglichen. Wurden unterschiedlich Länderkennung in der IBAN und in dem Swift angegeben und diese Kombination steht in der Ausnahme Liste so wird der Datensatz gespeichert andernfalls nicht

<p class="just-emphasize">Einrichtung</p>

Es müssen folgende Punkte eingerichtete werden.

<p class="just-emphasize">Optionen</p>

Hauptmenü > Administration > Steuerung > Optionen

oder Direktsprung **[OPT]** 

Um diese Option anzulegen wird mit **F8** oder ***Neu*** der Pfleger geöffnet. Als Option Name wird dann „***Pruefe_Bankstaat***“ ausgewählt. Unter Bediener wird eingetragen für welchen Bediener diese Option zur Verfügung steht. Als Wert wird der Name z.B. ***p_pruefe_bank_staat*** der Prüf Prozedur eingetragen. 

<p class="just-emphasize">AIS</p>

Hauptmenü > Administration > Werkzeuge > Informationssystem

oder Direktsprung **[AIS]**

Im AIS müssen vier Eingabefelder vom Feldtyp „Check-Box“ angelegt werden. Der Name der Gruppe zu diesen Feldern kann z.B. lauten Kundenaddon004.

Die Namen der Felder müssen wie folgt heißen:

1. ADMVendor

2. OceanFVendor

3. BankCtryisGoodsCtry

4. Beneficialownership

Auf der Registerkarte Datenbeschreibung muss als Herkunftstyp „Relation“ eingetragen werden. Als Relation ist „KundenAddon“ zu wählen. Das Ident Feld wird selbständig gefüllt.

Dann muss in der dritten Variante eine Zuordnung zwischen der Gruppe und der Maske TBUNSTB hergestellt werden.

| Masken Feld | Werte |
| --- | --- |
| Maske | TBKUNSTB |
| Gruppe | z.B. Kundenaddon004 |
| Ident Masken-Feldname/Wert | h.KundId$ |
| Darstellung | Register |

<p class="just-emphasize">AEZ</p>

Als erste muss überlegt werden, welches AEZ benutzt werden soll. In diesem Beispiel ist es aez2.

In der privaten Anwendung AEZ2 benötigen wir zwei Felder:

1. SwiftCountryCode

2. IBANCountryCode

Diese beiden Felder werden jetzt über **[OSQL]** in der Tabellen aeinszusatz2 angelegt.

```sql
alter table aeinszusatz2 add SwiftCountryCodechar(2) , add IBANCountryCode char(2)
```

Nach dem diese beiden Felder erfolgreich angelegt worden sind muss jetzt der Pfleger für die Maske mit **[AIS]** eingerichtet werden.

Nach dem der Pfleger erfolgreich erstellt worden ist. Muss noch die Auswahlliste für den Pfleger angepasst werden.

Direktsprung **[AEZ2]**

Mit **Shift+F2** kann jetzt der SQL Text für die Abfrage abgeändert werden, so dass die beiden neuen Felder in der Auswahlliste zu sehen sind.

```sql
// Auswahllistenfunktion : Aeins Zusatzinformationen
TITLE Aeins Zusatz 2
INFO Aeins Zusatz 2
MASK AW_MASK
FIELD Ident,Ident,I4,4
FIELD SwiftCountryCode, SwiftCountryCode, char, 20
FIELD IBANCountryCode, IBANCountryCode, char, 20
SQL
 select :FIELDS
 from AeinsZusatz2
 where 1=1
  :AUSW_IDENT // and (  ident between ':PAR1' and ':PAR2')
 order by Ident
RETURN Ident
IDENT Ident
IDSQL select *
IDSQL from AeinsZusatz2
IDSQL where Ident=:ID1
```

<p class="just-emphasize">Private Prüfprozedur</p>

```sql
-- Priv. Prozedur p_pruefe_bank_staat --- TM 18.06.2012
--
-- Beschreibung:
--
--
--
CREATE PROCEDURE p_pruefe_bank_staat (
                                        in in_kundid integer
                                       ,in in_BankIBAN char(30)
                                       ,in in_BankNummer integer default 0
                                       ,in in_modus integer
                                       ,in in_ADMVendor smallint
                                       ,in in_OceanFVendor smallint
                                       ,in in_BankCtryisGoodsCtry smallint
                                       ,in in_Beneficialownership smallint
                                       ,in in_StaatNummer integer
                                       ,in in_zusatz_parameter1 smallint default 0
                                       ,in in_zusatz_parameter2 smallint default 0
                                       ,in in_zusatz_parameter3 smallint default 0
                                     )
                               Result(
                                         Fehler integer
                                        ,Fehlertext char(255)
                                     )
--
BEGIN
--Sollten stehen gelassen werden, Variablen werden im WHEN OTHERS verwendet
  declare dc_StaatISOBezeich char(4);
  declare dc_vorhanden integer;
  declare dc_BankStaatISOBezeich char(4);
  declare dc_Bank char(255);
  declare dc_BankSwift char(11);
  declare dc_StaatNummer integer;
  declare dc_BankStaatNummer integer;
  declare dc_LandCodeSwift char(2);
  declare dc_LandCodeIBAN char(2);
  declare dc_KundNummer integer;
  declare dc_Fehler integer;
  declare dc_FehlerText char(255);
  declare dc_FehlerText2 char(255);
  declare dc_Fehlertemp char(255);
  declare dc_IBAN char(40);
  declare dc_SWIFT char(11);
  declare ECX_ERR_NOTFOUND EXCEPTION FOR SQLSTATE '02000';
  declare dc_ErrorMsg LONG VARCHAR;
  declare dc_SQLCODE integer;
  declare dc_SQLSTATE CHAR(10);
  set dc_Fehler = 0;
  set dc_FehlerText = '';
  select KundNummer into dc_KundNummer from KundenStamm where KundId = in_KundId;
  if (in_ADMVendor = 0 and in_OceanFVendor = 0 and in_BankCtryisGoodsCtry = 0  and in_Beneficialownership = 0) then
    select StaatISOBezeichnung, sts.StaatNummer into dc_StaatISOBezeich, dc_StaatNummer from StaatStamm sts
      join AnschriftStamm ans on ans.StaatNummer = sts.StaatNummer
      join KundenStamm ku on ku.AdressIdHauptAdr = ans.AdressId
      where KundId = in_kundid;
      if in_StaatNummer >-1 then
        set dc_StaatNummer = in_StaatNummer;
      end if;
    if in_modus = 1 then -- Valid funktionen
      if in_BankNummer > 0 then
        select bs.BankStaat, sts.StaatISOBezeichnung, bs.Banknummer || ' ' || bs.BankBezeich, bs.BankSwift into
          dc_BankStaatNummer, dc_BankStaatISOBezeich, dc_Bank, dc_BankSwift from Bankenstamm bs
         join StaatStamm sts on (sts.StaatNummer = bs.BankStaat)
            where BankNummer = in_BankNummer;
        if dc_BankStaatNummer != dc_StaatNummer then
           set dc_Fehler = 1;
           set dc_Fehlertext = amic_func_sprachtexte('a','b','Die ISO Bezeichnung des Staates %s passt nicht zum Landkennzeichen der Bank %s',-1,
              dc_StaatISOBezeich, dc_Bank);
        end if;
      end if;
      if in_BankIBAN != '' then
        set dc_LandCodeIBAN = substring(in_BankIBAN,1,2);
        set dc_LandCodeSwift = substring(dc_BankSwift,5,2);
        if dc_LandCodeIBAN != dc_LandCodeSwift then
          select first 1 into dc_vorhanden from aeinszusatz2
            where IBANCountryCode = dc_LandCodeIBAN and SwiftCountryCode  = dc_LandCodeSwift;
          if isnull(dc_vorhanden,0) = 0 then
            set dc_Fehler = 1;
            set dc_FehlerText2 = amic_func_sprachtexte('a','b','Der IBAN Country Code %s unterscheidet sich vom Swift Country Code %s.',-1,
             dc_LandCodeIBAN, dc_LandCodeSwift);
          end if;
        end if;
      end if;
    end if;
    if in_modus = 2 then --Massentest im Kundenstamm
      Begin
        declare StaatCr cursor for
          select sts.StaatISOBezeichnung, sts.StaatNummer, bs.BankNummer || ' ' || bs.BankBezeich  from StaatStamm sts
            join BankenStamm bs on ( bs.BankStaat = sts.StaatNummer )
            join KundenBank kb  on ( kb.BankNummer = bs.BankNummer)
            join KundenStamm ku on ( ku.kundid = kb.kundid)
            where ku.KundId = in_kundid;
        open StaatCr;
          Crsr:: loop
            fetch next StaatCr into dc_BankStaatISOBezeich, dc_BankStaatNummer, dc_Bank;
            if sqlstate <> ECX_ERR_NOTFOUND then
               if (dc_StaatNummer != dc_BankStaatNummer) then
                 set dc_Fehler = 1;
                 set dc_FehlerText = '-';
                 call AMIC_FEHLERPROT( 20, amic_func_sprachtexte('a', 'b', 'Prozedur', -1)
                ,amic_func_sprachtexte('a','b','Beim Kunden %s ist das Landkennzeichen des Staates %s unterschiedlich zum Landkenzzeichen der Bank %s',-1
                ,dc_KundNummer ,dc_StaatISOBezeich, dc_Bank ));
               end if;
            else
              leave Crsr;
            end if;
          end loop Crsr;
        close StaatCr;
      end;
      Begin
        declare BankCrs cursor for
          select kb.BankIBAN, BankSwift, bs.BankNummer || ' ' || bs.BankBezeich from BankenStamm bs
            join KundenBank kb on ( kb.BankNummer = bs.BankNummer)
            where KundId = in_KundId;
        open BankCrs;
          Crsr1:: loop
            fetch next BankCrs into dc_IBAN, dc_SWIFT, dc_Bank;
            if sqlstate <> ECX_ERR_NOTFOUND then
              set dc_vorhanden = null;
              set dc_LandCodeIBAN = substring(dc_IBAN,1,2);
              set dc_LandCodeSwift = substring(dc_SWIFT,5,2);
              if dc_LandCodeIBAN != dc_LandCodeSwift then
                select first 1 into dc_vorhanden from aeinszusatz2
                  where IBANCountryCode = dc_LandCodeIBAN and SwiftCountryCode  = dc_LandCodeSwift;
                if isnull(dc_vorhanden,0) = 0 then
                  set dc_Fehler = 1;
                  set dc_FehlerText2 = '-';
                  call AMIC_FEHLERPROT( 20, amic_func_sprachtexte('a', 'b', 'Prozedur', -1)
                     ,amic_func_sprachtexte('a','b','Der IBAN Country Code %s unterscheidet sich vom Swift Country Code %s.',-1, dc_LandCodeIBAN, dc_LandCodeSwift));
                end if;
              end if;
            else
              leave Crsr1;
            end if;
          end loop Crsr1;
        close BankCrs;
      End;
    end if;
  end if;
  if in_Modus = 2 then
    if isnull(dc_FehlerText,'') != '' and isnull(dc_FehlerText2,'') = '' then
      set dc_Fehlertemp = amic_func_sprachtexte('a','b', 'Daten können nicht gespeichert werden, da das Landkennzeichen des Staates nicht mit dem Landkenzzeichen der Bank übereinstimmt.'||
                      'Für eine genau Fehleranlayse schauen Sie bitte in das Fehlerprotokoll',-1);
    end if;
    if isnull(dc_FehlerText,'') = '' and  isnull(dc_FehlerText2,'') != '' then
      set dc_Fehlertemp = amic_func_sprachtexte('a','b', 'Daten können nicht gespeichert werden, da der Country Code der IBAN nicht mit dem Country Code des Swift übereinstimmt.'||
                      'Für eine genau Fehleranlayse schauen Sie bitte in das Fehlerprotokoll',-1);
    end if;
    if isnull(dc_FehlerText,'') != '' and  isnull(dc_FehlerText2,'') != '' then
      set dc_Fehlertemp = amic_func_sprachtexte('a','b', 'Daten können nicht gespeichert werden, da der Country Code der IBAN nicht mit dem Country Code des Swift übereinstimmt,'||
                        'und das Landkennzeichen des Staates nicht mit dem Landkenzzeichen der Bank übereinstimmt.',-1);
    end if;
  end if;
  if in_Modus = 1 then
    if isnull(dc_FehlerText,'') != '' and isnull(dc_FehlerText2,'') = '' then
      set dc_Fehlertemp = dc_FehlerText;
    end if;
    if isnull(dc_FehlerText,'') = '' and  isnull(dc_FehlerText2,'') != '' then
      set dc_Fehlertemp = dc_FehlerText2;
    end if;
    if isnull(dc_FehlerText,'') != '' and  isnull(dc_FehlerText2,'') != '' then
      set dc_Fehlertemp =  dc_FehlerText || ' ' || dc_FehlerText2;
    end if;
  end if;
  select dc_Fehler as Fehler, dc_Fehlertemp as FehlerText from dummy;
-- Hier kann die Verarbeitung beginnen
--
--
--
--Sollte stehen gelassen werden, damit die Fehlermeldungen ins FEHLP gehen
EXCEPTION
  when others then
    Select  ERRORMSG(), SQLCODE, SQLSTATE into dc_ErrorMsg, dc_SQLCODE, dc_SQLSTATE;
    call AMIC_FEHLERPROT( 20, amic_func_sprachtexte('a', 'b', 'Prozedur', -1) , amic_func_sprachtexte('a','b','Beim Ausführen der Prozedur %s ist ein Fehler aufgetreten.', -1, 'p_pruefe_bank_staat')
    || '\n\n'|| 'SQLCODE:: ' || dc_SQLCODE|| ' [' || dc_SQLSTATE || ']'|| '\n'|| dc_ErrorMsg, null);
END
```
