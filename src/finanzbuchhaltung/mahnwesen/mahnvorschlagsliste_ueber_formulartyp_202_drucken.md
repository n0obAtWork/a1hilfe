# Mahnvorschlagsliste über Formulartyp 202 drucken

<!-- source: https://amic.de/hilfe/mahnvorschlagslisteberformular.htm -->

Hauptmenü > Mahn-, Zahl-, Zinswesen > Mahnwesen > Mahnvorschläge bearbeiten > Funktion Liste über Formular **F8**

Direktsprung **[MHVB]**.

Es ist möglich, Mahnvorschläge über das Formular 202 zu drucken. Folgende Formularbereiche werden dabei verwendet.

- 301 MahnKopf Formularkopf
- 303 MahnAbschluß Seiten Ende der letzten Seite
- 304 Mahnposition Einzelne Zeile
- 305 MahnFolgekopf Überschrift der nächsten Seiten
- 306 MahnFuß Seiten Ende
- 308 MahnSummenKopf Überschrift Pro Konto
- 309 MahnSummenFuß SummenZeile Pro Konto

Folgende Variablen sind in allen Teilen (Kopf, Folgekopf, Fuß und Abschluss) verfügbar, die nicht Zeilentyp sind. Formularbereiche, die nicht separat mit aufgeführt werden, enthalten nur Festtext oder diese Felder!

| Bezeichnung | Typ | Nr | Beschreibung |
| --- | --- | --- | --- |
| Mahnlistnummer | Numerisch | 4 | Nummer der aktuell gedruckten Zahlungsliste |
| Mahnlistbezeich | Text | 3 | Bezeichnung der Mahnliste |
| Mahnlsitdatum | Datum | 5 | Erstelldatum |
| BedienerId | Numerisch | 4 | Id des Bedieners, der diese Liste erstellt hat. |
| Bedienerkurz | Text | 3 | Kurzbezeichnung –""- |
| Bedienername | Text | 3 | Name –""- |

<p class="just-emphasize">304 Positionszeile</p>

| Bezeichnung | Typ | Nr | Beschreibung |
| --- | --- | --- | --- |
| MahnStufPosition | Numerisch | | Mahnstufe ( 0 bei Verrechnung) |
| MahnVorPosbetrag | Numerisch | | Betrag der Mahnung ( Siehe FiBuVP_Betrag) |
| MahnVorPosVDatum | Datum | | Valutadatum |
| MahnVorPosZinsen | Numerisch | | Zinsen der Position |
| MahnVorPosZinsSatz | Numerisch | | Zinssatz, mit dem die Zinsen dieser Position berechnet wurden |
| MahnVorPosZinsTage | Numerisch | | Mit wie vielen Tage wurde gerechnet |
| MahnVorPosZinsGr | Numerisch | | Zinsgruppe mit der diese Zinsen berechnet wurden. |
| FiBuV_Id | Numerisch | | Intern |
| FiBuV_PosZaehler | Numerisch | | Intern |
| FiBuVP_BuchTyp | Numerisch | | Buchungstyp |
| Fibuv_klasse | Text | | Belegklasse (ZA AR AG ER EG.....) |
| FiBuV_Nummer | Text | | Belegnummer |
| NumKreisnummer | Numerisch | | Nummernkreis aus dem sich diese Nummer |
| FiBuV_NumNummer | Numerisch | | Numerischer Anteil der Belegnummer |
| FiBuV_FremdNr | Text | | Referenznummer |
| Perinumer | Numerisch | | Buchungsperiode |
| JahrNummer | Numerisch | | Jahrnummer |
| FiBuVPW_Kurs | Numerisch | | Kurs |
| FiBuV_ErfDatum | Datum | | Erfassungsdatum |
| Waehrungsnummer | Text | | Kurzbezeichnung der Währung |
| ZahlBedNummer | Numerisch | | Zahlungsbedingung |
| FiBuVP_SollHaben | Text | | Sollhabenkennzeichen des Betrages |
| FiBuVP_SkoBetrag | Numerisch | | Skontobetrag der Rechnung |
| FiBuVPW_SkoBetrag | Numerisch | | Skontobetrag in Fremdwährung |
| FiBuVP_Betrag | Numerisch | | Betrag ( Siehe MahnVorPosBetrag) |
| FiBuVPW_Betrag | Numerisch | | Betrag in Fremdwährung |
| FiBuVP_ValDatum, | Datum | | Fälligkeitsdatum |
| FiBuVP_SkoDatum | Datum | | Skontodatum |
| FiBuVP_Text | Text | | Belegtext |
| FIBuVP_AuszKennz | Numerisch | | Auszifferungskennzeichen |
| FiBuVP_AuszDatum | Text | | Datum der Auszifferung |

<p class="just-emphasize">308 Gruppenkopf</p>

| Bezeichnung | Typ | Nr | Beschreibung |
| --- | --- | --- | --- |
| MahnGrupNummer | Numerisch | | MahnGruppe |
| MahnStufMinimum | Numerisch | | Kleinste Mahnnummer bei Sammelmahnung |
| MahnStufMaximum | Numerisch | | Größte Mahnnummer bei Sammelmahnung |
| MahnStufnummer | Numerisch | | Mahnstufe bei Mahnen getrennt nach Mahnstufe |
| MahnVorbetrag | Numerisch | | Gesamtbetrag der Mahnung |
| MahnVorDatum | Datum | | Datum |
| MahnVorgebühr | Numerisch | | Mahngebühr |
| MahnVorzinsen | Numerisch | | Gesamtzinsen |
| KontoNummer | Numerisch | | |
| Kundbezeich | Text | | |
| Adressort | Text | | |
| AdressPLZ1 | Text | | |
| AdressOrtsTeil | Text | | |
| AdressStrasse | Text | | |
| AdressTelefon | Text | | |
| AdressTelefax | Text | | |
| AdressAnrede | Text | | |
| AdressVorname | Text | | |
| Adressname | Text | | |
| Saldo | Numerisch | | Gesamtsaldo zum Zeitpunkt des Druckes |
| SaldoSH | Text | | Sollhabenkennzeichen zum Zeitpunkt des Druckes |

<p class="just-emphasize">309 Gruppenende (Summen)</p>

| Bezeichnung | Typ | Nr | Beschreibung |
| --- | --- | --- | --- |
| Stufe0 | Numerisch | | Summe Positionen mit Stufe 0 ( Verrechnung oder Auflistung sonstiger Posten) |
| Stufe1 bis Stufe9 | Numerisch | | Summe aller Positionen mit Stufe n (1-9) |
| AbStufe1 bis AbStufe9 | Numerisch | | Summe aller Positionen mit Stufe größer als n (1-9) |
