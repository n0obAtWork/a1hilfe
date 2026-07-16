# Artikel löschen (inkl. 1+2+7+8+16)

<!-- source: https://amic.de/hilfe/artikellscheninkl127816.htm -->

Es werden die Daten in folgenden Tabellen gelöscht:

artikel

artikelstamm

artikeltext

ARTILISTENPREIS

ARTILPREISGRUPPE

ARTIstamgebinde

ARTIGEBINDE

ARTISTUECKKOMP

ARTIKELADDON

ARTIKELMASKEDATEN

ARTILPRPROTOKOLL

ArchivArtikelAuftrag

WAREOANALYSE

ARTIHERSTPREIS

ARTIAUSPRAEG

ARTIGEFAHRKLASSE

ARTIKELINFOSEITE

ARTILIEFERANT

ARTIGEFAHRKLASSE

ARTISUCHBEGR

ARTIAUSWEICH

ARTISEKUNDSCHL

ARTIBESTAUSPR

ARTIHERSTELL

ARTIAUSPGEBINDE

WAGRUSPEZARTIKEL

EZG_ARTIKELLISTE

ARTIINTRASTAT

ARTIZUSAMMENSETZ

ARTIKELSTAMMADDON

ARTIKUNDARNR

ARTIFOLGEARTIKEL

kalklistenpreis

kalkliprschema

repllistenpreis

preiskalkrefliste

artiindivpreis

ARTIIPREISGRUPPE

Bemerkung unter der Bedingung: where (BemerkTyp = 11) or (BemerkTyp = 12) or (BemerkTyp = 13) or (BemerkTyp = 15) or (BemerkTyp = 17)

SekundSchluessel unter der Bedingung: where SekuRelation = 'Artikelstamm'

nachhaltigkeit_artikelstammvorbelegung

nachhaltigkeit_massebilanzArtikelsummen

disposition

MarktStandTexte

marktstandangebote

ArtiMerkmal

ArbeitsBuchungen

KTRDISPOSITION

o_satzneu

SaatgutSaatentnahme

RohSorteKosten

RohSorteArtikel

BAUSTPREIS

BAUSTARTIMENGE

BAUSTARTIKEL

LVS_Ladetraegertyp

StreckenerfassungArtiStammID

sinfos1

ArtikelTextBlob  
ogznb

ARTISTUECKLISTE

HANDELSSTUECKLISTE

REZEPTURGRUPPE

ARTISTUECKPREIS

Es werden die Daten in folgenden Tabellen aktualisiert:

RohSorteArtikel mit Aktualisierung: set ArtikelId=0

RohSorteKosten mit Aktualisierung: set ArtikelId=0

Beim Löschen der Artikel werden automatisch die [Vorgänge Ware](./vorgaenge_ware_warenstatistiken_loeschen_einschl_kassenbeweg.md), [Vorgänge Finanzbuchhaltung](./vorgaenge_finanzbuchhaltung_loeschen_inkl_29.md), [Kontrakte](./kontrakte_loeschen.md), [Partien](./partien_loeschen.md) und [Saatgut](./rohwaren_tabellen_loeschen.md) mit gelöscht.
