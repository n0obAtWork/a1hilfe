# Kontrakte löschen

<!-- source: https://amic.de/hilfe/kontraktelschen.htm -->

Es dürfen keine Daten in KONTRBEWEGUNG vorhanden sein, sonst wird nicht gelöscht!

Es werden die Daten in folgenden Tabellen gelöscht:

KontraktAddon

KONTRAKTAKTSTAT

KONTRAKTANDIEN

KONTRAKTANDIPOSI

KONTRAKTARTIKEL

KONTRAKTBAUSTEIN (where isnull(ktrid,0) != 0)  
// Datensätze für Kontraktvarianten mit Verknüpfungen zu Textbausteinen (bemerkstamm) nicht mitlöschen  
KontraktZuStrecke

KontraktTextBlob

KONTRAKTERINNER

KONTRAKTFREIST

KONTRAKTKORREKT

KontraktMaskeDaten

KONTRAKTMENGE

KontraktMengeIst

KontraktMengeRoh

KONTRAKTMENGEZR

KONTRAKTPARTIE

KONTRAKTPREIS

KontraktPreisRoh

KONTRAKTPREISZR

KONTRAKTSTAMM

KONTRAKTSUMMEN

KONTRAKTTEMPLATE

KONTRANDIBEWEG

KontrArtiRohKost

KontrArtiRohQual

KONTRARTIROHWARE

KONTRAUSWARTI

KONTRAUSWLISTE

KontrAuswPreis

KONTRBEWEGUNG

KONTRDISPOZUORD

KONTRERINNBEWEG

KONTRERINNPOSIT

KONTRFREIBEWEG

KONTRFREIPOSIT

KontrKlNumKreis

KONTRPARILAGER

KontrPariZuAb

KONTRPARTIEMENGE

KONTRUNTER

KontrUnterKlasse

KONTRVARIANTE

KONTRVARIBAUST

KONTRVARIPOSIT

KONTRVARITEXT

Kontraktratierlich_protokoll

Bemerkung unter der Bedingung: where (BemerkTyp = 31) or (BemerkTyp = 32)
