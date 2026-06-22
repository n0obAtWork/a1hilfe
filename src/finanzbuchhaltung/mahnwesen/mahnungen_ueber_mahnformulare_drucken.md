# Mahnungen über Mahnformulare drucken

<!-- source: https://amic.de/hilfe/mahnungenbermahnformularedruck.htm -->

Das Formular ist, wenn nicht anders eingerichtet 2200. Es existieren zu diesem Typ folgende Formularbereiche:

• 301 Mahnkopf Formularkopf

• 302 Mahntexte Zeilentyp

• 303 Mahnabschluss Abschluss

• 304 Mahnposition Zeilentyp

• 305 Mahnfolgekopf Folgekopf

• 306 Mahnfuß Fuß

• 307 Mahnsummenzeile Zeilentyp

• 308 Mahnsummenkopf Zeilentyp

• 309 Mahnsummenfuß Zeilentyp

• 310 Mahnung Betreffzeile Mail Betreff

Folgende Variablen sind in allen Teilen (Kopf, Fuß und Zeilentyp) verfügbar. Formularbereiche, die nicht separat mit aufgeführt werden, enthalten nur Festtext oder diese Felder!

| Bezeichnung | Typ | Nr | Beschreibung |
| --- | --- | --- | --- |
| AdressId | Block | 6 | Hauptanschrift wie im Kundenstamm hinterlegt |
| AdressAnrede | Normal | 3 | Anrede wie im Anschriftenstamm hinterlegt |
| AdressName | Normal | 3 | Name wie im Anschriftenstamm hinterlegt |
| AdressKurzName | Normal | 3 | Kurzname wie im Anschriftenstamm hinterlegt |
| AdressBezeich | Normal | 3 | Bezeichnung wie im Anschriftenstamm hinterlegt |
| AdressTelefon | Normal | 3 | Telefon wie im Anschriftenstamm hinterlegt |
| AdressTelefax | Normal | 3 | Telefax wie im Anschriftenstamm hinterlegt |
| KundenNummer | Numerisch | 4 | Nummer des Kunden wie im Kundenstamm hinterlegt |
| KundNummer | | | S.o. |
| GegenNummer | Numerisch | 4 | Gegennummer wie im Kundenstamm hinterlegt |
| KundGegenNummer | | | S.o. |
| FilialNummer | Numerisch | 4 | |
| VerkGebNummer | Numerisch | 4 | Verkaufsgebiet wie im Kundenstamm hinterlegt |
| VertGrNummer | Numerisch | 4 | Vertretergruppe wie im Kundenstamm hinterlegt |
| VertGrBezeich | Normal | 3 | Vertretergruppenbezeichnung wie in Stammdaten Vertretergruppe hinterlegt |
| LetzteZahlung | Normal | 5 | Belegdatum der letzten eingegangenen Zahlung |
| Datum | Normal | 5 | Datum der Mahnung |
| MahnungDatum | | | S.o. |
| Zahlfrist | Normal | 5 | Frist bis wie beim Druck unter **Zahlungsfrist** eingetragen |
| ZahlDatum | Normal | 5 | Zahlungsdatum wie beim Druck unter **Zahlungsdatum** eingetragen |
| MahnBetrag | Numerisch | 4 | Mahnbarer Betrag |
| MahnungBetrag | | | S.o. |
| MahnGebuehr | Numerisch | 4 | Mahngebühr |
| MahnZinsen | Numerisch | 4 | Zinsen |
| MahnungZinsen | | | S.o. |
| MahnungSumme | Numerisch | 4 | Betrag + Gebühr + Zinsen |
| MahnungSaldo | Numerisch | 4 | Saldo aller in dieser Mahnung enthaltenen fälligen Beleg ( mit Verrechnung Soll und Haben) |
| MahnungSaldoZinsen | Numerisch | 4 | Zinsen dieser Mahnung ( mit Verrechnung Soll und Haben ) |
| MahnungSaldoSumme | Numerisch | 4 | Saldo + Zinsen |
| MahnungWBetrag | Numerisch | 4 | Mahnbarer Betrag in Fremdwährung.<br>Keine Gruppierung nach Währung! |
| MahnungWSaldo | Numerisch | 4 | Saldo in Fremdwährung aller in dieser Mahnung enthaltenen fälligen Beleg (mit Verrechnung Soll und Haben)<br>Keine Gruppierung nach Währung! |
| GesamtMahnBetrag | Numerisch | 4 | Betrag aller auf der Mahnung ausgewiesenen Posten, also nicht nur die mahnbaren (siehe Mahnbetrag). |
| GesamtMahnZinsen | Numerisch | 4 | Zinsen |
| GesamtMahnGebuehr | Numerisch | 4 | Gebühr |
| GesamtMahnSumme | Numerisch | 4 | GesamtMahnBetrag+Zinsen+Gebühr |
| MahnStufe | Numerisch | 4 | Mahnstufe (eigentlich nur interessant bei Mahnungen getrennt nach Mahnstufe, weil sonst 0 ) |
| MahnStufnummer | | | S.o. |
| MahnStufBezeich | Normal | 3 | Bezeichnung der Mahnstufe wie in Stammdaten Mahnstufe hinterlegt |
| Mahngruppe | Numerisch | 4 | Mahngruppe |
| MahnGrupNummer | | | S.o. |
| MahnGrupBezeich | Normal | 3 | Bezeichnung der Mahngruppe wie in Stammdaten unter Mahngruppe hinterlegt |
| MahnId | Numerisch | 4 | Identifikation der Mahnung |
| MahnungId | | | S.o. |
| Mahnnummer | Normal | 3 | Belegnummer der Zinsen falls Zinsen berechnet wurden |
| Mahnummer | Numerisch | 4 | Numerischer Anteil der Belegnummer der Zinsen |
| MahnKreis | Numerisch | 4 | NummernKreisnummer der Zinsen |
| MahnText | Block | 8 | Mahntext wie in den Stammdaten Mahntext unter besagter MahnGruppe und -stufe hinterlegt unter Mahntextnummer **1** |
| MahnTextkopf | | | S.o |
| MahnTextFuss | Block | 8 | <br>Mahntext wie in den Stammdaten Mahntext unter besagter MahnGruppe und -stufe hinterlegt unter Mahntextnummer **2** |
| MahntextKopf2 | Block | 8 | Mahntext wie in den Stammdaten Mahntext unter besagter Mahngruppe und -stufe hinterlegt unter Mahntextnummer **3** |
| MahnTextfuss2 | Block | 8 | Mahntext wie in den Stammdaten Mahntext unter besagter Mahngruppe und -stufe hinterlegt unter Mahntextnummer **4** |
| MandSteuerNummer | Normal | 3 | Steuernummer aus dem Mandantenstamm |
| MandUStStatKennz | Normal | 3 | USt-IdNr. aus dem Mandantenstamm |
| MandBezeich | Normal | 3 | Name des Mandanten |
| MandAdressKurzName | Normal | 3 | In der Anschrift des Mandanten hinterlegte Kurzbezeichnung |
| MandAdressAnrede | Normal | 3 | In der Anschrift des Mandanten hinterlegte Anrede |
| MandAdressVorName | Normal | 3 | In der Anschrift des Mandanten hinterlegter Vorname |
| MandAdressName | Normal | 3 | In der Anschrift des Mandanten hinterlegter Name |
| MandAdressOrt | Normal | 3 | In der Anschrift des Mandanten hinterlegter Ort |
| MandAdressPLZ1 | Normal | 3 | In der Anschrift des Mandanten hinterlegte Postleitzahl |
| MandAdressStrasse | Normal | 3 | In der Anschrift des Mandanten hinterlegte Straße |
| MandAdressTelefon | Normal | 3 | In der Anschrift des Mandanten hinterlegte Telefonnummer |
| MandAdressTelefax | Normal | 3 | In der Anschrift des Mandanten hinterlegte Faxnummer |

**Achtung:** Mahntexte können auch unter Formularbereich 302 separat ausgegeben werden!

• 302 Mahntexte

Fall 1 ( Bemerkid im Mahntext (Mahnstamm) ist 0 oder nicht eingetragen )

| Bezeichnung | Typ | Nr | Beschreibung |
| --- | --- | --- | --- |
| Text | Normal | 3 | Mahntext wie im Mahnstamm hinterlegt. Es werden so viele Zeilen, wie Text zu dieser Mahnstufe und -gruppe hinterlegt sind, gedruckt. |

Fall 2 ( Bemerkid im Mahntext (Mahnstamm) ist nicht 0 )

| Bezeichnung | Typ | Nr | Beschreibung |
| --- | --- | --- | --- |
| Text | Normal | 3 | Der Text wird aus den Bemerkungstexten entsprechend der BemerkungsId genommen und zwar pro eingetragenen Mahntext. |

Bei der Einrichtung der Mahntexte (Direktsprung **[FIMAT]**) können die Daten, die beim Mahndruck als Zahlungsdatum bzw. als Zahlungsfrist eingegeben werden, mit ausgegeben werden. Diese Werte können mit in den Mahntext eingebaut werden. Dabei ist zu beachten, dass folgende Variablen mit Doppelpunkt vor dem Wort in den Text eingebaut werden:

Zahldatum$

und

Zahlfristdatum$

**Achtung**: *Groß- und Kleinschreibung beachten!!!*

Beispiel:

Bitte zahlen Sie bis zum :Zahlfristdatum$ den angegebenen Betrag.

• 304 Mahnposition

| Bezeichnung | Typ | Nr. | Beschreibung |
| --- | --- | --- | --- |
| BelegKlasse | Numerisch | 4 | Vorgangsklasse des Beleges |
| FiBuV_Klasse | | | S.o. |
| FiBuV_KlKurzBez | Normal | 3 | Kurzbezeichnung der Klasse ( z.B. ZA, AR, AG,....) |
| FiBuV_KlBezeich | Normal | 3 | Ausführliche Bezeichnung |
| PosNummer | Numerisch | 4 | Position auf der Mahnung |
| MahnPosZaehl | | | S.o. |
| PosBetrag | Numerisch | 4 | Zu Mahnender Betrag |
| MahnPosBetrag | | | S.o. |
| PosZinsen | Numerisch | 4 | Zinsen zu dieser Mahnung |
| MahnPosZinsen | | | S.o. |
| MahnPosZinsSatz | Numerisch | 4 | Mit welchem Satz wurden die Zinsen berechnet |
| MahnPosZinsTage | Numerisch | 4 | Mit wie vielen Tagen wurden gerechnet |
| MahnStufNummer | Numerisch | 4 | Mahnstufe. Ist 0, wenn dieser Belegt nicht gemahnt wurde ( bei Verrechnung Soll/Haben o ä .). Soll die Tatsächliche genommen werden bitte die OPMahnstufe (S.u.)eintragen. |
| BelegMahnStufe | | | S.o. |
| BelegNummer | Normal | 3 | Belegnummer |
| FiBuV_Nummer | | | S.o. |
| EB_Belegnummer | Normal | 3 | Bei der Belegklasse EB wird die Referenznummer ausgegeben, ansonsten die Belegnummer |
| BelNummer | Numerisch | 4 | Numerischer Anteil der Belegnummer |
| FiBuV_NumNummer | | | S.o. |
| BelegId | Numerisch | 4 | Programminterne Eindeutige Identifikation des Beleges |
| FiBuV_Id | | | S.o. |
| Belegdatum | Normal | 5 | Belegdatum |
| FiBuV_Datum | | | S.o. |
| FremdNummer | Normal | 3 | Eventuell eingetragene Fremdnummer ( Bei ER und EG ) |
| FiBuV_FremdNr | | | S.o. |
| NummernKreis | Numerisch | 4 | Nummernkreis diese Beleges |
| NumKreisNummer | | | S.o. |
| Filiale | Numerisch | 4 | |
| FilialNummerFil | | | S.o. |
| Zentrale | Numerisch | 4 | |
| FilialNummerZen | | | S.o. |
| Jahr | Numerisch | 4 | Jahr und - |
| JahrNummer | | | S.o. |
| Peri | Numerisch | 4 | Periode diese Beleges |
| PeriNummer | | | S.o. |
| Waehrung | Numerisch | 4 | Währungsnummer |
| WaehrNummer | | | S.o. |
| WaehrBezeich | Normal | 3 | Die Währung in der der Beleg erfasst ist |
| KostenStelle | Numerisch | 4 | Kostenstelle |
| KostStelNummer | | | S.o. |
| ZahlBed | Numerisch | 4 | Zahlungsbedingung |
| ZahlBedNummer | | | S.o. |
| BelegBetrag | Numerisch | 4 | Betrag |
| FiBuVP_Betrag | | | S.o. |
| Sollbetrag | | | Betrag wenn Soll |
| Habenbetrag | | | Betrag wenn Haben |
| BelegSH | Normal | 3 | **\-** oder **+** |
| FiBuVP_SollHaben | | | S.o. |
| BelegWBetrag | Numerisch | 4 | Betrag in Fremdwährung |
| FiBuVPW_Betrag | | | S.o. |
| BelegWSkonto | Numerisch | 4 | Skontobetrag in Fremdwährung |
| FiBuVPW_SkoBetrag | | | S.o. |
| BelegSkonto | Numerisch | 4 | Skontobetrag |
| FiBuVP_SkoBetrag | | | S.o. |
| BelegValuta | Normal | 5 | Valutadatum dieses Beleges / Position |
| FiBuVP_ValDatum | | | S.o. |
| BelegSkontoDatum | Normal | 5 | Skontodatum dieses Beleges /Position |
| FiBuVP_SkoDatum | | | S.o. |
| BelegSkontoSatz | Numerisch | 4 | Skontosatz diese Beleges / Position |
| FiBuVP_SkoSatz | | | S.o. |
| BelegText | Normal | 3 | |
| FiBuVP_Text | | | S.o. |
| BelegMahnDatum | Normal | 5 | Letztes Mahndatum dieses Beleges |
| OPMahnDatum | | | S.o. |
| OPMahnStufe | Numerisch | 4 | Mahnstufe des Beleges (Nur interessant, wenn nicht getrennt nach Mahnstufe) |
| BelegMahnSperre | Numerisch | 4 | Eventuell in der OP-Verwaltung gesetzte Mahnsperre |
| OPMahnSperre | | | S.o. |

• 307 MahnSummenZeile

| Bezeichnung | Typ | Nr. | Beschreibung |
| --- | --- | --- | --- |
| MahnStufNummer | Numerisch | 4 | Mahnstufe dieser Belegposition |
| MahnStufBezeich | Normal | 3 | Bezeichnung der Mahnstufe wie im Stammdatenpfleger MAHNSTUFE hinterlegt. |
| MahnPosBetrag | Numerisch | 4 | Summe aller mahnbaren Beträge dieser Mahnstufe |
| MahnPosZinsen | Numerisch | 4 | Summe aller mahnbaren Zinsen dieser Mahnstufe |
| BelegSH | Normal | 3 | Sollhabenkennzeichen als **\-** |
| FiBuVP_SollHaben | Normal | 3 | S.o. |
| FiBuVP_Betrag | Numerisch | 4 | Summe aller Beträge dieser Mahnstufe |
| FiBuVPW_Betrag | Numerisch | 4 | Summe aller Beträge dieser Mahnstufe in Fremdwährung |

• 309 MahnSummenFuß

| Bezeichnung | Typ | Nr. | Beschreibung |
| --- | --- | --- | --- |
| MahnPosBetrag | Numerisch | 4 | Summe aller mahnbaren Beträge |
| MahnPosZinsen | Numerisch | 4 | Summe aller mahnbaren Zinsen |
| BelegSH | Normal | 3 | Sollhabenkennzeichen als **\-** |
| FiBuVP_SolHaben | Normal | 3 | S.o. |
| FiBuVP_Betrag | Numerisch | 4 | Summe aller Beträge |
| FiBuVPW_Betrag | Numerisch | 4 | Summe aller Beträge in Fremdwährung |

• 310 Mahnung Betreffzeile

Mit diesem Formularbereich kann für den Mailversand eine Betreffzeile definiert werden. Es stehen hier die Positionen zur Verfügung, die mit F3 ausgewählt werden können. Beim normalen Druck erscheint diese Zeile nicht.
