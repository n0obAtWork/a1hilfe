# Container

<!-- source: https://amic.de/hilfe/_container.htm -->

Siehe auch: [Container einrichten](./einrichten_eines_containers.md)

„Die Archivfunktion ist jetzt auf ein Containermodell umgestellt worden. Ein Archiv kann aus beliebig vielen Containern bestehen, die in schreibgeschützter Form zur Anzeige eingebunden sein müssen.“

<div class="table-wrapper">
  <table>
    <tbody>
      <tr>
        <td></td>
        <td></td>
        <td></td>
      </tr>
      <tr>
        <td>
          <p>Container</p>
        </td>
        <td>
          <p>Pflichtfeld</p>
        </td>
        <td>
          <p>Eindeutiger Name des Containers.</p>
          <p>Container die keine Datenbank-Relationen im Aeins sind, dürfen nicht den Namen einer eben solchen haben.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Abstufung</p>
        </td>
        <td>
          <p>Pflichtfeld</p>
        </td>
        <td>
          <p>Die Abstufung impliziert ein Rangsystem und kann z.B. dazu verwendet werden, eine Recherche-Reihenfolge vorzugeben.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Status</p>
        </td>
        <td>
          <p>Information</p>
        </td>
        <td>
          <p>Im Falle von Datenbank-Relationen wird die momentane Verfügbarkeit ermittelt.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Verfügbarkeit</p>
        </td>
        <td>
          <p>Information</p>
        </td>
        <td>
          <p>Gibt Auskunft über den Status im Falle einer Datenbank-Relation.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Datenbank-Recherche</p>
        </td>
        <td>
          <p>Optional</p>
        </td>
        <td>
          <p>Anbindung einer privaten Datenbank-Funktion in der die Container-behandlung abgewickelt wird.</p>
          <p>Hier ist z.B. frei definierbar, ob und wie die Namen der hinterlegten Container und Abstufungen behandelt werden.</p>
          <p>Erste Schritte lassen sich mit Hilfe des AMIC-Templates „p_ArchivContainer“ erzielen; dieses wird über die Funktion „recherche-Funktion …“ bei leerem Datenbank-Recherche-Feld einmalig im System initiiert. Das Template ist als Vorschlag und erstes Grundgerüst für private Recherchen zu verstehen.</p>
          <p>Die Datenbank-Recherche-Funktionen werden von A.eins im Rahmen der „Archiv anzeigen“-Funktionen aufgerufen, sobald A.eins mit internen Mitteln kein Dokument in der Relation Archiv finden kann. Archiv-Dokumente werden anhand ihrer GUID identifiziert. Die Rückgabe der Datenbank-Recherche muss die folgenden Felder enthalten:</p>
          <p>archiv_status:0 = OK, 1=Information, 2=Error, 3=keine Reaktion</p>
          <p>archiv_blob: das recherchierte Dokument</p>
          <p>archiv_message: ggf. User-Information, die via archiv_stati 2,3 zugestellt werden kann.</p>
        </td>
      </tr>
    </tbody>
  </table>
</div>
