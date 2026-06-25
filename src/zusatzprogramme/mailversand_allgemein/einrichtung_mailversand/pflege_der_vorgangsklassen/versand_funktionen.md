# Versand-Funktionen

<!-- source: https://amic.de/hilfe/_mailversandfunktionen.htm -->

Administration > Formulars / Abläufe > Formularzuordnung / Vorgangsunterklassen

Sind alle Kennzeichen korrekt eingerichtet, so wird beim ersten Druck des für Versand gekennzeichneten Beleges, die in der Formularzuordnung [FRZ] definierte Versandprozedur aufgerufen. Diese übernimmt den Versand.

<div class="table-wrapper">
  <table>
    <tbody>
      <tr>
        <td></td>
        <td>
          <p><b>Sofortiger Versand</b></p>
        </td>
        <td>
          <p><b>Späterer Versand</b></p>
        </td>
      </tr>
      <tr>
        <td></td>
        <td>
          <p>Die E-Mail wird umgehend beim Druck an das Versandsystem übergeben und zum Versand freigegeben</p>
        </td>
        <td>
          <p>Die E-Mail wird erst einmal vorgesehen, kann noch einmal gelöscht und erst später zum Versand freigegeben werden.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p><b>Ware</b></p>
        </td>
        <td>
          <p>AMIC_BELEGVERSAND_WARE_SOFORT<b></b></p>
        </td>
        <td>
          <p>AMIC_BELEGVERSAND_WARE_SPAETER<b></b></p>
        </td>
      </tr>
      <tr>
        <td>
          <p><b>Rohware</b></p>
        </td>
        <td>
          <p>AMIC_BELEGVERSAND_ROHWARE_SPAETER<b></b></p>
        </td>
        <td>
          <p>AMIC_BELEGVERSAND_ROHWARE_SPAETER<b></b></p>
        </td>
      </tr>
    </tbody>
  </table>
</div>

<p class="just-emphasize">Definition Parameter</p>

| Definition Parameter<br> |
| --- |
| FA_ID | enthält die FormulararchivId des zu versendenden Belegs |
| FA_MNDNR | enthält die Mandantennummer im Mehrmandantsystem in A.eins. |
| FA_EMPFAENGER | enthält eine kommagetrennte Liste der Empfängernmailadressen |
| FA_HTMLBODY | enthält den aus A.eins erzeugten HTML-Body. Dieser wird im Mailtext verwendet. |
| SUBJECT | enthält den Betreff der Mail |
| VPST | enthält die Nummer des heranzuziehenden Versandprofilstamms [VPST] |
| ANHAENGE | enthält in XML-Notation eine Liste von FormulararchivIds, die als Anhänge mit gesendet werden sollen.<br>Hinweis: Hier ist ggf. auch die Liste der zugehörigen eRechnungen enthalten |
