# Ladeschein – aus Auftrag zu Lieferschein/Rechnung

<!-- source: https://amic.de/hilfe/ladescheinausauftragzuliefersc.htm -->

Eine oder mehrere Auftragspositionen aus einem oder mehreren Aufträgen können zu einem Ladeschein zusammengestellt werden. Dies ist dort angezeigt, wo gemeinsam verladen werden soll.

Aber auch auf der Wareneingangsseite besteht die Möglichkeit eine oder mehrere Positionen aus einer oder mehreren Bestellungen zu einem Eingangs-Ladeschein (oder Entladeschein) zusammenzufassen.

In beiden Fällen soll im Vorgangsimport die Position des Ladescheins zu einem (Eingangs/Ausgangs)-Lieferschein bzw. eine (Eingangs/Ausgangs)-Rechnung gewandelt werden.

Dafür sind einige Voraussetzungen notwendig:

<p class="just-emphasize">ImportTyp</p>

<div class="table-wrapper">
  <table>
    <tbody>
      <tr>
        <td colspan="2">
          <p><strong>Importtyp im ImportVorgStamm</strong></p>
        </td>
      </tr>
      <tr>
        <td>
          <p>NULL</p>
        </td>
        <td>
          <p>Beleg wird in einen Lieferschein umgewandelt</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>0</p>
        </td>
        <td>
          <p>Beleg wird in einen Lieferschein umgewandelt</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>1</p>
        </td>
        <td>
          <p>ACHTUNG !!! Ladeschein wird erstellt !!!</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>2</p>
        </td>
        <td>
          <p>Beleg wird in eine Rechnung umgewandelt *</p>
        </td>
      </tr>
    </tbody>
  </table>
</div>

<p class="just-emphasize">Wabewerfassid in der ImportVorgPosition</p>

Um klarzustellen welche der ursprünglichen Auftrags/Bestell-Positionen umgewandelt werden soll, muss die WaBewErfassId aus der Warenposition des Ladescheins angegeben werden.

<p class="just-emphasize">RestAusbuchKennz in der ImportVorgPosition</p>

Zusätzlich ist es möglich das Feld „RestAusbuchKennz“ auf 1 zu setzen, um bei Mindermengen-Lieferung die Reste des Quell-Vorgangs (Auftrag/Bestellung) auszubuchen. \*

\* Hinweis: Diese Funktion ist nur bei gleichzeitiger Verwendung des UseCS=1 im ImportVorgstamm verwendbar!
