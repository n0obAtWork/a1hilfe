# XML-Struktur

<!-- source: https://amic.de/hilfe/xmlstruktur.htm -->

Beispiel einer Rechnungsverbuchung:

```xml
<?xml version="1.0"
encoding="ISO-8859-1"?>
<EDILD01>
    <Header>
<Empfaengername>Buchungsstelle</Empfaengername>
<Erstellungsdatum>18.09.07</Erstellungsdatum>
<Erstellungszeit>15:12</Erstellungszeit>
<Nachrichtentyp>accounting</Nachrichtentyp>
<Testuebertragung>1</Testuebertragung>
<externe-Referenz>AMIC-13652314315</externe-Referenz>
<AnzahlBelege>1</AnzahlBelege>
<LaufendeNr>1</LaufendeNr>
<Belegdaten>
<Belegart>Rechnung</Belegart>
<Rechnungsnummer>4426</Rechnungsnummer>
<GWS-Nr>461456141</GWS-Nr>
<Mandant>Musterfirma</Mandant>
<Verk-an-Deb-Nr>27754</Verk-an-Deb-Nr>
<Verk-an-Adresse>A-Strasse</Verk-an-Adresse>
<Verk-an-PLZ-Code>24011</Verk-an-PLZ-Code>
<Verk-an-Ort>Kiel</Verk-an-Ort>
<Verk-an-Name>Martin Steyer</Verk-an-Name>
<Faelligkeitsdatum>18.09.07</Faelligkeitsdatum>
<Soll-Haben>Soll</Soll-Haben>
<Rechnungssumme>1562,00</Rechnungssumme>
<Rechnungssumme-inkl-mwst>1858,78</Rechnungssumme-inkl-mwst>
<MWST-Summe>296,78</MWST-Summe>
<VST-Summe>0,00</VST-Summe>
<Belegdatum>18.09.07</Belegdatum>
<Buchungsdatum>18.09.07</Buchungsdatum>
<Erstdruckinformation>18.09.07</Erstdruckinformation>
<Storno-Beleg>0</Storno-Beleg>
<Belegsteuer>
<Soll-Haben-Steuer>Haben</Soll-Haben-Steuer>
<Steuertyp>Ust</Steuertyp>
<MWST-p>19,00</MWST-p>
<MWSTBetrag>296,78</MWSTBetrag>
</Belegsteuer>
<Positionsdaten>
<Zeilennr>2</Zeilennr>
<Art>Artikel</Art>
<Nr>Fleischwurst</Nr>
<Beschreibung>kleine Fleischwurst</Beschreibung>
<Beschreibung-2>Wurstfleisch</Beschreibung-2>
<Lieferungsnr>4426</Lieferungsnr>
<Lieferdatum>18.09.07</Lieferdatum>
<Lagerortcode>Saatlager</Lagerortcode>
<Menge-Basis>100,00</Menge-Basis>
<Menge-Verkaufseinheit>100,00</Menge-Verkaufseinheit>
<Verkaufseinheitencode>Stck</Verkaufseinheitencode>
<Basiseinheitencode>Stck</Basiseinheitencode>
<Soll-Haben-Position>Haben</Soll-Haben-Position>
<Steuertyp-Position>Ust</Steuertyp-Position>
<MWST-p>19,00</MWST-p>
<Betrag>1562,00</Betrag>
<Betrag-inkl-MWST>1858,78</Betrag-inkl-MWST>
<MWST-Betrag>296,78</ MWST-Betrag >
<Preis>15,62</Preis>
<LieferscheinNr>4426</LieferscheinNr>
<Artikeluntergruppencode>0</Artikeluntergruppencode>
<Artikeluntergruppe>ohne Warengruppe</Artikeluntergruppe>
<Artikelgruppencode>0</Artikelgruppencode>
<Artikelgruppe>ohne Oberwarengruppe</Artikelgruppe>
<Artikelobergruppencode>0</Artikelobergruppencode>
<Artikelobergruppe>ohne Hauptwarengruppe</Artikelobergruppe>
<Positionstexte>
<Zeilennr>1</Zeilennr>
<Beschreibung>aus reinstem Fleisch</Beschreibung>
</Positionstexte>
</Positionsdaten>
</Belegdaten>
    </Header>
</EDILD01>
```

<p class="just-emphasize">Feldbeschreibung</p>

| &lt;Header> | Start TAG Header |
| --- | --- |
| Empfaengername | Empfängername (freie Eingabe in den Stammdaten) |
| Erstellungsdatum | Erstellungsdatum des XML-Files |
| Erstellungszeit | Erstellungszeit des XML-Files |
| Nachrichtentyp | Nachrichtentyp (freie Eingabe in den Stammdaten) |
| Testuebertragung | Kennzeichen ob es sich um eine Testübertragung handelt (0 = Nein, 1 = Ja) |
| externe-Referenz | externe Referenz (freie Eingabe in den Stammdaten) |
| AnzahlBelege | Anzahl der Belege in der XML-Struktur (z.Zt. "1") |
| LaufendeNr | Laufende Nummer für die gesendeten Daten |
| &lt;Belegdaten> | Start TAG Belegdaten |
| Belegart | Belegart des Beleges (ausgeschrieben) |
| Rechnungsnummer | Belegnummer des Beleges |
| GWS-Nummer | Nummer wird von AMIC für Ihren Betrieb vergeben |
| Mandant | Vorname und Name des Kunden |
| Verk-an-Deb-Nr | Kundennummer des Kunden |
| Verk-an-Adresse | Straße des Kunden |
| Verk-an-PLZ-Code | PLZ des Kundenortes |
| Verk-an-Ort | Ort des Kunden |
| Verk-an-Name | Name des Kunden |
| Faelligkeitsdatum | Fälligkeitsdatum des Beleges |
| Soll-Haben | Soll/Haben Kennzeichen des Belegs |
| Rechnungssumme | Belegsumme ohne Steuern |
| Rechnungssumme-inkl-mwst | Belegsumme inklusive aller Steuern |
| MWST-Summe | Steuersumme der UST |
| VST-Summe | Steuersumme der VST |
| Belegdatum | Datum des Beleges |
| Buchungsdatum | Datum des Beleges |
| Erstdruckinformation | Datum des Erstdrucks |
| Storno-Beleg | Stornokennzeichen (0 = Nein, 1 = Ja) |
| &lt;Belegsteuer> | Start TAG Belegsteuer |
| Soll-Haben-Steuer | Soll/Haben Kennzeichen der Steuer |
| Steuertyp | Steuertyp (VST/UST) |
| MWST-p | Prozent des Steuerbetrags |
| MWSTBetrag | Betrag der Steuern |
| &lt;/Belegsteuer> | Ende TAG Belegsteuer |
| &lt;Positionsdaten> | Start TAG Positionsdaten |
| Zeilennr | Zeilennummer der Belegposition |
| Art | Art der Position (z.Zt. Nur "Artikel") |
| Nr | Artikelnummer |
| Beschreibung | Beschreibung des Artikels |
| Beschreibung-2 | Beschreibung des Artikels (Stammbeschreibung) |
| Lieferungsnr | Lieferscheinnummer |
| Lieferdatum | Datum des Lieferscheins |
| Lagerortcode | Bezeichnung des Lagers |
| Menge-Basis | Menge der Position |
| Menge-Verkaufseinheit | Menge der Position |
| Verkaufseinheitencode | Mengeneinheitscode der Position |
| Basiseinheitencode | Mengeneinheitscode der Position |
| Soll-Haben-Position | Soll/Haben Kennzeichen der Position |
| Steuertyp-Position | Steuertyp der Position (VST/UST ) |
| MWST-p | Prozent des Steuerbetrags der Position |
| Betrag | Warenwert Netto |
| Betrag-inkl-MWST | Warenwert Brutto |
| MWST-Betrag | Mehrwertsteuerbetrag |
| Preis | Preis der Position |
| LieferscheinNr | Nummer des Lieferscheins |
| Artikeluntergruppencode | Nummer der Warengruppe des Artikels |
| Artikeluntergruppe | Bezeichnung der Warengruppe des Artikels |
| Artikelgruppencode | Nummer der Oberwarengruppe des Artikels |
| Artikelgruppe | Bezeichnung der Oberwarengruppe des Artikels |
| Artikelobergruppencode | Nummer der Hauptwarengruppe des Artikels |
| Artikelobergruppe | Bezeichnung der Hauptwarengruppe des Artikels |
| &lt;Positionstexte> | Start TAG Positionstexte |
| Zeilennr | Zeilennummer des Positionstextes |
| Beschreibung | Text des Positionstextes |
| &lt;/Positionstexte> | Ende TAG Positionstexte |
| &lt;/Positionsdaten> | Ende TAG Positionsdaten |
| &lt;/Belegdaten> | Ende TAG Belegdaten |
| &lt;/Header> | Ende TAG Header |
