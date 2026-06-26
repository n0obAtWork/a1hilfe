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

<div class="table-wrapper">
  <table>
    <tbody>
      <tr>
        <td></td>
        <td>
          <p><b>Bezeichnung&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;</b></p>
        </td>
        <td>
          <p><b>Typ</b></p>
        </td>
        <td>
          <p><b>Nr.</b></p>
        </td>
        <td>
          <p><b>Bedeutung</b></p>
        </td>
      </tr>
      <tr>
        <td colspan="2">
          <p>Zinslistnummer</p>
        </td>
        <td>
          <p>Numerisch</p>
        </td>
        <td>
          <p>4</p>
        </td>
        <td>
          <p>Nummer der Zinsliste</p>
        </td>
      </tr>
      <tr>
        <td colspan="2">
          <p>Kontonummer</p>
        </td>
        <td>
          <p>Numerisch</p>
        </td>
        <td>
          <p>4</p>
        </td>
        <td>
          <p>Kontonummer des aktiven Kunden</p>
        </td>
      </tr>
      <tr>
        <td colspan="2">
          <p>ZinsAbrDatum</p>
        </td>
        <td>
          <p>Normal</p>
        </td>
        <td>
          <p>5</p>
        </td>
        <td>
          <p>Erstellungsdatum des Zinsabrechnung</p>
        </td>
      </tr>
      <tr>
        <td colspan="2">
          <p>ZinsAbrVonDatum</p>
        </td>
        <td>
          <p>Normal</p>
        </td>
        <td>
          <p>5</p>
        </td>
        <td>
          <p>Bereich von dieser Zinsabrechnung</p>
        </td>
      </tr>
      <tr>
        <td colspan="2">
          <p>ZinsAbrBisDatum</p>
        </td>
        <td>
          <p>Normal</p>
        </td>
        <td>
          <p>5</p>
        </td>
        <td>
          <p>Bereich bis dieser Zinsabrechnung</p>
        </td>
      </tr>
      <tr>
        <td colspan="2">
          <p>ZinsAbrDruKennz</p>
        </td>
        <td>
          <p>Numerisch</p>
        </td>
        <td>
          <p>4</p>
        </td>
        <td>
          <p>Wenn schon vorher einmal gedruckt dann 1 sonst 0</p>
        </td>
      </tr>
      <tr>
        <td colspan="2">
          <p>ZinsAbrSollZins</p>
        </td>
        <td>
          <p>Numerisch</p>
        </td>
        <td>
          <p>4</p>
        </td>
        <td>
          <p>Sollzinsen</p>
        </td>
      </tr>
      <tr>
        <td colspan="2">
          <p>ZinsAbrHabenZins</p>
        </td>
        <td>
          <p>Numerisch</p>
        </td>
        <td>
          <p>4</p>
        </td>
        <td>
          <p>HabenZinsen</p>
        </td>
      </tr>
      <tr>
        <td colspan="2">
          <p>ZinsAbrZinsSaldo</p>
        </td>
        <td>
          <p>Numerisch</p>
        </td>
        <td>
          <p>4</p>
        </td>
        <td>
          <p>Saldo Soll - Haben</p>
        </td>
      </tr>
      <tr>
        <td colspan="2">
          <p>ZinsAbrZinsSaldoSH</p>
        </td>
        <td>
          <p>Normal</p>
        </td>
        <td>
          <p>3</p>
        </td>
        <td>
          <p>Sollhaben des Saldos</p>
        </td>
      </tr>
      <tr>
        <td colspan="2">
          <p>ZinsAbrSollZinsOrig</p>
        </td>
        <td>
          <p>Numerisch</p>
        </td>
        <td>
          <p>4</p>
        </td>
        <td>
          <p>Sollzinsen</p>
        </td>
      </tr>
      <tr>
        <td colspan="2">
          <p>ZinsAbrHabenZinsOrig</p>
        </td>
        <td>
          <p>Numerisch</p>
        </td>
        <td>
          <p>4</p>
        </td>
        <td>
          <p>HabenZinsen</p>
        </td>
      </tr>
      <tr>
        <td colspan="2">
          <p>ZinsAbrZinsSaldoOrig</p>
        </td>
        <td>
          <p>Numerisch</p>
        </td>
        <td>
          <p>4</p>
        </td>
        <td>
          <p>Saldo Soll - Haben</p>
        </td>
      </tr>
      <tr>
        <td colspan="2">
          <p>ZinsAbrZinsSaldoOrigSH</p>
        </td>
        <td>
          <p>Normal</p>
        </td>
        <td>
          <p>3</p>
        </td>
        <td>
          <p>Sollhaben des Saldos</p>
        </td>
      </tr>
      <tr>
        <td colspan="2">
          <p>Druckdatum</p>
        </td>
        <td>
          <p>Normal</p>
        </td>
        <td>
          <p>5</p>
        </td>
        <td>
          <p>Tagesdatum</p>
        </td>
      </tr>
      <tr>
        <td colspan="2">
          <p>USER</p>
        </td>
        <td>
          <p>Normal</p>
        </td>
        <td>
          <p>3</p>
        </td>
        <td>
          <p>Wer hat das gedruckt?</p>
        </td>
      </tr>
      <tr>
        <td colspan="2">
          <p>MANDANT</p>
        </td>
        <td>
          <p>Normal</p>
        </td>
        <td>
          <p>3</p>
        </td>
        <td>
          <p>Mandantenbezeichnung</p>
        </td>
      </tr>
      <tr>
        <td colspan="2">
          <p>KontoBezeich</p>
        </td>
        <td>
          <p>Normal</p>
        </td>
        <td>
          <p>3</p>
        </td>
        <td>
          <p>Bezeichnung des Kontos aus dem Kontostamm</p>
        </td>
      </tr>
      <tr>
        <td colspan="2">
          <p>KontoTyp</p>
        </td>
        <td>
          <p>Numerisch</p>
        </td>
        <td>
          <p>4</p>
        </td>
        <td>
          <p>Typ des Kontos im Kontostamm (1=Sachkonto 2=PersonenKonto9</p>
        </td>
      </tr>
      <tr>
        <td colspan="2">
          <p>Adresse</p>
        </td>
        <td>
          <p>Block</p>
        </td>
        <td>
          <p>6</p>
        </td>
        <td>
          <p>Kundenadresse bzw. wie in Sachkontozins hinterlegt</p>
        </td>
      </tr>
      <tr>
        <td colspan="2">
          <p>AdressAnrede</p>
        </td>
        <td>
          <p>Normal</p>
        </td>
        <td>
          <p>3</p>
        </td>
        <td>
          <p>Anrede wie im Anschriftenstamm hinterlegt</p>
        </td>
      </tr>
      <tr>
        <td colspan="2">
          <p>AdressName</p>
        </td>
        <td>
          <p>Normal</p>
        </td>
        <td>
          <p>3</p>
        </td>
        <td>
          <p>Name wie im Anschriftenstamm hinterlegt</p>
        </td>
      </tr>
      <tr>
        <td colspan="2">
          <p>AdressKurzName</p>
        </td>
        <td>
          <p>Normal</p>
        </td>
        <td>
          <p>3</p>
        </td>
        <td>
          <p>Kurzname wie im Anschriftenstamm hinterlegt</p>
        </td>
      </tr>
      <tr>
        <td colspan="2">
          <p>AdressBezeich</p>
        </td>
        <td>
          <p>Normal</p>
        </td>
        <td>
          <p>3</p>
        </td>
        <td>
          <p>Bezeichnung wie im Anschriftenstamm hinterlegt</p>
        </td>
      </tr>
      <tr>
        <td colspan="2">
          <p>AdressTelefon</p>
        </td>
        <td>
          <p>Normal</p>
        </td>
        <td>
          <p>3</p>
        </td>
        <td>
          <p>Telefon wie im Anschriftenstamm hinterlegt</p>
        </td>
      </tr>
      <tr>
        <td colspan="2">
          <p>AdressTelefax</p>
        </td>
        <td>
          <p>Normal</p>
        </td>
        <td>
          <p>3</p>
        </td>
        <td>
          <p>Telefax wie im Anschriftenstamm hinterlegt</p>
        </td>
      </tr>
      <tr>
        <td colspan="2">
          <p>KundNummer</p>
        </td>
        <td>
          <p>Numerisch</p>
        </td>
        <td>
          <p>4</p>
        </td>
        <td>
          <p>Kundennummer ( nur bei Kontotyp=2)</p>
        </td>
      </tr>
      <tr>
        <td colspan="2">
          <p>ZinsGrupNummer</p>
        </td>
        <td>
          <p>Numerisch</p>
        </td>
        <td>
          <p>4</p>
        </td>
        <td>
          <p>Zinsgruppe dieses Kunden bzw. diese Sachkontos</p>
        </td>
      </tr>
      <tr>
        <td colspan="2">
          <p>KundZinsBagSoll</p>
        </td>
        <td>
          <p>Numerisch</p>
        </td>
        <td>
          <p>4</p>
        </td>
        <td>
          <p>Bagatellzinsen Soll</p>
        </td>
      </tr>
      <tr>
        <td colspan="2">
          <p>KundZinsBagHaben</p>
        </td>
        <td>
          <p>Numerisch</p>
        </td>
        <td>
          <p>4</p>
        </td>
        <td>
          <p>Bagatellzinsen Haben</p>
        </td>
      </tr>
      <tr>
        <td colspan="2">
          <p>KundZinsSockel</p>
        </td>
        <td>
          <p>Numerisch</p>
        </td>
        <td>
          <p>4</p>
        </td>
        <td>
          <p>Wie zum Zinskonto (kontotyp=1) eingetragen</p>
        </td>
      </tr>
      <tr>
        <td colspan="2">
          <p>VertGrNummer</p>
        </td>
        <td>
          <p>Numerisch</p>
        </td>
        <td>
          <p>4</p>
        </td>
        <td>
          <p>VertretterGruppenNummer</p>
        </td>
      </tr>
      <tr>
        <td colspan="2">
          <p>VertGrBezeich</p>
        </td>
        <td>
          <p>Normal</p>
        </td>
        <td>
          <p>3</p>
        </td>
        <td>
          <p>Bezeichnung der Vertretergruppe</p>
        </td>
      </tr>
      <tr>
        <td colspan="2">
          <p>Ziabr_Belegnummer</p>
        </td>
        <td>
          <p>Normal</p>
        </td>
        <td>
          <p>3</p>
        </td>
        <td>
          <p>Belegnummer dieses Beleges. !!Nur wenn Abrechnung bereits gebucht!!</p>
        </td>
      </tr>
      <tr>
        <td colspan="2">
          <p>Ziabr_BelegnummerSOLL</p>
        </td>
        <td>
          <p>Normal</p>
        </td>
        <td>
          <p>3</p>
        </td>
        <td>
          <p>Belegnummer der Abrechnung für Sollzinsen, wenn Soll und Haben nicht verrechnet wird.</p>
        </td>
      </tr>
      <tr>
        <td colspan="2">
          <p>Ziabr_BelegnummerHaben</p>
        </td>
        <td>
          <p>Normal</p>
        </td>
        <td>
          <p>3</p>
        </td>
        <td>
          <p>Belegnummer der Abrechnung für Habenzinsen, wenn Soll und Haben nicht verrechnet wird.</p>
        </td>
      </tr>
      <tr>
        <td colspan="2">
          <p>MandSteuerNummer</p>
        </td>
        <td>
          <p>Normal</p>
        </td>
        <td>
          <p>3</p>
        </td>
        <td>
          <p>Steuernummer aus dem Mandantenstamm</p>
        </td>
      </tr>
      <tr>
        <td colspan="2">
          <p>MandUStStatKennz</p>
        </td>
        <td>
          <p>Normal</p>
        </td>
        <td>
          <p>3</p>
        </td>
        <td>
          <p>USt-IdNr. aus dem Mandantenstamm</p>
        </td>
      </tr>
      <tr>
        <td colspan="2">
          <p>MandBezeich</p>
        </td>
        <td>
          <p>Normal</p>
        </td>
        <td>
          <p>3</p>
        </td>
        <td>
          <p>Name des Mandanten</p>
        </td>
      </tr>
      <tr>
        <td colspan="2">
          <p>MandAdressKurzName</p>
        </td>
        <td>
          <p>Normal</p>
        </td>
        <td>
          <p>3</p>
        </td>
        <td>
          <p>In der Anschrift des Mandanten hinterlegte Kurzbezeichnung</p>
        </td>
      </tr>
      <tr>
        <td colspan="2">
          <p>MandAdressAnrede</p>
        </td>
        <td>
          <p>Normal</p>
        </td>
        <td>
          <p>3</p>
        </td>
        <td>
          <p>In der Anschrift des Mandanten hinterlegte Anrede</p>
        </td>
      </tr>
      <tr>
        <td colspan="2">
          <p>MandAdressVorName</p>
        </td>
        <td>
          <p>Normal</p>
        </td>
        <td>
          <p>3</p>
        </td>
        <td>
          <p>In der Anschrift des Mandanten hinterlegter Vorname</p>
        </td>
      </tr>
      <tr>
        <td colspan="2">
          <p>MandAdressName</p>
        </td>
        <td>
          <p>Normal</p>
        </td>
        <td>
          <p>3</p>
        </td>
        <td>
          <p>In der Anschrift des Mandanten hinterlegter Name</p>
        </td>
      </tr>
      <tr>
        <td colspan="2">
          <p>MandAdressOrt</p>
        </td>
        <td>
          <p>Normal</p>
        </td>
        <td>
          <p>3</p>
        </td>
        <td>
          <p>In der Anschrift des Mandanten hinterlegter Ort</p>
        </td>
      </tr>
      <tr>
        <td colspan="2">
          <p>MandAdressPLZ1</p>
        </td>
        <td>
          <p>Normal</p>
        </td>
        <td>
          <p>3</p>
        </td>
        <td>
          <p>In der Anschrift des Mandanten hinterlegte Postleitzahl</p>
        </td>
      </tr>
      <tr>
        <td colspan="2">
          <p>MandAdressStrasse</p>
        </td>
        <td>
          <p>Normal</p>
        </td>
        <td>
          <p>3</p>
        </td>
        <td>
          <p>In der Anschrift des Mandanten hinterlegte Straße</p>
        </td>
      </tr>
      <tr>
        <td colspan="2">
          <p>MandAdressTelefon</p>
        </td>
        <td>
          <p>Normal</p>
        </td>
        <td>
          <p>3</p>
        </td>
        <td>
          <p>In der Anschrift des Mandanten hinterlegte Telefonnummer</p>
        </td>
      </tr>
      <tr>
        <td colspan="2">
          <p>MandAdressTelefax</p>
        </td>
        <td>
          <p>Normal</p>
        </td>
        <td>
          <p>3</p>
        </td>
        <td>
          <p>In der Anschrift des Mandanten hinterlegte Faxnummer</p>
        </td>
      </tr>
    </tbody>
    <tbody>
      <tr>
        <td></td>
        <td></td>
        <td></td>
        <td></td>
        <td></td>
      </tr>
    </tbody>
  </table>
</div>

**<p class="just-emphasize">314 Positionsteil Zinsabrechnung</p>**

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

**<p class="just-emphasize">605 Textzeile</p>**

| Bezeichnung | Typ | Nr. | Bedeutung |
| --- | --- | --- | --- |
| FiBuVP_Text | Normal | 3 | Text wie in den Folgezeilen hinterlegt |

#### 315 Zinsabrechnung Betreffzeile

Mit diesem Formularbereich kann für den Mailversand eine Betreffzeile definiert werden. Es stehen hier die Positionen zur Verfügung, die mit F3 ausgewählt werden können. Beim normalen Druck erscheint diese Zeile nicht.
