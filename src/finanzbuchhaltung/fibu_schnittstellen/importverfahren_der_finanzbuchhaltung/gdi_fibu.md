# GDI-Fibu

<!-- source: https://amic.de/hilfe/gdifibu.htm -->

Hauptmenü > Abschlussarbeiten > DATEV / Import / Export > Import > Funktion **F9** Import Starten > Funktion **F4** ***Importdatei lesen***

Direktsprung **[FIIM]**

Beim Import aus der GDI-Fibu handelt es sich um echte Finanzbuchhaltungsbelege, also um Eingangsrechnungen, Ausgangsrechnung sowie Zahlungsbelege. Der Satzaufbau der zu importierenden Datei hat wie folgt auszusehen:

<p class="just-emphasize">Satzaufbau</p>

Es werden nur die mit [Bu] beginnenden Datensätze ausgewertet.

<div class="table-wrapper">
  <table>
    <tbody>
      <tr>
        <td></td>
        <td>
          <p><strong>Kennzeichen</strong></p>
        </td>
        <td colspan="2">
          <p><strong>Feldinhalt</strong></p>
        </td>
      </tr>
      <tr>
        <td colspan="2">
          <p>&lt;Art&gt;=</p>
        </td>
        <td>
          <p>Belegart</p>
        </td>
        <td></td>
      </tr>
      <tr>
        <td colspan="2">
          <p>&lt;BNr&gt;=</p>
        </td>
        <td>
          <p>Belegnummer</p>
        </td>
        <td></td>
      </tr>
      <tr>
        <td colspan="2">
          <p>&lt;Dat&gt;=</p>
        </td>
        <td>
          <p>Buchungsdatum</p>
        </td>
        <td></td>
      </tr>
      <tr>
        <td colspan="2">
          <p>&lt;Txt&gt;=</p>
        </td>
        <td>
          <p>Buchungstext</p>
        </td>
        <td></td>
      </tr>
      <tr>
        <td colspan="2">
          <p>&lt;Btr&gt;=</p>
        </td>
        <td>
          <p>Buchungsbetrag (incl. S/H falls vorgegeben)</p>
        </td>
        <td></td>
      </tr>
      <tr>
        <td colspan="2">
          <p>&lt;StS&gt;=</p>
        </td>
        <td>
          <p>Steuerschlüssel (falls ein Steuerbetrag angegeben ist und der Steuerschlüssel nicht beim Sachkonto hinterlegt ist)</p>
        </td>
        <td></td>
      </tr>
      <tr>
        <td colspan="2">
          <p>&lt;StB&gt;=</p>
        </td>
        <td>
          <p>Steuerbetrag (nur bei Sachkontenbuchungen)</p>
        </td>
        <td></td>
      </tr>
      <tr>
        <td colspan="2">
          <p>&lt;ZBd&gt;=</p>
        </td>
        <td>
          <p>Zahlungsbedingung</p>
        </td>
        <td></td>
      </tr>
      <tr>
        <td colspan="2">
          <p>&lt;RBt&gt;=</p>
        </td>
        <td>
          <p>Rechnungsbetrag</p>
        </td>
        <td></td>
      </tr>
      <tr>
        <td colspan="2">
          <p>&lt;BDa&gt;=</p>
        </td>
        <td>
          <p>Belegdatum</p>
        </td>
        <td></td>
      </tr>
      <tr>
        <td colspan="2">
          <p>&lt;BlL&gt;=</p>
        </td>
        <td>
          <p>Belegnummer Lieferant (Nur bei Kreditoren)</p>
        </td>
        <td></td>
      </tr>
      <tr>
        <td colspan="2">
          <p>&lt;ZaV&gt;=</p>
        </td>
        <td>
          <p>Zahlvermerk (L/E/V)</p>
        </td>
        <td></td>
      </tr>
      <tr>
        <td colspan="2">
          <p>&lt;VDa&gt;=</p>
        </td>
        <td>
          <p>Valutadatum (nur Debitoren)</p>
        </td>
        <td></td>
      </tr>
      <tr>
        <td colspan="2">
          <p>&lt;SpV&gt;=</p>
        </td>
        <td>
          <p>Sperrvermerk</p>
        </td>
        <td></td>
      </tr>
      <tr>
        <td colspan="2">
          <p>&lt;ZDa&gt;=</p>
        </td>
        <td>
          <p>Zahldatum (nur bei Kreditoren)</p>
        </td>
        <td></td>
      </tr>
      <tr>
        <td colspan="2">
          <p>&lt;WKz&gt;=</p>
        </td>
        <td>
          <p>Währung Kennzeichen</p>
        </td>
        <td></td>
      </tr>
      <tr>
        <td colspan="2">
          <p>&lt;WBt&gt;=</p>
        </td>
        <td>
          <p>Fremdwährungsbetrag</p>
        </td>
        <td></td>
      </tr>
      <tr>
        <td colspan="2">
          <p>&lt;Kst&gt;=</p>
        </td>
        <td>
          <p>Kostenstelle</p>
        </td>
        <td></td>
      </tr>
      <tr>
        <td colspan="2">
          <p>&lt;Ktr&gt;=</p>
        </td>
        <td>
          <p>Kostenträger</p>
        </td>
        <td></td>
      </tr>
      <tr>
        <td colspan="2">
          <p>&lt;KtO&gt;=</p>
        </td>
        <td>
          <p>Kostenträger Originärbuchung</p>
        </td>
        <td></td>
      </tr>
      <tr>
        <td colspan="2">
          <p>&lt;Skt&gt;=</p>
        </td>
        <td>
          <p>Skonto</p>
        </td>
        <td></td>
      </tr>
      <tr>
        <td colspan="2">
          <p>&lt;ZAr&gt;=</p>
        </td>
        <td>
          <p>Zahlartnummer</p>
        </td>
        <td></td>
      </tr>
      <tr>
        <td colspan="2">
          <p>&lt;ISO&gt;=</p>
        </td>
        <td>
          <p>ISO-Währungscode</p>
        </td>
        <td></td>
      </tr>
      <tr>
        <td colspan="2">
          <p>&lt;SkW&gt;=</p>
        </td>
        <td>
          <p>Skonto in Fremdwährung</p>
        </td>
        <td></td>
      </tr>
      <tr>
        <td colspan="2">
          <p>&lt;SKf&gt;=</p>
        </td>
        <td>
          <p>Skontofähiger Betrag</p>
        </td>
        <td></td>
      </tr>
      <tr>
        <td colspan="2">
          <p>&lt;GKt&gt;=</p>
        </td>
        <td>
          <p>Gegenkonto</p>
        </td>
        <td></td>
      </tr>
    </tbody>
    <tbody>
      <tr>
        <td></td>
        <td></td>
        <td></td>
        <td></td>
      </tr>
    </tbody>
  </table>
</div>

Die Gegenbuchungen werden jeweils mit ‘<strong>&lt;GKt>=’</strong> eingeleitet. Es muss mindestens eine Gegenbuchung angegeben werden. Die Mindestangaben hierbei sind die Kontonummer und der Buchungsbetrag. Das Buchungsdatum wird immer aus der Buchung herangezogen. Die Felder Belegart, Belegnummer und Buchungstext werden aus der Buchung übernommen, falls sie nicht für die Gegenbuchung angegeben wurden. Die Summe aus den Buchungsbeträgen ‘<strong>&lt;Btr>=’</strong> und den Steuerbeträgen ‘<strong>&lt;StB>=’</strong> der Buchung und aller Gegenbuchungen müssen 0.00 DM ergeben.  
Alle in Buchungssätzen angegebenen Konten müssen in der Fibu angelegt sein bzw. vor der Buchung als Stammsatz übergeben worden sein. 

<p class="just-emphasize">Beispieldaten</p>

```text
[GDI-Fibu]:[GDI-FACTUR]:[GDI-Business-Line]
[Version]=GDI-Business-Line - Daten vom 23.03.2003 10:07:16
[VersExe]=2.0.3.4 vom 16.09.2003 15:57:22
[VersBDE]=5.2.0.2 vom 10.05.2001 10:00:00
[Mandant]=MPBS RADIUS DEMO - C:\Agentur\GDI\DEMO\Radius20\
[WAEHRUNG]=2
[ETW]=99,1.955830
[ETW]=1,1.000000
[Pk]=11101<SuN>=ABENHAUS<Nam>=Josef Abenhaus<Bra>=Marmor- u. Natursteine<Str>=Oderstr. 36<Ort>=48145 Mnster / Westf.<LKz>=I<MKz>=1<WKz>=1<NTg>=30<S1T>=8<S1%>= 2.00
[Bu]=11101<Art>=R<BNr>=980086<Dat>=230303<Txt>=Josef Abenhaus<Btr>=58<RBt>=58<BDa>=230303<ZaV>=<NTg>=30<S1T>=8<S1%>= 2.00<WBt>=58<WKz>=1
<GKt>=8400.0<Txt>=Josef Abenhaus<Btr>=-40<StB>=-6.4
<GKt>=8410.0<Txt>=Josef Abenhaus<Btr>=-10<StB>=-1.6
[Pk]=10000<SuN>=BAR<Nam>=BARVERKAUF<LKz>=I<WKz>=1
[Bu]=10000<Art>=R<BNr>=980087<Dat>=230303<Txt>=BARVERKAUF<Btr>=58<RBt>=58<BDa>=230303<ZaV>=<WBt>=58<WKz>=1
<GKt>=8400<Txt>=BARVERKAUF<Btr>=-40<StB>=-6.4
<GKt>=8410<Txt>=BARVERKAUF<Btr>=-10<StB>=-1.6
[Bu]=10000<Art>=Z<BNr>=980087<Dat>=230303<Txt>=BARVERKAUF<Btr>=-58<RBt>=-58<BDa>=230303<WKz>=1<WBt>=-58
<GKt>=1000<Art>=Z<Txt>=BARVERKAUF<Btr>=55
<GKt>=2400<Art>=S<Txt>=BARVERKAUF<Btr>=2.59<StB>=0.41
```
