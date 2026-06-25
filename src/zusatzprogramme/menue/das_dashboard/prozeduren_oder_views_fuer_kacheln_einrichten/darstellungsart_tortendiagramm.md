# Darstellungsart Tortendiagramm

<!-- source: https://amic.de/hilfe/kacheltortendiagramm.htm -->

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
          <p><img alt="Ein Bild, das Diagramm, Tortendiagramm enthält. Automatisch generierte Beschreibung" src="../../../../ImagesExt/image8_1499.png"></p>
        </td>
        <td>
          <p><strong>Tortendiagramm</strong></p>
          <p>In einem Tortendiagramm können bis zu zehn Datensätze („Tortenstücke“) angezeigt werden. Der Wert und die Bezeichnung des Datensatzes werden in der View/Prozedur mit den Feldern <b>Wert</b> und <b>Label</b> angegeben.</p>
          <p>Im Tortendiagramm besteht die Möglichkeit kleinere Tortenstücke in einem einzelnen Tortenstück („Sonstige“) zusammenzufassen. Dazu wird in der View/Prozedur dem Feld <b>OthersCategoryInPercent</b> ein Wert größer 0 zugewiesen. Mit diesem Wert gibt man eine Schwelle an, unter der alle Tortenstücke zusammengefasst werden. Beispiel:</p>
          <p>In der View wird für das Feld OthersCategoryInPercent eine 2 angegeben. Dann werden alle Datensätze, die weniger als 2% ausmachen, in dem Tortenstück „Sonstige“ zusammengefasst.</p>
          <p>Hinweis:</p>
          <p><i>Auf dem Tortenstück „Sonstige“ kann keine Klick-Funktion ausgeführt werden. Des Weiteren wird im Tooltip nur der Text „Sonstige“ angezeigt.</i></p>
          <p><u>Legende</u><u></u></p>
          <p>Mithilfe des Feldes <b>LegendVisible</b> kann eingestellt werden, ob die Legende standardmäßig ein- oder ausgeblendet ist. Unabhängig von dieser Option kann die Legende über die Funktion <strong><em>Legende ein-/ausblenden</em></strong> (rechte Maustaste auf der Kachel) aktiviert bzw. deaktiviert werden. Des Weiteren ist die Position (<b>LegendPosition</b>) und die Ausrichtung (<b>LegendOrientation</b>) der Legende über die View/Prozedur einstellbar. Mögliche Werte sind:</p>
          <p>LegendPosition</p>
          <ul>
            <li>Right</li>
            <li>Left</li>
            <li>Bottom</li>
            <li>Top<br>LegendOrientation</li>
            <li>Vertical</li>
            <li>Horizontal<br>Hinweis:<br><i>Im Tortendiagramm besteht die Möglichkeit die Klick-Funktion über die Legende auszuführen.</i><br><u>Tooltipp</u><u></u><br>Mit dem Feld <b>SliceTooltip</b> kann der Tooltip über HTML formatiert werden. Der Tooltip erscheint, wenn der Mauszeiger über einen Datenpunkt des Diagramms bewegt wird. Existiert das Feld <b>SliceTooltip</b> nicht in der View/Prozedur, so wird der Tooltip nicht angezeigt.<br>Beispielview:</li>
          </ul>
          <div>
            <pre><code>CREATE VIEW p_dash_torte AS
select
  top 10
      sum(wabewWert) as wert,
  'Top 10 Kunden' as
      header,         -- Wird kein
      header angegeben, steht der Platz dem Mittelteil zur Verfügung.
  'solid' as
      borderstyle,          --
      Mögliche Wert sind: &gt;none&lt;(standard), &gt;solid&lt;, &gt;raised&lt;,
      &gt;inset&lt;
  '68/68/68' as
      bordercolor,       -- Bei Borderstyle =
      Solid muss man noch die bordercolor festlegen
 -- Pro
      Tortenstück muss ein Datensatz mit dem
 -- &gt;wert&lt;
      des Kuchenstücks und dem
 -- &gt;label&lt;
      des Kuchenstücks zurückgeliefert werden
  'bottom' as LegendPosition,
  '0' as
      LegendVisible,
  'horizontal' as
      LegendOrientation,
  ks.kundBezeich
      as label,
  ks.KundId as
      Id1
  from warenbewegung wb
  join v_posiWare vp on vp.wabewId = wb.wabewId
  join vorgangstamm vs on vs.V_Id = vp.V_Id
  join
      Kundenstamm ks on ks.KundId = vs.KundIdZuord
  where
      vs.V_Klassnummer = 700 and ks.KundNummer
  group by
      ks.kundId, ks.kundNummer, ks.kundBezeich
  order by wert desc</code></pre>
          </div>
        </td>
      </tr>
    </tbody>
  </table>
</div>
