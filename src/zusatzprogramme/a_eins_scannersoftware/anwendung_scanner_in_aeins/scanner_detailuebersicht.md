# Scanner Detailübersicht

<!-- source: https://amic.de/hilfe/_scannerdetailuebersicht.htm -->

In dieser Variante werden alle erfassten Positionen zu einem Scanner angezeigt. Die Spalte D zeigt an welche Informationen angezeigt werden.

Status D Feld

<div class="table-wrapper">
  <table>
    <tbody>
      <tr>
        <td>
          <p>Bedeutung des D Feldes</p>
        </td>
        <td></td>
      </tr>
      <tr>
        <td>
          <p>Leer</p>
        </td>
        <td>
          <p>Hat die Zeile keine Markierung, so ist diese noch aktiv</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>*</p>
        </td>
        <td>
          <p>Ist eine Zeile mit einem * markiert, so wird diese beim nächsten Scanvorgang mit dem Scanner gelöscht.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>M</p>
        </td>
        <td>
          <p>Ist die Zeile mit einem M markiert, so werden Informationen zu einer Produktionsmaschinen angezeigt, welche gerade mit diesem Scanner bearbeitet wird.</p>
          <table>
            <tbody>
              <tr>
                <th>Spalte</th>
                <th>Beduetung</th>
              </tr>
              <tr>
                <td>Artikel</td>
                <td>Artikelnummer</td>
              </tr>
              <tr>
                <td>G.</td>
                <td>Maschinennummer</td>
              </tr>
              <tr>
                <td>Wert</td>
                <td>Status der Maschine<br>1.&nbsp;&nbsp; Maschinen befindet sich im ersten Lauf<br>2.&nbsp;&nbsp; Maschine befindet sich im zweiten Lauf</td>
              </tr>
              <tr>
                <td>DatenstromIdent</td>
                <td>Mischstatus der Maschine<br>0 Reinigen, Trocknen<br>1 Mischen</td>
              </tr>
              <tr>
                <td>Scanident</td>
                <td>Zeigt an auf den dementsprechenden Scandatensatz</td>
              </tr>
              <tr>
                <td>AI</td>
                <td>Belegnummer des Produktionsbeleges welcher abgearbeitet wird</td>
              </tr>
            </tbody>
          </table>
        </td>
      </tr>
      <tr>
        <td>
          <p>I</p>
        </td>
        <td>
          <p>Ist die Zeile mit einem I markiert so werden allgemein Informationen des Scanners angezeigt.</p>
          <table>
            <tbody>
              <tr>
                <th>Spalte</th>
                <th>Bedeutung</th>
              </tr>
              <tr>
                <td>Artikel</td>
                <td>Aktueller Artikel welcher mit dem Scanner bearbeitet wird</td>
              </tr>
              <tr>
                <td>G.</td>
                <td>In diesem Feld wird der Status des Scanner angezeigt</td>
              </tr>
              <tr>
                <td>Wert</td>
                <td>Nummerrische Nummer des erfassten Scancodes wie z.B. -101 Eingangslieferschein</td>
              </tr>
              <tr>
                <td>Scanident</td>
                <td>Aktuelle Scanident</td>
              </tr>
              <tr>
                <td>AI</td>
                <td>Aktuelle zu bearbeitende Belegnummer eines A.eins Vorganges</td>
              </tr>
              <tr>
                <td>Menge</td>
                <td>Status1</td>
              </tr>
            </tbody>
          </table>
        </td>
      </tr>
      <tr>
        <td>
          <p>S</p>
        </td>
        <td>
          <p>Ist diese Zeile mit einem S markiert, so werden die Informationen aus dem Scanner Stack angezeigt. Im Stack wird sich gemerkt welche Vorgänge mit dem Scanner gestartet wurden.</p>
          <table>
            <tbody>
              <tr>
                <th>Spalte</th>
                <th>Bedeutung</th>
              </tr>
              <tr>
                <td>G.</td>
                <td>Connectionident des Scanners.</td>
              </tr>
              <tr>
                <td>Wert</td>
                <td>Hier steht der erfasste Kommandowert, welcher den Vorgang auf dem Scanner gestartet hat.</td>
              </tr>
              <tr>
                <td>Scanident</td>
                <td>Zeiger auf die Position</td>
              </tr>
              <tr>
                <td>AI</td>
                <td>Ai Nummer des Vorgangs z.B. -101 Eingangslieferschein</td>
              </tr>
              <tr>
                <td>Menge</td>
                <td>Ident im Stack</td>
              </tr>
            </tbody>
          </table>
        </td>
      </tr>
    </tbody>
  </table>
</div>

Folgende Menüfunktionen stehen zur Verfügung:

1. [AI-Stammdaten](./einrichtung_des_scanners_an_der_zentral_datenbank/ai_stammdaten.md)

2. [Server starten](./einrichtung_des_scanners_an_der_zentral_datenbank/server_starten.md)

3. [Marktscanner Einrichten](../kundenspezifische_scannermodule/scanner_im_marktbereich.md)
