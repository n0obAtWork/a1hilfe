# Optionen (F10)

<!-- source: https://amic.de/hilfe/optionenf10.htm -->

Wenn man unter OSQL die Funktion ***Optionen*** **F10** auswählt, so öffnet sich folgender Dialog mit zwei Reitern:

#### Anwendung:

 ![](../../ImagesExt/image8_1456.png)

<div class="table-wrapper">
  <table>
    <tbody>
      <tr>
        <td></td>
        <td></td>
      </tr>
      <tr>
        <td>
          <p>(F3) Arbeitsverzeichnis</p>
          <p>(F3) Dateinamenserweiterung</p>
          <p>(F3) Zuletzt verwendete Datei</p>
        </td>
        <td>
          <p>Diese Einstellungen beziehen sich auf die Dialogmaske, die man über die Funktion <strong><em>SQL ausführen</em></strong> <strong>F3 </strong>erreicht.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>(F9) Arbeitsverzeichnis</p>
          <p>(F9) Datei</p>
          <p>(F9) Dateinamenserweiterung</p>
        </td>
        <td>
          <p>Diese Einstellungen beziehen sich auf die Dialogmaske, die man über die Funktionen <strong><em>Sichern Eingabe </em>SCF9, <em>Ausführen Statement</em> CF9 </strong>und<strong> <em>Editieren Statement</em> SF9 </strong>erreicht.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Ausgabedatei</p>
        </td>
        <td>
          <p>Dieser Dateiname wird dort als Vorbelegung verwendet, wo OSQL Daten&nbsp; in eine Datei schreiben soll.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Bei TAB Tabellennamen ergänzen</p>
        </td>
        <td>
          <p>Es wird, wenn man die TAB-Taste drückt, der nächste Tabellenname – bei Shift-TAB der vorherige – ergänzt.</p>
          <p>Beispiel:</p>
          <div>
            <code>Select * from Waehr&lt;TAB&gt;</code>
          </div>
          <p>Ergibt</p>
          <div>
            <code>Select * from WaehrIsoList</code>
          </div>
          <p>Beim erneuten drücken von Tab</p>
          <div>
            <code>Select * from WaehrUmrechInfo</code>
          </div>
        </td>
      </tr>
      <tr>
        <td>
          <p>Dateityp ….</p>
        </td>
        <td>
          <p>Die Dateitypen, die in den Dateiauswahldialogen angeboten werden sollen. Der Syntax dazu ist wie folgt:</p>
          <p>("Text", "Suchkriterium")</p>
          <p>Wobei der Text die Beschreibung enthält, wie z.B. „SQL-Skript (*.SQL)“. Der gesamte Test müsste folgendermaßen lauten:</p>
          <p>(SQL-Skript (*.SQL)“, „*.sql“)</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Trotz Fehler Skripte weiter ausführen</p>
        </td>
        <td>
          <p>Im Normalfall werden SQL-Skripte trotz Fehler weiter ausgeführt. Trägt man hier FALSE ein, so wird die Ausführung des Skripts sofort beendet.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Fehlermeldungen</p>
        </td>
        <td>
          <p>Man kann OSQL dazu bringen, dass keine Fehlermeldungen angezeigt werden. Dazu trägt man hier den Wert FALSE ein.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Warnungen anzeigen</p>
        </td>
        <td>
          <p>Sybase unterscheidet zwischen Fehlern und Warnungen. Warnungen der Datenbank werden von A.eins im Normalfall nicht ausgegeben. Unter OSQL werden Warnungen jedoch angezeigt (TRUE). Stellt man hier FALSE ein, so verhält sich OSQL wie der Rest des Programms.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Spaltentitel</p>
        </td>
        <td>
          <p>TRUE =&gt; Es wird immer der gesamte Spaltentitel angezeigt und die Spalte ggf. verbreitert.</p>
          <p>FALSE =&gt; Die Spalte wird in der Standardbreite des Feldes angezeigt. Will man hier den kompletten Spaltentitel sehen, kann man auf den Titel klicken und die Spalte wird dann ggf. verbreitert.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Vollbild (ZOOM)</p>
        </td>
        <td>
          <p>Hiermit wird voreingestellt, ob die Maske den gesamten Bildschirm ausfüllt (TRUE) oder nur einen Teil des Bildschirms(FALSE)</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Max. Zeilen<br>(0=Bildschirm füllen,<br>-1=alles Lesen)</p>
        </td>
        <td>
          <p>Hier existieren zwei Modi:</p>
          <p>0&nbsp;&nbsp;&nbsp; Es wird immer nur der Bildschirm gefüllt. Wenn man mehr Daten haben will, so muss man mit den Blättern-Tasten „Bild rauf“ und „Bild runter“ weiterblättern. Man erkennt, ob man alle Daten gelesen hat, an der Statuszeile. Dort steht dann „Gelesene Datensätze 99999“.</p>
          <p>-1&nbsp;&nbsp; Es werden alle Daten geladen. Während der Ladephase kann bereits das nächste Stamement erfasst werden. Sobald eine Funktion aufgerufen wird, bricht das laden ab.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Max. Spalten (bis 200).</p>
        </td>
        <td>
          <p>Anzahl der Spalten die angezeigt werden. Wenn nicht bereits anders eingestellt, wird hier 49 vorgeschlagen.</p>
        </td>
      </tr>
    </tbody>
  </table>
</div>

#### Datenbank:

Hier werden die Optionen der Datenbank angezeigt. Ein Ändern der Werte ist nicht möglich. Lesen sie dazu die Sybase Dokumentation zu „Database Options“

![](../../ImagesExt/image8_1457.png)
