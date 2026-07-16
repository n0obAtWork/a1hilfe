# Kommandos

<!-- source: https://amic.de/hilfe/_ prodI_command.htm -->

Das XML kennt folgende Kommandos im „COMMAND“-Tag:

<div class="table-wrapper">
  <table>
    <tbody>
      <tr>
        <td colspan="4">
          <p><strong>COMMAND</strong></p>
        </td>
      </tr>
      <tr>
        <td>
          <p><strong>Kommando</strong></p>
        </td>
        <td>
          <p><strong>Richtung</strong></p>
        </td>
        <td>
          <p><strong>LVS</strong></p>
        </td>
        <td>
          <p><strong>Bedeutung</strong></p>
        </td>
      </tr>
      <tr>
        <td>
          <p>BEGIN</p>
        </td>
        <td>
          <p>IN A.eins</p>
        </td>
        <td>
          <p>Nein</p>
        </td>
        <td>
          <p>Hier wird in die Tabelle „ProduktionsInfo“ die Linne eingetragen, auf der diese Produktion jetzt laufen soll. In die Tabelle „ProduktionsInfo“ wird eingetragen, in welchem Status sich die Produktion befindet.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>MATERIAL</p>
        </td>
        <td>
          <p>IN A.eins</p>
        </td>
        <td>
          <p>Ja</p>
        </td>
        <td>
          <p>Hier wird von der Produktion Material als Fertigware an das LVS gemeldet. Der angegebene Ladeträger wird in der Lokalität der Fertigware der angegebenen Linie mit Hilfe eines Vorgangsimports (LVS) erzeugt.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>END</p>
        </td>
        <td>
          <p>IN A.eins</p>
        </td>
        <td>
          <p>Nein</p>
        </td>
        <td>
          <p>Mit den Verbrauchsdaten und den Produktionsdaten wird die Produktion in A.eins korrigiert.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>MATERIALREQUEST</p>
        </td>
        <td>
          <p>IN A.eins</p>
        </td>
        <td>
          <p>Ja</p>
        </td>
        <td>
          <p>Hier wird eine Materialanforderung gegeben. Diese kann, muss aber nicht einer Produktion zugeordnet sein. Wichtig ist die Angabe der Linie, da diese im LVS die Bereitstellungszone bestimmt.</p>
          <p>Die Materialanforderung wird in die LVS_Materialorder geschrieben</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>PARTS</p>
        </td>
        <td>
          <p>AUS A.eins</p>
        </td>
        <td>
          <p>NEIN</p>
        </td>
        <td>
          <p>Dies ist die Stückliste der Produktion</p>
        </td>
      </tr>
    </tbody>
  </table>
</div>
