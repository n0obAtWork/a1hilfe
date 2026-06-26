# Einrichtung eines openTRANS-Imports

<!-- source: https://amic.de/hilfe/_OTVorgimport_einr.htm -->

Externe Kommunikation > openTRANS > openTRANS

**Direktsprung [OT]**

In der Variante Importprofile finden Sie die Einstellungsmöglichkeiten für die Importe.

<div class="table-wrapper">
  <table>
    <tbody>
      <tr>
        <td colspan="2">
          <p><strong>Vorgangsimport-Profil</strong></p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Ident</p>
        </td>
        <td>
          <p>Fortlaufende Nummer zur internen Identifikation</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Quelle</p>
        </td>
        <td>
          <p>Textfeld zur Repräsentation der Quelle – dieses Feld wird nur für Datenanzeigen verwendet.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Aktiv</p>
        </td>
        <td>
          <p>Gibt an, ob dieses Profil beim Import von Dateien verwendet werden soll.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Pfad</p>
        </td>
        <td>
          <p>Dateipfad, der angibt, wo die zu importierenden Dateien zu finden sind.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Archivpfad</p>
        </td>
        <td>
          <p>Dateipfad, der angibt, wohin die importierten Dateien abzulegen sind. Ist diese Angabe leer, werden die Dateien nach der Verarbeitung gelöscht.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Lagernummer</p>
        </td>
        <td>
          <p>Nummer des Lagers, das als Vorgabe für den Import verwendet werden soll, wenn sich nicht durch die Verwendung eines Makros eine andere Semantik-basierte Lagernummer ergibt.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Kunde</p>
        </td>
        <td>
          <p>Kundennummer, die für die Interpretation der Artikelnummern und anderer Absenderspezifischen Bezeichner im Mapping verwendet werden soll, wenn sich nicht durch ein Makro eine andere Semantik-basierte Kundennummer ergibt.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Präprozessor-Makro</p>
        </td>
        <td>
          <p>C#-Makro, das der Interpretation der zu importierenden Daten dient.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Postprozessor-Makro</p>
        </td>
        <td>
          <p>C#-Makro, das nach dem erfolgreichen Import aufgerufen wird.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Stylesheets</p>
        </td>
        <td>
          <p>Liste von Stylesheets, deren Anwendung für den Import der Dateien dieses Profils in Frage kommt.</p>
          <p>So können z.B. Bestellungen eines externen Systems in openTRANS-Aufträge, Rechnungen in Rechnungen und Lieferavise in Bestellungen gewandelt werden.</p>
        </td>
      </tr>
    </tbody>
  </table>
</div>
