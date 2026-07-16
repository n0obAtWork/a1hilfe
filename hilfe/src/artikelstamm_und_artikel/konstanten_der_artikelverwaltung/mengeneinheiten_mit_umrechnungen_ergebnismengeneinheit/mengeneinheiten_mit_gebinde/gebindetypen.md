# Gebindetypen

<!-- source: https://amic.de/hilfe/_gebindetypen.htm -->

Die Gebindetypen werden folgendermaßen interpretiert:

<div class="table-wrapper">
  <table>
    <tbody>
      <tr>
        <td colspan="3">
          <p><strong>Gebindetypen</strong></p>
        </td>
      </tr>
      <tr>
        <td>
          <p>0</p>
        </td>
        <td>
          <p>kein Gebinde</p>
        </td>
        <td>
          <p>Anzahl Liefereinheiten = Anzahl Mengeneinheiten</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>1</p>
        </td>
        <td>
          <p>lineares Gebinde (Anzahl)</p>
        </td>
        <td>
          <p>Anzahl mal Gebindemaß 1</p>
          <p>z.B. 10 Sack à 25 kg = 250 kg</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>2</p>
        </td>
        <td>
          <p>Gebinde 2. Stufe (Fläche)</p>
        </td>
        <td>
          <p>Anzahl mal Gebindemaß 1 mal Gebindemaß 2</p>
          <p>&nbsp;z.B. 10 Paletten à 24 Kartons a´ 6 Flaschen&nbsp;</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>3</p>
        </td>
        <td>
          <p>Gebinde 3. Stufe (Volumen)</p>
        </td>
        <td>
          <p>Anzahl mal Gebinde 1 mal Gebinde 2 mal Gebinde 3</p>
          <p>z.B.10 Behälter à 5 x 3 x 2 m = 300 cbm</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>4</p>
        </td>
        <td>
          <p>Addition (Gebi1 + Gebi2)</p>
        </td>
        <td>
          <p>Anzahl mal (Gebinde 1 + Gebinde 2)</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>5</p>
        </td>
        <td>
          <p>Subtraktion (Geb1 - Geb2)</p>
        </td>
        <td>
          <p>Anzahl mal (Gebinde 1 - Gebinde 2)</p>
          <p>z.B. Alter Zählerstand - neuer Zählerstand</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>6</p>
        </td>
        <td>
          <p>Anbruch, aufgerundet(deaktiv)</p>
        </td>
        <td>
          <p>Das Ergebnis einer Gebindeberechnung ist immer ein ganzzahlig aufgerundetes Vielfaches des ersten Gebindefaktors, also der "Packungsgröße".</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>7</p>
        </td>
        <td>
          <p>Anbruch, abgerundet(deaktiv)</p>
        </td>
        <td>
          <p>Das Ergebnis einer Gebindeberechnung ist immer ein ganzzahlig Vielfaches des ersten Gebindefaktors, also der "Packungsgröße" abzüglich einer Restmenge.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>8</p>
        </td>
        <td>
          <p>Faktor1 * Faktor2 / Faktor3</p>
        </td>
        <td></td>
      </tr>
      <tr>
        <td>
          <p>9</p>
        </td>
        <td>
          <p>Faktor1 * Faktor2 * Faktor3 * Faktor4</p>
        </td>
        <td>
          <p>Ein Gebindetyp, der es z.B. erlaubt Paletten zu fakturieren, die Lagen mit Kartons und diese wiederum in Dosen gepackt sind, aber artikelspezifische Gewichte pro Dose führen.</p>
        </td>
      </tr>
    </tbody>
  </table>
</div>
