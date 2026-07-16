# Archiv-Stammdatenpfleger

<!-- source: https://amic.de/hilfe/_archivstammdatenpfle.htm -->

Sie erreichen den Stammdaten-Pfleger in den Anwendungen und Varianten des Archivs. Außerdem ist er über die „**Archiv anzeigen**“ verfügbar.

<div class="table-wrapper">
  <table>
    <tbody>
      <tr>
        <td colspan="2">
          <p><strong>Felder</strong></p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Beleg-Referenz</p>
        </td>
        <td>
          <p>Referenz</p>
          <p>Die Referenz stellt eine Art „Klammer“ dar, die über das Archiv hinweg „zusammengehörige“ Dokumente strukturiert.</p>
          <p>Sie wird bei der Neuanlage von Objekten (z.B. Vorgänge, Artikel, usw.) mittels der „Archiv-Fakte“ ermittelt und im Laufe der Operationen – wie zum Beispiel beim Archivieren von Drucken – dem so entstandenen Dokument im Archiv zugeordnet. Die „Archiv anzeigen“-Methodiken erlauben dann in diesem Kontext diese Archiv-Einträge zu recherchieren.</p>
          <p>Grundsätzlich ist aber hier die „Beleg-Referenz“ frei wählbar.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Belegnummer</p>
        </td>
        <td>
          <p>Eine Beleg-Nummer.</p>
          <p>Diese wird standardmäßig beim Druck von Vorgängen die Beleg-Nummer sein, es können aber je nach Kontext auch z.B. externe Beleg-Nummern sein.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Belegdatum</p>
        </td>
        <td>
          <p>Das Beleg-Datum.</p>
          <p>Im Falle von Hinzufügungen (Belegklasse „Hinzufügung“, 9003) kann dieses Datum geändert werden.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Kundennummer</p>
        </td>
        <td>
          <p>Die zugeordnete Kundennummer.</p>
          <p>In aller Regel eine A.eins-Kundennummer.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Belegklasse</p>
        </td>
        <td>
          <p>Eine Dokument-Typisierung gemäß des AMIC-Formates FAKLASSE.</p>
          <p>Die Beleg-Klasse ergibt sich immer aus dem aktuellen Workflow und Entstehung des Archiv-Dokuments.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Kontierung</p>
        </td>
        <td>
          <p>Verarbeitungskennzeichen im Rahmen der Vorkontierung (in Entwicklung)</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Klassifizierung</p>
        </td>
        <td>
          <p>Auf Basis des Anwendungsformates AF_FA_KLASSE mögliche individuelle Klassifizierung eines Beleges.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Barcode</p>
        </td>
        <td>
          <p>Archiv Barcode</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Mandant</p>
        </td>
        <td>
          <p>Mandant</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Bedienerklasse</p>
        </td>
        <td>
          <p>Jedes Dokument ist einer Bedienerklasse zugeordnet.</p>
          <p>Bei Neuanlage ist es die Bedienerklasse des anlegenden Bedieners. Über das Sichtschutz-Konzept des Formulararchivs steuern Sie welche Bedienerklassen welche Dokumente anderer Bedienerklassen-Zuordnungen in den jeweiligen „Archiv anzeigen“-Auflistungen angezeigt bekommen.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Datei</p>
        </td>
        <td>
          <p>Im Hinzufügen-Modus ist hier der Pfad auf das hinzufügende Dokument erfassbar.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Archiv/Druckdatum</p>
        </td>
        <td>
          <p>Zeitpunkt der Archivierung bzw. des Druckes</p>
        </td>
      </tr>
      <tr>
        <td>
          <p><b>Info-Felder</b></p>
        </td>
        <td>
          <ul>
            <li>Name der Drucker-Queue auf dem das Dokument gedruckt wurde.</li>
            <li>Mime-Typ des Dokuments.</li>
            <li>Die dem Mime-Typ entsprechende Datei-Extension.</li>
            <li>Grösse des Dokumentes in Bytes.</li>
          </ul>
        </td>
      </tr>
      <tr>
        <td>
          <p>Belegtyp</p>
        </td>
        <td>
          <p>Ein freiwählbarer Text, bei bestimmten Abläufen finden sich hier unterstützende, durch die jeweiligen Systeme generierte Hinweise (z.B. beim Vorgangsdruck die Formularbezeichnung)</p>
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
          <p><strong>Datentabelle „Belegfluss“</strong></p>
        </td>
      </tr>
      <tr>
        <td>
          <p>angefordert</p>
        </td>
        <td>
          <p>Anforderungen an Belegfluss Dokumente</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>genehmigt</p>
        </td>
        <td>
          <p>Archivanforderungsgenehmigungen</p>
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
          <p><strong>Register „Allgemein“</strong></p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Betreff</p>
        </td>
        <td>
          <p>Optionale Zusatzinformation.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Kategorie</p>
        </td>
        <td>
          <p>Optionale Zusatzinformation.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Stichwörter</p>
        </td>
        <td>
          <p>Optionale Zusatzinformation.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Kommentar</p>
        </td>
        <td>
          <p>Optionale Zusatzinformation.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Titel</p>
        </td>
        <td>
          <p>Optionale Zusatzinformation.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Autor</p>
        </td>
        <td>
          <p>Optionale Zusatzinformation.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Dateiname</p>
        </td>
        <td>
          <p>Optionale Zusatzinformation.</p>
          <p>Bei der Angabe dieser Information wird diese beim Anzeigen eines Archiv-Dokumentes (PDF) in der Titelleiste mit angegeben.</p>
        </td>
      </tr>
      <tr>
        <td colspan="2">
          <p><strong>Register „Gruppe“</strong></p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Gruppentyp</p>
        </td>
        <td>
          <p>Formulararchiv-Gruppen</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Gruppentext</p>
        </td>
        <td>
          <p>Formulararchiv-Gruppen</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Gruppennr</p>
        </td>
        <td>
          <p>Formulararchiv-Gruppen</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Sortierung</p>
        </td>
        <td>
          <p>Formulararchiv-Gruppen</p>
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
          <p><strong>Funktionen</strong></p>
        </td>
      </tr>
      <tr>
        <td>
          <p><strong><em>Speichern F9</em></strong></p>
        </td>
        <td>
          <p>Speichern</p>
        </td>
      </tr>
      <tr>
        <td>
          <p><strong><em>Löschen F7</em></strong></p>
        </td>
        <td>
          <p>Löscht den Formulararchiv-Eintrag.</p>
          <p>Siehe auch <a href="./archiveintraege_loeschen.md">Archiveinträge löschen</a></p>
        </td>
      </tr>
      <tr>
        <td>
          <p><strong><em>Beleg-Referenz erzeugen F10</em></strong></p>
        </td>
        <td>
          <p>Spezial-Funktion</p>
          <p>Wenn möglich aus der Belegnummer und der Belegklasse unter Zuhilfenahme der <a href="./archiv_fakte.md">Archiv-Fakte</a> die Belegreferenz des dazu passenden Vorgangs ermitteln.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p><strong><em>Archiv anzeigen CF12</em></strong></p>
        </td>
        <td>
          <p>Öffnet hinterlegtes Dokument des Archives zur Ansicht.</p>
        </td>
      </tr>
    </tbody>
  </table>
</div>
