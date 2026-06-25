# Setup Filialsystem

<!-- source: https://amic.de/hilfe/maske_SetupFilialsystem.htm -->

<div class="table-wrapper">
  <table>
    <tbody>
      <tr>
        <td>
          <p><b>Felder</b></p>
        </td>
        <td></td>
        <td></td>
      </tr>
      <tr>
        <td>
          <p>Betrieb</p>
        </td>
        <td colspan="2">
          <p>Nummer der Betriebsstätte, danach dessen Bezeichnung.</p>
        </td>
      </tr>
      <tr>
        <td></td>
        <td>
          <p><i><u>Einrichtung Filialsystem</u><u></u></i></p>
        </td>
        <td>
          <p><i><u>Aktiv unter SQL Remote</u><u></u></i></p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Publikationen</p>
        </td>
        <td>
          <p>Eingerichtete Publikationen die für die unter der im Feld Betrieb angegebenen Betriebsstätte.</p>
        </td>
        <td>
          <p>Publikationen, die unter SQL Remote aktiv / eingetragen sind.</p>
          <p>( Diese Informationen findet man in <b>scview</b> unter Publikationen ).</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Remote User</p>
        </td>
        <td>
          <p>Nummer und Bezeichnung der angeschlossenen Filialen, wie sie im Filialsystem von A.eins eingerichtet sind.</p>
        </td>
        <td>
          <p>Zeigt die unter <b>scview</b> angegebenen SQL Remotebenutzer.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Subscriptions</p>
        </td>
        <td>
          <p>Nummer und Bezeichnung der im Filialsystem von A.eins eingerichteten Subskriptionen.</p>
        </td>
        <td>
          <p>SQL-Remote Benutzername und Subskription unter <b>scview</b></p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Auswahl Betrieb</p>
        </td>
        <td colspan="2">
          <p>Zeigt die Liste der angeschlossenen Betriebsstätten nach Nummer für die im Feld Betrieb angegebene Betriebsstätte.</p>
          <p>Sie wird für einige Funktionen aus der Funktionsbox benötigt.</p>
        </td>
      </tr>
    </tbody>
  </table>
</div>

| **Funktionen** | |
| --- | --- |
| Neuaufbau Replikation | Eine bestehende Replikation zu einem Kommunikationspartner wird gestoppt und neu aufgebaut. Alle nicht übertragenen oder verarbeiteten Nachrichten gehen verloren. Notoperation, um eine Synchronisation zwischen 2 Betriebsstätten unter Inkaufnahme von Datenverlust zu erzwingen.<br>**Der Kommunikationspartner muss die gleiche Funktion ausführen.** |
| Komplett Setup | Inkrementelles Setup auf alle Remote User und Publikationen. Alle Publikationen, Publisher, Remote User, Remote Subscriptions werden wie in der Filialeinrichtung vereinbart dem SQL Remote System hinzugefügt, im SQL Remote System geändert oder aus SQL Remote System entfernt. |
| Komplett deinstallieren | Entfernt alle SQL Remote Objekte aus der Datenbank. |
| Setup Publikationen | Inkrementelles Setup auf alle Publikationen. Alle Publikationen werden wie in der Filialeinrichtung vereinbart dem SQL Remote System hinzugefügt, im SQL Remote System geändert oder aus SQL Remote System entfernt. **Der Publisher wird angelegt, falls er nicht existiert.** Anwendung: Veränderungen an Publikationen werden in das SQL Remote System übertragen (z.B. neue Tabelle replizieren). |
| Setup Remote User | Der ausgewählte Remote User wird wie in der Filialeinrichtung vereinbart dem SQL Remote System hinzugefügt, im SQL Remote System geändert oder aus SQL Remote System entfernt. Weiterhin werden die Subskriptionen vorgenommen und gestartet. |
| Remote User de-installieren | Beendet für die ausgewählte Betriebsstätte die Möglichkeit, SQL Remote-Nachrichten von dieser Datenbank zu empfangen. |
| Publikation entfernen | Entfernt den Publisher und alle Publikationen aus dem SQL Remote System. |
