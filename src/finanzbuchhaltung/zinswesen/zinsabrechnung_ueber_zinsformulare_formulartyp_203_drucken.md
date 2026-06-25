# Zinsabrechnung über Zinsformulare (Formulartyp 203) drucken.

<!-- source: https://amic.de/hilfe/zinsabrechnungberzinsformulare.htm -->

Hauptmenü \> Mahn-/Zahl-/Zinswesen \> Zinswesen \> Zinsabrechnung bearbeiten \> Funktion ***Abrechnung drucken***

Direktsprung **[ZIB]**

Es existiert ein Standartformular „Zinsabrechnung“ mit der Nummer –16. Zu dem Formulartyp 203 existieren folgende Formularbereiche:

- 311 Kopf Zinsabrechnung Formkopf
- 312 Kopf Zinsabrechnung Fortsetzung Folgekopf
- 314 Positionsteil Zinsabrechnung Zeilentyp
- 605 Textzeile Zeilentyp
- 315 Zinsabrechnung Betreffzeile Mail Betreffzeile
- 316 Fuß Zinsabrechnung Fuß
- 313 Abschluss Zinsabrechnung Abschluss

Da Formulare nur gedruckt werden, muss mindestens ein Zeilentyp eingerichtet sein. Will man seinem Kunden keine detaillierte Aufstellung der Bewegungen schicken, dann reicht es einfach nur eine Textzeile (Bereich 605) einzurichten. Es wird dann nur - zusätzlich zu Kopf und Fuß - eine leere Zeile gedruckt.

Folgende Variablen sind in allen Teilen (Kopf, Fuß und Zeilentyp) verfügbar. Formularbereiche, die nicht separat mit aufgeführt werden, enthalten nur Festtext oder diese Felder!

| | **Bezeichnung** | **Typ** | **Nr.** | **Bedeutung** |
| --- | --- | --- | --- | --- |
| Zinslistnummer | Numerisch | 4 | Nummer der Zinsliste |
| Kontonummer | Numerisch | 4 | Kontonummer des aktiven Kunden |
| ZinsAbrDatum | Normal | 5 | Erstellungsdatum des Zinsabrechnung |
| ZinsAbrVonDatum | Normal | 5 | Bereich von dieser Zinsabrechnung |
| ZinsAbrBisDatum | Normal | 5 | Bereich bis dieser Zinsabrechnung |
| ZinsAbrDruKennz | Numerisch | 4 | Wenn schon vorher einmal gedruckt dann 1 sonst 0 |
| ZinsAbrSollZins | Numerisch | 4 | Sollzinsen |
| ZinsAbrHabenZins | Numerisch | 4 | HabenZinsen |
| ZinsAbrZinsSaldo | Numerisch | 4 | Saldo Soll - Haben |
| ZinsAbrZinsSaldoSH | Normal | 3 | Sollhaben des Saldos |
| ZinsAbrSollZinsOrig | Numerisch | 4 | Sollzinsen |
| ZinsAbrHabenZinsOrig | Numerisch | 4 | HabenZinsen |
| ZinsAbrZinsSaldoOrig | Numerisch | 4 | Saldo Soll - Haben |
| ZinsAbrZinsSaldoOrigSH | Normal | 3 | Sollhaben des Saldos |
| Druckdatum | Normal | 5 | Tagesdatum |
| USER | Normal | 3 | Wer hat das gedruckt? |
| MANDANT | Normal | 3 | Mandantenbezeichnung |
| KontoBezeich | Normal | 3 | Bezeichnung des Kontos aus dem Kontostamm |
| KontoTyp | Numerisch | 4 | Typ des Kontos im Kontostamm (1=Sachkonto 2=PersonenKonto9 |
| Adresse | Block | 6 | Kundenadresse bzw. wie in Sachkontozins hinterlegt |
| AdressAnrede | Normal | 3 | Anrede wie im Anschriftenstamm hinterlegt |
| AdressName | Normal | 3 | Name wie im Anschriftenstamm hinterlegt |
| AdressKurzName | Normal | 3 | Kurzname wie im Anschriftenstamm hinterlegt |
| AdressBezeich | Normal | 3 | Bezeichnung wie im Anschriftenstamm hinterlegt |
| AdressTelefon | Normal | 3 | Telefon wie im Anschriftenstamm hinterlegt |
| AdressTelefax | Normal | 3 | Telefax wie im Anschriftenstamm hinterlegt |
| KundNummer | Numerisch | 4 | Kundennummer ( nur bei Kontotyp=2) |
| ZinsGrupNummer | Numerisch | 4 | Zinsgruppe dieses Kunden bzw. diese Sachkontos |
| KundZinsBagSoll | Numerisch | 4 | Bagatellzinsen Soll |
| KundZinsBagHaben | Numerisch | 4 | Bagatellzinsen Haben |
| KundZinsSockel | Numerisch | 4 | Wie zum Zinskonto (kontotyp=1) eingetragen |
| VertGrNummer | Numerisch | 4 | VertretterGruppenNummer |
| VertGrBezeich | Normal | 3 | Bezeichnung der Vertretergruppe |
| Ziabr_Belegnummer | Normal | 3 | Belegnummer dieses Beleges. !!Nur wenn Abrechnung bereits gebucht!! |
| Ziabr_BelegnummerSOLL | Normal | 3 | Belegnummer der Abrechnung für Sollzinsen, wenn Soll und Haben nicht verrechnet wird. |
| Ziabr_BelegnummerHaben | Normal | 3 | Belegnummer der Abrechnung für Habenzinsen, wenn Soll und Haben nicht verrechnet wird. |
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
| | | | | |

<p class="just-emphasize">314 Positionsteil Zinsabrechnung</p>

| Bezeichnung | Typ | Nr. | Bedeutung |
| --- | --- | --- | --- |
| FiBuV_id | Numerisch | 4 | Interne Nummer des Beleges |
| FiBuV_PosZaehler | Numerisch | 4 | Interne Position im Beleg |
| GegenKonto | Numerisch | 4 | Nummer des Gegenkontos wenn nur ein Gegenkonto Vorhanden |
| GegenKontoBezeich | Normal | 3 | Bezeichnung des Gegenkontos ansonsten "diverse" |
| FiBuV_Klasse | Numerisch | 4 | Interne Klasse des Beleges |
| FiBuV_KlBezeich | Normal | 3 | Bezeichnung der Klasse |
| FiBuV_KlKurzBez | Normal | 3 | Kurzbezeichnung ( ZA, ER, AR, AG,....) |
| KostStelNummer | Numerisch | 4 | Nummer der Kostenstelle |
| KostStelBezeich | Normal | 3 | Bezeichnung der Kostenstelle |
| KSTRNummer | Numerisch | 4 | Nummer des Kostenträgers |
| KSTRBezeich | Normal | 3 | Bezeichnung des Kostenträgers |
| ZahlBedNummer | Numerisch | 4 | Zahlungsbedingung |
| SteuerKlasse | Numerisch | 4 | |
| SteuerGrNummer | Numerisch | 4 | |
| SteuerSchluessel | Numerisch | 4 | |
| SteuerAbDatum | Normal | 5 | |
| KontoNummerSko | Numerisch | 4 | Kontonummer des Skontokontos ( bei Rechnungen) |
| FiBuVP_Betrag | Numerisch | 4 | Belegbetrag |
| BelegSH | Normal | 3 | Sollhabenkennzeichen als 'S' oder 'H' |
| FiBuVP_SollHaben | | | S.o. |
| SollBetrag | Numerisch | 4 | Betrag Soll |
| SollKst | Numerisch | 4 | Kostenstelle Soll |
| HabenBetrag | Numerisch | 4 | Betrag Haben |
| HabenKst | Numerisch | 4 | Kostenstelle Haben |
| FiBuVP_BuchTyp | Numerisch | 4 | Interner Buchtyp |
| SteuerKlNetKennz | Numerisch | 4 | 1=Netto 0 = Brutto |
| FiBuVPW_Betrag | Numerisch | 4 | Währungsbetrag falls in Fremdwährung |
| FiBuVP_Steusatz | Numerisch | 4 | Steuersatz |
| FiBuVP_SteuForm | Numerisch | 4 | Steuerform (z.Zt immer 1 ) |
| FiBuVP_SteuWert | Numerisch | 4 | Anteil Steuer |
| FiBuVPW_Steuwert | Numerisch | 4 | Anteil Steuer in Fremdwährung falls eingetragen |
| FiBuVP_StManKennz | Numerisch | 4 | Manuel geändert =1 sonst 0 |
| FiBuVP_SkfBetrag | Numerisch | 4 | Skontierfähiger Betrag |
| FiBuVP_SkoBetrag | Numerisch | 4 | Skonto |
| FiBuVPW_SkoBetrag | Numerisch | 4 | Skonto in Fremdwährung falls eingetragen |
| FiBuVP_FordBetrag | Numerisch | 4 | Forderungsanteil |
| FiBuVP_FordBetSH | Normal | 3 | SollHaben als 'S' bzw. 'H' |
| SollFBetrag | Numerisch | 4 | Sollbetrag Forderungen |
| HabenFBetrag | Numerisch | 4 | Habenbetrag Forderungen |
| FiBuVP_VerbBetrag | Numerisch | 4 | Verbindlichkeiten |
| FiBuVP_VerbBetrSH | Normal | 3 | Sollhaben als 'S' oder 'H' |
| SollVBetrag | Numerisch | 4 | Sollbetrag Verbindlichkeiten |
| HabenVBetrag | Numerisch | 4 | Habenbetrag Verbindlichkeiten |
| FiBuVP_ValDatum | Normal | 5 | Valutadatum ( Fälligkeitsdatum) |
| FiBuVP_SkoSatz | Numerisch | 4 | Skontosatz |
| FiBuVP_Text | Normal | 3 | Belegtext oder "Übertrag" oder "Abschluss" oder "Neuer Zinssatz" |
| FiBuVP_TextFolge | Numerisch | 4 | Folgetexte=1 sonst 0 ( Ausdruck über Typ 605 s.u.) |
| FiBuVP_AuszKennz | Numerisch | 4 | Auszifferungskennzeichen diese Beleges |
| FiBuVP_Auszdatum | Normal | 5 | Datum der Auszifferung |
| FiBuVP_ZinsStat | Numerisch | 4 | Zinsstatus ( 2 = bearbeitet 4 = gebucht ) |
| FiBuVP_KontKennz | Numerisch | 4 | Ist als Kontoblatt gedruckt >0 |
| FiBuVP_Op_Kennz | Numerisch | 4 | ist dieser Posten noch unbezahlt = 1 |
| FiBuV_Datum | Normal | 5 | Belegdatum |
| NumKreisNummer | Numerisch | 4 | |
| FiBuV_NumNummer | Numerisch | 4 | Numerischer Anteil der Belegnummer |
| FiBuV_Nummer | Normal | 3 | Belegnummer aus der FiBu |
| FiBuV_FremdNr | Normal | 3 | Nummer des Fremdbeleges (ER) falls eingetragen. |
| FilialNummerfil | Numerisch | 4 | Nummer der Filiale |
| FilialNummerZen | Numerisch | 4 | Nummer der Zentrale |
| JahrNummer | Numerisch | 4 | Periodenzugehörigkeit dieses Beleges |
| PeriNummer | Numerisch | 4 | S.o. |
| WaehrNummer | Numerisch | 4 | Standartwährung =0 ansonsten siehe Währungsstamm |
| BedieneridNeu | Numerisch | 4 | Id des Erfassers |
| BedieneridBuch | Numerisch | 4 | Wer hat gebucht |
| BedieneridKorr | Numerisch | 4 | Wer hat zuletzt korrigiert |
| FiBuV_WaehKurs | Numerisch | 4 | Währungskurs ( bei Standartwährung immer 1 ) |
| FiBuV_ErfDatum | Normal | 5 | Erfassungsdatum des Beleges |
| FiBuV_Buchstat | Numerisch | 4 | Buchungsstatus 0=ungebucht / 3=gebucht / 4 = fehlerhaft |
| FiBuV_HerkTyp | Numerisch | 4 | 10-19 = Ware sonst Fibu |
| FiBuV_EfUpFehlt | Numerisch | 4 | Ungebuchte aus der Wahre sind 1 sonst 0 |
| ZinsAbrSaldo | Numerisch | 4 | Saldo am Zinsdatum |
| ZinsAbrSaldoSH | Normal | 3 | SollHabenkennzeichen Saldo am Zinsdatum |
| ZinsAbrTage | Numerisch | 4 | wieviel Tage |
| ZinsAbrSatz | Numerisch | 4 | welcher Zinssatz |
| ZinsAbrPosBetrag | Numerisch | 4 | Welcher Betrag |
| ZinsAbrPosBetragSH | Normal | 3 | Welches Sollhabenkennzeichen |
| ZinsAbrValdatum | Normal | 5 | Berechnungsdatum der Zinsposition |

<p class="just-emphasize">605 Textzeile</p>

| Bezeichnung | Typ | Nr. | Bedeutung |
| --- | --- | --- | --- |
| FiBuVP_Text | Normal | 3 | Text wie in den Folgezeilen hinterlegt |

<p class="just-emphasize">315 Zinsabrechnung Betreffzeile</p>

Mit diesem Formularbereich kann für den Mailversand eine Betreffzeile definiert werden. Es stehen hier die Positionen zur Verfügung, die mit F3 ausgewählt werden können. Beim normalen Druck erscheint diese Zeile nicht.
