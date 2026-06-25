# Crystal Report über JPP aufrufen

<!-- source: https://amic.de/hilfe/crystalreportberjppaufrufen.htm -->

Um einen Report programmgesteuert aufzurufen, existiert ein JPP Objekt mit dem Name <strong>JAnwendReport.</strong> Methoden, ohne die es nicht geht, sind fett geschrieben.

<div class="table-wrapper">
  <table>
    <tbody>
      <tr>
        <td>
          <p><strong>Methode</strong></p>
        </td>
        <td>
          <p><strong>Parameter</strong></p>
        </td>
        <td>
          <p><strong>Bedeutung</strong></p>
        </td>
      </tr>
      <tr>
        <td>
          <p><b>Read</b></p>
        </td>
        <td>
          <p>m_AnwRptId</p>
        </td>
        <td>
          <p>Die Reportdefinition des Reports mit der über m_AnwRptId angegebenen Ident wird gelesen. Liefert FALSE (0) wenn das Einlesen schiefgegangen ist. Muss als erste Anweisung erfolgen!</p>
        </td>
      </tr>
      <tr>
        <td>
          <p><b>FeldFormat</b></p>
        </td>
        <td></td>
        <td>
          <p>Übergibt die Werte der Formelfelder an den Report.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>CreatViews</p>
        </td>
        <td></td>
        <td>
          <p>Alle definierten Views werden angelegt</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>SetFileName</p>
        </td>
        <td>
          <p>Filename</p>
        </td>
        <td>
          <p>Dateinamen überschreiben. Parameter ist FILENAME. Dieser enthält Pfad und Dateiname des Reports.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>SetPrinterByNumber</p>
        </td>
        <td>
          <p>Printernumber</p>
        </td>
        <td>
          <p>Holt sich anhand der Druckernummer den Drucker, auf dem der Report gedruckt werden soll</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>GetSelectedPrinter</p>
        </td>
        <td>
          <p>Feldname</p>
        </td>
        <td>
          <p>Liefert den Drucker in das durch Feldname bezeichnete Feld zurück.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>SetVon</p>
        </td>
        <td>
          <p>IDX</p>
        </td>
        <td>
          <p>Überschreibt den Vonwert des Auswahlbereichs. IDX ist dabei der Index, der in der Spalte Idx des Auswahlbereichs steht.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>SetBis</p>
        </td>
        <td>
          <p>IDX</p>
        </td>
        <td>
          <p>Überschreibt den Biswert des Auswahlbereichs. IDX ist dabei der Index.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>SetWaehrung</p>
        </td>
        <td>
          <p>Waehrung</p>
        </td>
        <td>
          <p>Überschreibt die Währung, in der der Report ausgegeben wird. Dies gilt nur für bestimmte dafür vorgesehene Reporte.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>SetExportPfad</p>
        </td>
        <td>
          <p>Exportpfad</p>
        </td>
        <td>
          <p>Überschreibt das in den Stammdaten hinterlegte <a href="./crystal_report_definieren/basisdaten.md">Export-Verzeichnis</a>.</p>
        </td>
      </tr>
      <tr>
        <td rowspan="8">
          <p><b>ListenStart</b></p>
        </td>
        <td></td>
        <td>
          <p>Startet den Report.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Device</p>
        </td>
        <td>
          <p>Siehe nächste Tabelle.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>NurArchivieren</p>
        </td>
        <td>
          <p>Der Parameter NurArchivieren ist optional. Gibt man hier eine 1 an, wird der Report nicht gedruckt, sondern sofort ins Archiv gestellt.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>ASK</p>
        </td>
        <td>
          <p>Dieser Parameter gibt an, ob vor dem Druck der Drucker abgefragt werden soll. Gibt man 0 an, so erscheint die Druckerabfrage nicht.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>FA_Kundnummer</p>
        </td>
        <td>
          <p>Kundennummer für das Formulararchiv.</p>
          <p><b>HINWEIS:</b> <i>Wird dieser oder einer der folgenden drei Parameter angegeben, so werden die CRW-Archivdefinitionen nicht mehr ausgewertet</i><i></i></p>
        </td>
      </tr>
      <tr>
        <td>
          <p>FA_Belegnummer</p>
        </td>
        <td>
          <p>Belegnummer für das Formulararchiv.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>FA_Belegdatum</p>
        </td>
        <td>
          <p>Belegdatum für das Formulararchiv.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>FA_Belegreferenz</p>
        </td>
        <td>
          <p>Belegreferenz (Paginiernummer in der Fibu) für das Formulararchiv. Wenn leer, wird wie bisher die RRPTID eingetragen.</p>
        </td>
      </tr>
    </tbody>
  </table>
</div>

Folgende Ausgabemöglichkeiten (Device) können bei Listenstart angegeben werden:

| Device | Bedeutung |
| --- | --- |
| HTMLEXPORT | Ausgabe im HTML-Format auf das im Exportpfad hinterlegte Verzeichnis. |
| TEXTEXPORT | Ausgabe im Text-Format auf das im Exportpfad hinterlegte Verzeichnis. |
| CSVEXPORT | Ausgabe im CSV-Format auf das im Exportpfad hinterlegte Verzeichnis. |
| EXCELEXPORT | Ausgabe im Excel-Format auf das im Exportpfad hinterlegte Verzeichnis. |
| PDFEXPORT | Ausgabe im PDF-Format auf das im Exportpfad hinterlegte Verzeichnis. |
| WORDEXPORT | Ausgabe im Word-Format auf das im Exportpfad hinterlegte Verzeichnis. |
| PRINTER | Ausgabe auf den Drucker. |
| WINDOW | Ausgabe in einem Fenster ohne die Möglichkeit einer Bereichseingrenzung. |
| ACTIVEX | Dieser Aufruf ist für den Gebrauch des bekannten Vorschaumodus mit allen seinen Funktionalitäten vorgesehen. Er ruft nicht das Vorschaufenster auf. Programminterner gebraucht. |

Beispiel (JPL-Syntax):

```text
call JPP_NEW( "CRW", "JAnwendReport" )
  call JPP_IN ( "CRW", "m_AnwRptId", "ANKASTAMMBLATT"  )
// Reportdefinition laden
  if ( JPP_EX ( "CRW", "Read"  )== TRUE )
  {
//
// Report ist da und wurde erfolgreich geladen
// Den Bereich eingrenzen. Dazu mit SetVon und SetBis die von und Biswerte überschreiben
//
    call JPP_IN ("CRW", "VON", ":h.AnKaInventarNummer$" )
    call JPP_IN ("CRW", "IDX", "1" )
    call JPP_EX ("CRW", "SetVon" )
    call JPP_IN ("CRW", "VON", "0" )
    call JPP_IN ("CRW", "IDX", "2" )
    call JPP_EX ("CRW", "SetVon" )
    call JPP_IN ("CRW", "VON", "0" )
    call JPP_IN ("CRW", "IDX", "3" )
    call JPP_EX ("CRW", "SetVon" )
    call JPP_IN ("CRW", "VON", "0" )
    call JPP_IN ("CRW", "IDX", "4" )
    call JPP_EX ("CRW", "SetVon" )
    call JPP_IN ("CRW", "VON", "0" )
    call JPP_IN ("CRW", "IDX", "5" )
    call JPP_EX ("CRW", "SetVon" )
    call JPP_IN ("CRW", "BIS", "99999999" )
    call JPP_IN ("CRW", "IDX", "5" )
    call JPP_EX ("CRW", "SetBis" )
    call JPP_IN ("CRW", "VON", "0" )
    call JPP_IN ("CRW", "IDX", "6" )
    call JPP_EX ("CRW", "SetVon" )
    call JPP_IN ("CRW", "BIS", "99999999" )
    call JPP_IN ("CRW", "IDX", "6" )
    call JPP_EX ("CRW", "SetBis" )
// Alle Formelfelder wurden an den Report übergeben
    call JPP_EX ( "CRW", "FeldFormat" ) // Initialisierung aller Variablen VON[...] usw. laut Anwendcondition...
// Ausgabemedium setzen. Pflichtangabe!
    call JPP_IN ("CRW", "Device", "WINDOW" )
// Und den Report starten
    call JPP_EX ("CRW", "ListenStart")
  }
  call JPP_DEL("CRW" )
```
