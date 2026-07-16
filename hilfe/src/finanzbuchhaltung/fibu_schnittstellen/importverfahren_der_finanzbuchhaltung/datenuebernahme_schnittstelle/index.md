# Datenübernahme-Schnittstelle

<!-- source: https://amic.de/hilfe/datenbernahmeschnittstelle.htm -->

Hauptmenü > Abschlussarbeiten > DATEV / Import / Export > Datenübernahme

Direktsprung **[DUEB]**

Diese Schnittstelle ist eine allgemeine technische Lösung um Dateien gesichert einzulesen. In dieser Auswahlliste kann man die Schnittstelle definieren und ausführen.

#### Einrichtung

![Ein Bild, das Text, Screenshot, Software, Computersymbol enthält. Automatisch generierte Beschreibung](../../../../ImagesExt/image8_780.png "Ein Bild, das Text, Screenshot, Software, Computersymbol enthält. Automatisch generierte Beschreibung")

<div class="table-wrapper">
  <table>
    <tbody>
      <tr>
        <td>
          <p><strong>Feld</strong></p>
        </td>
        <td colspan="2">
          <p><strong>Besonderheiten</strong></p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Name</p>
        </td>
        <td>
          <p>Dies ist die eindeutige Bezeichnung des Übernahmeverfahrens, über die dann auf die Definition zugegriffen wird.</p>
        </td>
        <td></td>
      </tr>
      <tr>
        <td>
          <p>Verzeichnisse</p>
        </td>
        <td>
          <p>Es müssen vier Verzeichnisse eingetragen werden:</p>
          <p><b>In: </b>&nbsp;Die einzulesenden Dateien werden in dieses Verzeichnis gestellt. Der Name der Dateien ist dabei nicht wichtig, da alle Dateien in dem Verzeichnis verarbeitet werden. Er kann aber unter „Eingrenzung“ näher definiert werden.</p>
          <p><b>Run:</b> Wenn eine Datei in Arbeit ist, steht sie in diesem Verzeichnis.</p>
          <p><b>Done: </b>Ist<b> </b>die Verarbeitung Fehlerfrei abgelaufen, dann wird die Datei in dieses Verzeichnis verschoben.</p>
          <p><b>Fail: </b>Im Fehlerfall kommt die Datei in dieses Verzeichnis.</p>
          <p>Existieren die Verzeichnisse noch nicht, wird vom Programm versucht diese anzulegen.</p>
        </td>
        <td></td>
      </tr>
      <tr>
        <td>
          <p>Programm 1</p>
        </td>
        <td>
          <p>Dieses Programm ist eine JPL-Funktion. Wenn man als Parameter %F angibt, so wird dort der Dateiname übergeben. Die Funktion muss den Wert 0 (S_OK) zurückliefern, wenn die Verarbeitung fehlerfrei war. Ein Wert größer 0 beendet das Einlesen aller Dateien, egal ob noch Dateien im <b>In-</b>Verzeichnis stehen oder nicht. Ein Wert kleiner 0 beendet nur das Einspielen der aktuellen Datei.</p>
          <p>Von AMIC stehen mehrere Programme bereit, die hier per F3 ausgewählt werden können.</p>
        </td>
        <td></td>
      </tr>
      <tr>
        <td>
          <p>Programm 2</p>
        </td>
        <td>
          <p>Eine zweite optionale Prozedur, die nur aufgerufen wird, wenn die Funktion unter Progamm 1 fehlerfrei ausgeführt wurde. Die Funktion muss den Wert 0 (S_OK) zurückliefern, wenn die Verarbeitung fehlerfrei war. Ein Wert ungleich 0 bricht nur die Verarbeitung dieser Daten ab.</p>
        </td>
        <td></td>
      </tr>
      <tr>
        <td>
          <p>Prozedur</p>
        </td>
        <td>
          <p>Für den <a href="./fibu_csv_import.md">CSV-Import</a> und den <a href="./fibu_xlsx_import.md">XLSX-Import</a> müssen hier private Prozeduren festgelegt werden. Für den <a href="./fibu_xml_import.md">Fibu-XML-Import</a> ist eine Prozedur optional. Wird beim Fibu-XML-Import keine Prozedur angegeben, dann wird die Standardprozedur von AMIC verwendet.</p>
        </td>
        <td></td>
      </tr>
      <tr>
        <td>
          <p>Eintrag ins Fehlerprotokoll</p>
        </td>
        <td>
          <p>Dieser Wert wird nicht vom System ausgewertet, kann aber von den JPL-Funktionen verwendet werden.</p>
        </td>
        <td></td>
      </tr>
      <tr>
        <td>
          <p>Bei Fehler Ausführung blockieren</p>
        </td>
        <td>
          <p>Dieser Wert steht standardmäßig auf <b>Ja</b>. Dies bewirkt, dass eine Einspielung weiterer Dateien nur möglich ist, wenn das Fail-Verzeichnis leer ist.</p>
        </td>
        <td></td>
      </tr>
      <tr>
        <td>
          <p>Appendix im Fehlerfall</p>
        </td>
        <td>
          <p>Optional. Die Datei, die in das <b>Fail</b>-Verzeichnis geschrieben wird, bekommt diese Dateierweiterung.</p>
        </td>
        <td></td>
      </tr>
      <tr>
        <td>
          <p>Aktiv-Signalisierung</p>
        </td>
        <td>
          <p>Optional. Wird hier ein Dateiname angegeben (Vorbelegung ist „import.act“), wird bei aktivem Import diese Datei ins <b>In</b>-Verzeichnis geschrieben.</p>
        </td>
        <td></td>
      </tr>
      <tr>
        <td>
          <p>Eingrenzung</p>
        </td>
        <td>
          <p>Hier können die einzulesenden Dateien eingegrenzt werden. Wird *.* angegeben, so wird versucht alle Dateien im <b>In</b>-Verzeichnis zu verarbeiten.</p>
        </td>
        <td></td>
      </tr>
      <tr>
        <td>
          <p>Fehlerhafte Daten überschreiben?</p>
        </td>
        <td>
          <p>Mit dieser Option kann eingestellt werden, ob bei der Datenübernahme fehlerhafte Datensätze überschrieben werden dürfen.</p>
          <p><u>Voraussetzungen:</u><u></u></p>
          <p>1)&nbsp;&nbsp; Diese Option ist nur einrichtbar, wenn es sich um den <a href="./fibu_xml_import.md">FiBu-XML-Import</a> handelt. Für alle anderen Importe ist ein erneutes Einspielen nicht möglich.</p>
          <p>2)&nbsp;&nbsp; Es kann nur der Import von fehlerhaften Datensätzen wiederholt werden. Wurde ein Datei bereits erfolgreich eingespielt, so ist ein erneuter Import nicht möglich.</p>
          <table>
            <tbody>
              <tr>
                <th>Wert</th>
                <th>Beschreibung</th>
              </tr>
              <tr>
                <td>Nein</td>
                <td>Ein erneutes Einspielen ist nicht möglich (Standardwert).</td>
              </tr>
              <tr>
                <td>Ja, mit Abfrage<br>(bei Automatik: Nein)</td>
                <td>Es wird beim Starten der Datenübernahme abgefragt, ob die Datei erneut eingespielt werden soll.<br><u>Hinweis:</u><br>Handelt es sich um eine automatische Einspielung, so ist unter dieser Einstellung eine erneute Einspielung nicht möglich. Stattdessen ist die Einstellung „Immer zulassen“ zu wählen.</td>
              </tr>
              <tr>
                <td>Immer zulassen</td>
                <td>Fehlerhaften Daten werden überschrieben, ohne dass eine Abfrage erscheint.</td>
              </tr>
            </tbody>
          </table>
        </td>
        <td></td>
      </tr>
      <tr>
        <td>
          <p>Schemalocation</p>
        </td>
        <td>
          <p>Beim Import von <u>XML-Dateien</u> wird der Aufbau der Datei gegen eine Schemadefinition validiert. Ist die Schemalocation nicht in der XML-Datei selbst angegeben, hat man hier die Möglichkeit diese anzugeben.</p>
        </td>
        <td></td>
      </tr>
    </tbody>
  </table>
</div>

#### Verarbeitung

Hat man in der Auswahlliste ein definiertes Verfahren ausgewählt, kann man mit der Funktion ***Starten*** **F9** die Datenübernahme starten.

![](../../../../ImagesExt/image8_781.png)

In der Spalte ganz rechts steht die Anzahl der Dateien, die sich in den Verzeichnissen befinden. Wenn die Option „Bei Fehler Ausführung blockieren“ gesetzt ist, muss das Verzeichnis **Fail** leer sein, damit der Import starten kann.  
    
Die Datenübernahme kann auch mit dem [Aufgabenplaner](./automation_aufgabenplaner.md) automatisiert werden.

<p class="siehe-auch">Siehe auch:</p>

- [Automation/Aufgabenplaner](./automation_aufgabenplaner.md)
- [FIBU-XML-Import](./fibu_xml_import.md)
- [FIBU-CSV-Import](./fibu_csv_import.md)
- [FIBU-XLSX-Import](./fibu_xlsx_import.md)
- [Resultset der FIBU-Datenübernahme](./resultset_der_fibu_datenuebernahme.md)
