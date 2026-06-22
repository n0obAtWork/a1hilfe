# Relation VorgangUebergabe

<!-- source: https://amic.de/hilfe/relationvorganguebergabe.htm -->

Die Relation VorgangUebergabe nimmt die Vorgangsrohdaten auf, die nicht für die Rohware bestimmt sind.

Aus dieser Zwischenrelation werden über die Aeins-Funktion VorgangUebergabeBelErz (Aufruf des Pascal-Scripts **VorgangEinspielung**) die Vorgänge erzeugt.

ArtikelNummer char 20 0 .................... Y N 

BedienerIdKorr integer 4 0 0 Y N 

BedienerIdNeu integer 4 0 current user Y N 

BelegDatum date 4 0 today(\*) Y N 

BelegNummer integer 4 0 0 Y N 

CreateTime integer 4 0 .................... Y N 

Datum date 4 0 today(\*) Y N 

FilialNummer integer 4 0 0 Y N 

JahrNummer integer 4 0 0 Y N 

KontraktNummer integer 4 0 0 Y N 

KundNummer integer 4 0 0 Y N 

LagerNummer integer 4 0 0 Y N 

LagerNummerZug integer 4 0 0 Y N 

LagerPlatzNrZug integer 4 0 0 Y N 

LagerPlatzNummer integer 4 0 0 Y N 

Lfd_Nummer integer 4 0 .................... N N 

LKW_Nummer integer 4 0 .................... N N 

ME_Nummer integer 4 0 0 Y N 

ME_NummerPreis integer 4 0 0 Y N 

Menge numeric 15 6 0.0 Y N 

PartieNummer integer 4 0 0 Y N 

PeriNummer integer 4 0 0 Y N 

Preis numeric 15 6 0.0 Y N 

PreisBrutto smallint 2 0 0 Y N 

PreisEinheit numeric 15 6 1.0 Y N 

PreisGesamt smallint 2 0 0 Y N 

referenz integer 4 0 0 Y N 

SatzId integer 4 0 .................... N Y 

SortenNummer integer 4 0 0 Y N 

Status integer 4 0 0 Y N 

SteuerSchluessel integer 4 0 0 Y N 

UebernahmeId integer 4 0 .................... N N 

v_Id integer 4 0 .................... Y N 

V_KlassNummer integer 4 0 0 Y N 

v_numnummer integer 4 0 0 Y N 

VertGrNummer integer 4 0 0 Y N 

vorgident integer 4 0 0 Y N

Wiegenummer integer 4 0 0 N N 

Die Attribute **vorgident** und **referenz** dienen der internen Referenzierung bei der Umwandlung der Belege in Vorgänge. **UebernahmeId** und **SatzId** werden bei dem Dateiimport vergeben. In die Felder **v_numnummer** und **v_id** werden die entsprechenden Inhalte aus dem entstandenen Vorgang eingetragen und referenzieren jedes für sich eindeutig den Vorgang.
