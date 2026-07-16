# PDF-Drucken

<!-- source: https://amic.de/hilfe/pdfdrucken.htm -->

Diese Funktion ist über [Dokumentenverwaltung- Multifunktionsleiste](./index.md) und über die Anwendung

[Anwendung Formulararchiv](../../archiv_administration/anwendung_formulararchiv/index.md) verfügbar.

Nach Auswahl von Dokumenten werden die PDF-Dokumente gefiltert und zum Druck angeboten.

<div class="table-wrapper">
  <table>
    <tbody>
      <tr>
        <td colspan="3">
          <p><strong>Felder:</strong></p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Drucker</p>
        </td>
        <td>
          <p>Pflichtfeld</p>
        </td>
        <td>
          <p>Angabe des Windows-Druckers</p>
          <p>Die Online-Verfügbarkeit wird geprüft und optisch durch einen grünen Haken belegt.</p>
          <img src="../../../ImagesExt/image8_848.png" alt="">
          <p>Mausklick auf dieses Sysmbol oder betätigen der F3-Taste ruft den „Windows-Drucker-Auswahl-Dialog“ auf.</p>
          <p>Die Angabe des Druckers wird sich für den erneuten Aufruf gemerkt und ist bei Ersteintritt der Windows-Standard-Drucker.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>PDF-Dokumente</p>
        </td>
        <td>
          <p>Anzeige</p>
        </td>
        <td>
          <p>Anzahl der zu druckenden Dokumente</p>
        </td>
      </tr>
    </tbody>
  </table>
</div>

<div class="table-wrapper">
  <table>
    <tbody>
      <tr>
        <td colspan="3">
          <p><strong>Funktionen:</strong></p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Drucken</p>
        </td>
        <td>
          <p>F9</p>
        </td>
        <td>
          <p>Druckt die vorgesehenen Dokumente auf den ausgewählten Drucker</p>
        </td>
      </tr>
    </tbody>
  </table>
</div>

Der PDF-Druck ist programmatisch durchführbar.

1) Für Makro2 siehe IArchiv.PrintPDF.

2) Für andere Scriptsprachen steht die JPP-Methode „PrintPdf“ im JPP-Objekt „JFA_View“ zur Verfügung.

<div class="table-wrapper">
  <table>
    <tbody>
      <tr>
        <td colspan="3">
          <p><strong>Parameter:</strong></p>
        </td>
      </tr>
      <tr>
        <td>
          <p>fa_id</p>
        </td>
        <td></td>
        <td>
          <p>Schlüssel</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>fa_mndnr</p>
        </td>
        <td></td>
        <td>
          <p>Schlüssel (Angabe optional)</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>printer</p>
        </td>
        <td></td>
        <td>
          <p>Drucker</p>
        </td>
      </tr>
    </tbody>
  </table>
</div>
