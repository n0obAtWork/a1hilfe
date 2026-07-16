# EPA der Zahlungsmaske

<!-- source: https://amic.de/hilfe/epaderzahlungsmaske.htm -->

Auf der Zahlungsmaske ziehen folgende EPAs, die auch für andere Finanzvorgänge ziehen, die über diese Maske abgewickelt werden:

<div class="table-wrapper">
  <table>
    <tbody>
      <tr>
        <td colspan="2">
          <p>Auf der Zahlungsmaske ziehen folgende EPAs, die auch für andere Finanzvorgänge ziehen, die über diese Maske abgewickelt werden:</p>
        </td>
      </tr>
      <tr>
        <td>
          <p><strong>EPA</strong></p>
        </td>
        <td>
          <p><strong>Beschreibung</strong></p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Abfrage beim Abschluss der Zahlung?</p>
        </td>
        <td>
          <p>Ist dieser EPA auf Ja gesetzt, wird beim Validieren des Zahlungsbetrages bei Barvorgängen noch eine Abfrage geschaltet; bei Finanzvorgängen, die auf derselben Maske operieren, erfolgt keine Abfrage, da dort der Vorgang explizit mit F9 erfolgt.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Soll Zahlungsart Scheck aktiv sein?</p>
        </td>
        <td>
          <p>Durch diesen EPA kann die Zahlungsart Scheck deaktiviert werden.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Soll Zahlungsart Gutschein aktiv sein?</p>
        </td>
        <td>
          <p>Durch diesen EPA kann die Zahlungsart Gutschein deaktiviert werden</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Soll Zahlungsart Kreditkarte aktiv sein?</p>
        </td>
        <td>
          <p>Durch diesen EPA kann die Zahlungsart Kreditkarte deaktiviert werden.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Soll Zahlungsart Bankeinzug aktiv sein?</p>
        </td>
        <td>
          <p>Durch diesen EPA kann die Zahlungsart Bankeinzug deaktiviert werden.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Sollen Einzahlungen,... über Formularsteuerung gedruckt werden?</p>
        </td>
        <td>
          <p>Ist dieser EPA auf Ja gesetzt, werden bei Finanzvorgängen zusätzliche Formulare 51-55 aus dem Formularwesen auf den in DRZ hinterlegten Drucker gedruckt. Bei der Tresenkasse hat dieser EPA keine Auswirkungen. Ist er auf Nein gesetzt, wird nur ein Standardbeleg über den Finanzvorgang ausgedruckt.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Soll ein Scheck gedruckt werden?</p>
        </td>
        <td>
          <p>Nur wenn dieser EPA auf Ja gesetzt ist, wird auf den in DRZ hinterlegten Drucker bei Zahlungsart Scheck ein Scheckformular mit den erfassten Informationen gedruckt. Dieses Scheckformular ist über den Wert in Kasseneinstellungen, Formularnummer Scheck im Formularwesen unter entsprechendem Wert hinterlegt. Ist hier Nein eingetragen, wird kein Scheck gedruckt.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Funktion Schublade öffnen auf Maske laden?</p>
        </td>
        <td>
          <p>Nur wenn dieser EPA auf Ja gesetzt ist, erscheint die Funktion zum Schublade öffnen auf der Maske. Ansonsten ist die Öffnung der Schublade explizit an Zeitpunkte im Programm gebunden.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Soll bei Geldentnahme Steuersatz wählbar sein?</p>
        </td>
        <td>
          <p>Dieser EPA zieht nur bei Geldentnahmen und lässt dort eine Wahl des Steuersatzes zu (wenn das entsprechende Sachkonto diese Änderung zulässt).</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Darf Skonto gewährt werden?</p>
        </td>
        <td>
          <p>Durch diesen EPA kann man bei Barvorgängen die Gewährung von Skonto abschalten.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Soll Journal bei Kassenvorgängen gedruckt werden?</p>
        </td>
        <td>
          <p>Dieser EPA zieht nur bei Finanzvorgängen.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Eine Rechnung pro Zahlungsmeldung?</p>
        </td>
        <td>
          <p>Ist dieser EPA auf Nein gesetzt ist es möglich, beim Finanzvorgang Zahlungsmeldung nacheinander mehrere Zahlungsmeldung zu erfassen, die dann zu einer Meldung zusammengefasst werden und zusammen beglichen werden können.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Reduzierte Displayanzeige?</p>
        </td>
        <td>
          <p>Ist dieser EPA auf Ja gesetzt, erscheint bei gewissen internen Finanzvorgängen keine Displayanzeige.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Soll ein Lastschrift-Formular gedruckt werden?</p>
        </td>
        <td>
          <p>Nur wenn dieser EPA auf Ja gesetzt ist, wird auf den in DRZ hinterlegten Drucker bei Zahlungsart Kreditkarte ein EC-Lastschriftformular mit den erfassten Informationen gedruckt. Dieses Formular ist über den Wert in Kasseneinstellungen, Formularnummer EC-Karte im Formularwesen unter entsprechendem Wert hinterlegt. Ist hier Nein eingetragen, wird kein Lastschriftformular gedruckt.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Automatische Steuer bei Entnahmen ?</p>
        </td>
        <td>
          <p>Ist dieser EPA auf Ja gesetzt, wird bei Geldentnahmen automatisch der Steuersatz aus dem Sachkontenstamm des gewählten Kontos. .übernommen</p>
        </td>
      </tr>
    </tbody>
  </table>
</div>
