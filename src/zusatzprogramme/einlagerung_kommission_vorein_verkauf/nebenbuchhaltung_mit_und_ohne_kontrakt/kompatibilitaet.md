# Kompatibilität

<!-- source: https://amic.de/hilfe/kompatibilitt.htm -->

Die Angabe des Buchungstyps löst die alten Kontraktklassen 2 und 12 (Fremdlager VK, Fremdlager EK) ab. Aus Kompatibilitätsgründen können in der Übersicht auch alte Vorverkaufs- bzw. Voreinkaufskontrakte der Kontraktklassen 2 und 12 angezeigt werden. Neu erzeugt werden diese Kontraktklassen jedoch nicht mehr.

Bisher war die Definition eines Kontrakts über die Kontraktklasse möglich. Die Kontraktklassen 2 bzw. 12 zeigten an, dass es sich um Fremdware bzw. Fremdlagerkontrakte handelte.

Mit Beginn der Einlagerung laufen diese Kontrakte aus. Es wird ein zusätzliches Kennzeichen, der KtrBuchTyp eingeführt. Dieser Buchungstyp gibt bei Einkaufs- und Verkaufskontrakten künftig an, um welche Art von Kontrakt es sich handelt.

<div class="table-wrapper">
  <table>
    <tbody>
      <tr>
        <td colspan="4">
          <p><strong>Felder aus der Tabelle Kontraktstamm</strong></p>
        </td>
      </tr>
      <tr>
        <td>
          <p><b>Kontraktklasse</b></p>
          <p><b>KtrKlasse</b></p>
        </td>
        <td>
          <p><b>Buchungstyp</b></p>
          <p><b>KtrBuchTyp</b></p>
        </td>
        <td>
          <p><b>Alte</b></p>
          <p><b>Kontraktklasse</b></p>
        </td>
        <td>
          <p><b>Beschreibung</b></p>
        </td>
      </tr>
      <tr>
        <td>
          <p>1</p>
        </td>
        <td>
          <p>0</p>
        </td>
        <td>
          <p>1</p>
        </td>
        <td>
          <p>Verkaufskontrakt</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>1</p>
        </td>
        <td>
          <p>1</p>
        </td>
        <td>
          <p>2</p>
        </td>
        <td>
          <p>Vorverkaufskontrakt (Fremdware Verkauf)</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>1</p>
        </td>
        <td>
          <p>4</p>
        </td>
        <td>
          <p>---</p>
        </td>
        <td>
          <p>Kommission Verkauf</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>11</p>
        </td>
        <td>
          <p>0</p>
        </td>
        <td>
          <p>11</p>
        </td>
        <td>
          <p>Einkaufskontrakt</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>11</p>
        </td>
        <td>
          <p>2</p>
        </td>
        <td>
          <p>12</p>
        </td>
        <td>
          <p>Voreinkaufskontrakt (Fremdlager Einkauf)</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>11</p>
        </td>
        <td>
          <p>3</p>
        </td>
        <td>
          <p>---</p>
        </td>
        <td>
          <p>Einlagerungskontrakt</p>
        </td>
      </tr>
    </tbody>
  </table>
</div>

Bei allen Reporten und Auswertungen muss also künftig diese neue Konstellation parallel zu der auslaufenden alten Konstellation berücksichtigt werden. Bestehende privater Reports müssen angepasst werden.

Die ausgelieferten Reports, Auswahllisten und Itemboxen sind bereits angepasst worden, zeigen jedoch zum Teil nur die bisher verfügbaren Informationen an. Für Wünsche zur Ergänzung oder Hinweise zu unbearbeiteten Listen sind wir dankbar. Kontaktieren Sie bitte den Support.
