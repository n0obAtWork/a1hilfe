# Feldtyp im SQL-Text

<!-- source: https://amic.de/hilfe/_FeldTypimSQLText.htm -->

Die Auswahlliste stellt Felder je nach Feldtyp dar. Dabei kann sich der Feldtyp in der Datenbank vom angezeigten Feldtypen unterscheiden. Ein Typische Beispiel dafür wären die FS-Formate, die einen „smallint“ oder „integer“ von der Datenbank erwarten und als anzeige erscheint der zugehörige Text. Bei der Angabe des Feldtypen wird nicht zwschen Groß- und Kleinschreibung unterschieden.

<div class="table-wrapper">
  <table>
    <tbody>
      <tr>
        <td>
          <p>Feldtyp</p>
        </td>
        <td>
          <p><b>Datenbanktyp</b></p>
        </td>
        <td>
          <p><b>Gütigkeitsbereich</b></p>
        </td>
        <td>
          <p><b>Beschreibung</b></p>
        </td>
      </tr>
      <tr>
        <td>
          <p>CHAR</p>
        </td>
        <td>
          <p>CHAR<br>VARCHAR<br>LONG VARCHAR</p>
        </td>
        <td></td>
        <td>
          <p>Alle möglichen Character-Typen, die in der alten AW mit bis zu 255 Zeichen Dargestellt werden. In der neuen AW 2.0 könnten die Texte komplett dargestellt werden. Es ist jedoch anzuraten, die Texte schon im SQL-Statement auf eine vernünftige Länge zu casten, da ansonsten unnötig Ressourcen verbraucht werden.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>I2</p>
          <p>I4</p>
        </td>
        <td>
          <p>SMALLINT</p>
          <p>INTEGER</p>
        </td>
        <td></td>
        <td>
          <p>Zahl ohne Nachkommastellen</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>N0 bis N6</p>
        </td>
        <td>
          <p>NUMERIC</p>
        </td>
        <td></td>
        <td>
          <p>Eine Numerische Zahl. Die Zahl hinter dem N bestimmt die Anzahl der Nachkommstellen.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>BS, BF</p>
        </td>
        <td>
          <p>NUMERIC</p>
        </td>
        <td></td>
        <td>
          <p>Betrag <b>S</b>tandardwährung bzw. Betrag <b>F</b>remdwährung. Die Nachkommastellen werden aus dem Währungsstamm bestimmt.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>PS, PF</p>
        </td>
        <td>
          <p>NUMERIC</p>
        </td>
        <td></td>
        <td>
          <p>Preis <b>S</b>tandardwährung bzw. Preis <b>F</b>remdwährung. Die Nachkommastellen werden aus dem Währungsstamm bestimmt.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>S0 bis S6</p>
        </td>
        <td>
          <p>NUMERIC</p>
        </td>
        <td></td>
        <td>
          <p>Spezielles Format für die Finanzbuchhaltung. Wie N0 bis N6, nur dass das Vorzeichen des Betrags bestimmt, ob die Zahl mit einem „S“ – bei positiven Zahlen – oder mit einem „H“ bei negativen Zahlen dargestellt werden.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>SS …..</p>
        </td>
        <td>
          <p>NUMERIC</p>
        </td>
        <td></td>
        <td>
          <p>Spezielles Format für die Finanzbuchhaltung. Hierbei bestimmt das Sollhabenkennzeichen und der Betrag die Darstellung. Mit diesem Format können auch negative Soll bzw Haben Beträge dargestellt werden.</p>
          <div>
            <code>FIELD Betrag,FiBuVP_SaldoBetrag,SS FiBuVP_SaldoSH,16</code>
          </div>
        </td>
      </tr>
      <tr>
        <td>
          <p>D2, DT, D4</p>
        </td>
        <td>
          <p>DATE</p>
        </td>
        <td></td>
        <td>
          <p>Datum im Format TT.MM.JJJJ. In der AW 2.0 wird das Datum immer mit vierstelliger Jahreszahl dargestellt, in der alten Auswahlliste wird bei Verwendung des Formats D2 die Jahreszahl nur zweistellig angezeigt.<br><br></p>
        </td>
      </tr>
      <tr>
        <td>
          <p>FS ….</p>
        </td>
        <td>
          <p>SMALLINT oder INTEGER</p>
        </td>
        <td></td>
        <td>
          <p>In der Anwendung „Formatliste“ (Direktsprung „FORMA“) können Texte zu Zahlen hinterlegt werden. Das einfachste Beiepiel ist das FS-Formt JANEIN. Mithilfe dieses Formats lassen sich die Zahlen 0 als <b>Ja</b> und 1 als <b>Nein </b>darstellen.</p>
          <div>
            <code>FIELD ,fs.FormLstKeineSprache,FS JaNein,4</code>
          </div>
        </td>
      </tr>
      <tr>
        <td>
          <p>TIMESTAMP</p>
        </td>
        <td>
          <p>TIMESTAMP</p>
        </td>
        <td>
          <p>AW 2.0</p>
        </td>
        <td>
          <p><b>Zeitstempel-Format:</b> TT.MM.JJJJ HH:mm:ss</p>
          <ul>
            <li>TT =&gt; Tag (01–31)</li>
            <li>MM =&gt; Monat (01–12)</li>
            <li>JJJJ =&gt; Jahr (z. B. 2026)</li>
            <li>HH =&gt; Stunde im 24-Stunden-Format (00–23)</li>
            <li>mm =&gt; Minuten (00–59)</li>
            <li>ss =&gt; Sekunden (00–59)<br><u>Beispiel</u>:<br>09.05.2026 11.37.58<br>Dieser Zeitstempel stellt Datum und Uhrzeit dar und wird typischerweise für präzise Protokollierung oder eindeutige Zeitangaben verwendet. Wird ein Timestamp anstelle von Char zur Darstellung in der Auswahlliste verwendet, so kann die Sortierung über die Titel-Zeile korrekte erfolgen.</li>
          </ul>
        </td>
      </tr>
      <tr>
        <td>
          <p>ICON</p>
        </td>
        <td>
          <p>CHAR</p>
        </td>
        <td>
          <p>AW 2.0</p>
        </td>
        <td>
          <p>Möglichkeit einen Status als Grafik darzustellen. Bisher existieren folgen Grafiken, die über die Bezeichnung('clip ', 'stapel', …) angesprochen werden :</p>
          <table>
            <tbody>
              <tr>
                <th><b>clip</b></th>
                <th><b>stapel</b></th>
                <th><b>Ok</b></th>
                <th><b>forbidden</b></th>
                <th><b>stop</b></th>
                <th><b>warnung</b></th>
              </tr>
              <tr>
                <td><img src="../../ImagesExt/image8_821.png" alt=""><b></b></td>
                <td><b><img src="../../ImagesExt/image8_1305.png" alt=""></b></td>
                <td><img src="../../ImagesExt/image8_1341.png" alt=""><b></b></td>
                <td><img src="../../ImagesExt/image8_1342.png" alt=""><b></b></td>
                <td><img src="../../ImagesExt/image8_1343.png" alt=""><b></b></td>
                <td><img src="../../ImagesExt/image8_1344.png" alt=""><b></b></td>
              </tr>
            </tbody>
          </table>
          <p>In FIELD</p>
          <div>
            <code>FIELD ,StapelICON,ICON,2,TIPTEXT="Zeigt an, ob der Vorgang in einem Stapel ist."</code>
          </div>
          <p><b>Hinweis: </b>Bei Icons kann man die Breite während des Arbeitens in der Auswahlliste zwar größer ziehen, jedoch wird sie beim nächsten Betreten wieder auf die Breite gesetzt, die in FIELDS steht..<b></b></p>
          <p>Im SQL</p>
          <div>
            <pre><code>Select
  if isnull(stapel_id,0) &gt; 0 then 'stapel' endif as stapelIcon
…</code></pre>
          </div>
          <p><b><u>ACHTUNG:</u> Die Bezeichnung muss immer klein geschrieben werden.</b></p>
        </td>
      </tr>
      <tr>
        <td>
          <p>MIME</p>
        </td>
        <td>
          <p>CHAR</p>
        </td>
        <td>
          <p>AW 2.0</p>
        </td>
        <td>
          <p>Bildliche Darstellung von Mime-Typen (Media Type). Wird zurzeit nur von der Archivanwendung verwendet.</p>
          <p>Beispiele:</p>
          <ul>
            <li>text/html =&gt; HTML-Seite</li>
            <li>image/png =&gt; PNG-Bild</li>
            <li>application/pdf =&gt; PDF-Dokument</li>
            <li>…<br><b></b>&nbsp;</li>
          </ul>
        </td>
      </tr>
    </tbody>
  </table>
</div>
