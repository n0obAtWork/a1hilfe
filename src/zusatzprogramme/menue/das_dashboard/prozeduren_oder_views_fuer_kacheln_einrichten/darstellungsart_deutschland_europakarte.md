# Darstellungsart Deutschland-/Europakarte

<!-- source: https://amic.de/hilfe/kachelkarte.htm -->

Administration > Menü > Dashboard > Variante Kachel

oder

Direktsprung **[DASH]** \> Variante Kachel

Neben den hier beschriebenen Feldern stehen zusätzlich alle Felder aus dem [Basisdesign](./basisdesign.md) zur Verfügung.

<div class="table-wrapper">
  <table>
    <tbody>
      <tr>
        <td></td>
        <td></td>
      </tr>
      <tr>
        <td><img alt="Ein Bild, das Karte enthält. Automatisch generierte Beschreibung" src="../../../../ImagesExt/image8_1512.png"></td>
        <td>
          <p><strong>Deutschland- und Europakarte</strong></p>
          <p>Es steht eine Karte von Deutschland und Europa zur Verfügung, in der man sich Google-Maps Koordinaten anzeigen kann. Zusätzlich zu den Standardfeldern müssen die Felder <b>Label, X, Y</b> angegeben werden.</p>
          <p>Der Label lässt sich mit HTML formatieren und wird angezeigt, wann man mit der Maus über einen Punkt fährt.</p>
          <p>Ein weiteres optionales Feld „<b>Serie</b>“ bestimmt das Symbol und die Farbe. Serie muss im Bereich von 0 bis 9 liegen. Jede Serie wird mit einer anderen Farbe und einem anderen Symbol dargestellt:</p>
          <table>
            <tbody>
              <tr>
                <th>Serie 0</th>
                <th>Serie 1</th>
                <th>Serie 2</th>
                <th>Serie 3</th>
                <th>Serie 4</th>
              </tr>
              <tr>
                <td><img alt="Ein Bild, das Karte enthält. Automatisch generierte Beschreibung" src="../../../../ImagesExt/image8_1513.png"></td>
                <td><img alt="Ein Bild, das Karte enthält. Automatisch generierte Beschreibung" src="../../../../ImagesExt/image8_1514.png"></td>
                <td><img alt="Ein Bild, das Karte enthält. Automatisch generierte Beschreibung" src="../../../../ImagesExt/image8_1515.png"></td>
                <td><img alt="Ein Bild, das Karte enthält. Automatisch generierte Beschreibung" src="../../../../ImagesExt/image8_1516.png"></td>
                <td><img alt="Ein Bild, das Karte enthält. Automatisch generierte Beschreibung" src="../../../../ImagesExt/image8_1517.png"></td>
              </tr>
              <tr>
                <td></td>
                <td></td>
                <td></td>
                <td></td>
                <td></td>
              </tr>
              <tr>
                <td>Serie 5</td>
                <td>Serie 6</td>
                <td>Serie 7</td>
                <td>Serie 8</td>
                <td>Serie 9</td>
              </tr>
              <tr>
                <td><img alt="Ein Bild, das Karte enthält. Automatisch generierte Beschreibung" src="../../../../ImagesExt/image8_1518.png"></td>
                <td><img alt="Ein Bild, das Karte enthält. Automatisch generierte Beschreibung" src="../../../../ImagesExt/image8_1519.png"></td>
                <td><img alt="Ein Bild, das Karte enthält. Automatisch generierte Beschreibung" src="../../../../ImagesExt/image8_1520.png"></td>
                <td><img alt="Ein Bild, das Karte enthält. Automatisch generierte Beschreibung" src="../../../../ImagesExt/image8_1521.png"></td>
                <td><img alt="Ein Bild, das Karte enthält. Automatisch generierte Beschreibung" src="../../../../ImagesExt/image8_1522.png"></td>
              </tr>
            </tbody>
          </table>
          <p>Zu einer Serie kann man einen <b>SeriesTitle</b> angeben, der erscheint, wenn man auf den Knopf rechts oben klickt. Dieser Knopf ist nur dann sichtbar, wenn man mehrere Serien verwendet. Hierüber können dann einzelne Serien ein- und ausgeblendet werden.:</p>
          <img alt="Ein Bild, das Text, Brief enthält. Automatisch generierte Beschreibung" src="../../../../ImagesExt/image8_1523.png">
          <p>Mit dem mittleren Mausrad oder den Tasten Bild▼ und Bild▲ lässt sich ein Ausschnitt vergrößern oder verkleinern.</p>
          <p>Mit Strg+Maus lässt sich ein Bereich auswählen.</p>
          <p>Mit den Pfeiltasten lässt sich der Bereich verschieben.</p>
          <p>Mit Pos1 wird die Anfangsgröße wieder herstellen.</p>
          <p>Fährt man mit der Maus auf ein Symbol der Serie, so wird der mit Label angegebene Text eingeblendet.</p>
          <p>Ist eine Klick-Funktion angegeben, so wird ein Hand-Symbol als Mauszeiger angezeigt, wenn man mit der Maus über einen Punkt fährt.</p>
          <p>Beispielview mit Rückgabe der Adressid der angezeigten Anschrift:</p>
          <div>
            <pre><code>CREATE VIEW p_dash_geographicMap AS
 select
   'solid' as borderstyle,
   '#333333' as bordercolor,
-- Pro Angezeigter Position muss ein Datensatz mit dem
-- &gt;label&lt; und den Google-Maps Koordinaten zurückgeliefert werden
   if Adresstyp = 11 then 0
   else if Adresstyp = 12 then 1
   else if Adresstyp = 15 then 2
   else 3
   endif endif endif as Serie,
   if Adresstyp in (11,12,15) then
     AMIC_FORMLST_GETBEZEICH('ADRESSTYP',Adresstyp)
   else
     'Sonstige'
   endif as SeriesTitle,
   ans.Adressid as ID1,
    '&lt;u&gt;&lt;b&gt;' || ans.adressname || '&lt;/b&gt;&lt;/u&gt;&lt;br&gt;'
             || ans.adressStrasse || '&lt;br&gt;'
              || ans.adressPLZ1 || ' ' || ans.adressOrt || '&lt;br&gt;' as label,
    cast(posi.ST_LAT() as numeric(15,6))  as X,
    cast(posi.ST_LONG() as numeric(15,6))  as Y
    from anschriftgeodata geo join anschriftstamm ans on ans.adressid=geo.adressid where ans.adressnummer &gt;= 0</code></pre>
          </div>
        </td>
      </tr>
    </tbody>
  </table>
</div>
