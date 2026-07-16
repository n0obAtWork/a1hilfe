# Prolongation / Verlängerung eines Wechsels bei nicht weitergebebenen Wechseln

<!-- source: https://amic.de/hilfe/prolongationverlngerungeineswe.htm -->

Hauptmenü \> Finanzbuchhaltung \> Mahn-/Zahl-/Zinswesen \> Wechselbuchhaltung > Wechsel bearbeiten

Direktsprung **[WEB]**

Kann der Bezogene am Verfalltag die Wechselsumme nicht bezahlen, dann besteht die Möglichkeit der Prolongation, also der Verlängerung der Zahlungsfrist. Dazu muss man in der Anwendung **Wechsel bearbeiten** den Wechsel auswählen und mit F5 ändern. Dort steht einem die Funktion **F9** für **Prolongation** zur Verfügung

![](../../../ImagesExt/image8_751.png)

Es wird im Wechselstamm ein neuer Wechsel mit dem ehemaligen Verfallsdatum als Ausstellungsdatum hinterlegt. Es erfolgt jedoch <strong><em>keine Buchung in der FiBu</em></strong><em>!</em> 

Der alte Wechsel wird als verlängert gekennzeichnet. Folgende Prolongationsstati sind implementiert:

<div class="table-wrapper">
  <table>
    <tbody>
      <tr>
        <td>
          <p>0</p>
        </td>
        <td>
          <p>Originalwechsel nicht verlängert</p>
        </td>
        <td>
          <p>Gültig</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>1</p>
        </td>
        <td>
          <p>Originalwechsel verlängert</p>
        </td>
        <td>
          <p>Verfallener Wechsel aus Status 0</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>2</p>
        </td>
        <td>
          <p>Verlängerter (neuer) Wechsel</p>
        </td>
        <td>
          <p>(folgt auf Status 0 oder 3)</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>3</p>
        </td>
        <td>
          <p>Erneut verlängert</p>
        </td>
        <td>
          <p>Verfallener Wechsel aus Status 2</p>
        </td>
      </tr>
    </tbody>
  </table>
</div>
