# Abkürzungen

<!-- source: https://amic.de/hilfe/abkrzungen.htm -->

I2 smallint

I4 integer

N4 numeric(15,4)

TS timestamp

Char(?) character-string mit ? vielen Zeichen

D4 Datum

Feld ohne Erklärung bedeutet, dass es zurzeit nicht benutzt / versorgt wird

Benutzte Relationen:

**AcashBelg:** Diese Tabelle beinhaltet das Kassenbuch und ist eine Liste über alle

an der Kasse erfassten Vorgänge.

 Schlüsselfelder:

 BelegId (I4) bei Vorgängen handelt es sich um die V_Id, bei Finanzvorgängen

(Belegarten 10-20) um einen fortlaufend kleiner werdenden negativen Integerwert.

 BelegKs(I4) Kassennummer, an der dieser Beleg erfasst wurde (entspricht Eintrag

in Kassenverwaltung)

FilialNummer(I4) Nummer der Filiale, zu der der Beleg gehört (aus Mandantenstamm)

 Weitere Felder:

 BelegArt(I2) die Belegart des Beleges (s.o.)

 BelegBedNr(I4) die Bedienernummer des Bedieners, der den Beleg erzeugt hat

 BelegBVENr(I4) Sitzungsnummer des zugehörigen Barverkaufssystems (Kasse 0), zu

der dieser Beleg gehört

 BelegDatum(TS) Uhrzeit, an dem dieser Beleg erfasst wurde

BelegGegenKs(I4) Bei Geldübergaben / Geldübernahmen : die Nummer der Gegenkasse

BelegKennz1(I2) 

BelegKennz2(I2)

BelegKennz3(I2)

BelegKennz4(I2)

BelegKennz5(I2)

BelegKennz6(I2)

BelegKennz7(I2)

BelegKennz8(I2) 0=Tresenverkauf/Finanzvorgang, 1=POS-Vorgang

BelegKennz9(I2) Bei Fremdwährungsbelegen steht hier die Währungsnummer

BelegKennz10(I2) 0=Unterkasse, 1=Hauptkasse (nur bei Abschöpfung)

BelegKsi(I4) Sitzungsnummer der Kasse, zu der dieser Beleg gehört

BelegKsy(I4) Kassensystem aus Kassensystemverwaltung, zu der dieser

Beleg gehört

BelegKunde(I4) Kundennummer / Lieferantennummer, wenn der Vorgang

Kundenbezug hat

BelegKundeId(I4) 

BelegLagPosAnz(I4)

BelegNr(I4) bei Vorgängen die Belegnummer, bei Belegarten 1-10 eine eigene

fortlaufende Nummer

BelegPosAnz(I4) Anzahl der Positionen des Beleges

 BelegSBelegNr(I4) Id des Steuersatzes des Sachkontos bei Geldentnahmen

 BelegSH(I2) Soll/Haben-Kennzeichen

–1: Kassenausgang, 1: Kasseneingang

 BelegSKF(N4) skontierfähiger Betrag (nur Tresenkasse)

 BelegSkontoBetrag(N4) gewährter Skontobetrag (nur Tresenkasse)

 BelegSkontoSatz(N4) gewährter Skontosatz (nur Tresenkasse)

 BelegStatus(I2) 

 BelegStorno(I2) 2: Beleg ist storniert

 BelegSummeBrutto(N4) Betrag in Belegwährung

 BelegSummeZahlung(N4) Betrag abzüglich Skonto

 BelegText(char(50)) BelegText gemäß Belegart

 BelegText1(char(100)) 

 BelegText2(char(100))

 BelegText3(char(100))

 BelegText4(char(100))

 BelegText5(char(100))

 BelegText6(char(100))

 BelegText7(char(100))

 BelegText8(char(100))

 BelegText9(char(100)) Bei Währungsbelegen: Kurztext der Währung

 BelegText10(char(100))

 BelegWert1(N4) Steuersatz bei Entnahmen

 BelegWert2(N4) Steuersumme bei Entnahmen

 BelegWert3(N4) 

 BelegWert4(N4) Währungskurs bei Fremdwährungsbelegen

 BelegWert5(N4) Betrag in Buchwährung

 BelegWert6(N4)

 BelegWert7(N4)

 BelegWert8(N4)

 BelegWert9(N4)

 BelegWert10(N4) Differenzbetrag bei Zählungen

 BelegZahlAnz(I4) Anzahl der Zahlungswege

**AcashBelgKsiz:** In dieser Tabelle werden die Bestände für die Kassenberichte

verwaltet und sofort aktualisiert, wenn ein Vorgang abgeschlossen wurde.

 Schlüsselfelder:

 KsiASK Sitzungsnummer

 KsiKsNr Kassennummer (referenziert auf KsNr aus AcashStmdKsse)

 Filialnummer zugehörige Filialnummer

 Weitere Felder:

 KsiAbbruchAnz(I4) Anzahl der Abbrüche bei der POS-Kasse (wird hochgezählt,

wenn man die Maske über ESC verlässt, aber keine Positionen

erfasst hat) +

Anzahl der Abbrüche bei der Tresenkasse (immer wenn eine

geleistete Zahlung zurückgesetzt wird, korrespondiert mit der

Meldung „Der letzte Kassiervorgang ... wird zurückgesetzt)

KsiAbschlussDat(TS) Zeitpunkt, an dem die Kasse abgeschlossen wird

 KsiAbschlussTim(char(8)) Uhrzeit des Kassenabschlusses

 KsiArtikelAnz(I4) Anzahl fakturierter Artikel

 KsiAufnStoAnz(I4) Anzahl stornierter Kassenstürze (Belegart 20)

(Bem.: Storno Kassensturz nicht möglich !)

KsiAufnStoSum(N4) Stornosumme stornierter Kassenstürze (Belegart 20)

 (Bem.: Storno Kassensturz nicht möglich !)

KsiAuszahlungsAnz(I4) Anzahl durchgeführter Geldauszahlungen (Belegart 15)

KsiAuszahlungsSum(N4) Summe durchgeführter Geldauszahlungen (Belegart 15)

 KsiAuszStoAnz(I4) Anzahl durchgeführter Stornierungen von Geldauszahlungen

 (Belegart 15)

 KsiAuszStoSum(N4) Summe durchgeführter Stornierungen von Geldauszahlungen

 (Belegart 15)

 KsiBarvStoAnz(I4) 

 KsiBarvStoSum(N4)

 KsiBEKAnz(I4) Anzahl über die Tresenkasse erfasster Bareinkäufe

(Belegart 6)

 KsiBEKSum(N4) Summe der über die Tresenkasse erfasster Bareinkäufe

 (Belegart 6)

 KsiBRBAnz(I4) 

 KsiBRBSum(N4)

 KsiBrEKStoAnz(I4) Anzahl der stornierten Bareinkäufe (Belegart 6)

 KsiBrEKStoSum(N4) Summe der stornierten Bareinkäufe (Belegart 6)

 KsiBRNAnz(I4)

 KsiBRNSum(N4)

 KsiBrreStoAnz(I4) Anzahl der stornierten Barverkäufe (Belegart 2)

 KsiBrreStoSum(N4) Summe der stornierten Barverkäufe (Belegart 2)

 KsiBVAnz(I4) Anzahl der an der Tresenkasse bzw. POS-Kasse

durchgeführten Barverkäufe (Belegart 2)

KsiBVENr(I4) Sitzungsnummer des Barverkaufssystems (Kasse 0), zu der dieser Satz gehört

 KsiBVSum(N4) Summe der an der Tresenkasse bzw. POS-Kasse

durchgeführten Barverkäufe (Belegart 2)

 KsiDiffAnz(I4) wie oft gab es beim Kassensturz eine Differenz

 KsiDiffBarSum(N4) Summe über die Bargeld-Differenzen beim Kassensturz

 KsiDiffBarSum2(N4)

 KsiDiffBarSum3(N4)

 KsiDiffBarSum4(N4)

 KsiDiffBezAnz(I4) 

KsiDiffBezSum(N4)

 KsiDiffGutAnz(I4)

 KsiDiffGutSum(N4)

 KsiDiffKreAnz(I4)

 KsiDiffKreSum(N4)

 KsiDiffKrevAnz(I4)

 KsiDiffKrevSum(N4)

 KsiDiffSchAnz(I4)

 KsiDiffSchSum(N4)

 KsiDiffSum(N4) Summe über alle Differenzen beim Kassensturz

 KsiEinrAnz(I4) Anzahl der Abschöpfungen (Belegart 17)

 KsiEinrStoAnz(I4) Anzahl der Stornierungen von Abschöpfungen (Belegart 17)

 KsiEinrStoSum(N4) Summe der Stornierungen von Abschöpfungen (Belegart 17)

 KsiEinrSum(N4) Summe der Abschöpfungen (Belegart 17)

 KsiEinzahlungsAnz(I4) Anzahl der Geldeinzahlungen (Belegart 11)

 KsiEinzahlungSum(N4) Summe der Geldeinzahlungen (Belegart 11)

 KsiEinzStoAnz(I4) Anzahl der Stornierungen von Einzahlungen (Belegart 11)

 KsiEinzStoSum(N4) Summe der Stornierungen von Einzahlungen (Belegart 11)

 KsiEntnahmenAnz(I4) Anzahl der Geldentnahmen (Belegart 14)

 KsiEntnahmenSum(N4) Summe der Geldentnahmen (Belegart 14)

 KsiEntnStoAnz(I4) Anzahl der Stornierungen von Entnahmen (Belegart 14)

 KsiEntnStoSum(N4) Summe der Stornierungen von Entnahmen (Belegart 14)

 KsiEroeffnungDat(TS) Zeitpunkt, an dem die Kasse eröffnet wird

 KsiEroeffnungTim(char(8)) Uhrzeit der Kasseneröffnung

 KsiESkontoAnz(I4) Anzahl erhaltener Skonti im Einkauf

 KsiESkontoSum(N4) Summe erhaltener Skonti im Einkauf

 KsiMnetto1(N4) 

 KsiMnetto2(N4)

 KsiMnetto3(N4)

 KsiMnetto4(N4)

 KsiMwst1(N4)

 KsiMwst2(N4)

 KsiMwst3(N4)

 KsiMwst4(N4)

 KsiMwstSatz1(N4)

 KsiMwstSatz2(N4)

 KsiMwstSatz3(N4)

 KsiMwstSatz4(N4) 

 KsiRetourenAnz(I4) Anzahl der Barverkauf-Gutschriften (Belegart 4)

 KsiRetourenSum(N4) Summe der Barverkauf-Gutschriften (Belegart 4) 

 KsiRetrStoAnz(I4) Anzahl der stornierten Barverkauf-Gutschriften (Belegart 4)

 KsiRetrStoSum(N4) Summe der stornierten Barverkauf-Gutschriften (Belegart 4)

 KsiSchubladenoeffnungen(I4) Anzahl der Schubladenöffnungen

 KsiSollBarSum(N4) Sollbestand an Bargeld

 KsiSollBarSum2(N4)

 KsiSollBarSum3(N4)

 KsiSollBarSum4(N4)

 KsiSollBezAnz(I4) Sollbestand (Anzahl) an Bankeinzügen

 KsiSollBezSum(N4) Sollbestand (Summe) an Bankeinzügen

 KsiSollGutAnz(I4) Sollbestand (Anzahl) an Gutscheinen

 KsiSollGutSum(N4) Sollbestand (Summe) an Gutscheinen

 KsiSollKreAnz(I4) Sollbestand (Anzahl) an Kreditkarten

 KsiSollKreSum(N4) Sollbestand (Summe) an Kreditkarten

 KsiSollKrevAnz(I4) 

 KsiSollKrevSum(N4)

 KsiSollSchAnz(I4) Sollbestand (Anzahl) an Schecks

 KsiSollSchSum(N4) Sollbestand (Summe) an Schecks

 KsiSorwAnz(I4) Anzahl durchgeführter Sortenwechsel (Belegart 18)

 KsiSorwStoAnz(I4) wie oft sind Sortenwechsel storniert worden (Belegart 18)

 KsiSorwStoSum(N4) Summe der stornierten Sortenwechsel (Belegart 18)

 KsiSorwSum(N4) Summe der durchgeführten Sortenwechsel (Belegart 18)

 KsiStatus(I2) Status des Satzes

0: nicht eröffnet

1: eröffnet

2: unterbrochen

9: abgeschlossen

 KsiStornoAnz(I4) 

 KsiUebgAnz(I4) Anzahl der Geldübergaben (Belegart 16)

 KsiUebgStoAnz(I4) Anzahl der stornierten Geldübergaben (Belegart 16)

 KsiUebgStoSum(N4) Summe der stornierten Geldübergaben (Belegart 16)

KsiUebgSum(N4) Summe der Geldübergaben (Belegart 16)

 KsiUebnAnz(I4) Anzahl der Geldübernahmen (Belegart 12)

 KsiUebnStoAnz(I4) Anzahl der stornierten Geldübergaben (Belegart 12)

 KsiUebnStoSum(N4) Summe der stornierten Geldübergaben (Belegart 12)

KsiUebnSum(N4) Summ der Geldübergaben (Belegart 12)

 KsiUmsBarAnz(I4) Umsatz (Anzahl) der erfassten Barzahlungen

 KsiUmsBarAnz2(I4) 

 KsiUmsBarAnz3(I4)

 KsiUmsBarAnz4(I4)

 KsiUmsBarSum(N4) Umsatz (Summe) der erfassten Barzahlungen

 KsiUmsBarSum2(N4)

 KsiUmsBarSum3(N4)

 KsiUmsBarSum4(N4)

 KsiUmsBezAnz(I4) Umsatz (Anzahl) der Bankeinzüge

 KsiUmsBezSum(N4) Umsatz (Summe) der Bankeinzüge

 KsiUmsGutAnz(I4) Umsatz (Anzahl) der Gutscheine

 KsiUmsGutSum(N4) Umsatz (Summe) der Gutscheine

 KsiUmsKreAnz(I4) Umsatz (Anzahl) der Kreditkarten

 KsiUmsKreSum(N4) Umsatz (Summe) der Kreditkarten

 KsiUmsKrevAnz(I4)

 KsiUmsKrevSum(N4)

 KsiUmsSchAnz(I4) Umsatz (Anzahl) der Schecks

 KsiUmsSchSum(N4) Umsatz (Summe) der Schecks

 KsiVnetto1(N4)

 KsiVnetto2(N4)

 KsiVnetto3(N4)

 KsiVNetto4(N4)

 KsiVorBarSum(N4) Vortrag (Summe) des Bargelds

 KsiVorBarSum2(N4)

 KsiVorBarSum3(N4)

 KsiVorBarSum4(N4)

 KsiVorBezAnz(I4) Vortrag (Anzahl) der Bankeinzüge

 KsiVorBezSum(N4) Vortrag (Summe) der Bankeinzüge

 KsiVorGutAnz(I4) Vortrag (Anzahl) der Gutschriften

 KsiVorGutSum(N4) Vortrag (Summe) der Gutschriften

 KsiVorKreAnz(I4) Vortrag (Anzahl) der Kreditkarten

 KsiVorKreSum(N4) Vortrag (Summe) der Kreditkarten

 KsiVorKrevAnz(I4) 

 KsiVorKrevSum(N4)

 KsiVorSchAnz(I4) Vortrag (Anzahl) der Schecks

 KsiVorSchSum(N4) Vortrag (Summe) der Schecks

 KsiVSkontoAnz(I4) Anzahl gewährter Skonti im Verkauf (nur Tresenkasse)

 KsiVSkontoSum(N4) Summe gewährter Skonti im Verkauf (nur Tresenkasse)

 KsiVst1(N4)

 KsiVst2(N4)

 KsiVst3(N4)

 KsiVst4(N4)

 KsiVstSatz1(N4)

 KsiVstSatz2(N4)

 KsiVstSatz3(N4)

 KsiVstSatz4(N4)

 KsiZameAnz(I4) Anzahl der Zahlungsmeldungen (Belegart 10)

 KsiZameStoAnz(I4) Anzahl der stornierten Zahlungsmeldungen (Belegart 10)

 KsiZameStoSum(N4) Summe der stornierten Zahlungsmeldungen (Belegart 10)

KsiZameSum(N4) Summe der stornierten Zahlungsmeldungen (Belegart 10)

**AcashBelgStoProto:** In dieser Relation werden Informationen abgelegt, wenn ein

Finanzvorgang (Vorgänge mit Belegarten zwischen 10 und 20)

storniert wurde.

 Schlüsselfelder:

 BelegKs(I4) an welcher Kasse wurde der Beleg erfasst ?

 BelegStoId(I4) Verweis auf die Id des Urbelegs

 BelegStoKs(I4) an welcher Kasse wurde der Beleg storniert ?

 FilialNummer(I4) in welcher Filiale wurde storniert ?

 Weitere Felder:

 BedienerId(I4) Bedienernummer des Bedieners, der storniert hat

 Belegart(I2) Belegart, für die dieser Storno durchgeführt wurde

 BelegKsi(I4) Sitzung, in der dieser Beleg erfasst wurde

 BelegStoBetrag(N4) Betrag in Buchwährung über den stornierten Betrag

 BelegStoDatum(D4) Datum, an dem storniert wurde

 BelegStoKsi(I4) Sitzung, in der dieser Beleg storniert wurde

**AcashBelgZami:** In dieser Relation werden alle Zahlungsmittel verwaltet, die erfasst

wurden.

 Schlüsselfelder:

 Zamiidnr(I4) eindeutige Identnummer

 Weitere Felder:

 FilialNummer(I4) in welcher Filiale wurde dieses Zahlungsmittel erfasst

 Zamiaid(char(4)) für EC-Kartenverarbeitung

 Zamiaid_param(char(5)) für EC-Kartenverarbeitung

 Zamiart(I2) Zahlungsart des Zahlungsmittelsatzes

 Zamibelegnr(I4) zu welchem Beleg gehört das Zahlungsmittel

 Zamibemerkung(char(255)) Bemerkungstext des Zahlungsmittels

 Zamibetrag(N4) Betrag in der Währung, in der das Zahlungsmittel erfasst wurde

 Zamiblz(char(50)) mögliche Bankleitzahl

 ZamiDatum(D4) Datum, an dem das Zahlungsmittel erfasst wurde

 Zamidmbetrag(N4) Betrag des Zahlungsmittels in Buchwährung

 Zamidtaus(char(1)) J: Zahlungsmittel wurde per Lastschrift übertragen

 N: Zahlungsmittel wurde nicht übertragen

 Zamidtausdatum(D4) Datum, an dem das Zahlungsmittel übertragen wurde

 Zamiexport(I2) ZG: Kennzeichen, ob Zahlungsmittel exportiert wurde

 Zamifaktor(N4) 

 Zamigegenkonto

 Zamigueltigbis(char(7)) Gültigkeitsdauer von EC-Karten

 Zamikartenart(char(1)) Kartenart der EC-Karte (z.Zt.: 1)

 Zamikonto(char(50) Kontonummer des Zahlungsmittels

 Zamiks(I4) Kassennummer, dem dieses Zahlungsmittel zugeordnet ist

 Zamiksi(I4) Sitzungsnummer, dem dieses Zahlungsmittel zugeordnet ist.

 Zaminame(char(255)) Verweis auf einen Kundennamen, dem dieses Zahlungsmittel gehörte.

 Zaminr(char(50)) Scheck/Gutscheinnummer des Zahlungsmittels

 Zamioffline(char(1)) Kennzeichen, ob EC-Karte Offline erfasst wurde

 Zamistatus(I2) Status des Zahlungsmittels (1: in Kasse, 2: entnommen, 3:storniert,

 4: nachstorniert, 5: nachumgewandelt), dabei bedeutet nach..., dass die

Aktion nach der Entnahme des Zahlungsmittel passierte.

 Zamiwaehrung(char(50)) Währungstext, in der das Zahlungsmittel erfasst wurde

 Zamizielks(I4) 

**AcashBelgZhlg:** In dieser Relation werden alle Zahlungssätze weggeschrieben.

Schlüsselfelder:

ZahlBelegId(I4) Id des Beleges, dem dieser Zahlungssatz zugeordnet ist

ZahlKs(I4) Kassennummer, dem dieser Zahlungssatz zugeordnet ist

 ZahlNummer(I4) fortlaufende Nummer des Zahlungssatzes innerhalb des Beleges

 FilialNummer(I4) Filialnummer, dem dieser Beleg zugeordnet ist.

 Weitere Felder:

 Zahlart(I2) in welcher Zahlungsart ist dieser Satz erfasst worden ?

 ZahlBelegart(I2) die Belegart, zu der dieser Zahlungssatz gehört

 ZahlBelegNr(I2) Belegnummer, zu der dieser Zahlungssatz gehört

 ZahlBemerkung(char(50)) Bemerkungstext, der bei gewissen bargeldlosen Zahlungsarten

zusätzlich erfassbar ist.

 ZahlBetrag(N4) Zahlungsbetrag dieses Zahlungssatze in Buchwährung

 ZahlBLZ(char(50)) Bei gewissen Zahlungssätzen wird die Bankleitzahl erfasst.

 ZahlFaktor(N4) 

 ZahlKennz1(I2)

 ZahlKennz2(I2)

 ZahlKennz3(I2)

 ZahlKennz4(I2)

 ZahlKennz5(I2)

 ZahlKonto(char(50)) auch die Kontonummer wird bei bargeldlosen Zahlungssätzen

gebraucht.

 ZahlKsi(I4) zu welcher Sitzung gehört der Zahlungssatz

 ZahlKst(I4) bei Vorgängen: Nummer des Kassenkontos

 Sonst: Kontonummer des Gegenkontos

 ZahlName(char(50)) hier kann bei bargeldlosen Zahlungssätzen der Name erfasst werden. ZahlScheckNr(char(50)) hier kann die Schecknummer erfasst werden

 ZahlStueckAnz(I4)

 ZahlStueckwert(N4)

 ZahlText1(char(50)) bei bargeldlosen Zahlungssätzen steht hier der Bankname

 ZahlText2(char(50)) 

 ZahlText3(char(50))

 ZahlText4(char(50))

 ZahlText5(char(50))

 ZahlWaehrung(char(50)) hier wird die Währung in Textform abgelegt, in der dieser

Zahlungssatz abgelegt ist

 ZahlWBetrag(N4) hier steht ein Fremdwährungsbetrag drin (wie er erfasst wurde)

 ZahlWert1(N4)

 ZahlWert2(N4)

 ZahlWert3(N4)

 ZahlWert4(N4)

 ZahlWert5(N4)

 ZahlZamiIdNr(I4) hier wird auf einen Zahlungsmittelsatz verwiesen, wenn bargeldlos

bezahlt wurde. (Referenz auf AcashBelgZami)

**AcashBelgZM:** Hier werden manuelle Eingaben der Belegart Zahlungsmeldung

abgelegt. Wenn in der Zahlungsroutine auf mehrere Rechnungen

verwiesen wird, werden diese Eingaben einzeln abgelegt.

 Schlüsselfelder:

 ZameId (I4) Id des Beleges, dem diese Zahlungsmeldung zugeordnet ist.

 ZameKs(I4) Kassennummer des Beleges, dem diese Zahlungsmeldung zugeordnet

ist.

 ZameNr(I4) fortlaufende Nummer, da einem Beleg mehrere Zahlungsmeldungen

 Zugeordnet werden können.

 FilialNummer(I4) Filiale, dem dieser Beleg zugeordnet ist.

 Weitere Felder:

 ZameBetrag(N4) Betrag, den diese Zahlungsmeldung begleichen soll

 ZameText(char(30)) Text, mit dem diese Zahlungsmeldung auf einen zugehörigen Offenen

Posten referenzieren kann.

**AcashFiBu:** In dieser Relation werden Informationen abgelegt, die der

Mandantenserver für den Übertrag in die FiBu benötigt, wenn

Finanzvorgänge durchgeführt werden. (Belegarten 10 – 20). 

 Schlüsselfeld: 

 AcashFibId(I4) fortlaufende Id über die Finanzvorgänge

 Weitere Felder:

 BedienerId(I4) Bedienernummer desjenigen, der den Finanzvorgang erzeugt hat

 Betrag(N4) der Betrag, der an die FiBu übergeben wird (in Buchwährung)

 Datum(D4) Datum, an dem der Finanzvorgang erzeugt wurde

 Gegenkonto(I4) Konto, gegen das gebucht wird (z.B. Kundenkonto,

Bankverrechnungskonto, ...)

 Hauptkonto(I4) im Normalfall: Kassenkonto

 Haupttext(char(100)) Bezeichnung des Hauptkontos

 NettoKennz(I2) nur bei Geldentnahmen befüllt (Netto/Brutto)

 SH(I2) Soll/Haben-kennzeichen (1: Auszahlungen,... , 2: Einzahlungen,...)

 Steuergruppe(I2) Steuergruppe bei Geldentnahmen

 Steuerklasse(I2) Steuerklasse bei Geldentnahmen

 Steuerschluessel(I2) Steuerschlüssel bei Geldentnahmen

 Text(char(100)) Bezeichnung des Gegenkontos

**AcashKasSturz:** Hier werden detailliert Informationen über die Zählung abgelegt.

Dabei werden nur die Stückelungen weggeschrieben, wo wirklich was

eingegeben wurde.

Schlüsselfelder:

KasSturzId(I4) Id des Beleges, dem diese Zählung zugeordnet ist

KasSturzKs(I4) Kassennummer, dem diese Zählung zugeordnet ist

KasSturzNr(I4) fortlaufende Nummer (je Stückelung, wird innerhalb einer Zählung

diese Nummer vergeben)

 FilialNummer(I4) Filialnummer, an der die Zählung erzeugt wurde

 Weitere Felder:

 KasSturzAnz(I4) Anzahl der Münzen/Scheine innerhalb der Stückelung

 KasSturzBetrag(N4) KasSturzAnz \* KasSturzWert

 KasSturzWert(char(15)) Stückwert der Münzen/Scheine innerhalb der Stückelung

(z.B. 0.01 für die kleinste Währungseinheit)

**AcashKonfKsys:** In dieser Relation sind die Eintragungen der Kassensystemverwaltung

abgelegt. (Hardwareansteuerungen)

 Schlüsselfeld:

 KsyNr(I4) logische Nummer des Kassensystems (diese wird mit dem AHOI.INI-

Eintrag verprobt und wird über KsKSY mit der Relation

AcashStmdKsse verdrahtet; gleiches gilt für BelegKsy aus der Relation AcashBelg)

 Weitere Felder:

 KSyASK(I4) aktuelle Sitzungsnummer, in der sich das Kassensystem befindet.

 KsyBelDrDev(char(100))

 KsyBelDrTyp(char(50))

 KsyBez(char(50)) Bezeichnung des Kassensystems 

 KsyBonDr(I4) hier wird der Bondrucker aus den Druckertypen abgelegt

 KsyBonDrDev(char(100)) Schnittstelle, an der der Bondrucker hängt (z.B. LPT1)

 KsyBonDrPar(char(20)) Baudrate, Parity,… (normal : 9600,n,8,1) 

 KsyBonDrRec(char(10)) Puffer-Größe Empfang (normal :1024)

 KsyBonDrTra(char(20)) Puffer-Größe Senden (normal :1024)

 KsyBonDrTyp(char(50)) Bezeichnung des Druckertyps (wird aus Druckertypen

 übernommen) 

 KsyCardNr(I4) welches Kartenlesegerät ist zugeordnet ? (entspricht dem

Schlüsselfeld aus der Relation EC_Lesegeraet

 KsyCardRdrDev(char(50))

 KsyCardRdrTyp(char(50))

 KsyDat(TS) Anlagedatum des Kassensystems

 KsyDisplayDev(char(50)) Schnittstelle, an der das Display hängt (z.B. COM1)

 KsyDisplayPar(char(20)) Baudrate, Parity,… (normal : 9600,n,8,1) 

 KsyDisplayDrRec(char(10)) Puffer-Größe Empfang (normal :1024)

 KsyDisplayTra(char(20)) Puffer-Größe Senden (normal :1024)

 KsyDisplayTyp(char(50)) Bezeichnung des Displays

 KsyJouDrDev(char(100)) 

 KsyJouDrPar(char(20)) 

 KsyJouDrRec(char(10)) 

 KsyJouDrTra(char(20)) 

 KsyJouDrTyp(char(50)) 

 KsyLadenDev(char(100)) wo ist die Schublade angeschlossen (z.B. COM2)

 KsyLadenTyp(char(50)) Bezeichnung des Schubladentyps

 KsyLokal(char(50))

 KsyLstDrDev(char(100))

 KsyLstDrTyp(char(50))

 KsyTastatur(char(50))

 KsyUpdate(TS) Datum der letzten Änderung in der Kassensystemverwaltung

**AcashSteuSeq:** Hier werden Steuersequenzen hinterlegt (z.B. für Displayansteuerung,

Druckerspezifisches Schublade Öffnen,...)

Bem.: die Steuersequenzen fürs das EPSON-Display sind unter einer negativen Drucknummer hinterlegt; beim Einspielen der passenden Drucker über OSQL-Methoden (epson_bon.sql, star.sql,... geschieht diese Versorgung automatisch)

 Schlüsselfeld:

 DruckNr(I4) fortlaufender Schlüssel

 Weitere Felder:

 DruckBez(I2) über ein Formatstring ist hier auswählbar, für welche Aktion

Steuersequenzen hinterlegt werden.

 Drucker(I4) hier wird ein Druckertyp ausgewählt

 DruckWert(char(100)) hier wird die eigentliche Steuersequenz hinterlegt.

**AcashStmdKsse:** In dieser Relation sind die Eintragungen der Kassenverwaltung

abgelegt.

 Schlüsselfeld:

KsNr(I4) logische Kassenummer (hierauf beruhen die Kassennummern in AcashBelgKsiz, AcashBelg, AcashBelgZhlg, ...)

Weitere Felder:

KsAnmDat(TS) Datum, an dem die Kasse das letzte Mal eröffnet wurde

KsASK(I4) aktuelle Sitzungsnummer dieser Kasse

KsBedNr(I4) Bedienernummer des Bedieners, der aktuell Vorgänge erfasst

KsBelId(I4) BelegId des letzten an dieser Kasse erfassten Vorgangs

KsBelNr(I4) Belegnummer des letzten an dieser Kasse erfassten Vorgangs

KsBez(char(50)) Bezeichnung der Kasse

KsDat(TS) Datum, an dem die Kasse angelegt wurde

KsFiBuKto(I4) Kassenkonto in der FiBu

KsFiBuKto2(I4) Verrechnungskonto in der FiBu (Vorbelegung bei Geldentnahmen)

KsHauptKs(I2) 1: es handelt sich bei dieser Kasse um eine Hauptkasse

 0: diese Kasse ist eine Unterkasse

 KsHauptKsNr(I4) bei Unterkassen: Nummer der zugeordneten Hauptkasse

 Bei Hauptkassen: Kassennummer

KsKSY(I4) welches Kassensystem aus der Kassensystemverwaltung ist dieser Kasse zugeordnet

KsStatus(I2) 

KsWechsel(N4) Betrag des Wechselgeldes, das defaultmäßig eingelegt wird

**AcashStmdOpti:** Hier werden die Kasseneinstellungen verwaltet

Schlüsselfelder:

OptiGruppe(char(50)) logische Gruppe, zu der die Kasseneinstellung gehört (z.Zt. ex. die

Gruppen A.eins, Allgemein, Barverkauf, Displaytext, Formulare, Geld,

Kasse, Konten, Kunden, Waehrung und Zahlungsmeldung)

 OptiNr(I4) logische Nummer innerhalb einer Gruppe

 VorlageNummer(I4) fortlaufende Nummer eines kompletten Satzes von Einstellungen,

wird referenziert in Kassenverwaltung (hierdurch kann jede Kasse auf

einen eigenen Satz von Einstellungen zurückgreifen)

weitere Felder:

OptiBez(char50)) erläuternde Bezeichnung des Satzes

OptiHilfe(char(255)) Kurzbeschreibung, was dieser Satz bewirkt

OptiWert(char(50)) einziges pflegbares Feld; hier wird die eigentliche Einstellung vorgenommen.

**AcashStoKsiz:** Hier wird pro Sitzung der Kasse 0 (Barverkaufssystem) die Anzahl

der Umwandlungen / Stornierungen von Zahlungsmitteln

bestandsmäßig weggeschrieben.

 Schlüsselfelder:

 StoKsiASK(I4) Sitzungsnummer des Barverkaufssystems

 FilialNummer(I4) Filialnummer

 Weitere Felder:

 BarUmwNeuAnz(I4) Anzahl der Umwandlungen in Bargeld

 BarUmwNeuSum(N4) Summe der Beträge bei Umwandlungen in Bargeld

 BezStoAnz(I4) Anzahl der Stornierungen von Bankeinzügen

 BezStoSum(N4) Summe der Beträge von stornierten Bankeinzügen

 BezUmwAltAnz(I4) Anzahl der Bankeinzüge, die umgewandelt wurden

 BezUmwAltSum(N4) Summe der Bankeinzüge, die umgewandelt wurden

 BezUmwNeuAnz(I4) Anzahl der Zahlungsmittel, die in Bankeinzüge umgewandelt wurden

 BezUmwNeuSum(N4) Summe der Zahlungsmittel, die in Bankeinzüge umgewandelt wurden

 GutStoAnz(I4) Anzahl der Stornierungen von Gutscheinen

 GutStoSum(N4) Summe der Beträge von stornierten Gutscheinen

 GutUmwAltAnz(I4) Anzahl der Gutscheine, die umgewandelt wurden

 GutUmwAltSum(N4) Summe der Gutscheine, die umgewandelt wurden

 GutUmwNeuAnz(I4) Anzahl der Zahlungsmittel, die in Gutscheine umgewandelt wurden

 GutUmwNeuSum(N4) Summe der Zahlungsmittel, die in Gutscheine umgewandelt wurden

 KreStoAnz(I4) Anzahl der Stornierungen von Kreditkarten

 KreStoSum(N4) Summe der Beträge von stornierten Kreditkarten

 KreUmwAltAnz(I4) Anzahl der Kreditkarten, die umgewandelt wurden

 KreUmwAltSum(N4) Summe der Kreditkarten, die umgewandelt wurden

 KreUmwNeuAnz(I4) Anzahl der Zahlungsmittel, die in Kreditkarten umgewandelt wurden

 KreUmwNeuSum(N4) Summe der Zahlungsmittel, die in Kreditkarten umgewandelt wurden

 SchStoAnz(I4) Anzahl der Stornierungen von Schecks

 SchStoSum(N4) Summe der Beträge von stornierten Schecks

 SchUmwAltAnz(I4) Anzahl der Schecks, die umgewandelt wurden

 SchUmwAltSum(N4) Summe der Schecks, die umgewandelt wurden

 SchUmwNeuAnz(I4) Anzahl der Zahlungsmittel, die in Schecks umgewandelt wurden

 SchUmwNeuSum(N4) Summe der Zahlungsmittel, die in Schecks umgewandelt wurden

**AcashStoZamiProto:** Hier wird protokolliert, wenn ein Zahlungsmittel storniert wurde.

Schlüsselfeld:

ZamiStoId(I4) Id des stornierten Zahlungsmittels (entspricht dem Schlüssel der Relation AcashBelgZami)

Weitere Felder:

BedienerId(I4) Bedienernummer desjenigen Bedieners, der die Stornierung durchgeführt hat.

Zamistobetrag(N4) Betrag des stornierten Zahlungsmittels

Zamistornodatum(D4) Datum der Stornierung

**AcashUmwZamiProto:** Hier wird protokolliert, wenn ein Zahlungsmittel umgewandelt

wurde.

 Schlüsselfeld:

 ZamiUmwId(I4) Id des umgewandelten Zahlungsmittels (entspricht dem Schlüssel der

Relation AcashBelgZami)

 ZamiUmwNummer(I4) Fortlaufende Nummer des Umwandlungsschrittes (da eine Mehrfach-

 Umwandlung möglich ist)

 Weitere Felder:

 BedienerId(I4) Bedienernummer desjenigen Bedieners, der die Umwandlung

durchgeführt hat

 zamiumwalt(I2) Zahlungsart vor der Umwandlung

 zamiumwbetrag(N4) Betrag des umgewandelten Zahlungsmittels

 zamiumwdatum(D4) Datum der Umwandlung

 zamiumwneu(I2) Zahlungsart nach der Umwandlung

**EC_Lesegeraet:** Hier können Daten über Ausleserichtlinien für unterschiedliche

Kartenlesegeräte hinterlegt werden.

 Schlüsselfeld:

 LesegeraetNr(I4) Nummer des Lesegerätes (wird in Kassensystemverwaltung

referenziert)

 weitere Felder:

 BLZSpalte(I2) ab welcher Spalte soll die Bankleitzahl ausgelesen werden

 BLZStellig(I2) welche Stelligkeit hat die Bankleitzahl (Default 8)

 BLZZeile(I2) in welcher Zeile steht die Bankleitzahl

 Gueltigjjspalte(I2) ab welcher Spalte soll das Gültigkeitsjahr ausgelesen werden

 Gueltigjjstellig(I2) welche Stelligkeit hat das Gültigkeitsjahr (Default 2)

 Gueltigjjzeile(I2) in welcher Zeile steht das Gültigkeitsjahr

 Gueltigmmspalte(I2) ab welcher Spalte soll der Gültigkeitsmonat ausgelesen werden

 Gueltigmmstellig(I2) welche Stelligkeit hat der Gültigkeitsmonat (Default 2)

 Gueltigmmzeile(I2) in welcher Zeile steht der Gültigkeitsmonat

 KontoSpalte(I2) ab welcher Spalte steht die Kontonummer

 KontoStellig(I2) welche Stelligkeit hat die Kontonummer (Default 10)

 KontoZeile(I2) in welcher Zeile steht die Kontonummer

 LesegeraetBez(char(50)) Bezeichnung des Lesegerätes

 PruefSpalte(I2) ab welcher Spalte steht die Prüfziffer

 Pruefzeile(I2) in welcher Zeile steht die Prüfziffer

 Pruefziffer(char(50)) hier wird die Prüfziffer eingetragen

Zur Zeit redundante Relationen (sie bleiben es vermutlich auch)

AcashBelgPosi

AcashKonfDrus

AcashKonfForm

AcashKonfHilf

AcashKonfTast

AcashStatBelg

AcashStatBelgPosi

AcashStatBelgZhlg

AcashStatFreq

AcashStmdArti

AcashStmdArtiTemp

AcashStmdBank

AcashStmdBedi

AcashStmdBediTest

AcashStmdBediZugr

AcashStmdKost

AcashStmdKund

AcashStmdMand

AcashStmdSeku

AcashStmdUstr

AcashStmdWaer

AcashStmdWagr
