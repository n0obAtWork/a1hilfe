# Archiv-Fakte

<!-- source: https://amic.de/hilfe/_archivfakte.htm -->

Hauptmenü > Administration > Archiv > Administration > Archiv-Fakte

Direktsprung **[FAREF]**

Bestimmte Objekte in A.eins (z.B. Vorgänge, Kunden, Artikel, u.a.) haben feste Regeln hinterlegt, mit deren Hilfe sie automatisch Archiv-Referenznummern vorgeschlagen bekommen, wenn sie neu ins System kommen. Diese Objekte zusammen mit ihren Definitionen sind in A.eins die Archiv-Fakte.

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
          <p>Auslieferung</p>
        </td>
        <td>
          <p>Ja/Nein</p>
          <p>Gibt an, ob das vorliegende Archiv-Fakt von AMIC ausgeliefert wird.</p>
          <p>AMIC-Auslieferungen sind solche Archiv-Fakte die mit <b>fam_ref_ </b>beginnen.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Archiv-Fakte</p>
        </td>
        <td>
          <p>Eindeutiger Schlüsselbegriff der Archiv-Fakte.</p>
          <p>Es stehen maximal 30 Zeichen zur Verfügung.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Beschriftung</p>
        </td>
        <td>
          <p>Repräsentation des Archiv-Faktes in der Benutzeroberfläche.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>AMIC-Referenz</p>
        </td>
        <td>
          <p>Datenbank-Funktion zur Ermittlung der Referenz eines Archiv-Faktes.</p>
          <p>Die jeweilige Datenbank-Funktion die AMIC ausliefert ist als Beispiel zu sehen. Es kann durchaus sein, dass vor Ort individuelle Anpassungen durchgeführt werden müssen.</p>
          <p>Allerdings – so zeigt die Praxis – ist das selten von Nöten.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Privat-Referenz</p>
        </td>
        <td>
          <p>Die Möglichkeit die Funktionalität der AMIC-Referenz zu ersetzen.</p>
          <p>Wenn man hier deine existierende private Datenbank-Funktion einträgt, dann wird zur Ermittlung der Referenz genommen, und nicht die AMIC-Referenz!</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Relation</p>
        </td>
        <td>
          <p>Die Relation, in der die Kerndaten sowie die Archiv-Referenz des Archiv-Faktes gespeichert werden. Das ist ein Fakt.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Referenz-Spalte</p>
        </td>
        <td>
          <p>Der Name der Spalte der Relation, in der die Archiv-Referenz gespeichert wird. Das ist ein weiteres Fakt.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Aufruf-Parameter</p>
        </td>
        <td>
          <p>Die Parameter für den Aufruf der jeweiligen Datenbank-Funktion.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p><b>Ohne Referenz!</b></p>
        </td>
        <td>
          <p>Eine statistische Aussage darüber, wie viele Exemplare eines bestimmten Archiv-Faktes KEINE eingetragene Archiv-Referenz in der Referenz-Spalte der Relation hinterlegt haben.</p>
          <p>Dabei handelt es sich vornehmlich um Alt-Bestände vor Aktivierung der Archivierung in A.eins.</p>
          <p>Beispiel: Eine Angabe von 130000 im Archiv-Fakt „fam_ref_fibu“ bedeutet, dass es 130.000 Einträge in der Relation „fibuvorgstamm“ gibt, die keine eingetragene Archiv-Referenz in der Spalte „fibuv_paginiernr“ haben.</p>
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
          <p><strong>Suchkriterien</strong></p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Suchen</p>
        </td>
        <td>
          <p>Like</p>
          <p>Sucht in allen Feldern außer „Auslieferung“ und „Ohne Referenz!“ nach der Begrifflichkeit.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Referenz-Information</p>
        </td>
        <td>
          <p>Ja/Nein</p>
          <p>Such-Schalter, aktiviert bei Einstellung „Ja“ die Analyse, wie viele Archiv-Fakte ohne Archiv-Referenz ausgestattet sind und stellt die Information unter „<b>Ohne Referenz!</b>“ zur Verfügung.</p>
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
          <p><strong><em>Neu</em></strong> <strong>F8</strong></p>
        </td>
        <td>
          <p>Anlage eines neuen Archiv-Faktes. Es können private Archiv-Fakte definiert werden!</p>
          <p>Für Details siehe Archiv-Fakte-Pfleger.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p><strong><em>Ändern</em></strong> <strong>F5</strong></p>
        </td>
        <td>
          <p>Ändern eines Archiv-Faktes.</p>
          <p>Hiermit lässt sich die „Privat-Referenz“ einpflegen.</p>
          <p>Für Details siehe Archiv-Fakte-Pfleger.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p><strong><em>Ansehen</em></strong> <strong>F6</strong></p>
        </td>
        <td>
          <p>Ansehen eines Archiv-Faktes.</p>
          <p>Für Details siehe Archiv-Fakte-Pfleger.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p><strong><em>Löschen</em></strong> <strong>F7</strong></p>
        </td>
        <td>
          <p>Löscht ein Archiv-Fakt.</p>
          <p>Auslieferungen lassen sich nur durch System-Entwickler löschen.</p>
          <p>Für Details siehe Archiv-Fakte-Pfleger.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p><strong><em>Nachreferenzieren …</em></strong></p>
        </td>
        <td>
          <p>Mit Hilfe dieser Funktion ist es möglich, selektierte Archiv-Fakte so zu bearbeiten, dass die Archiv-Referenz-Nummer nachreferenziert wird, falls keine angegeben ist.</p>
          <p>Nach Abschluss der Operation ist die Anzahl der Archiv-Fakte ohne Archiv-Referenz 0.</p>
        </td>
      </tr>
    </tbody>
  </table>
</div>
