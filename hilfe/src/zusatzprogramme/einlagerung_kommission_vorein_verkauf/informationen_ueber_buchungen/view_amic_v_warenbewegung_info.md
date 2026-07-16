# View AMIC_V_Warenbewegung_Info

<!-- source: https://amic.de/hilfe/viewamicvwarenbewegunginfo.htm -->

Zuweilen wollen Sie sicher zusätzliche Informationen zu den Warenbewegungen bekommen. Diese bietet Ihnen die View Warenbewegung_Info.

Diese View kann mit dem Feld wabew_id an die Tabelle Warenbewegung oder andere Views gejoint werden, die die wabew_id enthalten.

<div class="table-wrapper">
  <table>
    <tbody>
      <tr>
        <td colspan="3">
          <p><strong>AMIC_V_Warenbewegung_Info</strong></p>
          <p><strong>Gibt zusätzliche Informationen zu Warenbewegungen</strong></p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Feld</p>
        </td>
        <td>
          <p>Typ</p>
        </td>
        <td>
          <p>Bezeichnung</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>wabew_id</p>
        </td>
        <td>
          <p>Integer</p>
        </td>
        <td>
          <p>ID der Warenbewegung</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>tmp_ist</p>
        </td>
        <td>
          <p>Integer</p>
        </td>
        <td>
          <p>temporäre Zwischenergebnise</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>tmp_fremd</p>
        </td>
        <td>
          <p>Numeric(15,4)</p>
        </td>
        <td>
          <p>temporäre Zwischenergebnise</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>tmp_ktrdiff</p>
        </td>
        <td>
          <p>Numeric(15,4)</p>
        </td>
        <td>
          <p>temporäre Zwischenergebnise</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>tmp_wert</p>
        </td>
        <td>
          <p>Numeric(15,4)</p>
        </td>
        <td>
          <p>temporäre Zwischenergebnise</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>wbc_Typ_EKVK</p>
        </td>
        <td>
          <p>smallint</p>
        </td>
        <td>
          <p>Einkauf/Verkaufskennzeichen (EK=1, VK=2)</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>wbc_SigniEigenware</p>
        </td>
        <td>
          <p>Numeric(15,4)</p>
        </td>
        <td>
          <p>Vorzeichen Eigenware</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>wbc_SigniEigenwareKtrDiff</p>
        </td>
        <td>
          <p>Numeric(15,4)</p>
        </td>
        <td>
          <p>Vorzeichen Eigenware Kontraktdifferenz</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>wbc_SigniFremdware_VVK</p>
        </td>
        <td>
          <p>Numeric(15,4)</p>
        </td>
        <td>
          <p>Vorzeichen Fremdware Vorverkauf</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>wbc_SigniFremdlager_VEK</p>
        </td>
        <td>
          <p>Numeric(15,4)</p>
        </td>
        <td>
          <p>Vorzeichen Eigenware Voreinkauf</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>wbc_SigniFremdware_EINL</p>
        </td>
        <td>
          <p>Numeric(15,4)</p>
        </td>
        <td>
          <p>Vorzeichen Fremdware Einlagerung</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>wbc_SigniFremdlager_KOM</p>
        </td>
        <td>
          <p>Numeric(15,4)</p>
        </td>
        <td>
          <p>Vorzeichen Fremdlager Kommission</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>wbc_SigniEigenBestand</p>
        </td>
        <td>
          <p>Numeric(15,4)</p>
        </td>
        <td>
          <p>Vorzeichen Eigenbestand</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>wbc_SigniLagerBestand</p>
        </td>
        <td>
          <p>Numeric(15,4)</p>
        </td>
        <td>
          <p>Vorzeichen Lagerbestand</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>wbc_SigniEinkauf</p>
        </td>
        <td>
          <p>Numeric(15,4)</p>
        </td>
        <td>
          <p>Vorzeichen Einkauf</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>wbc_SigniVerkauf</p>
        </td>
        <td>
          <p>Numeric(15,4)</p>
        </td>
        <td>
          <p>Vorzeichen Verkauf</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>wbc_Eigenware</p>
        </td>
        <td>
          <p>Numeric(15,4)</p>
        </td>
        <td>
          <p>Menge Eigenware</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>wbc_Fremdware_VVK</p>
        </td>
        <td>
          <p>Numeric(15,4)</p>
        </td>
        <td>
          <p>Menge Vorverkauf</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>wbc_Fremdlager_VEK</p>
        </td>
        <td>
          <p>Numeric(15,4)</p>
        </td>
        <td>
          <p>Menge Voreinkauf</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>wbc_Fremdware_EINL</p>
        </td>
        <td>
          <p>Numeric(15,4)</p>
        </td>
        <td>
          <p>Menge Einlagerung</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>wbc_Fremdlager_KOM</p>
        </td>
        <td>
          <p>Numeric(15,4)</p>
        </td>
        <td>
          <p>Menge Kommission</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>wbc_EigenBestand</p>
        </td>
        <td>
          <p>Numeric(15,4)</p>
        </td>
        <td>
          <p>Menge Eigenbestand</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>wbc_LagerBestand</p>
        </td>
        <td>
          <p>Numeric(15,4)</p>
        </td>
        <td>
          <p>Menge Lagerbestand</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>wbc_Einkauf</p>
        </td>
        <td>
          <p>Numeric(15,4)</p>
        </td>
        <td>
          <p>Menge Einkauf</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>wbc_Verkauf</p>
        </td>
        <td>
          <p>Numeric(15,4)</p>
        </td>
        <td>
          <p>Menge Verkauf</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>wbc_Eigenware_Wert</p>
        </td>
        <td>
          <p>Numeric(15,4)</p>
        </td>
        <td>
          <p>Wert Eigenware</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>wbc_Fremdware_VVK_Wert</p>
        </td>
        <td>
          <p>Numeric(15,4)</p>
        </td>
        <td>
          <p>Wert Vorverkauf</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>wbc_Fremdlager_VEK_Wert</p>
        </td>
        <td>
          <p>Numeric(15,4)</p>
        </td>
        <td>
          <p>Wert Voreinkauf</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>wbc_Fremdware_EINL_Wert</p>
        </td>
        <td>
          <p>Numeric(15,4)</p>
        </td>
        <td>
          <p>Wert Einlagerung</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>wbc_Fremdlager_KOM_Wert</p>
        </td>
        <td>
          <p>Numeric(15,4)</p>
        </td>
        <td>
          <p>Wert Kommission</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>wbc_EigenBestand_Wert</p>
        </td>
        <td>
          <p>Numeric(15,4)</p>
        </td>
        <td>
          <p>Wert Eigenbestand</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>wbc_LagerBestand_Wert</p>
        </td>
        <td>
          <p>Numeric(15,4)</p>
        </td>
        <td>
          <p>Wert Lagerbestand</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>wbc_Einkauf_Wert</p>
        </td>
        <td>
          <p>Numeric(15,4)</p>
        </td>
        <td>
          <p>Wert Einkauf</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>wbc_Verkauf_Wert</p>
        </td>
        <td>
          <p>Numeric(15,4)</p>
        </td>
        <td>
          <p>Wert Verkauf</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>wbc_BewArt</p>
        </td>
        <td>
          <p>smallint</p>
        </td>
        <td>
          <p>Siehe unten</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>wbc_BewCode</p>
        </td>
        <td>
          <p>smallint</p>
        </td>
        <td>
          <p>Siehe unten</p>
        </td>
      </tr>
    </tbody>
  </table>
</div>

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

<div class="table-wrapper">
  <table>
    <tbody>
      <tr>
        <td colspan="2">
          <p><strong>Bewegungscode</strong></p>
          <p>Der wbc_BewCode beschreibt die Herkunft der Warenbewegung. Während Codes kleiner 10 die eigentliche Tätigkeit beschreiben, kennzeichnen Codes zwischen 11 und 19 die jeweiligen Folgeschritte. Die Codes zwischen 21 und 29 sind für die Rückabwicklungen reserviert. Die Codes 10 und 20 stehen für Einkauf bzw. Verkauf.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>1</p>
        </td>
        <td>
          <p>Vorverkauf</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>2</p>
        </td>
        <td>
          <p>Voreinkauf</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>3</p>
        </td>
        <td>
          <p>Einlagerung</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>4</p>
        </td>
        <td>
          <p>Kommission</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>10</p>
        </td>
        <td>
          <p>Einkauf</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>11</p>
        </td>
        <td>
          <p>Vorverkauf Abholung</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>12</p>
        </td>
        <td>
          <p>Voreinkauf Anlieferung</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>13</p>
        </td>
        <td>
          <p>Einlagerung Vereinnahmung</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>14</p>
        </td>
        <td>
          <p>Kommission Verkauf</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>20</p>
        </td>
        <td>
          <p>Verkauf</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>21</p>
        </td>
        <td>
          <p>Vorverkauf Rücknahme</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>22</p>
        </td>
        <td>
          <p>Voreinkauf Rückgabe</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>23</p>
        </td>
        <td>
          <p>Einlagerung Abholung</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>24</p>
        </td>
        <td>
          <p>Kommission Rücknahme</p>
        </td>
      </tr>
    </tbody>
  </table>
</div>
