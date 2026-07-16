# Benutzerinformation

<!-- source: https://amic.de/hilfe/_systeminfobenutzerinfo.htm -->

<details>
<summary>Server</summary>

| Feldname | Beschreibung |
| --- | --- |
| Name | Name des Datenbankservers<br> |
| RequestLogFile | Hier wird der Name der Datei zur Anforderungsprotokollierung angezeigt, wenn eine existiert.<br> |
| Technische Verbindungen | Anzahl aller technischen Verbindungen die zurzeit zur Datenbank bestehen<br> |
| Unterschiedliche Benutzer | Anzahl der unterschiedlichen Benutzer (ein Benutzer kann mehrere Verbindungen zu Datenbank haben wird hier aber nur einmal gezählt)<br> |

</details>

 

<details>
<summary>Benutzerinformation</summary>

| Kopfdaten | Beschreibung |
| --- | --- |
| Name | Kürzel des aktuellen Bedieners |
| Eigene Id | Die Id des aktuellen Bedieners |

Hier erhält man Informationen zu den Benutzern in der Datenbank. Man kann die Verbindung eines Benutzers durch Doppelklick trennen oder allen Benutzern eine Meldung schicken.

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
          <p>Name</p>
        </td>
        <td>
          <p>Kürzel des aktuellen Bedieners</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Eigene Id</p>
        </td>
        <td>
          <p>Die Id über die der aktuelle Bediener zurzeit verbunden ist</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Id</p>
        </td>
        <td>
          <p>Id über die der Bediener verbunden ist</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Benutzer</p>
        </td>
        <td>
          <p>Kürzel des Bedieners</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>DBName</p>
        </td>
        <td>
          <p>Name der Datenbank</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Nodeadress</p>
        </td>
        <td>
          <p>IP-Adresse des Rechners über den der Benutzer mit der Datenbank verbunden ist.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Commlink</p>
        </td>
        <td>
          <p>Verbindungsart z.B. TCPIP</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>LastRegTime</p>
        </td>
        <td>
          <p>Registrierte Zeit der letzten Anforderung durch den Benutzer</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Schreib</p>
        </td>
        <td>
          <p>Anzahl der Schreibanforderungen</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Lese</p>
        </td>
        <td>
          <p>Anzahl der Leseanforderungen</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Blocknr.</p>
        </td>
        <td>
          <p>Falls die aktuelle Verbindung nicht blockiert ist, wird der Wert Null angezeigt. Sonst entspricht der Wert der Verbindungsnummer der Verbindung, die aufgrund eines Sperrenkonflikts blockiert ist.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Connection</p>
        </td>
        <td>
          <p>Art der Verbindung / Programm mit dem man sich mit der Datenbank verbunden hat</p>
        </td>
      </tr>
    </tbody>
  </table>
</div>

</details>
