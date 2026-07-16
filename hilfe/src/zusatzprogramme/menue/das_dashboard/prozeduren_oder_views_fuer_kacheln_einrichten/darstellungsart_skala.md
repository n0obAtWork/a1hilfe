# Darstellungsart Skala

<!-- source: https://amic.de/hilfe/kachelskala.htm -->

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
        <td><img alt="Ein Bild, das Text, Uhr, Screenshot enthält. Automatisch generierte Beschreibung" src="../../../../ImagesExt/image8_1495.png"> <img alt="Ein Bild, das Text, Uhr enthält. Automatisch generierte Beschreibung" src="../../../../ImagesExt/image8_1496.png"> <img alt="Ein Bild, das Diagramm enthält. Automatisch generierte Beschreibung" src="../../../../ImagesExt/image8_1497.png"></td>
        <td>
          <p><strong>Skala</strong></p>
          <p>Die Skala ähnelt sehr dem Fortschrittsbalken, hat jedoch ein paar mehr Einstellmöglichkeiten:</p>
          <table>
            <tbody>
              <tr>
                <th><b>Minimum</b></th>
                <th>muss den Datenbanktypen integer liefern. Standard ist 0</th>
              </tr>
              <tr>
                <td><b>Maximum</b></td>
                <td>muss den Datenbanktypen integer liefern. Standard ist 100.</td>
              </tr>
              <tr>
                <td><b>Value</b></td>
                <td>Der Wert wird durch den Zieger dargestellt. Er muss den Datenbanktypen integer liefern und zwischen Minimum und Maximum liegen.</td>
              </tr>
              <tr>
                <td><b>Majorinterval</b></td>
                <td>Das Intervall des Markers unter den Zahlen. Standartmäßig wird dieses Intervall mit (Maximum-Minimum) / 5 berechnet.</td>
              </tr>
              <tr>
                <td><b>Minorinterval</b></td>
                <td>Das Intervall für die kleineren Markierungen. Standartmäßig wird dieses Intervall mit Majorintervall / 5 berechnet.</td>
              </tr>
              <tr>
                <td colspan="2"><u>Farbangaben</u><br>Die Skala kann in bis zu drei Farbbereiche unterteilt werden.</td>
              </tr>
              <tr>
                <td><b>LowerFillingColor</b><br><b></b>&nbsp;</td>
                <td>Die Farbe am linken Rand, in den Beispielabbildungen ist es die Farbe #FF3333. Wenn keine Farbe angegeben wird, dann wird die Hintergrundfarbe verwendet. Es wird ein Verlauf von LowerFillingColor auf Fillingcolor dargestellt.</td>
              </tr>
              <tr>
                <td><b>LowerFillingTo</b><br><b></b>&nbsp;</td>
                <td>Eine Zahl, die zwischen Minimum und Maximum liegen muss. Diese Zahl gibt die Breite an, die LowerFillingColor einnehmen darf. Setzt man diesen Wert auf Minimum werden nur FillingColor bzw. UpperFillingColor dargestellt.<br><u>Hinweis:</u> <i>Soll es so aussehen, als ob nur zwei Farben verwendet werden, so setzt man LowerFillingTo auf Minimum.</i><br><i></i>&nbsp;</td>
              </tr>
              <tr>
                <td><b>FillingColor</b><br><b></b>&nbsp;</td>
                <td>Die Farbe wird zwischen zwischen LowerFillingTo und UpperFillingFrom dargestellt.</td>
              </tr>
              <tr>
                <td><b>UpperFillingColor</b><br><b></b>&nbsp;</td>
                <td>Die Farbe am rechten Rand, in den Beispielabbildungen ist es die Farbe #33FF33. Wenn keine Farbe angegeben wird, dann wird die Hintergrundfarbe verwendet. Es wird ein Verlauf von Fillingcolor auf UpperFillingcolor dargestellt.</td>
              </tr>
              <tr>
                <td><b>UpperFillingFrom</b><br><b></b>&nbsp;</td>
                <td>Eine Zahl, die angibt ab welcher Position der Bereich für UpperFillingColor beginnt. Er muss zwischen Minimum und Maximum liegen.</td>
              </tr>
              <tr>
                <td><b>Orientation</b><br><b></b>&nbsp;</td>
                <td>Orientation kann sein <b>horizontal</b> (Standard) oder <b>vertical</b></td>
              </tr>
              <tr>
                <td><b>Overlap</b><br><b></b>&nbsp;</td>
                <td>Sollen die Markierungen über dem Farbbalken sitzen, dann muss hier eine 1 geliefert werden. Standard ist 0.</td>
              </tr>
            </tbody>
          </table>
          <p>Beispielview:</p>
          <div>
            <pre><code>create view p_dash_skala
select
   0   as Minimum ,               -- Wenn nicht angegeben, dann 0
   100 as Maximum,                -- Wenn nicht angegeben, dann 100
   32  as Value,                  -- muss sollte zwischen minimum und maximum liegen.
  -- 20 majorinterval,            -- Optional. Wenn nicht gesetzt dann (maximun-minimum)/5
  -- 6 minorinterval,             -- Optional. Wenn nicht gesetzt dann majorinterval/5
   '#ff3333' LowerFillingColor ,  -- Linker Farbwert
   '#ffff66' FillingColor ,       -- mittlerer Farbwert
   '#33ff33' UpperFillingColor ,  -- rechter Farbwert
   30 LowerFillingTo,             -- Grenze zwischen linkem und mittlerem Farbwert
   60 UpperFillingFrom,           -- Grenze zwischen mittlerem und rechtem Farbwert
   'horizontal' as orientation,   -- Ausrichtung der Skala. Kann &gt;vertical&lt; oder &gt;horizontal&lt; sein(default).
   0 as Overlap                   -- Bei 1 liegt die Farbe hinter der Skala</code></pre>
          </div>
        </td>
      </tr>
    </tbody>
  </table>
</div>
