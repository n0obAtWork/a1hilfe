# Lagernummervorbelegung (SPA 260)

<!-- source: https://amic.de/hilfe/_SPA_260.htm -->

<div class="table-wrapper">
  <table>
    <tbody>
      <tr>
        <td colspan="2">
          <p><strong>SPA-Einstellungen</strong></p>
        </td>
      </tr>
      <tr>
        <td>
          <p>0 – nie vorbelegen</p>
        </td>
        <td>
          <p>Die Lagernummer wird nie vorbelegt</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>1 – wie letzte Auswahl</p>
        </td>
        <td>
          <p>Die Lagernummer wird aus den Vorgangskonstanten vorbelegt. Eine Änderung der Lagernummer ändert auch die Vorgangskonstanten (nicht empfohlen) **</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>2 – aus Vorgangskonstanten</p>
        </td>
        <td>
          <p>Die Lagernummer wird bei jedem neuen Vorgang aus den Vorgangskonstanten vorbelegt. **</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>3 – aus VKONS b. Mehrbeleg wie vorheriger Vorg.</p>
        </td>
        <td>
          <p>Die Lagernummer wird bei der ersten Erfassung aus den Vorgangskonstanten vorbelegt. Im Fall der Mehrbelegserfassung wird die letzte verwendete Lagernummer weiterverwendet. Eine Änderung der Vorgangskonstanten wie in Einstellung 2 findet jedoch nicht statt. **</p>
        </td>
      </tr>
    </tbody>
  </table>
</div>

\*\* Bei Einstellung 1-3 – Es wird die Vorbelegung in Abhängigkeit folgender Einstellungen vorgenommen, die einander (von oben nach unten) überlagern können:

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
          <p>Vorgangskonstanten</p>
        </td>
        <td>
          <p>Die in [VKONS] eingetragene Lagernummer wird vorbelegt</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>WWW-Konstante des Bedieners</p>
        </td>
        <td>
          <p>Im Bedienerstamm können sog. WWW-Konstanten definiert werden. Diese überlagern bei dem aktuellen Bediener die Einstellung in den Vorgangskonstanten</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>UFLD-Feld</p>
        </td>
        <td>
          <p>Eingaben in einem UFLD-Feld überlagern sowohl die Vorgangskonstanten als auch die WWW-Kontante des Bedieners</p>
        </td>
      </tr>
    </tbody>
  </table>
</div>
