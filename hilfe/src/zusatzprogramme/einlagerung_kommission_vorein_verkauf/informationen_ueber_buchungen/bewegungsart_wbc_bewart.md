# Bewegungsart (wbc_BewArt)

<!-- source: https://amic.de/hilfe/bewegungsartwbcbewart.htm -->

Das Feld wbc_BewArt findet sich in der View [AMIC_V_Warenbewegung_info](./view_amic_v_warenbewegung_info.md).

<div class="table-wrapper">
  <table>
    <tbody>
      <tr>
        <td colspan="2">
          <p><strong>Bewegungsart</strong></p>
          <p>Ist wbc_BewArt gleich 0, so handelt es sich um eine reine buchhalterische Buchung, bei der keine Ware physisch bewegt wird (Ausnahme reiner Einkauf, reiner Verkauf). Dies kann das bisherige Kennzeichen WaBewBestTyp der Warenbewegung ersetzen.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>0</p>
        </td>
        <td>
          <p>Alle Buchungen, die buchhalterisch relevante Bestände berühren (EK, VK, Vereinnahmung und Kommissionsverkauf)</p>
          <p>Diese Bewegungsart wurde in früheren Versionen auch als Eigenbewegung bezeichnet.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>1</p>
        </td>
        <td>
          <p>Buchung, die nur einen physikalischen Bestand berührt – Vorverkauf Abholung</p>
          <p>Diese Bewegungsart wurde in früheren Versionen auch als FremdwareVerkauf bezeichnet</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>2</p>
        </td>
        <td>
          <p>Buchung, die nur einen physikalischen Bestand berührt – Voreinkauf Anlieferung</p>
          <p>Diese Bewegungsart wurde in früheren Versionen auch als Fremdlager Einkauf bezeichnet</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>3</p>
        </td>
        <td>
          <p>Buchung, die nur einen physikalischen Bestand berührt – Einlagerung und Abholung</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>4</p>
        </td>
        <td>
          <p>Buchung, die nur einen physikalischen Bestand berührt – Kommission und Rücknahme</p>
        </td>
      </tr>
    </tbody>
  </table>
</div>
