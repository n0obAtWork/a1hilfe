# Belegeingabe

<!-- source: https://amic.de/hilfe/_strecke_planung_belegeingabe.htm -->

Auf dieser Maske lassen sich schnell Belege für Spedition, Befrachter, Makler, Einladekontrolleur und Löschkontrolleur erzeugen. Je nachdem wie die Maske aufgerufen wird, werden die Belege unterschiedlich vorbelegt.

Folgende Felder stehen auf der Maske zur Verfügung.

<div class="table-wrapper">
  <table>
    <tbody>
      <tr>
        <td>
          <p><strong>Feld</strong></p>
        </td>
        <td>
          <p><strong>Beschreibung</strong></p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Kunde</p>
        </td>
        <td>
          <p>Die Vorbelegung des Kunden erfolgt je nach Typ unterschiedlich.</p>
          <p><strong>Makler</strong></p>
          <p>Es wird versucht einen Kunden aus dem Vertreterstamm des ersten gefunden Kontrakts der Strecke zu ermitteln.</p>
          <p><strong>Spedition / Befrachter, Einladekontrolleur, Löschkontrolleur</strong></p>
          <p>Je Typ wird hier versucht den Kunden aus dem Stammsatz oder Positionsstammsatz zu ermitteln.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Lager</p>
        </td>
        <td>
          <p>Die Vorbelegung des Lagers erfolgt aus dem Beleg.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Artikel</p>
        </td>
        <td>
          <p>Die Vorbelegung des Artikels erfolgt aus den <a href="../../../artikelstamm_und_artikel/parameter_des_artikelstamms/sekundaerschluessel.md">Sekundärschlüsseln</a> des Artikelstamms. Dort kann beim Schlüsseltyp „Referenzartikel“ unterschiedliche Artikel hinterlegt werden:</p>
          <table>
            <tbody>
              <tr>
                <th><strong>Position</strong></th>
                <th><strong>Typ</strong></th>
              </tr>
              <tr>
                <td>1</td>
                <td>Spedition / Befrachter</td>
              </tr>
              <tr>
                <td>2</td>
                <td>Einladekontrolleur</td>
              </tr>
              <tr>
                <td>3</td>
                <td>Löschkontrolleur</td>
              </tr>
              <tr>
                <td>4</td>
                <td>Makler</td>
              </tr>
            </tbody>
          </table>
          <p>Wenn am Artikelstamm <u>kein</u> passender Artikel gefunden wird, gilt der übergebene Artikel.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Menge</p>
        </td>
        <td>
          <p>Die Vorbelegung der Menge erfolgt aus dem Beleg</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Preis</p>
        </td>
        <td>
          <p>Die Vorbelegung des Preises erfolgt je nach Typ unterschiedlich.</p>
          <p><strong>Makler</strong></p>
          <p>Es wird versucht den Preis aus der Vertreterprovision des ersten gefundenen Kontrakts der Strecke zu ermitteln.</p>
          <p><strong>Spedition / Befrachter, Einladekontrolleur, Löschkontrolleur</strong></p>
          <p>Es wird versucht den Preis aus der Fracht des ersten gefundenen Kontrakts der Strecke zu ermitteln.</p>
        </td>
      </tr>
    </tbody>
  </table>
</div>
