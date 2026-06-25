# Behandlung nicht aufgelöster Vorgangstexte (SPA 884)

<!-- source: https://amic.de/hilfe/_SPA_884.htm -->

Den Vorgangsklassen können Texte zugeordnet werden, die Platzhalter enthalten, welche schließlich durch manuelle Eingabe aufgelöst werden. Diese Auflösung findet je nach Einstellung vor Beginn der Positionsteilerfassung oder vor dem Abschluss der Erfassung statt.

Bei der Umwandlung eines Vorgangs in einen neuen Vorgang einer anderen Vorgangsklasse (z.B. von Angebot in Auftrag) findet keine Erfassung statt und damit auch keine Auflösung dieser Texte. Aus diesem Grund ist eine Behandlung der Texte erforderlich, damit im erstellten Beleg keine Platzhalter dargestellt werden.

Mit Hilfe dieses Steuerparameters kann die Behandlung der Vorgangstexte bei Umwandlung festgelegt werden.

Die gleiche Behandlung wird bei der Erstellung eines Vorgangs mit Makro durchgeführt, da auch hier keine Auflösung durch manuelle Eingabe erfolgen kann.

<div class="table-wrapper">
  <table>
    <tbody>
      <tr>
        <td colspan="2">
          <p><strong>Einstellungen</strong></p>
        </td>
      </tr>
      <tr>
        <td>
          <p>0 – nichts ändern</p>
        </td>
        <td>
          <p>Diese Einstellung ist die voreingestellte Behandlung wie bisher. Es findet keinerlei Aktion statt.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>1 – nur warnen</p>
        </td>
        <td>
          <p>Eine Warnung weist bei Umwandlung darauf hin, dass nicht aufgelöste Texte vorhanden sind.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>2 – Auflösen und warnen</p>
        </td>
        <td>
          <p>Platzhalter werden bei Umwandlung durch Leerzeichen ersetzt.</p>
          <p>Eine Warnung weist darauf hin.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>3 – Auflösen</p>
        </td>
        <td>
          <p>Platzhalter werden durch Leerzeichen ersetzt.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>4 – Löschen und warnen</p>
        </td>
        <td>
          <p>Nicht aufgelöste Textzeilen werden bei Umwandlung entfernt.</p>
          <p>Eine Warnung weist darauf hin.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>5 – Löschen</p>
        </td>
        <td>
          <p>Nicht aufgelöste Textzeilen werden entfernt.</p>
        </td>
      </tr>
    </tbody>
  </table>
</div>
