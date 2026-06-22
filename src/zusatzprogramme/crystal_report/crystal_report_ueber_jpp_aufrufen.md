# Crystal Report über JPP aufrufen

<!-- source: https://amic.de/hilfe/crystalreportberjppaufrufen.htm -->

Um einen Report programmgesteuert aufzurufen, existiert ein JPP Objekt mit dem Name **JAnwendReport.** Methoden, ohne die es nicht geht, sind fett geschrieben.

| Methode | Parameter | Bedeutung |
| --- | --- | --- |
| **Read** | m_AnwRptId | Die Reportdefinition des Reports mit der über m_AnwRptId angegebenen Ident wird gelesen. Liefert FALSE (0) wenn das Einlesen schiefgegangen ist. Muss als erste Anweisung erfolgen!  
 |
| **FeldFormat** | | Übergibt die Werte der Formelfelder an den Report.  
 |
| CreatViews | | Alle definierten Views werden angelegt  
 |
| SetFileName | Filename | Dateinamen überschreiben. Parameter ist FILENAME. Dieser enthält Pfad und Dateiname des Reports.  
 |
| SetPrinterByNumber | Printernumber | Holt sich anhand der Druckernummer den Drucker, auf dem der Report gedruckt werden soll  
 |
| GetSelectedPrinter | Feldname | Liefert den Drucker in das durch Feldname bezeichnete Feld zurück.  
 |
| SetVon | IDX | Überschreibt den Vonwert des Auswahlbereichs. IDX ist dabei der Index, der in der Spalte Idx des Auswahlbereichs steht.  
 |
| SetBis | IDX | Überschreibt den Biswert des Auswahlbereichs. IDX ist dabei der Index.  
 |
| SetWaehrung | Waehrung | Überschreibt die Währung, in der der Report ausgegeben wird. Dies gilt nur für bestimmte dafür vorgesehene Reporte.  
 |
| SetExportPfad | Exportpfad | Überschreibt das in den Stammdaten hinterlegte [Export-Verzeichnis](./crystal_report_definieren/basisdaten.md).  
 |
| **ListenStart** | | Startet den Report.  
 |
| Device | Siehe nächste Tabelle.  
 |
| NurArchivieren  
 | Der Parameter NurArchivieren ist optional. Gibt man hier eine 1 an, wird der Report nicht gedruckt, sondern sofort ins Archiv gestellt.  
 |
| ASK | Dieser Parameter gibt an, ob vor dem Druck der Drucker abgefragt werden soll. Gibt man 0 an, so erscheint die Druckerabfrage nicht.  
 |
| FA_Kundnummer | Kundennummer für das Formulararchiv.  
**HINWEIS:** *Wird dieser oder einer der folgenden drei Parameter angegeben, so werden die CRW-Archivdefinitionen nicht mehr ausgewertet*  
 |
| FA_Belegnummer | Belegnummer für das Formulararchiv.  
 |
| FA_Belegdatum | Belegdatum für das Formulararchiv.  
 |
| FA_Belegreferenz | Belegreferenz (Paginiernummer in der Fibu) für das Formulararchiv. Wenn leer, wird wie bisher die RRPTID eingetragen.  
 |

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

// Reportdefinition
laden

  if ( JPP_EX (
"CRW", "Read"  )== TRUE )
  {

//
// Report ist da und wurde
erfolgreich geladen
// Den Bereich eingrenzen.
Dazu mit SetVon und SetBis die von und Biswerte überschreiben
//

    call JPP_IN ("CRW", "VON",
":h.AnKaInventarNummer$" )
    call JPP_IN ("CRW", "IDX",
"1" )
    call JPP_EX ("CRW", "SetVon" )

    call JPP_IN ("CRW", "VON",
"0" )
    call JPP_IN ("CRW", "IDX",
"2" )
    call JPP_EX ("CRW", "SetVon" )

    call JPP_IN ("CRW", "VON",
"0" )
    call JPP_IN ("CRW", "IDX",
"3" )
    call JPP_EX ("CRW", "SetVon" )

    call JPP_IN ("CRW", "VON",
"0" )
    call JPP_IN ("CRW", "IDX",
"4" )
    call JPP_EX ("CRW", "SetVon" )

    call JPP_IN ("CRW", "VON",
"0" )
    call JPP_IN ("CRW", "IDX",
"5" )
    call JPP_EX ("CRW", "SetVon" )

    call JPP_IN ("CRW", "BIS",
"99999999" )
    call JPP_IN ("CRW", "IDX",
"5" )
    call JPP_EX ("CRW", "SetBis" )

    call JPP_IN ("CRW", "VON",
"0" )
    call JPP_IN ("CRW", "IDX",
"6" )
    call JPP_EX ("CRW", "SetVon" )

    call JPP_IN ("CRW", "BIS",
"99999999" )
    call JPP_IN ("CRW", "IDX",
"6" )
    call JPP_EX ("CRW", "SetBis" )

// Alle Formelfelder wurden
an den Report übergeben

    call JPP_EX ( "CRW", "FeldFormat" ) //
Initialisierung aller Variablen VON[...] usw. laut Anwendcondition...

// Ausgabemedium setzen.
Pflichtangabe!
    call JPP_IN ("CRW", "Device", "WINDOW" )

// Und den Report
starten
    call JPP_EX ("CRW", "ListenStart")
  }
  call JPP_DEL("CRW" )
```
