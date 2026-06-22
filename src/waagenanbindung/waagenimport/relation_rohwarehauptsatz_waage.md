# Relation RohwareHauptsatz_Waage

<!-- source: https://amic.de/hilfe/relationrohwarehauptsatzwaage.htm -->

Die Relation RohwareHauptsatz_Waage nimmt die Rohwarendaten auf.

Aus dieser Zwischenrelation werden über die Aeins-Funktionen CWLU_EK (für Einkauf) und CWLU_VK (für Verkauf) (Aufruf der JPL-Prozedur **cwegvorb**) die Lieferscheine erzeugt.

Anmerkung:

Die zugehörigen Qualitäten werden in einer Unterrelation **RohwareZusatzQualitaet_Waage** gespeichert (s. unten).

Artikelnummer char 20 0 .................... N N 

BedKennz_InVorgang integer 4 0 .................... Y N 

BedKennz_VonWaage integer 4 0 .................... Y N 

BelDatum_InVorgang date 4 0 .................... Y N 

BelNummer_InVorgang integer 4 0 .................... Y N 

CreateTime_VonWaage integer 4 0 .................... Y N 

Datum_InVorgang date 4 0 .................... Y N 

Datum_VonWaage date 4 0 .................... Y N 

EKVK_Kennzeichen integer 4 0 0 N N 

Fakturiergruppe integer 4 0 0 Y N 

Filialnummer integer 4 0 -1 N N 

FilName_VonWaage char 12 0 .................... Y N 

Haupt_Kontraktnummer integer 4 0 0 N N 

Haupt_Partienummer integer 4 0 0 N N 

Kundennummer integer 4 0 .................... N N 

Lagernummer integer 4 0 .................... N N 

Lagerplatznummer integer 4 0 0 Y N 

LfdNummer_VonWaage integer 4 0 .................... Y N 

Lieferscheindatum date 4 0 today(\*) N N 

Lieferscheinnummer integer 4 0 0 N N 

LKW_Nummer integer 4 0 .................... N N 

Menge numeric 15 4 .................... N N 

Mengeneinheit integer 4 0 .................... Y N 

Preis numeric 15 4 0 N N 

Preiseinheit numeric 15 4 .................... Y N 

Preismengeneinheit integer 4 0 .................... Y N 

SatzId integer 4 0 0 N Y 

Sortennummer integer 4 0 0 N N 

Status integer 4 0 1 N N 

Steuerschluessel integer 4 0 -1 N N 

UebernahmeID integer 4 0 .................... N N 

Verkaufsgebiet integer 4 0 0 Y N 

Versandart integer 4 0 0 Y N 

Vertretergruppe integer 4 0 -1 N N 

Wiegenummer integer 4 0 0 N N
