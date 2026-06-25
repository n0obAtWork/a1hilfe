# Oberfläche - Zusatz

<!-- source: https://amic.de/hilfe/oberflchezusatz.htm -->

Auf der Registerkarte ***Zusatz*** werden die weiteren Daten zum Kunden sowie zur Verarbeitung angezeigt.

![Ein Bild, das Text, Screenshot, Software, Computersymbol enthält. KI-generierte Inhalte können fehlerhaft sein.](../../ImagesExt/image8_190.jpg "Ein Bild, das Text, Screenshot, Software, Computersymbol enthält. KI-generierte Inhalte können fehlerhaft sein.")

Auf dem Register ***Zusatz*** sind folgende Felder zu sehen:

<div class="table-wrapper">
  <table>
    <tbody>
      <tr>
        <td colspan="2">
          <p><strong>Ansprechpartner Kontaktdaten Default</strong></p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Telefon</p>
        </td>
        <td>
          <p>Die Telefonnummer des Ansprechpartners, wenn nicht im Bediener hinterlegt.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>E-Mail-Adresse</p>
        </td>
        <td>
          <p>Die E-Mail-Adresse des Ansprechpartners, wenn nicht im Bediener hinterlegt.</p>
        </td>
      </tr>
    </tbody>
  </table>
</div>

Der Ansprechpartner stellt eine Person im eigenen Unternehmen dar, welche Sie bei Fragen oder Problemen zu dieser Rechnung kontaktieren können.

<div class="table-wrapper">
  <table>
    <tbody>
      <tr>
        <td colspan="2">
          <p><strong>Verarbeitung</strong></p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Dateipfad</p>
        </td>
        <td>
          <p>Dort werden die erstellten XMLs hinterlegt.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Dateipfad Test</p>
        </td>
        <td>
          <p>Dort werden die XMLs hinterlegt, welche durch die Funktion <strong><em>Rechnung testen</em></strong> erstellt wurden.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Webservice</p>
        </td>
        <td>
          <p>Ob die eRechnung an einen Webservice zur Verifikation weitergeleitet wird, dabei gibt es folgende Möglichkeiten:</p>
          <p><strong>0</strong> - nicht durchführen: Das erzeugte Xml wird nicht zur Verifikation an den Webservice gesendet.</p>
          <p><strong>1</strong> - nur protokollieren: Das erzeugte Xml wird zur Verifikation an den Webservice gesendet, das Ergebnis wird aber lediglich protokolliert.&nbsp;</p>
          <p><strong>2</strong> - immer beachten: Das erzeugte Xml wird immer an den Webservice gesendet - schlägt die Verifikation fehlt, wird der Export sofort wieder gelöscht, nur archiviert.&nbsp;</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Versandprozedur</p>
        </td>
        <td>
          <p>(! Nur im Exportformat UBL !)</p>
          <p>Hier kann eine Versandprozedur angegeben werden. Diese muss zwei Eingabeparameter haben. Diese sind die <strong>Fa_id</strong> und die <strong>Fa_MndNr</strong> des Archiveintrags, der nach der Erstellung versendet werden soll. Die Ziel-Mailadresse wird hier drin ermittelt und der Versandprofilstammeintrag u. U. fest verdrahtet eingetragen.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Dateinamensfkt.</p>
        </td>
        <td>
          <p>Hier kann eine Datenbankfunktion zur Findung des Dateinamens eingetragen werden. Als Standard gilt „AMIC_STD_XRE_Filename“.</p>
        </td>
      </tr>
    </tbody>
  </table>
</div>
