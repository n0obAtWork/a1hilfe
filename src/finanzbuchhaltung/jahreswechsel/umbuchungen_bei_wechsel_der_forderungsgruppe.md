# Umbuchungen bei Wechsel der Forderungsgruppe

<!-- source: https://amic.de/hilfe/umbuchungenbeiwechselderforder.htm -->

Man hat die Möglichkeit für Personenkonten die Forderungsgruppe zu wechseln. Wenn man dies macht, ergibt sich das Problem, dass auf den „alten“ Forderungs-/Verbindlichkeitskonten Beträge stehen, die aber ab dem Zeitpunkt des Wechsels auf die „neuen“ Forderungs-/Verbindlichkeitskonten gehören würden:

<div class="table-wrapper">
  <table>
    <tbody>
      <tr>
        <td></td>
        <td>
          <p>Personenkonto</p>
        </td>
        <td>
          <p>„altes“<br>Forderungskonto</p>
        </td>
        <td>
          <p>„neues“ Forderungskonto</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Eröffnung</p>
        </td>
        <td>
          <p>10.000,00</p>
        </td>
        <td>
          <p>10.000,00</p>
        </td>
        <td>
          <p>0,00</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Bewegungen laufendes Jahr</p>
        </td>
        <td>
          <p>2.000,00</p>
        </td>
        <td>
          <p>2.000,00</p>
        </td>
        <td>
          <p>0,00</p>
        </td>
      </tr>
      <tr>
        <td colspan="4">
          <p>Ab Periode x des laufenden Jahres wird im Personenkonto eine neue Forderungsgruppe und somit ein neues Forderungskonto eingetragen.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Bewegungen ab Periode x</p>
        </td>
        <td>
          <p>295,00</p>
        </td>
        <td>
          <p>0,00</p>
        </td>
        <td>
          <p>295,00</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Saldo der einzelnen Konten</p>
        </td>
        <td>
          <p>12.295,00</p>
        </td>
        <td>
          <p>12.000,00</p>
        </td>
        <td>
          <p>295,00</p>
        </td>
      </tr>
      <tr>
        <td colspan="4">
          <p>Beim Jahreswechsel wird jedoch, genau wie in den Normalperioden, der Saldo des Forderungskontos aus den Buchungen des Personenkontos in der Abschlussperiode gebildet. Dieser Buchung kann nur <b>einem</b> Forderungskonto zugewiesen werden:</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Jahreswechsel</p>
        </td>
        <td>
          <p>12.295,00</p>
        </td>
        <td>
          <p>0</p>
        </td>
        <td>
          <p>12.295,00</p>
        </td>
      </tr>
      <tr>
        <td colspan="4">
          <p>Das ist auch richtig, denn es fehlt eine Umbuchung vom „alten“ auf das „neue“ Forderungskonto. Würde man diese Umbuchung weglassen, so würden auf dem „alten“ Forderungskonto die Beträge auf alle Ewigkeit stehen bleiben und das „neue“ Forderungskonto hätte irgendwann einen negativen Saldo.</p>
          <p>Also erfolgt bei Jahreswechsel automatisch eine Umbuchung in der letzten Normalperiode. Es wird hier empfohlen – genau wie für den automatischen Abschluss von Unterkonten auf ihre Hauptkonten - eine 13. Normalperiode einzurichten</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Umbuchung</p>
        </td>
        <td></td>
        <td>
          <p>-12.000,00</p>
        </td>
        <td>
          <p>12.000,00</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Saldo der einzelnen Konten</p>
        </td>
        <td>
          <p>12.295,00</p>
        </td>
        <td>
          <p>12.000,00</p>
        </td>
        <td>
          <p>295,00</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Umbuchung + Saldo</p>
        </td>
        <td>
          <p>12.295,00</p>
        </td>
        <td>
          <p>0,00</p>
        </td>
        <td>
          <p>12.295,00</p>
        </td>
      </tr>
      <tr>
        <td colspan="4">
          <p>Soll beim Jahreswechsel diese automatische Umbuchung nicht durchgeführt werden, so kann man mit Hilfe des Steuerungsparameters 968 „Forderungskonten umbuchen“ dieses Verhalten abschalten.</p>
        </td>
      </tr>
    </tbody>
  </table>
</div>
