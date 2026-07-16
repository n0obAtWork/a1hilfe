# Belegimport (SPA 829)

<!-- source: https://amic.de/hilfe/_SPA_829.htm -->

In diesem Steuerparameter können Optionen für den Belegimport eingestellt werden.

Zur Einstellung stehen verschiedene Typen zur Verfügung.

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
          <p>IMPORTPFAD</p>
        </td>
        <td>
          <p>Standardpfad für den Import der XML-Daten. Der Pfad muss dabei auf einen gültigen Pfad auf dem Datenbankserver zeigen.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>IMPORTPROZEDUR</p>
        </td>
        <td>
          <p>Alternative Datenbankfunktion für den Import der XML-Daten.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>KUNDE</p>
        </td>
        <td>
          <p>Standardkundennummer für die Eingangsrechnungsbelege.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>MAKRO_KOPF_START</p>
        </td>
        <td>
          <p>Hier kann ein Makro eingetragen werden, welches vor der Funktion „StartVorgang“ aufgerufen wird.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>MAKRO_KOPF_ENDE</p>
        </td>
        <td>
          <p>Hier kann ein Makro eingetragen werden, welches nach der Funktion „StartVorgang“ aufgerufen wird.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>MAKRO_POSI_START</p>
        </td>
        <td>
          <p>Hier kann ein Makro eingetragen werden, welches vor der Funktion „PositionNeu“ aufgerufen wird.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>MAKRO_POSI_ZWISCHEN</p>
        </td>
        <td>
          <p>Hier kann ein Makro eingetragen werden, welches nach der Funktion „PositionNeu“ und vor „PositionAdd“ aufgerufen wird.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>MAKRO_POSI_ENDE</p>
        </td>
        <td>
          <p>Hier kann ein Makro eingetragen werden, welches nach der Funktion „PositionAdd“ aufgerufen wird.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>UNTERKLASSUMSCHLUESSEL</p>
        </td>
        <td>
          <p>Hier wird die Klasse des Umschlüsselwerks eingetragen, welche eine Zuordnung zwischen dem A.eins Lager und der Vorgangsunterklasse herstellt. Ist diese Ausprägung nicht gesetzt, so wird die Unterklasse 0 genommen.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>BELEGDATUMTODAY</p>
        </td>
        <td>
          <p>Mit diesem Steuerparameter kann eingestellt werden, ob die A.eins Eingangsrechnung das Belegdatum der Terresrechnung erhalten soll. Oder ob die A.eins Eingangsrechnung das Tagesdatum erhält.</p>
          <table>
            <tbody>
              <tr>
                <th><strong>Wert</strong></th>
                <th><strong>Bedeutung</strong></th>
              </tr>
              <tr>
                <td>0</td>
                <td>A.eins Eingangsrechnung erhält das Datum der Terres Rechnung.</td>
              </tr>
              <tr>
                <td>1</td>
                <td>Es wird das Tagesdatum verwendet.</td>
              </tr>
            </tbody>
          </table>
        </td>
      </tr>
    </tbody>
  </table>
</div>

#### Info zu Makro

Beim Aufruf der „MAKRO_KOPF…“ und „MAKRO_POSI…“ werden folgende Parameter aufgerufen.

<div class="table-wrapper">
  <table>
    <tbody>
      <tr>
        <td>
          <p><strong>Parameter</strong></p>
        </td>
        <td>
          <p><strong>Beschreibung</strong></p>
        </td>
      </tr>
      <tr>
        <td>
          <p>PARAM1</p>
        </td>
        <td>
          <p>Dieser Parameter enthält den Modus, durch welchen das Makro aufgerufen wurde. Mögliche Werte stehen in der folgenden Tabelle.</p>
          <table>
            <tbody>
              <tr>
                <th><strong>Makrotyp</strong></th>
                <th><strong>Wert</strong></th>
              </tr>
              <tr>
                <td>MAKRO_KOPF_START</td>
                <td>KOPFSTART</td>
              </tr>
              <tr>
                <td>MAKRO_KOPF_ENDE</td>
                <td>KOPFENDE</td>
              </tr>
              <tr>
                <td>MAKRO_POSI_START</td>
                <td>POSISTART</td>
              </tr>
              <tr>
                <td>MAKRO_POSI_ZWISCHEN</td>
                <td>POSIZWISCHEN</td>
              </tr>
              <tr>
                <td>MAKRO_POSI_ENDE</td>
                <td>POSIENDE</td>
              </tr>
            </tbody>
          </table>
        </td>
      </tr>
      <tr>
        <td>
          <p>PARAM2</p>
        </td>
        <td>
          <p>Dieser Parameter enthält den Namen des aktuellen „Vorgangshelper“ JPP-Objekts.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>PARAM3</p>
        </td>
        <td>
          <p>Dieser Parameter enthält den JVARS-Owner in dem die Vorgangskopfdaten liegen.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>PARAM4</p>
        </td>
        <td>
          <p>Dieser Parameter enthält den JVARS-Owner in dem die Positionsdaten der aktuellen Position liegen.</p>
        </td>
      </tr>
    </tbody>
  </table>
</div>
