# Formulare für Kontoblätter (Typ 220) einrichten.

<!-- source: https://amic.de/hilfe/formularefrkontoblttertyp220ei.htm -->

Hauptmenü > Administration > Formulare / Abläufe > Formulare

Direktsprung **[FRM]**

Es existieren zu diesem Typ folgende Formularbereiche:

- 182 Steuersummendruck Kopf für den Einkauf
- 183 Steuersummendetails für den Einkauf
- 184 Steuersummen Fuß für den Einkauf
- 185 Steuersummendruck Kopf für den Verkauf
- 186 Steuersummendetails für den Verkauf
- 187 Steuersummen Fuß für den Verkauf
- 195 Kontrakt-Restmengen-Hinweis Kontenblatt Zeilentyp
- 196 Kontrakt-Hinweis-Überschrift Kontenblatt Zeilentyp
- 197 Warenbewegungs-Hinweiszeile Zeilentyp
- 198 Warenbewegungs-Summenzeile Zeilentyp
- 199 Warenbewegungs-Überschrift Zeilentyp
- 600 Kopf Buchungsjournal/Kontoblatt Formkopf
- 601 Kopf Buchungsjournal/Kontoblatt(Folgekopf) Folgekopf
- 602 Hauptzeile Buchungsjournal/Kontoblatt Zeilentyp
- 603 Gegenzeile Buchungsjournal Zeilentyp
- 605 Textzeile Buchungsjournal/Kontoblatt Zeilentyp
- 606 Bjournal/Kontoblatt Belegartensummen Zeilentyp
- 607 Buchungsjournal Ford/Verb.-Summen Zeilentyp
- 608 Bjournal/Kontoblatt Steuersummen Zeilentyp
- 609 Fuß Buchungsjournal/Kontoblatt Abschluss
- 610 Fuß Bjournal/Koblatt Zwischenabschluss Fuß
- 612 BJ Summenzeile Einzelbewegungen Zeilentyp
- 616 BJ Belegsummen Einleitung Zeilentyp
- 617 BJ Fordersummen Einleitung Zeilentyp
- 618 BJ Steuersummen Einleitung Zeilentyp
- 619 Artikel-Folgetextzeile Kontenblatt Zeilentyp
- 620 Finanzbewegung-Summenzeile Zeilentyp
- 621 Finanzbewegung-Überschrift Zeilentyp
- 622 Finanzbewegung EK-Warebeleg
- 623 Finanzbewegung VK-Warebeleg

Variablen in Kopf und Fußzeile

Folgende Variablen sind in allen Teilen (Kopf, Folgekopf, Fuß und Abschluss) verfügbar. Formularbereiche, die nicht separat mit aufgeführt werden, enthalten nur Festtext oder diese Felder!

| ID | Bezeichnung | Typ | Nummer | Bedeutung |
| --- | --- | --- | --- | --- |
| ID_KKD_KONTOBLZAEHLER | KontoBlZaehler | Numerisch | 4 | Nummer des aktuellen Kontoblattes |
| ID_KKD_KONTOBLDATUM | KontoBlDatum | Datum | 5 | Erstelldatum des KontoBlattes |
| ID_KKD_KONTOBLBISDATUM | KontoBlBisDatum | Datum | 5 | Abgrenzungsdatum |
| ID_KKD_KONTOBLSTATUS | KontoBlStatus | Numerisch | 4 | Status des Kontoblattes |
| ID_KKD_KONTOBLBEZEICH | KontoBlBezeich | Text | 3 | Bezeichnung, die beim Erstellen des Kontoblattes erfasst wurde |
| ID_KKD_KONTOBEZEICH | KontoBezeich | Text | 3 | Die Kontobezeichnung |
| ID_KKD_KONTOTYP | KontoTyp | Numerisch | 4 | 1=SachKonto; 2=Personenkonto |
| ID_KKD_ADRESSIDHAUPTADR | AdressIdHauptAdr Block | Numerisch | 4 | Identifikation der Adress des Kunden. Nur für Personenkonten Adresse S.o. |
| ID_KKD_KUNDNUMMER | KundNummer | Numerisch | 4 | Kundennummer |
| ID_KKD_KUNDENNUMMER | KundenNummer | S.o. | | Kundennummer |
| ID_KKD_KUNDGEGENNUMMER | Kundegegennummer | Text | 3 | Gegennummer des Kunden |
| ID_KKD_VERTGRNUMMER | VertGrNummer | Numerisch | 4 | Nummer der Vertretergruppe dieses Kunden |
| ID_KKD_VERTGRBEZEICH | VertGrbezeich | Text | 3 | Bezeichnung der Vertretergruppe, aber nur im Bereich 600 bzw. 609 |
| ID_KKD_ZAHLARTID_DEBIT | Zahlartid_Debit | Numerisch | 4 | Nummer der debitdorischen Zahlart |
| ID_KKD_ZAHLARTBEZ_DEBIT | Zahlartbez_Debit | Text | 3 | Bezeichnung der debitdorischen Zahlart |
| ID_KKD_ZAHLARTID_KREDIT | Zahlartid_Kredit | Numerisch | 4 | Nummer der kreditdorischen Zahlart |
| ID_KKD_ZAHLARTBEZ_KREDIT | Zahlartbez_Kredit | Text | 3 | Bezeichnung der kreditdorischen Zahlart |
| ID_KKD_MITGLNUMMER | Mitgellnummer | Numerisch | 4 | Mitgliedsnummer, aber nur im Bereich 600 bzw. 609 |
| ID_KKD_PERIBEZEICH | PeriBezeich | Text | 3 | Bezeichnung der „Abi-Periode“ |
| ID_KKD_PERIBEZEICHBIS | PeriBezeichBis | Text | 3 | Periodenbezeichnung bis |
| ID_KKD_PERIBEZEICHVON | PeriBezeichVon | Text | 3 | Bezeichnung der „Biss-Periode“ |
| ID_KKD_MANDANT | Mandant | Text | 3 | Bezeichnung des aktuellen Mandanten |
| ID_KKD_KONTONUMMER | KontoNummer | Numerisch | 4 | Kontonummer |
| ID_KKD_BEDIENERID | Bedienarid | Numerisch | 4 | Id des Bedieners, der die Kontoblätter erstellt hat. |
| ID_KKD_USER | USER | Text | 3 | Textkürzel des Bedieners, der die Kontoblätter druckt. |
| ID_KKD_DRUCKDATUM | Druckdatum | Datum | 5 | Tagesdatum |
| ID_KKD_JAHRNUMMER | Jahrnummer | Numerisch | 4 | Jahrnummer |
| ID_KKD_PERINUMMER | Perinummer | Numerisch | 4 | Bis-Periode |
| ID_KKD_RECHNUMMER | RechNummer | Numerisch | 4 | Rechnungsnummer, jedoch nur bei KOKORE |
| ID_KKD_KONBLZAEABDATUMALT | KonBlZaeAbDatumAlt | Datum | 5 | Abgrenzungsdatum des Vorgänger KontoBlattes |
| ID_KKD_KONBLZAEDATUMALT | KonBlZaeDatumAlt | Datum | 5 | Abgrenzungsdatum des Vorgänger KontoBlattes |
| ID_KKD_HABENALT | HabenAlt | Numerisch | 4 | Habenbetrag des Vorgänger-Kontenblattes |
| ID_KKD_SOLLALT | SollAlt | Numerisch | 4 | Sollbetrag des Vorgänger-Kontenblattes |
| ID_KKD_HABENNEU | HabenNeu | Numerisch | 4 | Neuer Sollbetrag |
| ID_KKD_SOLLNEU | SollNeu | Numerisch | 4 | Neuer Habenbetrag |
| ID_KKD_SOLLSUMME | SollSumme | Numerisch | 4 | Sollneu- Sollalt |
| ID_KKD_HABENSUMME | HabenSumme | Numerisch | 4 | Habenneu- Habenalt |
| ID_KKD_SOLLVJ | SollVj | Numerisch | 4 | Soll des letzten Kontoblattes des Vorjahres |
| ID_KKD_HABENVJ | HabenVj | Numerisch | 4 | Haben des letzten Kontoblattes des Vorjahres |
| ID_KKD_SALDO | Saldo | Numerisch | 4 | Sollsumme - Habensumme |
| ID_KKD_SH | SH | Text | 3 | Sollhaben Kennteichen des Saldos (S oder H ) |
| ID_KKD_SALDOALT | SaldoAlt | Numerisch | 4 | Sollalt – Habenalt |
| ID_KKD_SHALT | SHAlt | Text | 3 | Sollhabenkennzeichen des SaldoAlts ( S oder H ) |
| ID_KKD_SALDONEU | SaldoNeu | Numerisch | 4 | SollNeu-habenNeu |
| ID_KKD_SHNEU | SHNeu | Text | 3 | Sollhabenkennzeichen des SaldoNeus ( S oder H ) |
| ID_KKD_SALDOVJ | SaldoVJ | Numerisch | 4 | SollVj – HabenVj |
| ID_KKD_SHVJ | SHVJ | Text | 3 | Sollhabenkennzeichen des SaldoVJs ( S oder H ) |
| ID_KKD_SALDOEB | SaldoEB | Numerisch | 4 | Saldo der Eröffnungsperiode ( nur in Bereich 600 und 609) |
| ID_KKD_SALDOEBSH | SaldoEBSH | Text | 3 | Sollhaben der Eröffnungsperiode ( nur in Bereich 600 und 609) |
| ID_KKD_ZAHLBEDINGUNGZ1 | ZahlBedingungZ1 | Text | 3 | Zahlungsbedingungstext |
| ID_KKD_ZAHLBEDINGUNGZ2 | ZahlBedingungZ2 | Text | 3 | Zahlungsbedingung 2 |
| ID_KKD_ZAHLBEDINGUNGZ3 | ZahlBedingungZ3 | Text | 3 | Zahlungsbedingung 3 |
| ID_KKD_ZAHLBEDINGUNGZ4 | ZahlBedingungZ4 | Text | 3 | Zahlungsbedingung 4 |
| ID_KKD_MAXSEITE | MaxSeite Numerisch | Text | 3 | Nur versorgt, wenn das Formular nur die Bereiche 600,602,603 und 609 hat. |
| ID_KKD_ADRESSE | Adresse | Text | 3 | Identifikation der Adresse des Kunden. Nur für Personenkonten Adresse |
| ID_KKD_BISPERIODE | bisPeriode | Text | 3 | Bis Periode |
| ID_KKD_KONBLZAEBISDATUM | KonBlZaeBisDatum | Datum | 5 | Kontoblattzähler bis Datum |
| ID_KKD_KONBLZAEABDATUM | KonBlZaeAbDatum | Datum | 5 | Kontoblattzähler ab Datum |
| ID_KKD_KONTOBLABDATUM | KonBlZaeAbDatum | Datum | 5 | Kontoblatt ab Datum |
| ID_KKD_UMSATZEKVJ | UmsatzEKVJ | Numerisch | 3 | Umsatz des Einkaufs im Vorjahr |
| ID_KKD_UMSATZVKVJ | UmsatzVKVJ | Numerisch | 3 | Umsatz des Verkaufes im Vorjahr |
| ID_KKD_ZAHLUNGSEINGANGVJ | ZahlungsEingangVJ | Numerisch | 3 | Zahlungseingang des Vorjahres |
| ID_KKD_ZAHLUNGSAUSGANGVJ | ZahlungsAusgangVJ | Numerisch | 3 | Zahlungsausgang des Vorjahres |
| ID_KKD_SKONTOGEWAEHRTVJ | SkontoGewaehrtVJ | Numerisch | 3 | Gewährter Skonto des Vorjahres |
| ID_KKD_SKONTOGEZOGENVJ | SkontoGezogenVJ | Numerisch | 3 | Gezogener Skonto des Vorjahres |
| ID_KKD_FAELLIGERSALDOSH | FaelligerSaldoSH | Numerisch | 3 | Fälliger Saldo |
| ID_KKD_SHVJPER | SHVjPer | Numerisch | 3 | Soll Haben des Vorjahres in Abhängigkeit der Periode |
| ID_KKD_KONBLZAEBISDATUMALT | KonBlZaeBisDatumAlt | Text | 4 | Kontoblattzähler bis Datum alt |
| ID_KKD_SALDOALTHABEN | SaldoAltHaben | Numerisch | 3 | Alter Saldo Haben |
| ID_KKD_SALDOALTSOLL | SaldoAltSoll | Numerisch | 3 | Alter Saldo Soll |
| ID_KKD_SALDONEUHABEN | SaldoNeuHaben | Numerisch | 3 | Neues haben im Saldo |
| ID_KKD_SALDONEUSOLL | SaldoNeuSoll | Numerisch | 3 | Neues Soll im Saldo |
| ID_KKD_FAELLIGERSALDO | FaelligerSaldo | Numerisch | 3 | Fälliger Saldo |
| ID_KKD_WECHSEL_OBLIGO | WECHSEL_OBLIGO | Numerisch | 3 | Wechsel Obligo |
| ID_KKD_WERBETEXT | WerbeText | Text | 4 | Werbetext |
| ID_KKD_EINKOMMENSTEUERNUMMER | EinkommenSteuerNummer | Text | 4 | Einkommensteuernummer des Kunden /Lieferanten / Kontokorrenten |
| ID_KKD_USTKENNZEICHEN | USTKENNZEICHEN | Text | 4 | Umsatzsteuerkennzeichen |

<p class="just-emphasize">Formular Bereich Kopfversorgung(616, 617, 618)</p>

| ID | Bezeichnung | Typ | Nummer | Bedeutung |
| --- | --- | --- | --- | --- |
| ID_KKD_FAELLIGERSALDO | FaelligerSaldo | Numerisch | 4 | Fälliger Saldo |
| ID_KKD_HABENVJPER | HabenVjPer | Numerisch | 4 | Haben des Vorjahres in Abhängigkeit einer Periode |
| ID_KKD_SALDOVJPER | SaldoVjPer | Numerisch | 4 | Saldo des Vorjahres in Abhängigkeit der Periode |
| ID_KKD_SOLLVJPER | | Numerisch | 4 | Soll des Vorjahres in Abhängigkeit der Periode |
| ID_KKD_VONPERIODE | vonPeriode | Text | 3 | Von Periode |

<p class="just-emphasize">Formular Bereiche 602/603/197/619</p>

(Beleginformationen Finanzbuchhaltung)

| ID | Bezeichnung | Typ | Nummer | Bedeutung |
| --- | --- | --- | --- | --- |
| ID_KKD_FIBUV_ID | FiBuV_id | Numerisch | 4 | Interne Identifikation des Beleges |
| ID_KKD_FIBUV_POSZAEHLER | FiBuV_PosZaehler | Numerisch | 4 | Interne Identifikation der Belegzeile |
| ID_KKD_FAELLIGERSALDO | FaelligerSaldo | Numerisch | 4 | Fälliger Saldo |
| ID_KKD_GEGENKONTO | GegenKonto | Numerisch | 4 | Gegenkonto, falls nur ein Gegenkonto sonst leer. |
| ID_KKD_GEGENKONTOBEZEICH | GegenKontoBezeichnung | Text | 3 | Bezeichnung Gegenkonto falls nur ein Gegenkonto sonst ?Diverse? |
| ID_KKD_FIBUV_KLASSE | FiBuV_Klasse | Numerisch | 4 | Interner Belegklasse |
| ID_KKD_FIBUV_KLBEZEICH | FiBuV_klbezeich | Text | 3 | Bezeichnung der Klasse |
| ID_KKD_FIBUV_KLKURZBEZ | FiBuV_KlKurzbez | Text | 3 | Bezeichnung der Klasse ( z.B. ER für Eingangsrechnung ) |
| ID_KKD_ZAHLBEDNUMMER | ZahlBednummer | Numerisch | 4 | Nummer der Zahlungsbedingung dieses Beleges |
| ID_KKD_KOSTSTELNUMMER | KostStellnummer | Numerisch | 4 | Kostenstelle |
| ID_KKD_KOSTSTELBEZEICH | KostStelBezeich | Text | 3 | Bezeichnung der Kostenstelle |
| ID_KKD_STEUERKLASSE | SteuerKlasse | Text | 3 | Die Steuerklasse. Diese gehört zur Klasse, Gruppe, Schlüssel, Abdatum-kombination, aus der der verwendete Steuersatz bestimmt wird |
| ID_KKD_STEUERKLBEZEICH | Steuerklbezeich | Text | 3 | Bezeichnung der Steuerklasse |
| ID_KKD_STEUERGRNUMMER | SteuerGrNummer | Text | 3 | Die Steuergruppe. Diese gehört zur Klasse, Gruppe, Schlüssel, Abdatum-kombination, aus der der verwendete Steuersatz bestimmt wird |
| ID_KKD_STEUERSCHLUESSEL | Steuerschluessel | Text | 3 | Der Steuerschlüssel. Diese gehört zur Klasse, Gruppe, Schlüssel, Abdatum-kombination, aus der der verwendete Steuersatz bestimmt wird |
| ID_KKD_STEUERABDATUM | Steuerabdatum | Datum | 5 | Diese gehört zur Klasse, Gruppe, Schlüssel, Abdatum-kombination, aus der der verwendete Steuersatz bestimmt wird |
| ID_KKD_KONTONUMMERSKO | Kontonummersko | Text | 3 | Die Skonto Kontonummer |
| ID_KKD_FIBUVP_BETRAG | FiBuVP_Betrag | Numerisch | 4 | Nettobetrag |
| ID_KKD_FIBUVP_SOLLHABEN | FiBuVP_SollHaben | Numerisch | 4 | Soll Haben in der Fibuvorgangsposition |
| ID_KKD_SOLLFBETRAG | SollBetrag | Numerisch | 4 | Bei Personenkonten Forderungen im Soll sonst leer |
| ID_KKD_SOLLKST | SollKst | Text | 3 | Kostenstelle |
| ID_KKD_HABENBETRAG | Habenbetrag | Numerisch | 4 | Der Haben Betrag |
| ID_KKD_HABENKST | Habenkst | Numerisch | 4 | Kostenstelle des Habens |
| ID_KKD_FIBUVP_NETKENNZ | FiBuVP_NetKennz | Text | 3 | Nettokennzeichen in der Fibuvorgangsposition |
| ID_KKD_FIBUVPW_BETRAG | FiBuVPW_Betrag | Numerisch | 4 | Nettobetrag in der Fibuvorgangsposition |
| ID_KKD_FIBUVP_STEUSATZ | FiBuVP_SteuSatz | Text | 3 | Steuersatz in der Fibuvorgangsposition |
| ID_KKD_FIBUVP_STEUFORM | FiBuVP_SteuForm | Text | 3 | Steuerform in der Fibuvorgangsposition |
| ID_KKD_FIBUVP_STEUWERT | FiBuVP_SteuWert | Numerisch | 4 | Steuer |
| ID_KKD_FIBUVPW_STEUWERT | FiBuVPW_SteuWert | Numerisch | 4 | Steuerwert in der Fibuvorgangsposition |
| ID_KKD_FIBUVP_STMANKENN | FiBuVP_StManKenn | Text | 3 | Wurde die Steuer manuell angepasst |
| ID_KKD_FIBUVP_SKOBETRAG | FiBuVP_SkoBetrag | Numerisch | 4 | Skontobetrag in der Fibuvorgangsposition |
| ID_KKD_FIBUVP_SKOSATZ | FiBuVP_SkoSatz | Numerisch | 4 | Skontosatz in der Fibuvorgangsposition |
| ID_KKD_FIBUVPW_SKOBETRAG | FiBuVPW_SkoBetrag | Numerisch | 4 | Skontobetrag in Fremdwährung |
| ID_KKD_FIBUVP_FORDBETR | FiBuVP_FordBetr | Numerisch | 4 | Forderungsbetrag in der Fibuvorgangsposition |
| ID_KKD_FIBUVP_FORDBETSH | FiBuVP_FordBetSH | Numerisch | 4 | Sollhabenkennzeichen des Forderungsbetrags |
| ID_KKD_FIBUVP_VERBBETR | FiBuVP_VerbBetr | Numerisch | 4 | Verbindlichkeitsbetrag |
| ID_KKD_FIBUVP_VERBBETSH | FiBuVP_VerbBetSH | Numerisch | 4 | Sollhabenkennzeichen des Verbindlichkeitsbetrags |
| ID_KKD_SOLLFBETRAG | SollFBetrag | Numerisch | 4 | Bei Personenkonten Forderungen im Soll sonst leer |
| ID_KKD_FIBUVP_VALDATUM | FiBuVP_Valdatum | Datum | 5 | Valuta Datum in der Fibuvorgangsposition |
| ID_KKD_FIBUVP_TEXT | FiBuVP_Text | Text | 3 | Belegtext |
| ID_KKD_FIBUVP_TEXTFOLGE | FiBuVP_TextFolge | Text | 3 | Textfolge in der Fibuvorgangsposition |
| ID_KKD_FIBUVP_AUSZKENNZ | FiBuVP_AuszKennz | Text | 3 | Auszifferungskennzeichen in der Fibuvorgangsposition |
| ID_KKD_FIBUVP_AUSZDATUM | FiBuVP_AuszDatum | Datum | 5 | Auszifferungsdatum in der Fibuvorgangsposition |
| ID_KKD_FIBUVP_ZINSSTAT | FiBuVP_ZinsStat | Text | 3 | Zinsstatus: 0=Nicht zu verzinsen |
| ID_KKD_FIBUVP_KONTKENNZ | FiBuVP_KontKennz | Text | 3 | Kontokennzeichen in der Fibuvorgangsposition |
| ID_KKD_FIBUVP_OP_KENNZ | FiBuVP_OP_Kennz | Text | 3 | OP Kennzeichen in der Fibuvorgangsposition |
| ID_KKD_FIBUVP_BWA_KENNZ | FiBuVP_BWA_Kennz | Text | 3 | \*\*\*\*\*\*\*\* WIRD NICHT VERWENDET |
| ID_KKD_FIBUVP_JOUSTATUS | FiBuVP_Joustatus | Text | 3 | Journalstatus in der Fibuvorgangsposition |
| ID_KKD_FIBUV_DATUM | FiBuV_Datum | Datum | 5 | Vorgangsdatum in der Fibuvorgangsposition |
| ID_KKD_NUMKREISNUMMER | NumKreisNummer | Text | 3 | Nummernkreisnummer |
| ID_KKD_FIBUV_NUMNUMMER | FiBuV_NumNummer | Text | 3 | Numerische Nummer des Vorgangs in der Fibu |
| ID_KKD_FIBUV_NUMMER | FiBuV_Nummer | Text | 3 | Nummer des Vorgangs in der Fibu |
| ID_KKD_FIBUV_FREMDNR | FiBuV_FremdNr | Text | 3 | Fremdnummer des Vorgangs in der Fibu |
| ID_KKD_JAHRNUMMER | JahrNummer | Text | 3 | Jahrnummer |
| ID_KKD_PERINUMMER | Perinummer | Text | 3 | Periodennummer |
| ID_KKD_WAEHRNUMMER | Waehrnummer | Text | 3 | Währungsnummer |
| ID_KKD_WAEHRBEZEICH | WaehrBezeich | Text | 3 | Währungsbezeichnung |
| ID_KKD_BEDIENERIDNEU | BedienerIdNeu | Text | 3 | Bedienerid des Anlegers |
| ID_KKD_BEDIENERIDKORR | BedienerIdKorr | Text | 3 | Bedienerid des Belegkorrigierer |
| ID_KKD_BEDIENERIDBUCH | BedienerIdBuch | Text | 3 | Bedienerid des Buchers |
| ID_KKD_ZAHLARTID | ZahlArtId | Text | 3 | Zahlartid |
| ID_KKD_FIBUVPW_KURS | FiBuVPW_Kurs | Numerisch | 4 | Währungskurs in der Fibuvorgangsposition |
| ID_KKD_FIBUV_ERFDATUM | FiBuV_erfdatum | Datum | 5 | Erfassungsdatum |
| ID_KKD_FIBUV_BUCHSTAT | FiBuV_Buchstat | Text | 3 | Buchungsstatus |
| ID_KKD_FIBUV_HERKTYP | FiBuV_HerkTyp | Text | 3 | Herkunftstyp |
| ID_KKD_FIBUJOURNUMMER | FiBuJourNummer | Text | 3 | Journalnummer |
| ID_KKD_FIBUJOURZEILE | FiBuJourZeile | Text | 3 | Journalzeile |
| ID_KKD_SOLLVBETRAG | SollVBetrag | Numerisch | 4 | Bei Personenkonten Verbindlichkeiten im Soll sonst leer |
| ID_KKD_STEUERKLNETKENNZ | SteuerKlNetKennz | Text | 3 | Wurde der Betrag Netto oder Brutto erfasst? Netto = 1 |
| ID_KKD_FIBUVP_BUCHTYP | FiBuVP_BuchTyp | Text | 3 | Buchtyp der Finanzbuchhaltung |
| ID_KKD_KONTONUMMERSKO | KONTONUMMERSKO | Text | 3 | Die Skonto Kontonummer |
| ID_KKD_FIBUVP_SKFBETRAG | FiBuVP_SkfBetrag | Numerisch | 4 | \*\*\*\*\*\*\*\* WIRD NICHT VERWENDET |
| ID_KKD_FILIALNUMMERFIL | FilialNummerFil | Text | 3 | Fililalnummer der Filiale |
| ID_KKD_FILIALNUMMERZEN | FilialNummerZen | Text | 3 | Fililalnummer der Zentrale |
| ID_KKD_FIBUV_ERFUPFEHLT | FIBUV_ERFUPFEHLT | Text | 3 | \*\*\*\*\*\*\*\* WIRD NICHT VERWENDET |
| ID_KKD_STEUERSATZFREMD | SteuerSatzFremd | Numerisch | 4 | Steuersatzfremd |
| ID_KKD_HABENFBETRAG | HabenFBetrag | Numerisch | 4 | Bei Personenkonten Forderung im Haben sonst leer |
| ID_KKD_HABENVBETRAG | HabenVBetrag | Numerisch | 4 | Bei Personenkonten Verbindlichkeiten im Haben sonst leer |
| ID_KKD_RPREIS | RPREIS | Numerisch | 4 | Preis der Warenbewegung |
| ID_KKD_STEUERSATZGR | STEUERSATZ | Numerisch | 4 | Der Steuersatz. |
| ID_KKD_GESAMTSKONTO | GESAMTSKONTO | Numerisch | 4 | Das Gesamtkonto |
| ID_KKD_WABEWID | WABEWID | Text | 3 | Warenbewegungsid |
| ID_KKD_HABENVJPER | HabenVjPer | Numerisch | 4 | Haben des Vorjahres in Abhängigkeit einer Periode |
| ID_KKD_SALDOVJPER | SaldoVjPer | Numerisch | 4 | Saldo des Vorjahres in Abhängigkeit der Periode |
| ID_KKD_SOLLVJPER | SollVjPer | Numerisch | 4 | Soll des Vorjahres in Abhängigkeit der Periode |
| ID_KKD_VONPERIODE | vonPeriode | Numerisch | 4 | Von Periode |

<p class="just-emphasize">Formularbereich 605 . Folgetexte</p>

| ID | Bezeichnung | Typ | Nummer | Bedeutung |
| --- | --- | --- | --- | --- |
| ID_KKD_FIBUVP_TEXT | FiBuVP_Text | Text | 3 | Folgetext wie in der FiBu erfasst. |

<p class="just-emphasize">Formularbereich 197.</p>

Siehe auch oben (Beleginformationen Finanzbuchhaltung) 

| ID | Bezeichnung | Typ | Nummer | Bedeutung |
| --- | --- | --- | --- | --- |
| ID_KKD_ARTIKELNUMMER | Artikelnummer | Text | 3 | Artikelnummer |
| ID_KKD_LAGERNUMMER | Lagernummer | Text | 3 | Die Lagernummer |
| ID_KKD_KONTONRERLOES | KontoNrErloes | Text | 3 | Der Kontenerlöss |
| ID_KKD_BEZEICHNUNG | Bezeichnung | Text | 3 | Bezeichnung |
| ID_KKD_ARTTEXT | Arttext | Text | 3 | Artikeltext |
| ID_KKD_PREIS | Preis | Numerisch | 4 | Der Preis |
| ID_KKD_PREISNACKT | Preisnackt | Numerisch | 4 | Der Preis ohne zu oder Abschläge |
| ID_KKD_PREISEINHEIT | Preiseinheit | Numerisch | 4 | Die Preiseinheit des Preises |
| ID_KKD_EZHT | EZHT | Numerisch | 4 | Preiseinheitszeichen |
| ID_KKD_ME_TEXTPREIS | ME_TextPreis | Text | 3 | Der Mengeneinheitsnummer des Preises als Text |
| ID_KKD_MENGE | Menge | Text | 3 | Die Menge |
| ID_KKD_GEBINDE | Gebinde | Text | 3 | Gebinde |
| ID_KKD_ME_TEXT | ME_Text | Text | 3 | Der Mengeneinheitsnummer als Text |
| ID_KKD_NETTO | Netto | Numerisch | 4 | Das Netto |
| ID_KKD_NETTOSOLL | NettoSoll | Numerisch | 4 | Das Nettosoll |
| ID_KKD_NETTOHABEN | NettoHaben | Numerisch | 4 | Das Nettohaben |
| ID_KKD_STEUER | Steuer | Numerisch | 4 | Die Steuer |
| ID_KKD_STEUERSOLL | SteuerSoll | Numerisch | 4 | Steuerbetrag wenn Soll sonst leer |
| ID_KKD_STEUERHABEN | SteuerHaben | Numerisch | 4 | Steuerbetrag wenn Haben sonst leer |
| ID_KKD_BRUTTO | Brutto | Numerisch | 4 | Brutto |
| ID_KKD_BRUTTOSOLL | BruttoSoll | Numerisch | 4 | Brutto Soll |
| ID_KKD_BRUTTOHABEN | BruttoHaben | Numerisch | 4 | Brutto Haben |
| ID_KKD_LIEFDAT | Liefdat | Datum | 5 | Das Lieferdatum |
| ID_KKD_LIEFNUMMER | Liefnummer | Text | 3 | Die Liefernummer |
| ID_KKD_RECHDAT | Rechdat | Datum | 5 | Das Rechnungsdatum |
| ID_KKD_RECHNUNGSNUMMER | Rechnungsnummer | Text | 3 | Die Rechnungsnummer |
| ID_KKD_WIEGENUMMER | Wiegenummer | Text | 3 | Wiegenummer |
| ID_KKD_STEUERSCHLUESSEL | Steuerschluessel | Text | 3 | Der Steuerschlüssel. |
| ID_KKD_STEUERSATZ | Steuersatz | Text | 3 | Der Steuersatz |
| ID_KKD_LAGERPLATZ | Lagerplatz | Text | 3 | Der Lagerplatz |
| ID_KKD_PARTIENUMMER | Partienummer | Text | 3 | Die Partienummer |
| ID_KKD_PARTIEBEZEICH | Partiebezeich | Text | 3 | Die Partiebezeichnung |
| ID_KKD_PARTIEMATCH | Partiematch | Text | 3 | Der Matchcode einer Partie |
| ID_KKD_VALUTADATUM | Valutadatum | Datum | 5 | Valuta Datum |
| ID_KKD_STEUERSCHLUESSELGR | STEUERSCHLUESSEL | Text | 3 | Der Steuerschlüssel. Diese gehört zur Klasse, Gruppe, Schlüssel, Abdatum-kombination, aus der der verwendete Steuersatz bestimmt wird |

<p class="just-emphasize">Formularbereich 619 . Folgetexte aus der Ware</p>

| ID | Bezeichnung | Typ | Nummer | Bedeutung |
| --- | --- | --- | --- | --- |
| ID_KKD_ARTTEXT | ARTText | Text | 3 | Folgetext wie in der Ware erfasst. |

<p class="just-emphasize">Formularbereich 620, 198. Bereichsummenzeilen</p>

| ID | Bezeichnung | Typ | Nummer | Bedeutung |
| --- | --- | --- | --- | --- |
| ID_KKD_TYPTEXT | Typtext | Text | 3 | "Typ Text: ""Finanzbewegungen"", ""Wareneinkauf"", ""Warenverkauf""" |
| ID_KKD_NETTO | Netto | Numerisch | 4 | Das Netto |
| ID_KKD_STEUER | Steuer | Numerisch | 4 | Die Steuer |
| ID_KKD_BRUTTO | Brutto | Numerisch | 4 | Das Brutto |
| ID_KKD_SOLL | Soll | Numerisch | 4 | Soll |
| ID_KKD_HABEN | Haben | Numerisch | 4 | Das Haben |
| ID_KKD_BRUTTOSOLL | BruttoSoll | Numerisch | 4 | Brutto Soll |
| ID_KKD_BRUTTOHABEN | Bruttohaben | Numerisch | 4 | Brutto Haben |
| ID_KKD_MENGE | Menge | Numerisch | 4 | Die Menge |

<p class="just-emphasize">Formularbereich 612. Gesamtsumme</p>

| ID | Bezeichnung | Typ | Nummer | Bedeutung |
| --- | --- | --- | --- | --- |
| ID_KKD_SOLLSUMME | Sollsumme | Numerisch | 4 | Sollsumme |

<p class="just-emphasize">Formularbereich 606. Summen nach Belegart</p>

(Überschrift hierzu ist Bereich 616 )

| **ID** | **Bezeichnung** | **Typ** | **Nummer** | **Bedeutung** |
| --- | --- | --- | --- | --- |
| ID_KKD_SOLLBETRAG | Sollbetrag | Numerisch | 4 | Betrag, wenn soll sonst leer |
| ID_KKD_HABENBETRAG | HabenBetrag | Numerisch | 4 | Betrag, wenn haben sonst leer |
| ID_KKD_FIBUVP_BETRAG | FiBuVP_Betrag | Numerisch | 4 | Nettobetrag |
| ID_KKD_FIBUVP_SOLLHABEN | FiBuVP_SollHaben | Numerisch | 4 | Fibuvorgangposition Sollhaben |
| ID_KKD_FIBUV_KLASSE | FiBuV_Klasse | Text | 3 | Vorgangsklasse der Fibu |
| ID_KKD_FIBUV_KLBEZEICH | FiBuV_KlBezeich | Text | 3 | Vorgangsklassenbezeichnung der Fibu |
| ID_KKD_FIBUV_KLKURZBEZ | FiBuV_KLKurzBez | Text | 3 | Kurzbezeichnung der Vorgangsklasse in der Fibu |

<p class="just-emphasize">Formularbereich 607 Buchungsjournal Ford/Verb.-Summen Zeilentyp</p>

(Überschrift hierzu ist Bereich 617)

| ID | Bezeichnung | Typ | Nummer | Bedeutung |
| --- | --- | --- | --- | --- |
| ID_KKD_SOLLBETRAG | Sollbetrag<br> | Numerisch | 4 | Betrag, wenn soll sonst leer |
| ID_KKD_FIBUVP_SOLLHABEN | FiBuVP_SollHaben<br> | Numerisch | 4 | Soll Haben in der Fibuvorgangsposition |
| ID_KKD_HABENBETRAG<br> | Habenbetrag<br> | Numerisch | 4 | Der Haben Betrag |
| ID_KKD_FIBUVP_BETRAG<br> | FiBuVP_Betrag<br> | Numerisch | 4 | Nettobetrag |
| ID_KKD_KONTONUMMER<br> | KontoNummer<br> | Text | 3 | Kontonummer |
| ID_KKD_KONTOBEZEICH<br> | KontoBezeich<br> | Text | 3 | Die Kontobezeichnung |

<p class="just-emphasize">Formularbereich 608. Summen nach Forderungsgruppen</p>

(Überschrift hierzu ist Bereich 618)

| **ID** | **Bezeichnung** | **Typ** | **Nummer** | **Bedeutung** |
| --- | --- | --- | --- | --- |
| ID_KKD_SOLLBETRAG | Sollbetrag | Numerisch | 4 | Betrag, wenn soll sonst leer |
| ID_KKD_HABENBETRAG | HabenBetrag | Numerisch | 4 | Der Haben Betrag |
| ID_KKD_FIBUVP_BETRAG | FiBuVP_Betrag | Numerisch | 4 | Nettobetrag |
| ID_KKD_FIBUVP_STEUWERT | FiBuVP_SteuWert | Numerisch | 4 | Steuer |
| ID_KKD_FIBUVP_SOLLHABEN | FiBuVP_SollHaben | Numerisch | 4 | Soll Haben in der Fibuvorgangsposition |
| ID_KKD_SOLLSTEUER | Sollsteuer | Numerisch | 4 | Steuer wenn Betrag im Soll, sonst leer |
| ID_KKD_HABENSTEUER | Habensteuer | Numerisch | 4 | Die Haben Steuer |
| ID_KKD_STEUERKLBEZEICH | SteuerKlBezeich | Text | 3 | Bezeichnung der Steuerklasse |
| ID_KKD_STEUERGRNUMMER | SteuerGrNummer | Text | 3 | Die Steuergruppe. Diese gehört zur Klasse, Gruppe, Schlüssel, Abdatum-kombination, aus der der verwendete Steuersatz bestimmt wird |
| ID_KKD_STEUERSCHLUESSEL | SteuerSchluessel | Text | 3 | Der Steuerschlüssel. Diese gehört zur Klasse,Gruppe, Schlüssel, Abdatum-kombination, aus der der verwendete Steuersatz bestimmt wird |
| ID_KKD_STEUERSATZ | Steuersatz | Text | 3 | Der Steuersatz |
| ID_KKD_KONTOBEZEICH | KontoBezeich | Text | 3 | Die Kontobezeichnung |
| ID_KKD_STEUERWERTERWERB | SteuerWertErwerb | Numerisch | 4 | Steuerbetrag bei innergemeinschaftlichem Erwerb |
| ID_KKD_STEUERSATZERWERB | SteuerWertErwerb | Numerisch | 4 | Steuersatz bei Innergemeinschaftlichem Erwerb |

<p class="just-emphasize">Formularbereich 195. Kontraktinfos</p>

(Überschrift hierzu ist Bereich 196)

| **ID** | **Bezeichnung** | **Typ** | **Nummer** | **Bedeutung** |
| --- | --- | --- | --- | --- |
| ID_KKD_KTRABDATUM | KtrAbDatum | Datum | 5 | Das Kontrakt Abdatum |
| ID_KKD_KTRBEZEICH | KtrBezeich | Text | 3 | Die Kontraktbezeichnung |
| ID_KKD_KTRBISDATUM | KtrBisDatum | Datum | 5 | Das Kontrakt Bisdatum |
| ID_KKD_KTRBISDATUMFIX | KtrBisDatumFix | Datum | 5 | Das Kontrakt Fixdatum |
| ID_KKD_KTRGESAMTMENGE | KtrGesamtMenge | Numerisch | 4 | Die Gesamtmenge des Kontraktes |
| ID_KKD_KTRGESAMTWERT | KtrGesamtWert | Numerisch | 4 | Der Gesamtwert des Kontraktes |
| ID_KKD_KTRRESTWERT | KtrRestWert | Numerisch | 4 | Der Kontraktrestwert |
| ID_KKD_KTRGRUPID | KtrGrupId | Text | 3 | Die Kontraktgruppenid |
| ID_KKD_KTRKLASSE | KtrKlasse | Text | 3 | Die Klasse des Kontraktes |
| ID_KKD_KTRMEKURZ | KtrMeKurz | Text | 3 | Die Kurzbezeichnung der Mengeneinheit des Kontraktes |
| ID_KKD_KTRNUMMER | KtrNummer | Text | 3 | Die Kontraktnummer |
| ID_KKD_KTRREFERENZNR | KtrRefernzNr | Text | 3 | Die Kontraktrerferenznummer |
| ID_KKD_KTRVORAUSANZAHL | KtrVorausAnzahl | Text | 3 | Die Kontraktvorausanzahl |
| ID_KKD_KTRDATUM | KtrDatum | Datum | 5 | Das Kontraktdatum |
| ID_KKD_KONTRAKTID | KontraktId | Text | 3 | Die Kontraktid |
| ID_KKD_KTRMENGENEINHEIT | KtrMengeneinheit | Text | 3 | Die Kontraktmengeneinheit |
| ID_KKD_KTRRESTMENGE | KTRRESTMENGE | Numerisch | 4 | Die Restmenge des Kontraktes |
| ID_KKD_KTRRESTMENGETOTAL | KTRRESTMENGETOTAL | Numerisch | 4 | Die Totale Restmenge des Kontraktes |
| ID_KKD_KTRRESTWERTTOTAL | KTRRESTWERTTOTAL | Numerisch | 4 | Der Totale Restwert des Kontraktes |

<p class="just-emphasize">Formularbereich zu 600 Kopf und Fuss</p>

| **ID** | **Bezeichnung** | **Typ** | **Nummer** | **Bedeutung** |
| --- | --- | --- | --- | --- |
| ID_KKD_WECHSELOBLIGO | WechselObligo | Numerisch | 4 | Aktueller Obligo zum am Stichtag - "Bis Belegdatum" auf der Erstellungsmaske - . Es wird die Summe der Wechsel genommen, deren Ausstelldatum vor dem Stichtag und deren Verfalldatum hinter diesem Stichtag liegt. |
| ID_KKD_WECHSELOBLIGOSH | WechselObligoSH | Numerisch | 4 | Sollhabenkennzeichen des Wechselobligo's |
| ID_KKD_WECHSELSTICHTAG | WechselStichtag | Datum | 5 | S.o. |
| ID_KKD_UMSATZEKALT | UmsatzEKAlt | Numerisch | 4 | Umsatz Einkauf wie auf dem vorherigen Kontoblatt als Summe(UmsatzEKNeu) ausgewiesen |
| ID_KKD_UMSATZEK | UmsatzEK | Numerisch | 4 | Umsatz Einkauf während dieser Kontoblattperiode |
| ID_KKD_UMSATZEKNEU | UmsatzEKNeu | Numerisch | 4 | Umsatz Einkauf Summe UmsatzEKAlt + UmsatzEK |
| ID_KKD_UMSATZVKALT | UmsatzVKAlt | Numerisch | 4 | Umsatz Verkauf wie auf dem vorherigen Kontoblatt als Summe(UmsatzVKNeu) ausgewiesen |
| ID_KKD_UMSATZVK | UmsatzVK | Numerisch | 4 | Umsatz Verkauf wärmend dieser Kontoblattperiode |
| ID_KKD_UMSATZVKNEU | UmsatzVKNeu | Numerisch | 4 | Umsatz Verkauf Summe UmsatzVKAlt + UmsatzVK |
| ID_KKD_ZAHLUNGSEINGANGALT | ZahlungsEingangAlt | Numerisch | 4 | Zahlungseingänge wie auf dem vorherigen Kontoblatt als Summe(ZahlungsEingangNeu) ausgewiesen |
| ID_KKD_ZAHLUNGSEINGANG | ZahlungsEingang | Numerisch | 4 | Zahlungseingang wärmend dieser Kontoblattperiode |
| ID_KKD_ZAHLUNGSEINGANGNEU | ZahlungsEingangNeu | Numerisch | 4 | Zahlungseingänge Summe ZahlungsEingangAlt + ZahlungsEingang |
| ID_KKD_ZAHLUNGSAUSGANGALT | ZahlungsAusgangAlt | Numerisch | 4 | Zahlungseingänge wie auf dem vorherigen Kontoblatt als Summe(ZahlungsAusgangNeu) ausgewiesen |
| ID_KKD_ZAHLUNGSAUSGANG | ZahlungsAusgang | Numerisch | 4 | Zahlungseingang wärmend dieser Kontoblattperiode |
| ID_KKD_ZAHLUNGSAUSGANGNEU | ZahlungsAusgangNeu | Numerisch | 4 | Zahlungseingänge Summe ZahlungsAusgangAlt + ZahlungsAusgang |
| ID_KKD_SKONTOGEWAEHRTALT | SkontoGewaehrtAlt | Numerisch | 4 | Gewährte Skonti wie auf dem vorherigen Kontoblatt als Summe(SkontoGewaehrtNeu) ausgewiesen |
| ID_KKD_SKONTOGEWAEHRT | SkontoGewaehrt | Numerisch | 4 | Gewährte Skonti wärmend dieser Kontoblattperiode |
| ID_KKD_SKONTOGEWAEHRTNEU | SkontoGewaehrtNeu | Numerisch | 4 | Gewährte Skonti Summe SkontoGewaehrtAlt + SkontoGewaehrt |
| ID_KKD_SKONTOGEZOGENALT | SkontoGezogenAlt | Numerisch | 4 | Gezogene Skonti wie auf dem vorherigen Kontoblatt als Summe(SkontoGezogenNeu) ausgewiesen |
| ID_KKD_SKONTOGEZOGEN | SkontoGezogen | Numerisch | 4 | Gezogene Skonti wärmend dieser Kontoblattperiode |
| ID_KKD_SKONTOGEZOGENNEU | SkontoGezogenNeu | Numerisch | 4 | Gezogene Skonti Summe SkontoGezogenAlt + SkontoGezogen |

<p class="just-emphasize">Steuersummen für Einkauf und Verkauf Druckbereiche 183/186</p>

| ID | Bezeichnung | Typ | Nummer | Bedeutung |
| --- | --- | --- | --- | --- |
| ID_KKD_ST_SATZ | ST_SATZ | Numerisch | 4 | Steuersatz |
| ID_KKD_ST_NETTO | ST_NETTO | Numerisch | 4 | Steuer Netto |
| ID_KKD_ST_STEUER | ST_STEUER | Numerisch | 4 | Steuer |
| ID_KKD_ST_SKONTO_F | ST_SKONTO_F | Numerisch | 4 | Skontofähig |
| ID_KKD_ST_SKONTO | ST_SKONTO | Numerisch | 4 | Steuerskonto |

<p class="just-emphasize">Formularbereich 622 /623 (Finanzbewegung EK/VK Warebeleg)</p>

| ID | Bezeichnung | Typ | Nummer | Bedeutung |
| --- | --- | --- | --- | --- |
| ID_KKD_ROHWAREANLIEFDATUM | RohWareAnliefDatum | Datum | 5 | Das Anlieferungsdatum eines Rohwarenbeleges |
| ID_KKD_ROHWAREANLIEFNUMMER | RohWareAnliefNummer | Numerisch | 4 | Die Anlieferungsnummer eines Rohwarenbeleges |
| ID_KKD_ROHWAREBELEGDATUM | RohWareBelegDatum | Datum | 5 | Das Belegdatum eines Rohwarenbeleges |
| ID_KKD_ROHWAREBELEGNUMMER | RohWareBelegNummer | Numerisch | 4 | Die Rohwarenbelegnummer |
| ID_KKD_ROHWAREINFO | RohWareInfo | Numerisch | 4 | Rohware Info |
| ID_KKD_ROHWARESORTE | RohWareSorte | Numerisch | 4 | Rohwarensorte |
| ID_KKD_ROHWAREWIEGENUMMER | RohWareWiegeNummer | Numerisch | 4 | Die Wiegenummer eines Rohwarenbeleges |

<p class="just-emphasize">Bereichsüberschrift 199,621</p>

| ID | Bezeichnung | Typ | Nummer | Bedeutung |
| --- | --- | --- | --- | --- |
| ID_KKD_TYPTEXTVAR | TYPTEXTVAR | Text | 3 | Wie TypText, nur sind diese Texte unter OPTIONEN hinterlegt |
