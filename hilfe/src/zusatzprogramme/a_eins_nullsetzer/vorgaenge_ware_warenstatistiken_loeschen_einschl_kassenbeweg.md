# Vorgänge Ware & Warenstatistiken löschen (einschl. Kassenbewegungsdaten)

<!-- source: https://amic.de/hilfe/_nullsetzer_vorgangwarestatistik.htm -->

Es werden die Daten in folgenden Tabellen gelöscht:

V_PosiUebertrag

V_PosiRohQualiZw

TourStation

WLZ_TempFeuProt

FAKtoWLZ_Report

WLZ_V_Addon

V_PosiGrZuAbWaeh

FAKtoWLZExport

WLZ_verify

VorgStornoProto

V_PosiWareGefahr

Vorganguebergabe

VorgangMaskeDaten

Reklam

ArchivWBAuftrag

ArchivBelegAuftrag

ImportVorgStamm  
VorgReservierung  
VorgangUngebu

VorgangUngedru

VorgangUnerled

VorgGlobalZuab

V_Markier 

VorgangDropProt

KontraktSoftLock 

V_PosiArtiAuspr

ArtiAusPraeg

V_PosiArtiText

V_PosiBauBeginn

V_PosiBauEnde

V_PosiBaustein

V_PosiBaustelle

V_PosiBeginn

V_PosiDispo

V_PosiEnde

V_PosiGebDispo

V_PosiGebQuDispo

V_PosiGebinde

V_PosiPreisGebindeFaktoren

V_PosiGefaGut

V_PosiGrupAlter

V_PosiGrupBegin

V_PosiGrupEnde

V_PosiGrZuAb

V_PosiGrZuAbSt

V_PosiGrZuAbWaeh

V_PosiKontrakt

V_PosiLeerZeile

V_PosiPartie

V_PosiKlammer

V_PosiPartieNachtrag

V_PosiQuellDispo

V_PosiRohKosten

V_PosiRohLiefer

V_PosiRohLiefWS

V_PosiRohQuali

V_PosiRohQualiZw  
V_PosiRohSortier

V_PosiRohSortWS

V_PosiStLiKomp

V_PosiStListe

V_PosiText

v_positextblob

V_Position

V_PosiUmbuBeginn

V_PosiUmbuEnde

V_PosiUebertrag

V_PosiUV_Ansch

V_PosiUV_Beginn

V_PosiUV_Ende

V_PosiUV_Kunde

V_PosiWaehrung

V_PosiWare

V_PosiWareRes

V_PosiZ_Waeh

V_PosiFolgeArtikel

V_PosiZ_Zuab

V_PosiZw_Summe

V_RohWare

V_RohWAreAbsch

RohWareHauptsatz_Waage

RohWareZusatzQualitaet_Waage

RohWareZusatzWare_Waage

V_RohwareNachVerg

V_RohwareNachVText

vorgfibulink

fibuvorgposwabew

V_Waehrung

V_ZahlBeding

V_KassenInfo

V_ProdVorgang

VorgAktivStatus

Vorgangaddon

VorgangStamm

VorgGelangensBest

Vorgbemerkung

VorgFibuProto

VorgInkassoBeleg

VorgStammUmbuch

VorgStapel

VorgStapelPosit

VorgSteuer

v_vieraugenprinzip

stapel_content

KontraktDispoVorgang

SaatgutSaatentnahme

MaschinenTagebuchPosition

Rohware_Qual_Nachtrag

VorgText (where isnull(v_id,0) != 0)

// Datensätze in der Relation Vorgtext mit Verknüpfungen zu Textbausteinen (bemerkstamm) nicht mitlöschen  
    

VorgTransAuftrag

VorgVersAdresse

BemerkPosition mit (BemerkId in (select BemerkId from BemerkStamm where (BemerkTyp in (101, 102, 202, 203, 212))))

BemerkPositionWERTE mit (BemerkId in (select BemerkId from BemerkStamm where (BemerkTyp in (101, 102, 202, 203, 212))))

BemerkStamm mit (BemerkTyp in (101, 102, 202, 203, 212))

Bemerkung mit (BemerkTyp in (101, 102, 202, 203, 212))

locker

SV_WARE_TEILDISPO

VorgVorErfStapel

VorgVorErfassung

VorgVorErfPosit

DatenStrom

ReverseTransaction

KontraktMengeIst

KontraktMengeRoh

Aktualisierung von KontrArtiRohware:  
set KtrArtiRohVMenge=0, KtrArtiRohVWert=0, KtrArtiRohVKorMe=0, KtrArtiRohVKorWe=0

KontrBewegung

KontrFreiBeweg

KontrAndiBeweg

KontrErinnBeweg

KontraktSummen

BaustBewegung

BaustArtiMenSumm

LeergutBewegung

PartieBewegung

Akzualisierung von PartieArtiMenIst:  
set PartMenD_AbMeKor=0, PartMenD_AbMenge=0, PartMenD_ZuMeKor=0, PartMenD_ZuMenge=0,PartMenE_AbMeKor=0, PartMenE_AbMenge =0, PartMenE_AbWert=0, PartMenE_ZuMeKor=0,PartMenE_ZuMenge=0, PartMenE_ZuWert=0

PartieBestandPur

WarenBewGebinde 

WarenBewegung

Nach Löschen der WarenBewegung wird in der Tabelle Mengeinheit das Feld me_inbenutzung für alle Mengeneinheiten auf 0 zurückgesetzt:

Mengeinheit : set me_inbenutzung=0

Warenbewegung_Protokoll

TAB_WABU

ArchivWarenbuch  
WarenBewLink

PalEtiketten

PalEtikettStamm

WarenBewegungAddOn

Warenbewegung_Intrastat

WarenBewKonto  
WarenBewRohKost

WarenBewRohQuali

WarenBewRohware

WarenBewSteuer

WarenBewWaehrung

WarenBewZuAb

WarenBuch

WarenBuchBonus

WarenProvision

WarenStatistik

wabewzusatz

warenflusskontrolle

ArtiBestand

ArtiBestandFremdKonto

ArtiBestandSumme

ArtiSummen

ArtiSummExtra

FehlerListe

InventurBeleg

InventurBelegPartie

InventurBelegPartieAddon

InventurBestand

InventurBewPreise

KundenSummen", "where (PeriBereich = 2)

lagerplatzsummen

TransPortAuftrag

VertSummen

v_sammelreli

bedarf

BPoolQuelle

BPoolZiel

BePool

FakToNum3

FakToWLZ

Leergut

LeergutKonto

// Kassenbewegungsdaten löschen

AcashBelgKsiz

AcashBelg

AcashBelgZhlg

AcashBelgZami

AcashKasSturz

AcashBelgZM

AcashStoKsiz

AcashStoZamiProto

AcashUmwZamiProto

AcashBelgStoProto

AcashFibu

AcashFibuLink

AcashUebergabe

AcashBelgKsizFW

AcashLocker

AcashBelegAbbrProto

Aktualisieren:  
AcashStmdKsse: set KsASK=0

AcashKonfKsys: set KsyASK=0

// EDIFAK  
EDIVorgang  
EDISession

EDIExVorgang

EDIExSession

// Sonstiges

Wiedervorlage mit (WiederTyp = 3)

WARENBEWEGUNG_NACHHALTIGKEIT

NACHHALTIGKEIT_MASSEBILANZ

VORGANGSKLAMMER

Anschriften vom AdressTyp  
21 (Vorgangs-Anschrift),  
22 (Vorgangs-Versandanschrift) und  
23 (Untervorgangs-Anschrift):   
    
Anschriftstamm 

AnschriftAddOn 

| Waage Tabellen |
| --- |
| OWaage |
| OWaage_KontraktZuordnung |
| owaage_ladetraeger |
| owaage_qualitaeten |
| OWaageAddon |
| owaageQual |
| owaageQualWert |
