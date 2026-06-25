# openTRANS-Import

<!-- source: https://amic.de/hilfe/_info_otimport.htm -->

Der Import von openTRANS ist vielfältig. Deshalb wird dieser durch Makros individuell gestaltet. Das AddIn Thebe bereitet lediglich die Bearbeitung vor. Im Folgenden ist die Einrichtung und die Verarbeitung beschrieben.

###### Einrichtung

Warenverkauf > openTRANS Import > Variante „Profile für den Importbereich“

<div class="table-wrapper">
  <table>
    <tbody>
      <tr>
        <td colspan="2">
          <p><strong>Pfleger des openTRANS-Importprofils</strong></p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Profilname</p>
        </td>
        <td>
          <p>Name des Profile</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Klasse</p>
        </td>
        <td>
          <p>Vorgangsklasse</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Unterklasse</p>
        </td>
        <td>
          <p>Vorgangsunterklasse</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Absendertyp</p>
        </td>
        <td>
          <p>Welchen Typ hat der im Dokument beschriebene Absender</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Importlager</p>
        </td>
        <td>
          <p>Lager auf das importiert wird</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Makro</p>
        </td>
        <td>
          <p>Makro, das zum Import aufgerufen wird Das Makro wird von einer Funktion in der Variante „Dokumentenverarbeitung“ benutzt. (siehe <a href="./opentrans_import.md#EAI_OpenTRANS_Thebe_Import_Makro">Import per Makro</a>)</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Gebinde nicht exportieren</p>
        </td>
        <td></td>
      </tr>
    </tbody>
  </table>
</div>

Beim Speichern eines neuen Profils wird eine Funktion in der Optionbox der Variante „Dokumentenverarbeitung“ angelegt, die den Import von Dokumenten mit Hilfe dieses Profils startet.

###### Verarbeitung

<p class="just-emphasize">Quellen</p>

Die openTRANS-Dokumente können aus verschiedenen Quellen gewonnen werden. Je nach Typ können sie erst nach Extrakt oder sofort weiterverarbeitet werden. In jedem Fall werden die Dateien zunächst ins Formulararchiv importiert und mit einer entsprechenden Belegklasse versehen.

<div class="table-wrapper">
  <table>
    <tbody>
      <tr>
        <td colspan="3">
          <p><strong>Belegklassen für openTRANS im Formulararchiv</strong></p>
        </td>
      </tr>
      <tr>
        <td>
          <p><strong>Typ</strong></p>
        </td>
        <td>
          <p><strong>Belegklasse</strong></p>
        </td>
        <td>
          <p><strong>Verarbeitung</strong></p>
        </td>
      </tr>
      <tr>
        <td>
          <p>E-Mail mit Anhang</p>
        </td>
        <td rowspan="2">
          <p>8031 – openTRANS unbearbeitet</p>
        </td>
        <td rowspan="2">
          <p>Mit Extraktion</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>PDF mit openTRANS-Anhang</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>openTRANS-XML-Datei</p>
        </td>
        <td>
          <p>8032 – openTRANS extrahiert</p>
        </td>
        <td></td>
      </tr>
    </tbody>
  </table>
</div>

Warenverkauf > openTRANS Import > Variante „Dokumentenverarbeitung“

<p class="just-emphasize">Schritt 1 : Extraktion</p>

Beginnen Sie zunächst mit dem Status „extrahierbare“ Dokumente.

Hier sehen Sie im Formulararchiv abgelegte (importierte Dokumente) mit der Vorgangsklasse „8031 – openTRANS unbearbeitet“. Dies können eMails oder PDF-Dokumente mit eingeschlossenen Anhängen sein. Diese müssen zunächst ins Archiv extrahiert werden.

Verwenden Sie die Funktion „Extrahieren“, um die markierten Einträge zu entpacken.

<p class="just-emphasize">Schritt 2 : Import per Makro</p>

Wechseln Sie nun in den Status „zur Verarbeitung“ anstehende Dokumente

Hier sehen Sie im Formulararchiv abgelegte (importierte Dokumente) mit der Vorgangsklasse „8032 – openTRANS extrahiert“.

Dies können extrahierte E-Mails, PDF-Dokumente und deren eingeschlossenen Anhänge sein.

Alle extrahierten Anhänge heben die gleiche Archivreferenz, wie das Ursprungsdokument.

Verwenden Sie die Funktion „Import ….“, um das zum jeweiligen Profil gehörenden Makro zu starten.

Hinweis:

Es werden alle Dokumente der gleichen Belegreferenz importiert !

<p class="just-emphasize">Ansicht verarbeiteter Dokumente</p>

Wechseln Sie nun in den Status „verarbeitet“ .

Hier sehen Sie im Formulararchiv abgelegte (importierte Dokumente) mit der Vorgangsklasse „8033 – openTRANS verarbeitet“.

Hier werden alle Einträge aus dem Formulararchiv angezeigt, die mit Hilfe des Importmakros bearbeitet wurden.
