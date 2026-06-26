# Darstellungsart Text

<!-- source: https://amic.de/hilfe/kacheltext.htm -->

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
        <td>
          <p><img alt="Ein Bild, das Text enthält. Automatisch generierte Beschreibung" src="../../../../ImagesExt/image8_1493.png"></p>
        </td>
        <td>
          <p><strong>Text</strong></p>
          <p>Für die Darstellungsart Text benötigt die View zusätzlich zu den Standardfeldern nur das Feld <b>Text</b>. Optional kann noch Textalign verwendet werden, um anzugeben, wo der Text dargestellt wird. Mögliche Werte sind 'left', 'center' und 'right'. Wird Textalig nicht angegeben, so wird der Text zentriert dargestellt.</p>
          <p>Beispielview:</p>
          <div>
            <pre><code>CREATE VIEW p_dash_button_oder_text AS
  select
    'Auftragsvolumen' as Header,
    trim(AMIC_FSTR(sum(WaBewWert), 20, 2)) as Text,
'center' as Textalign
    'vom 01.01.'|| year(today(*)) || ' bis heute'  as Footer,
    '255/255/255' as color,
    'solid' as borderstyle,
    '#333333' as bordercolor
   from
  . . .</code></pre>
          </div>
        </td>
      </tr>
    </tbody>
  </table>
</div>
