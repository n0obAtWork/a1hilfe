# Registerkarte Silo

<!-- source: https://amic.de/hilfe/_prozess_silo.htm -->

Auf dieser Registerkarte werden die Einstellungen für das Silo in der Waage vorgenommen.

<div class="table-wrapper">
  <table>
    <tbody>
      <tr>
        <td>
          <p><strong>Feldname</strong></p>
        </td>
        <td>
          <p><strong>Bedeutung</strong></p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Silostandort Festhalten</p>
        </td>
        <td>
          <p>Mit diesem Schalter kann eingestellt werden, ob das Silo / Ladeträger auf eine andere Lokalität umgebucht werden soll, wenn in der Wiegung eine andere Lokalität als die Lokalität des Silos / Ladeträgers ausgewählt worden ist.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Aktivitätstyp</p>
        </td>
        <td>
          <p>Hier kann ein Aktivitätstyp für die LVS / Silo Buchung ausgewählt werden. Das Format für den Aktivitätstyp ist ein Anwenderformat „AF_LVSAKTTYP“.</p>
          <p>Dabei ist zu beachten, dass die ersten 100 Einträge seitens der Firma AMIC gepflegt werden.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Prozesstyp</p>
        </td>
        <td>
          <p>Hier kann eingestellt werden um welchen Prozesstyp es sich handelt. Diese Prozesse werden benötigt, um bestimmte Silobuchungen über Waage abzubilden. Wie z.B. die Leermeldung. Der jeweilige Prozesstyp gilt für das Lager, welches im Prozess eingetragen worden ist. Dies heißt bei n Läger müssen auch n Prozesse eingerichtet werden.</p>
          <table>
            <tbody>
              <tr>
                <th>Prozesstyp</th>
                <th>Bedeutung</th>
              </tr>
              <tr>
                <td>--</td>
                <td>Kein Prozess</td>
              </tr>
              <tr>
                <td>Leermeldung Eingangsbuchung</td>
                <td>Dieser Prozess muss vorhanden sein, um ein Silo von einer negativen Menge auf 0 zu bringen. Dies bedeutet, es muss als Wiegetyp ein Wareneingang gewählt werden</td>
              </tr>
              <tr>
                <td>Leermeldung Ausgangsbuchung</td>
                <td>Dieser Prozess muss vorhanden sein, um die restliche Menge von einem Silo auf 0 zu bringen. Dies bedeutet, es muss als Wiegetyp ein Warenausgang gewählt werden.</td>
              </tr>
              <tr>
                <td>Leermeldung Schwundsilobuchung</td>
                <td>Dieser Prozess muss vorhanden sein, wenn die Leermeldungsmenge auf ein Schwundsilo gebucht werden soll. Ist einem anderen Prozess ein Schwundsilo zugeordnet, und es existiert kein Prozess mit diesem Typ, so wird die Leermeldung nicht durchgeführt.</td>
              </tr>
              <tr>
                <td>Artikelumbuchung Abgang</td>
                <td>Um eine Artikelumbuchung durchzuführen, muss ein Prozess für die Artikelumbuchung Abgang eingerichtet werden. Es müssen beide Prozesse eingerichtet sein, damit eine Artikelumbuchung durchgeführt werden kann.</td>
              </tr>
              <tr>
                <td>Artikelumbuchung Zugang</td>
                <td>Um eine Artikelumbuchung durchzuführen, muss ein Prozess für die Artikelumbuchung Abgang eingerichtet werden. Es müssen beide Prozesse eingerichtet sein, damit eine Artikelumbuchung durchgeführt werden kann.</td>
              </tr>
            </tbody>
          </table>
        </td>
      </tr>
      <tr>
        <td>
          <p>Schwundsilo</p>
        </td>
        <td>
          <p>Hier kann ein alternatives Silo hinterlegt werden, auf welches die Schwundmenge bei der Leermeldung gebucht werden soll. Es kann nur ein Silo ausgewählt werden, welches sich auf dem Lager befindet, welches im Prozess hinterlegt worden ist.</p>
        </td>
      </tr>
    </tbody>
  </table>
</div>
