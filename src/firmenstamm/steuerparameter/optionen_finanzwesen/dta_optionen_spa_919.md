# DTA – Optionen (SPA 919)

<!-- source: https://amic.de/hilfe/_SPA_919.htm -->

Hier können Optionen für [DTA](../../../finanzbuchhaltung/zahlungsverkehr/zahlungen_bearbeiten/dta.md) gepflegt werden.

<div class="table-wrapper">
  <table>
    <tbody>
      <tr>
        <td>
          <p><strong>Typ</strong></p>
        </td>
        <td>
          <p><strong>Wert</strong></p>
        </td>
      </tr>
      <tr>
        <td>
          <p>CSATZ Name Herkunft</p>
        </td>
        <td>
          <p>Hier kann eingestellt werden wie der CSATZ_NAME ermittelt wird.</p>
          <table>
            <tbody>
              <tr>
                <th><strong>Wert</strong></th>
                <th><strong>Bedeutung</strong></th>
              </tr>
              <tr>
                <td></td>
                <td>
                  Standardmäßig wird der CSATZ_NAME wie folgt gebildet:
                  <br>
                  <ul>
                    <li>Empfänger / Zahlungspflichtiger aus der Kundenbank</li>
                    <li>Wenn in der Kundenbank kein Zahlungspflichtiger eingetragen ist, dann der Empfänger / Zahlungspflichtige aus dem Kundenstamm (Fibu-Merkmale)</li>
                    <li>Wenn auch dieser Wert leer ist dann Kundenbezeichnung</li>
                  </ul>
                </td>
              </tr>
              <tr>
                <td>1</td>
                <td>Ermittlung des CSATZ_NAME <u>nur</u> aus dem Namen der Hauptanschrift.<br><br></td>
              </tr>
            </tbody>
          </table>
        </td>
      </tr>
    </tbody>
  </table>
</div>
