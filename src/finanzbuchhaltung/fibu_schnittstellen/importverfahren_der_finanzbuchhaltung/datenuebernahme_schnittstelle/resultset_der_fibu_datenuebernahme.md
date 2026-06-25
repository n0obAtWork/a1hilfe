# Resultset der FIBU-Datenübernahme

<!-- source: https://amic.de/hilfe/resultsetderfibudatenbernahme.htm -->

Die Importverfahren FIBU-XML-Import, FIBU-CSV-Import und FIBU-XLSX-Import haben alle das gleiche Resultset. Der Import ist an die Belegerfassung Finanzbuchhaltung angelehnt und die Felder haben somit auch die gleiche Bedeutung. **Fett** dargestellte Felder sind Pflichtfelder.

<div class="table-wrapper">
  <table>
    <tbody>
      <tr>
        <td></td>
        <td></td>
      </tr>
      <tr>
        <td>
          <p>uebertragungskennung und uebertragungsnummer<br><br></p>
        </td>
        <td>
          <p>Beim XML-Import werden diese Felder verwendet, um sicherzustellen, dass Daten nicht mehrfach eingespielt werden. D.h. in jeder Datei muss sich Kennung oder Nummer von den bereits importierten Daten unterscheiden.</p>
          <p>Im FIBU-CSV-Import und FIBU-XLSX-Import enthalten sie die FA_ID bzw. die FA_MNDNR unter der die Datei im Archiv abgelegt wurde.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>ident</p>
        </td>
        <td>
          <p>Zeilen mit gleicher Ident werden versucht zu einem Beleg zusammenzufassen. Neben der Ident dürfen sich auch Fibuv_klasse, Fibuv_datum, Hauptkonto, Hauptkoststel, Hauptkstr, Hauptksobj, (fibuvp_sollhaben ), Jahrnummer und Perinummer nicht unterscheiden. Ansonsten wird ein neuer Beleg erstellt.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>poszaehler</p>
        </td>
        <td>
          <p>Gibt zum einen die Reihenfolge innerhalb eines Beleges an und dient auch im Fehlerfall zur Identifizierung der Zeile</p>
        </td>
      </tr>
      <tr>
        <td>
          <p><b>fibuv_klasse</b></p>
        </td>
        <td>
          <p>Für die Belegklasse sind folgende Werte erlaubt:</p>
          <p>1&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp; Zahlungen<br>2&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp; Ausgangsrechnung<br>3&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp; Ausgangsgutschriften<br>4&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp; Eingangsrechnung<br>5&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp; Eingangsgutschriften<br>6&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp; Sonstige Belege<br><br><b>Hinweis:</b> <i>Fibuv_klasse und Sollhaben sind für AR, AG, ER und EG eng miteinander verknüpft.</i><i></i></p>
          <p><i>Wird bei der Belegklasse 2 (AR), 3 (AG), 4 (ER) oder 5 (EG) das falsche Sollhabenkennzeichen angegeben, so wird die zum Sollhabenkennzeichen passende Belegklasse verwendet. Wird also z.B: bei der Belegklasse 4 (ER) das Sollhabenkennzeichen 2 (Haben) angegeben, geht das System davon aus, dass die Belegklasse die 5 (EG) sein sollte.</i></p>
        </td>
      </tr>
      <tr>
        <td>
          <p>fibuv_herktyp</p>
        </td>
        <td>
          <p>Kennzeichen, wie der Beleg entstanden ist. Der Standard-Wert ist 30, wenn nichts angegeben wurde. Eigene Herkunftstypen sollten ab 100 gewählt werden.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>numkreisnummer</p>
        </td>
        <td>
          <p>Soll ein spezieller Nummernkreis verwendet werden, so kann er hier angegeben werden. Ist kein Nummernkreis angegeben, so wird der für die Belegklasse (fibuv_klasse) unter <strong>NKF</strong> hinterlegte Nummernkreis verwendet.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>fibuv_numNummer</p>
        </td>
        <td>
          <p>Wird in diesem Feld ein Wert zurückgeliefert, so wird dieser Wert zur Bildung der Belegnummer verwendet. Ansonsten kommt dieser Wert aus dem Nummernkreis.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>fibuv_nummer</p>
        </td>
        <td>
          <p>Textliche Darstellung der Belegnummer. Ist diese nicht angegeben, so wird sie anhand des Nummernkreises bzw. der fibuv_numnummer bestimmt.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>fibuv_fremdnr</p>
        </td>
        <td>
          <p>Die Referenznummer.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>fibuv_paginiernr</p>
        </td>
        <td>
          <p>Paginiernummer.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p><b>fibuv_datum</b></p>
        </td>
        <td>
          <p>Belegdatum in der Form TT-MM-JJJJ.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>jahrnummer</p>
        </td>
        <td rowspan="2">
          <p>Wird die Jahrnummer bzw. Perinummer nicht angegeben, so werden diese Felder anhand des Belegdatums (Fibuv_datum) bestimmt. Die Periode muss existieren und offen sein.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>perinummer</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>fibuv_eingangsdatum</p>
        </td>
        <td>
          <p>Eingangsdatum in der Form TT-MM-JJJJ. Dieses Feld ist optional und kann für die Belegklassen „Eingangsrechnungen“, „Eingangsgutschriften“ und „Sonstige Belege“ übertragen werden. Ob eine Prüfung des Eingangsdatums gegen das Belegdatum vorgenommen wird, lässt sich mit dem Steuerparameter 1130 „Eingangsdatum muss hinter dem Belegdatum liegen“ einstellen. Vorbelegung ist <b>Ja</b>.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p><b>hauptkonto</b></p>
        </td>
        <td>
          <p>Das Hauptkonto muss in A.eins existieren. Wie in der Belegerfassung muss für Zahlungen (fibuv_klasse=1) das Hauptkonto ein Sachkonto sein, für Sonstige Belege(fibuv_klasse=6) kann es ein Sach- oder Personenkonto sein. Für alle anderen Belegklassen sind nur Personenkonten zugelassen.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>hauptkoststel</p>
        </td>
        <td>
          <p>Wenn das Hauptkonto ein GuV-Konto ist und es sich um einen Sonstigen Beleg(fibuv_klasse=6) handelt, kann diesem eine Kostenstelle zugeordnet werden. Ist keine Kostenstelle zugeordnet, dann wird die Kostenstelle so zugewiesen, wie sie im Sachkonto hinterlegt ist. Ist im Sachkonto eine Sperre für Kostenstellen eingetragen, so wird der Eintrag hier ignoriert.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>hauptkstr</p>
        </td>
        <td>
          <p>Kostenträger. Siehe Hauptkoststel</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>hauptksobj</p>
        </td>
        <td>
          <p>Nur bei aktiver Lizenz. Ansonsten siehe Haupkoststel</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>fibuvp_haupttext</p>
        </td>
        <td>
          <p>Der Text, der dem Hauptkonto/Beleg zugeordnet wird.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>fibuvp_ustid_kunde</p>
        </td>
        <td rowspan="2">
          <p>Steht der Steuerparameter 703 „Umsatzsteuer-Identifikationsnummern auf Mandanten und Belegebene“ auf <b>Ja</b> oder auf „<b>Mit Vorbelegung</b>“, dann werden auch die Spalten „fibuvp_ustid_kunde“ und „fibuvp_ustid_mandant“ ausgewertet und in die Belege mit der fibuv_klasse 2 (AR), 3 (AG), 4 (ER) und 5 (EG) übernommen. Die hier übergebenen Werte müssen in den Stammdaten existieren.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>fibuvp_ustid_mandant</p>
        </td>
      </tr>
      <tr>
        <td>
          <p><b>gegenkonto</b></p>
        </td>
        <td>
          <p>Das Konto muss in A.eins existieren. Bei Zahlungen und Sonstigen-Belegen (Fibuv_klasse=1 und 6) kann es Sach - oder Personenkonto sein, sonst nur Sachkonto.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>koststelnummer</p>
        </td>
        <td>
          <p>Ist das Gegenkonto ein Sachkonto, so kann diesem eine Kostenstelle zugeordnet werden. Ist keine Kostenstelle zugeordnet, dann wird die Kostenstelle so zugewiesen, wie sie im Sachkonto hinterlegt ist. Ist im Sachkonto eine Sperre für Kostenstellen eingetragen, so wird der Eintrag hier ignoriert.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>kstrnummer</p>
        </td>
        <td>
          <p>Kostenträger. Siehe koststelnummer.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>ksobjnummer</p>
        </td>
        <td>
          <p>Nur bei aktiver Lizenz. Ansonsten siehe koststelnummer</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>zahlbednummer</p>
        </td>
        <td>
          <p>Nummer der Zahlungsbedingung. Wenn nicht angegeben, dann wird die Zahlungsbedingung aus dem Kundenstamm verwendet</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>fibuvp_valdatum</p>
        </td>
        <td>
          <p>Ist das Valutadatum nicht angegeben, so wird es laut Zahlungsbedingung errechnet.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>fibuvp_skodatum</p>
        </td>
        <td>
          <p>Ist das Skontodatum nicht angegeben, so wird es laut Zahlungsbedingung errechnet.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>fibuvp_skosatz</p>
        </td>
        <td>
          <p>Ist das Valutadatum nicht angegeben, so wird der Skontosatz aus der Zahlungsbedingung verwendet.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>fibuvp_skobetrag</p>
        </td>
        <td>
          <p>Ist der Skontobetrag nicht angegeben, dann wird er laut Skontosatz berechnet.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>steuerklasse</p>
        </td>
        <td>
          <p>Wenn nicht angegeben, dann laut Vorbelegung aus dem Sachkontenstamm.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>steuergrnummer</p>
        </td>
        <td>
          <p>Wenn nicht angegeben, dann laut Eintrag im Personenkonto. Siehe auch „<a href="./resultset_der_fibu_datenuebernahme.md#SteuerGruppeOption">steuergruppeausku</a>“ bzw. „steuergruppetest“</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>steuerschluessel</p>
        </td>
        <td>
          <p>Wenn nicht angegeben, dann laut Vorbelegung aus dem Sachkontenstamm.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>steuerabdatum</p>
        </td>
        <td>
          <p>Wenn nicht angegeben, dann wird das Belegdatum zur Bestimmung des Steuersatzes herangezogen.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>fibuvp_steusatz</p>
        </td>
        <td>
          <p>Wenn angegeben, dann wird dieser Steuersatz zur Berechnung der Steuer verwendet, ansonsten wird der Steuersatz wie er in den Stammdaten in A.eins hinterlegt ist, verwendet.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p><b>fibuvp_betrag</b></p>
        </td>
        <td>
          <p>Der Betrag ist Pflicht. Wird nur der Betrag angegeben, so bestimmt die Steuerklasse, ob es sich um einen Nettobetrag(Steuerklasse=1 und 101) oder um einen Bruttobetrag handelt (Steuerklasse=2 und 102). Die Beträge, die abgespeichert werden, werden errechnet. Es wird immer der Nettobetrag gespeichert.</p>
          <table>
            <tbody>
              <tr>
                <th><b>Betrag</b></th>
                <th><b>Netto/Brutto</b></th>
                <th colspan="2"><b>Abgespeicherter Betrag</b></th>
              </tr>
              <tr>
                <td>1000</td>
                <td>Netto</td>
                <td>1000</td>
                <td>190</td>
              </tr>
              <tr>
                <td>1190</td>
                <td>Brutto</td>
                <td>1000</td>
                <td>190</td>
              </tr>
            </tbody>
          </table>
        </td>
      </tr>
      <tr>
        <td>
          <p>fibuvp_steuwert</p>
        </td>
        <td>
          <p>Wird der Steuerwert angegeben, so geht das Programm davon aus, dass die Beträge so übermittelt werden, wie sie abgespeichert werden. Das Kennzeichen, ob es sich um Brutto oder Netto handelt, sorgt nur dafür, dass der Betrag in den Auswertungen/Listen brutto oder netto dargestellt wird:</p>
          <table>
            <tbody>
              <tr>
                <th><b>Betrag</b></th>
                <th><b>Netto/Brutto</b></th>
                <th colspan="2"><b>Abgespeicherter Betrag</b></th>
              </tr>
              <tr>
                <td>1000</td>
                <td>Netto</td>
                <td>1000</td>
                <td>190</td>
              </tr>
              <tr>
                <td>1190</td>
                <td>Brutto</td>
                <td>1000</td>
                <td>190</td>
              </tr>
            </tbody>
          </table>
        </td>
      </tr>
      <tr>
        <td>
          <p><b>fibuvp_sollhaben</b></p>
        </td>
        <td>
          <p>Erlaubte Werte sind 1 für Soll und 2 für Haben. Wird bei Eingangsrechnungen, Eingangsgutschriften bzw. Ausgangsrechnungen, Ausgangsgutschriften ein falsches Sollhabenkennzeichen angegeben, so ändert sich die Belegklasse von Gutschrift auf Rechnung bzw. umgekehrt.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>fibuvp_text</p>
        </td>
        <td>
          <p>Text zur Belegposition</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>mahndatum und</p>
          <p>mahnstufe</p>
        </td>
        <td>
          <p>Optional: Angabe nur bei den Belegklassen AR, AG, ER, EG möglich. Wird bei diesen Klassen keine Information angegeben, so wird dieser Beleg als nicht gemahnt gekennzeichnet</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>steuergruppetest</p>
        </td>
        <td>
          <p>Option:</p>
          <p><b>1 = Standard, wenn nichts angegeben wurde. Es wird getestet, ob bei angegebener Steuergruppe diese mit dem Kundenstamm übereinstimmt und bei Nichtübereinstimmung als Fehler ausgegeben.</b></p>
          <p>0 = Es findet kein Test statt. Dies ist die Einstellung, die im Template verwendet wird.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>steuergruppeausku</p>
        </td>
        <td>
          <p>Option:</p>
          <p><b>1 = Standard, wenn nichts angegeben wurde. Es wird unabhängig von der Steuergruppe, die im Importdatensatz steht, immer die aus dem Kundenstamm verwendet.</b></p>
          <p>0 = Steuergruppe aus dem Importdatensatz. Dies ist die Einstellung, die im Template verwendet wird.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>mitpaginiernummer</p>
        </td>
        <td>
          <p>Option:</p>
          <p>1 = Es wird, falls keine Paginiernummer angegeben wurde, dem Beleg automatisch eine Paginiernummer über das Archiv-Fakt fam_ref_fibu zugewiesen.</p>
          <p><b>0 = Standard: Die Paginiernummer wird nicht automatisch gebildet</b>. Dies ist die Einstellung, die im Template verwendet wird.<b></b></p>
        </td>
      </tr>
    </tbody>
  </table>
</div>
