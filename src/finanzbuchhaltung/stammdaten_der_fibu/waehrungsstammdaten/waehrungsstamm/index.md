# Währungsstamm

<!-- source: https://amic.de/hilfe/whrungsstamm.htm -->

Hauptmenü > Finanzbuchhaltung > Stammdaten > Währungsstamm

Direktsprung **[WAE]**

In diesem Eingabebildschirm können die nachfolgenden Felder bearbeitet werden:

<div class="table-wrapper">
  <table>
    <tbody>
      <tr>
        <td></td>
        <td>
          <p><strong>Beschreibung</strong></p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Nummer</p>
        </td>
        <td>
          <p>Dies ist die Identifikation für die Währung in anderen Tabellen, der als Verweis auf die Währungstabelle zeigt.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Bezeichnung</p>
        </td>
        <td>
          <p>Ausführliche Bezeichnung der Währung, z. B. <strong>"Norwegische Kronen"</strong>, <strong>"Euro"</strong>.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Kurztext<br><br></p>
        </td>
        <td>
          <p>Hier kann ein von der ISO-Währungsbezeichnung abweichender Text eingetragen werden wie z.B. € statt EUR oder $ statt USD.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>ISO-Währungscode (4217)<br><br></p>
        </td>
        <td>
          <p>Hier muss die ISO-Währungsbezeichnung in standardisierter Form der ISO 4217 hinterlegt werden. Die ersten zwei Stellen stehen dabei nach ISO 3166 für das Land, die letzte Stelle für den Anfangsbuchstaben der Währung. So steht zum Beispiel:<br>USD für das ISO-Land United States = Kennzeichen US<br>und die Währung Dollar =Kennzeichen D<br>INFO: Die ISO-Währungsbezeichnung ist immer 3-stellig.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p><strong>Hedgeteiler</strong><br><br></p>
        </td>
        <td>
          <p>Für die Arbeit mit Hedge ist für einige Währungen ein Teiler vorgesehen. Dieser ist bei Hedge-Währungen in der Regel 1000. Bei&nbsp; Isländischen Kronen (ISK) oder Indonesischen Rupien (IDR) wird der Teiler 100000 verwendet.</p>
          <p>Ist der Hedgeteiler = 0, so werden für Kontrakte mit dieser Währung keine Hedge-Ordern geschrieben.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p><strong>Hedgelocation</strong></p>
        </td>
        <td>
          <p>Die Hedgelocation bestimmt den Ort, an dem die Order dieser Gegenfinanzierung für ein Geschäft mit dieser Währung platziert wird.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p><strong>Kurs</strong>faktor<br><br></p>
        </td>
        <td>
          <p>Angabe der Wechselkurse je x Einheiten der Währung. Beispielsweise wurden Lire-Kurse üblicherweise in DM/1000 Lit, Dollar in DM/1$ und die meisten Währungen in DM/100 Einheiten angegeben.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Sperre</p>
        </td>
        <td>
          <p>Kennzeichen, dass eine Währung nicht mehr benutzt werden darf<br>(z. B. Im Falle einer Währungsreform hinfällig geworden, oder wenn der Euro alleinige Währung ist, kann hier die Währung <strong>DEM</strong> gesperrt werden).</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Kurssperre<br><br></p>
        </td>
        <td>
          <p>Für Länder, die zur <strong>EWU</strong> gehören muss hier <b>Ja</b> eingetragen werden da diese Kurse ab dem <strong>01.01.1999</strong> nicht mehr änderbar sein dürfen.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Kursdienst</p>
        </td>
        <td>
          <p>Nur die Währungen, bei denen hier ein <b>Ja</b> eingetragen ist, werden bei der automatischen Ermittlung der Währungskurs berücksichtigt. Sie dazu unter <a href="../waehrungskurse/waehrungskurse_automatisch_einstellen.md">Währungskurs automatisch einstellen</a></p>
        </td>
      </tr>
      <tr>
        <td>
          <p>NachkPreis</p>
        </td>
        <td>
          <p>Anzahl zu rundender Nachkommastellen für Einzelpreise. Eingabemöglichkeiten: 0 bis 4. Für Euro muss hier eine 2 eingetragen werden.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>NachkBetrag</p>
        </td>
        <td>
          <p>Anzahl zu rundender Nachkommastellen für Beträge / Werte. Eingabemöglichkeiten: 0 bis 4. Für Euro muss hier eine 2 eingetragen werden. Diese Einstellung wird in der Finanzbuchhaltung verwendet.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Rundung Preis</p>
        </td>
        <td>
          <p>Lässt sich die gewünschte Rundung für Einzelpreise nicht durch Nachkommastellen ausdrücken, so kann hier ein Faktor für die Rundung angegeben werden.<br>Beispiel:</p>
          <p>Rundung auf 5 Rappen: Faktor = 0,05.&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp; <strong>(Rechenformel: x = Round(x / Faktor, 0) * Faktor</strong><strong></strong></p>
          <p>Für Euro muss hier 0.01 hinterlegt werden.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Rundung Betrag</p>
        </td>
        <td>
          <p>Lässt sich die gewünschte Rundung für Beträge nicht durch Nachkommastellen ausdrücken, so kann hier ein Faktor für die Rundung angegeben werden. Diese Einstellung wird in der Finanzbuchhaltung verwendet.</p>
          <p><br>Beispiel:</p>
          <p>Rundung auf 5 Rappen: Faktor = 0,05.</p>
          <p><strong>(Rechenformel: x = Round(x / Faktor, 0) * Faktor</strong></p>
          <p>Für Euro muss hier 0.01 hinterlegt werden.<strong></strong></p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Kto.Kursgewinn</p>
        </td>
        <td>
          <p>Identifikation eines Kontos, das ggf. Kursgewinne aufnehmen soll. Bei der Umrechnung von Fremdwährung in Standardwährung fallen evtl. Gewinne aus Kursdifferenzen an.<br>Diese müssen auf einem bestimmten Konto verbucht werden. Bei der Standardwährung fällt kein Kursgewinn an. Man kann hier 0 eingetragen.</p>
          <p>Weitere Information über die Verwendung dieses Kontos findet man im Abschnitt „<a href="../../../waehrungsbehandlung_in_der_finanzbuchhaltung/op_fuehrung_in_fremdwaehrung.md">Währungsbehandlung in der Finanzbuchhaltung</a>“</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Kto.Kursverlust<br><br></p>
        </td>
        <td>
          <p>Identifikation eines Kontos, das ggf. Kursverluste aufnehmen soll. Bei der Umrechnung von Fremdwährung in Standardwährung fallen evtl. Verluste aus Kursdifferenzen an.<br>Diese müssen auf einem bestimmten Konto verbucht werden. Bei der Standardwährung fällt kein Kursgewinn an. Man kann hier 0 eingetragen.</p>
          <p>Weitere Information über die Verwendung dieses Kontos findet man im Abschnitt „<a href="../../../waehrungsbehandlung_in_der_finanzbuchhaltung/op_fuehrung_in_fremdwaehrung.md">Währungsbehandlung in der Finanzbuchhaltung</a>“</p>
        </td>
      </tr>
      <tr>
        <td>
          <p><strong>Ausgleichskonto</strong></p>
        </td>
        <td>
          <p>Dort muss die Kontonummer eingetragen werden, die bei Währungsrundungsdifferenzen herangezogen werden soll. Ist kein Konto angegeben, wird verfahren wie zuvor!</p>
          <p><strong>Beispiel:</strong></p>
          <table>
            <tbody>
              <tr>
                <th><strong>Fremdwährung</strong></th>
                <th><strong>Kurs</strong></th>
                <th><strong>Buchwährung</strong></th>
              </tr>
              <tr>
                <td><strong>5,00</strong></td>
                <td><strong>1,533</strong></td>
                <td><strong>7,67</strong></td>
              </tr>
              <tr>
                <td><strong>5,00</strong></td>
                <td><strong>1,533</strong></td>
                <td><strong>7,67</strong></td>
              </tr>
              <tr>
                <td><strong>Ergibt als Summe</strong></td>
                <td>⇨<strong></strong></td>
                <td><strong>15,34</strong></td>
              </tr>
              <tr>
                <td><strong>10,00</strong></td>
                <td><strong>1,533</strong></td>
                <td><strong>15,33</strong></td>
              </tr>
              <tr>
                <td><strong>Differenz</strong></td>
                <td>⇨<strong></strong></td>
                <td><strong>0,01</strong></td>
              </tr>
            </tbody>
          </table>
          <p>Diese Differenz wird dann in einer separaten Buchungszeile ausgewiesen.</p>
          <table>
            <tbody>
              <tr>
                <th><b>Fremdwährung</b></th>
                <th><b>Kurs</b></th>
                <th><b>Buchwährung</b></th>
              </tr>
              <tr>
                <td>5,00</td>
                <td>1,533</td>
                <td>7,67</td>
              </tr>
              <tr>
                <td>5,00</td>
                <td>1,533</td>
                <td>7,67</td>
              </tr>
              <tr>
                <td>Ausbuchung gegen das Rundungskonto</td>
                <td></td>
                <td>
                  <ul>
                    <li>0,01</li>
                  </ul>
                </td>
              </tr>
              <tr>
                <td>Ergibt als Summe</td>
                <td>⇨</td>
                <td>15,33</td>
              </tr>
              <tr>
                <td>10,00</td>
                <td>1,533</td>
                <td>15,33</td>
              </tr>
              <tr>
                <td><b>Differenz</b></td>
                <td>⇨<b></b></td>
                <td><b>0,00</b></td>
              </tr>
            </tbody>
          </table>
          <p>Weitere Information über die Verwendung dieses Kontos findet man im Abschnitt „<a href="../../../waehrungsbehandlung_in_der_finanzbuchhaltung/automatische_umrechnung.md">Währungsbehandlung in der Finanzbuchhaltung</a>“.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Euro<strong></strong></p>
        </td>
        <td>
          <p>Hier wird gekennzeichnet, welche Währung der Euro ist. Es kann nur bei einer Währung dieses Kennzeichen auf <b>Ja</b> gesetzt werden.<b></b></p>
        </td>
      </tr>
      <tr>
        <td>
          <p>DTA-Methode</p>
        </td>
        <td>
          <p>hier wird hinterlegt, ob zu diesen Währungen DTA möglich ist, bzw. welche Art des DTA's verwendet werden soll. Hier gibt es drei Möglichkeiten:</p>
          <ul>
            <li>Kein DTA</li>
            <li>Standart DTA</li>
            <li>Euro-DTA<br>Standard DTA ist der vor der Einführung des Euros gültige DTA Methode. Für Euro und alle Fremdwährungen; die für die Bankbuchungen umgerechnet werden sollen, muss hier Euro DTA hinterlegt werden.</li>
          </ul>
        </td>
      </tr>
      <tr>
        <td>
          <p>Lokale Bezeichnung</p>
        </td>
        <td>
          <p>Hier wird die lokale Bezeichnung (als Text) der Währung eingetragen. Diese Hinterlegung kann z.B. verwendet werden, wenn es darum geht, textuelle Darstellung von Währung auszugeben.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Restposten in Buchwährung umrechnen</p>
        </td>
        <td>
          <p>Restposten werden nicht in der Ursprungswährung, sondern in Euro dargestellt. Dies hat folgenden Hintergrund:<b></b></p>
          <p>Die OP-Verwaltung in A.eins kann nur die Verrechnung der Buchwährung mit einer Fremdwährung gleichzeitig. Nun kann es seit der Euroumstellung zu einem Problem kommen, wenn man zum Beispiel einen Kunden hatte, der bereits Rechnungen in Dollar hatte. Nach der Umstellung auf Buchwährung Euro sind alle DM-OPs sozusagen OPs in Fremdwährung. Um nun auch diese OPs mit OPs in der Währung Dollar verrechnen zu können, wurde dieser Schalter eingeführt. Man muss dann erst die DM-OPs zusammenfassen und kann dann den Restposten, der dann in Euro dargestellt wird, mit den Rechnungen in Dollar verrechnen.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Stärker als Buchwährung</p>
        </td>
        <td>
          <p>Hier kann eingestellt werden, ob die Buchwährung stärker ist als die Fremdwährung. Das Kennzeichen wird für die Erstellung einer Hedge Zeichenkette benötigt. Das Kennzeichen wird an die dritte Stelle des ISO-Währungscodes gesetzt. Alle nachfolgenden Buchstaben werden abgeschnitten.</p>
          <p>Ist die Fremdwährung stärker als die Buchungswährung, so wird in der Hedge Zeichenkette das Buchungswährungskennzeichen der Fremdwährung auf D gesetzt und die Buchungswährung erhält das Buchungswährungskennzeichen Y.</p>
          <p>Bei der Neuanlage einer Währung wird Standardmäßig angenommen, dass die Buchungswährung stärker ist als die Fremdwährung.</p>
          <p>Beispiel:</p>
          <p>Buchwährung PL Fremdwährung EUR ergibt bei „Stärker als Buchwährung“ gleich <b>Ja</b> EUD und PLY in der Hedge Zeichenkette.</p>
          <p>Buchwährung PL Fremdwährung CZ ergibt bei „Stärker als Buchwährung“ gleich <b>Nein</b> PLD und CZY in der Hedge Zeichenkette.</p>
        </td>
      </tr>
    </tbody>
  </table>
</div>

<p class="siehe-auch">Siehe auch:</p>

- [Liste der ISO-Codes](./liste_der_iso_codes.md)
