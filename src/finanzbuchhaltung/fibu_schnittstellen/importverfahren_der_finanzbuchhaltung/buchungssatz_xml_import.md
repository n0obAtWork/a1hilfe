# Buchungssatz XML Import

<!-- source: https://amic.de/hilfe/buchungssatzxmlimport.htm -->

Über diese Schnittstelle können Buchungssätze, die in einer [XML Datei](./buchungssatz_xml_import.md#BSXI_XMLSTRUKTUR) enthalten sind, importiert werden. Die Daten werden in die Tabelle [FIBUIMPORT](./index.md) geschrieben.

<p class="just-emphasize">Auswahlliste</p>

Die einzuspielenden und eingespielten Dateien werden in der Tabelle Buchungssatzimport gespeichert, diese Dateien können unter [BSSIX] oder Finanzbuchhaltung -> Abschlussarbeiten -> *DATEV/Import/Export* -> Buchungssatz XML Import angezeigt werden.

In der Variante „Buchungssatzimport“ können importierte Dateien gelöscht werden. Das Löschen hier hat keinen Einfluss auf bereits erfolgreich in die Finanzbuchhaltung eingespielte Belege oder Daten die noch im FIBUIMPORT stehen.

Des Weiteren wird angezeigt, ob eine Datei erfolgreich in den FIBUIMPORT eingespielt wurde. Ist eine Datei nicht erfolgreich in die FIBUIMPORT eingespielt worden, da z.B. die Kontonummer oder die Gegenkontonummer nicht im System eingerichtet sind, wird beim nächsten Importlauf noch einmal versucht diese Datei einzuspielen.

Um einen Import durchzuführen, klicken Sie bitte auf ***Buchungssatz Import XML*** **F9**.

<p class="just-emphasize">Bereitstellung der XML Datei</p>

Die zu importierende Datei mit den Buchungsätzen muss auf dem Rechner vorhanden sein, damit diese in die Tabelle Buchungssatzimport gespeichert werden können.

Ist die Datei nicht auf dem Rechner vorhanden, so kann diese per VBA, VBS Skript geladen und gespeichert werden, oder per Explorer kopiert werden.

Den Namen des VBA oder VBS Skriptes wird in den [Einrichterparameter](../../../firmenstamm/einrichterparameter/buchungssatz_import_xml_epa_ledgerimport.md) der Maske hinterlegt.

<p class="just-emphasize">Privates VBS oder VBA</p>

Ein Beispiel für das Laden und Speichern einer Datei von einem FTP Server finden Sie unter dem Direktsprung [VBA]. Der Name des VBA Skriptes lautet AMIC_FTP_LEDGERIMPORT. Als Parameter werden folgende Werte übergeben:

1\. /GUID=Die ID des Import ( Siehe Auswahlliste )

2\. /File=Der Dateiname oder Dateipfad. Bei Pfadangabe werden alle nicht eingespielten XML-Dateien Importiert.

<p class="just-emphasize">Fehlerbehandlung im Skript:</p>

Um der Import Schnittstelle mitzuteilen, dass in dem Skript ein Fehler aufgetreten ist wird die JVAR JVAR_TEMPWERT_2 mit Besitzer 3551 gesetzt. Der Wert 1 bedeutet, dass kein Fehler aufgetreten ist. Desweitern sollte in dem Skript ein Eintrag in das Fehlerprotokoll mit dem Bereich FiBuImport28 gemacht werden, da die Importschnittstelle nach dem ausführen des Skriptes alle Fehler aus diesem Bereich auf einer speziellen Maske angezeigt wird.

Das Setzten der JVAR mit 0 im Script bewirkt, dass der Import für diese Datei nicht gestartet werden kann. Existieren mehrerer Dateien in der Tabelle Buchungssatzimport, wird versucht die nächste einzuspielen.

<p class="just-emphasize">Datenaufbereitung</p>

Die XML Datei mit den Buchungssätzen wird in einer Datenbankprozedur([fibu_ledgerimport](./buchungssatz_xml_import.md#BSXI_BEISPIELPROZEDUR)) so aufgearbeitet, dass die Daten eingelesen und importiert werden können.

Anstelle der Standardprozedur kann die Prozedur auf der Maske durch eine private Prozedur ersetzt werde (Auswahl mit F3).

<p class="just-emphasize">Wichtig</p>

Das Hauptkonto und das Gegenkonto müssen immer numerisch sein. In der Beispiel [XML Struktur](./buchungssatz_xml_import.md#BSXI_XMLSTRUKTUR) ist das Hauptkonto alphanumerisch. Um die Datei trotzdem erfolgreich zu importieren, wird eine Zuweisung von dem alphanumerischen zu dem numerischen Konto benötigt. Um eine Zuweisung zwischen dem alphanumerischen und numerischen Konto herzustellen rufen Sie bitte den Importumsetzer auf (Direktsprung [IMPUM]) auf.

Als erstes wird in der Variante Import-Schlüssel über F8 ein neuer Schlüssel angelegt.

In die Felder Referenz und Schlüsselklasse wird z.B. die Nummer 20 eingetragen. Als Klasse wird „Buchungssatz Import“ ausgewählt.

Nachdem hier eine neue Schlüsselklasse angelegt worden ist, wird in der Variante Import-Umsetzer mit F8 ein neuer Datensatz angelegt.

Es können folgende Felder in der Standard Prozedur umgeschlüsselt werden.

Hauptkonto

Gegenkonto

KostStelNummer

KSTRNummer

| **Maskenfelder** | **Bedeutung** |
| --- | --- |
| Eingangsschlüssel | Hier wird der Wert der alphanummerischen Kontonummer eingetragen. In der Beispieldatei ist dies 047A11. |
| Umsetzung | In die Umsetzung wird die numerische Kontonummer eingetragen. In der Beispieldatei ist dies 123456 |
| Schlüsselklasse | Hier wird die Schlüsselklasse wie z.B. 20 eingetragen. |
| Info-text | Hier kann ein Informationstext eingetragen werden, wie z.B. „Ledger Import“. |

In den [Einrichterparametern](../../../firmenstamm/einrichterparameter/buchungssatz_import_xml_epa_ledgerimport.md) der Maske des Buchungssatz XML Import wird die Schlüsselklasse hinterlegt.

Werden mehrere Dateien mit einem Skript geladen, so muss in den [Einrichterparametern](../../../firmenstamm/einrichterparameter/buchungssatz_import_xml_epa_ledgerimport.md) der Parameter „Dateiprüfung und Einspielung passiert im privaten VBA oder VBS Script“ auf **Ja** gestellt werden. Dann reicht es aus in der Maske Buchungssatz XML Import nur den Pfad anzugeben, wo die Dateien zwischengespeichert werden sollen. Dafür muss dann die Dateinamensbehandlung im Skript stattfinden.

<p class="just-emphasize">Maskenfelder</p>

| Name | Bedeutung |
| --- | --- |
| Name der Importdatei | Pfad zum Speicherort der XML Datei. Der Pfad muss auch angegeben werden, wenn die Datei per VBA oder VBS gespeichert wurde. |
| Name der Importprozedur | Name der Prozedur, welche mir die Daten aus der XML Datei zurückliefert. Hier kann auch eine private Prozedur hinterlegt werden. |
| Nummernkreis | Nummernkreis für das Einspielen der Buchungssätze. |
| Arbeitsschritt | Informationsfenster welches die nächsten Schritte anzeigt. |

Um einen Import zu starten klicken Sie bitte in der Optionbox auf „Import Starten“ oder drücken Sie die Taste F9.

 

<p class="just-emphasize">XML Struktur</p>

Die XML Struktur muss genauso wie in diesem Beispiel aufgebaut sein. Bei den XML Tags muss auf Groß und Klein Schreibung geachtet werden.

```xml
<LedgerImport>

<Account>

<FiBuV_Klasse>6</FiBuV_Klasse>

<HauptKonto>047A11</HauptKonto>

<HauptText>FOREIGNEXCHANGE</HauptText>

<GegenKonto>123456</GegenKonto>

<GegenText>FX
IDT Clearing</GegenText>

<Betrag>0128053.00</Betrag>

<WBetrag>033000.00</WBetrag>

<SollHaben>2</SollHaben>

<BelDatum>2010-04-26</BelDatum>

<JahrNummer></JahrNummer>

<PeriNummer></PeriNummer>

<KostStelNummer></KostStelNummer>

<KSTRNummer></KSTRNummer>

<FiBuV_FremdNr></FiBuV_FremdNr>

<PaginierNr></PaginierNr>

<FiBuVPW_Kurs>0.25770501</FiBuVPW_Kurs>

<FiBuVPW_RechFormel>0</FiBuVPW_RechFormel>

<FiBuVPW_Faktor>1</FiBuVPW_Faktor>

<FiBuVPW_Typ>2</FiBuVPW_Typ>

<WaehrISOCode>
EUR </WaehrISOCode>

</Account>
</LedgerImport>
```

 

<p class="just-emphasize">Beispielprozedur</p>

Die Beispielprozedur „fibu_ledgerimport“ entpackt die XML Struktur und speichert diese in einer lokalen temporären Tabelle zwischen. Sobald sich die Daten in der Tabelle befinden, können diese dann dort abgeändert werden. 

In der Beispiel Prozedur werden die Jahrnummer und die Perinummer versorgt, sowie die Währnummer anhand des Währung ISO-Code aus dem Währungsstamm gezogen.

Bei einer privaten Prozedur müssen immer diese drei Übergabeparameter angegeben werden.

| Parameter | Bedeutung |
| --- | --- |
| in_GUID | GUID für das Suchen der Datei in der Tabelle amic_datei  
4F160BB2-810F-4D83-94DA-0CF5205DE060 |
| in_NumkreisNummer | Nummernkreis für das Einspielen der Buchungssätze |
| In_ImportSchluessel | Die Schlüsselklasse des Import Umsetzers |

Als Ergebnis erwarten wir folgende Paramater von der Prozedur zurück.

| Name | Typ | Bedeutung |
| --- | --- | --- |
| IDENT | integer | Nummer des Satzes |
| NumKreisNummer | integer | Nummernkreis für das Einspielen der Buchungssätze |
| HauptKonto | integer | Hauptkonto |
| HauptText | Char(100) | Text |
| GegenKonto | Integer | Gegenkonto |
| GegenText | Char(100) | Text |
| FiBuVP_Betrag | Numeric(15,4) | |
| FiBuVPW_Betrag | Numeric(15,4) | |
| FiBuVP_SollHaben | integer | |
| FiBuV_Datum | date | |
| JahrNummer | integer | Jahrnummer |
| PeriNummer | integer | Periodennummer |
| KostStelNummer | integer | Kostenstelle |
| KSTRNummer | integer | Kostenträger |
| FiBuV_FremdNr | Char(20) | |
| FiBuV_PaginierNr | Char(40) | |
| FiBuVPW_Kurs | Numeric(15,6) | |
| FiBuVPW_RechFormel | integer | |
| FiBuVPW_Faktor | Numeric(15,4) | |
| FiBuVPW_Typ | integer | |
| Waehrnummer | integer | Währungsnummer |
| Dateiname | Char(255) | Gibt den Namen der eingespielten Datei zurück. Wird benötigt, damit der Status eingespielt richtig gesetzt wird. |
| | | | |

```sql
CREATE PROCEDURE
fibu_ledgerimport (

in in_GUID char(256)

,in in_NumkreisNummer integer

,in in_ImportSchluessel integer

)
BEGIN
  declare
dc_Fehler integer;
  declare
dc_Ident integer;
  declare
dc_HauptKontoNumeric integer;
  declare
dc_HauptKonto char(30);
  declare
dc_ListFehlerKonto char(255);
  declare
dc_KostStelNummer integer;
  declare
dc_KostStelNummerUm integer;
  declare
dc_Dateiname char(255);
  declare
dc_err_notfound exception for sqlstate value
'02000';
  DECLARE
dc_ErrorMsg         LONG VARCHAR;
  DECLARE
dc_SQLCODE          INTEGER;
  DECLARE
dc_SQLSTATE         CHAR(10);

  declare local temporary table lttBuchungssatzImport(

Ident
integer not
null default
autoincrement

,FibuV_Klasse
integer

,HauptKonto
char(30)

,HauptText
char(100)

,GegenKonto
integer

,GegenText
char(100)

,FiBuVP_Betrag
numeric(15,4)

,FiBuVPW_Betrag     numeric(15,4)

,FiBuVP_SollHaben   integer

,FiBuV_Datum
date

,JahrNummer
char(10)

,PeriNummer
char(10)

,KostStelNummer     char(10)

,KSTRNummer
char(10)

,FiBuV_FremdNr     char(20)

,FiBuV_PaginierNr   char(40)

,FiBuVPW_Kurs
numeric(15,6)

,FiBuVPW_RechFormel integer

,FiBuVPW_Faktor     numeric(15,4)

,FiBuVPW_Typ
integer

,WaehrISOCode
char(34)

,Belegmappe
char(255)

  ) on commit delete rows ;
  if
VAREXISTS( 'ledgerimport_xml' )
<> 0 then
    drop VARIABLE ledgerimport_xml;
  end if;
  CREATE VARIABLE
ledgerimport_xml XML;
  Begin
    declare LeseDaten dynamic scroll cursor for select Datei,
Dateiname
      from BuchungssatzImport where Id = in_GUID
and eingespielt = 0;
     open LeseDaten;
     LeseDatenloop:
     loop fetch next LeseDaten into
ledgerimport_xml, dc_Dateiname;
       if sqlstate<>dc_err_notfound then
         insert into
GTT_AMic_Ident (typ, ident1, text1) values(in_GUID, 1,
dc_Dateiname);
         insert into
lttBuchungssatzImport with auto name
(select

FibuV_Klasse

,if isnumeric(HauptKonto) = 1 then

umsetzer(in_ImportSchluessel,cast(cast(HauptKonto as integer) as char(255)))

else

umsetzer(in_ImportSchluessel, HauptKonto)

endif as
HauptKonto

,HauptText

,if isnumeric(GegenKonto) = 1 then

umsetzer(in_ImportSchluessel, cast(cast(GegenKonto as integer)as char(255)))

else

umsetzer(in_ImportSchluessel, GegenKonto)

endif as
GegenKonto

,GegenText

,FiBuVP_Betrag

,FiBuVPW_Betrag

,FiBuVP_SollHaben

,FiBuV_Datum

,if isnumeric(Jahrnummer)=1 then

cast(Jahrnummer as integer )

else

isnull((select Jahrnummer
from GeschJahrStamm where FiBuV_Datum

between JahrBeginn and JahrEnde ),-1)

endif as JahrNummer

,if isnumeric(Perinummer)=1 then

cast(Perinummer as integer )

else

isnull((select Perinummer
from PeriStamm

where FiBuV_Datum between PeriBeginn and PeriEnde and
perityp=1

and peribereich =1 and peristatus
in ( 1, 2 ) ),-1)

endif as
Perinummer

,umsetzer(in_ImportSchluessel, isnull(KostStelNummer,'0')) as

KostStelNummer

,umsetzer(in_ImportSchluessel, isnull(KSTRNummer,'0')) as
KSTRNummer

,FiBuV_FremdNr

,FiBuV_PaginierNr

,FiBuVPW_Kurs

,FiBuVPW_RechFormel

,FiBuVPW_Faktor

,FiBuVPW_Typ

,if isnumeric(WaehrISOCode)=1
then

cast ( WaehrISOCode as
integer )

else

if isnull(trim(WaehrISOCode),'')='' then

DB_ZENTRALWAEHRUNG

else

isnull( (select (Waehrnummer) from waehrungsstamm

where waehrungsstamm.WaehrISOBezeich=Trim(WaehrISOCode)),-1)

endif

endif as WaehrISOCode

,isnull(dc_Dateiname,'') as Belegmappe

from openXML(ledgerimport_xml,'LedgerImport/Account'

)

WITH

(

FibuV_Klasse       integer       'FiBuV_Klasse'

,HauptKonto
char(255)     'HauptKonto'

,HauptText
char(100)     'HauptText'

,GegenKonto
integer
'GegenKonto'

,GegenText
char(100)     'GegenText'

,FiBuVP_Betrag
numeric(15,4) 'Betrag'

,FiBuVPW_Betrag     numeric(15,4) 'WBetrag'

,FiBuVP_SollHaben   integer       'SollHaben'

,FiBuV_Datum
date
'BelDatum'

,JahrNummer
char(10)      'JahrNummer'

,PeriNummer
char(10)      'PeriNummer'

,KostStelNummer     char(10)      'KostStelNummer'

,KSTRNummer
char(10)      'KSTRNummer'

,FiBuV_FremdNr     char(20)       'FiBuV_FremdNr'

,FiBuV_PaginierNr   char(40)      'PaginierNr'

,FiBuVPW_Kurs
numeric(15,6) 'FiBuVPW_Kurs'

,FiBuVPW_RechFormel integer       'FiBuVPW_RechFormel'

,FiBuVPW_Faktor     numeric(15,4) 'FiBuVPW_Faktor'

,FiBuVPW_Typ
integer
'FiBuVPW_Typ'

,WaehrISOCode
char(34)      'WaehrISOCode'

));

      else
        leave
LeseDatenloop;
      end if;
    end
loop LeseDatenloop;
  close
LeseDaten;
    set
dc_Fehler = 0;
  End;
  insert into FibuImport with
auto name
(select

IDENT
         ,FibuV_Klasse  as BELKLASSE
         ,if dc_Fehler > 0 then 0 else cast(HauptKonto as integer) endif as HauptKonto
         ,HauptText
         ,GegenKonto
         ,HauptText as
GegenText
         ,FiBuVP_Betrag as
Betrag
         ,FiBuVPW_Betrag  as WBetrag
         ,FiBuVP_SollHaben as
SOLLHABEN
         ,FiBuV_Datum as
VALDATUM
         ,FiBuV_Datum as  BELDatum
         ,JahrNummer
         ,PeriNummer
         ,KostStelNummer as
KOSTSTEL
         ,KSTRNummer
         ,FiBuV_FremdNr as
REFNUMMER
         ,FiBuV_PaginierNr as
PaginierNr
         ,FiBuVPW_Kurs as
WKurs
         ,FiBuVPW_RechFormel as WRechFormel
         ,FiBuVPW_Faktor as
WFaktor
         ,FiBuVPW_Typ as
WTyp
         ,cast(WaehrISOCode as
integer) as Waehrnummer
         ,Belegmappe  from lttBuchungssatzImport);
  update
GTT_AMic_Ident set ident1 = 0 where typ = in_GUID;
  EXCEPTION
  WHEN OTHERS
THEN
    SELECT  ERRORMSG()

,SQLCODE

,SQLSTATE
    INTO    dc_ErrorMsg

,dc_SQLCODE

,dc_SQLSTATE;
    CALL AMIC_FEHLERPROT( 20

,amic_func_sprachtexte('a', 'b', 'FiBuImport28', -1)

,amic_func_sprachtexte

('a'

,'b'

,'Beim Ausführen der
Prozedur "%s" ist ein Fehler aufgetreten.', -1, 'FIBU_LedgerImport'

)

|| '\n'

|| amic_func_sprachtexte

('a'

,'b'

,'Parameter (%s):
%s', -1, 'in_GUID',
in_GUID

)

|| '\n'

|| amic_func_sprachtexte

('a'

,'b'

,'Parameter (%s):
%s', -1, 'in_NumkreisNummer',
in_NumkreisNummer

)

|| '\n'

|| amic_func_sprachtexte

('a'

,'b'

,'Parameter (%s):
%s', -1, 'in_ImportSchluessel', in_ImportSchluessel

)

|| '\n'

|| 'SQLCODE: '
|| dc_SQLCODE

|| ' [' || dc_SQLSTATE ||
']'

|| '\n'

|| dc_ErrorMsg

,-10171

);
END
```
