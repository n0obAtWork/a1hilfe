# Funktionen

<!-- source: https://amic.de/hilfe/_mailversandfunktioneninmail.htm -->

<div class="table-wrapper">
  <table>
    <tbody>
      <tr>
        <td colspan="2">
          <p><b>Funktionen</b></p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Freigeben/Versenden</p>
        </td>
        <td>
          <p>Gibt Einträge frei zur Versendung. Wenn der Versand <a href="../../mailversand_allgemein/einrichtung_mailversand/synchron_oder_asynchron/index.md">Synchron</a> erfolgen soll, so werden die E-Mails auch sofort versendet. Anderenfalls werden sie zyklisch durch den Dienst versendet.</p>
          <p>Bei E-Mails mit Mailversandquelle = Ware-Beleg oder Mailversandquelle = Eohware-Sammeldruck wird bei erfolgreichem Versand das Kennzeichen V_StatusBelegVersand im Vorgangstamm beziehungsweise V_RohwareStatusMail in der Relation V_Rohware auf den Wert ‚Versendet‘ gesetzt.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Zurückstellen</p>
        </td>
        <td>
          <p>Stellt den Eintrag zurück. Dies kann eine bewusste Rückstellung zur späteren Klärung sein.</p>
          <p>Bereits versendete Einträge lassen sich nicht zurückstellen.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Löschen</p>
        </td>
        <td>
          <p>Löscht den Eintrag</p>
          <p>Bereits versendete Einträge lassen sich nicht löschen</p>
          <p>Bei E-Mails mit Mailversandquelle = Ware-Beleg oder Mailversandquelle = Eohware-Sammeldruck wird bei erfolgreichem Löschen das Kennzeichen V_StatusBelegVersand im Vorgangstamm beziehungsweise V_RohwareStatusMail in der Relation V_Rohware auf den Wert ‚Zurück genommen‘ gesetzt.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Email ändern</p>
        </td>
        <td>
          <p>Öffnet einen Pfleger zum nachträglichen Bearbeiten der Mailadressen. Wenn die Mail bereits versendet wurde, wird ein neuer Eintrag erzeugt. Der Inhalt bleibt der Originalinhalt. Der Verweis zeigt auf die ursprüngliche Mail und nicht auf die ursprüngliche Quelle.</p>
          <p>Wenn eine Mail mit bereits gelöschter Verpostung erneut versendet werden soll, muss beim Ändern eine neue Verpostung ausgewählt werden.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Bereich</p>
        </td>
        <td>
          <p>Filter der Auswahlliste</p>
        </td>
      </tr>
    </tbody>
  </table>
</div>
