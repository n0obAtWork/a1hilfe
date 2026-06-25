# Registerkarte Markt

<!-- source: https://amic.de/hilfe/register_artikelstamm_markt.htm -->

Alle Felder auf dieser Registerkarte werden nicht von A.eins ausgewertet oder verwendet. Diese Felder stehen der [Datendrehscheibe](../../externe_kommunikation/datendrehscheibe_terres/index.md) oder Privaten Anwendungen zur Verfügung.

<div class="table-wrapper">
  <table>
    <tbody>
      <tr>
        <td>
          <p><strong>Feld</strong></p>
        </td>
        <td>
          <p><strong>Bedeutung</strong></p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Grundpreiseinheit</p>
        </td>
        <td>
          <p>In diesem Feld wird die Grundpreiseinheit wie z.B. Kg eingetragen.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Grundpreis Faktor</p>
        </td>
        <td>
          <p>In diesem Feld wird der Faktor eingetragen mit dem der Preis multipliziert wird.</p>
          <p>Mit dem Faktor lässt sich dann der Grundpreis des Artikels berechnen. Mit diesen Informationen kann dann auf ein Etikett der Grundpreis gedruckt werden.</p>
          <p>Wird die Datendrehscheibe verwendet so werden diese beiden Felder von Terres versorgt.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Preispflege per Datendrehscheibe</p>
        </td>
        <td>
          <p>Wird dieser Schalter auf&nbsp; „unterdrücken“ gestellt so wird ein neuer Preis der über die Datendrehscheibe importiert wird nicht übernommen.</p>
          <p>Innerhalb von privaten Prozeduren oder Anwendungen muss dieser Schalter berücksichtigt werden, wenn dieser ausgewertet werden soll. Da A.eins dies nicht automatisch macht.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Artikelpflege per Datendrehscheibe</p>
        </td>
        <td>
          <p>Wird dieser Schalter auf „unterdrücken“ gestellt, so wird der Artikel nicht durch den Import über die Datendrehscheibe verändert.</p>
          <p>Innerhalb von privaten Prozeduren oder Anwendungen muss dieser Schalter berücksichtigt werden, wenn dieser ausgewertet werden soll. Da A.eins dies nicht automatisch macht.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Warengruppenpflege per Datendrehscheibe</p>
        </td>
        <td>
          <p>Wird dieser Schalter auf „unterdrücken“ gestellt, so wird die Warengruppe nicht durch den Import per Datendrehscheibe geändert. Alle anderen Änderungen werden übernommen.</p>
          <p>Innerhalb von privaten Prozeduren oder Anwendungen muss dieser Schalter berücksichtigt werden, wenn dieser ausgewertet werden soll. Da A.eins dies nicht automatisch macht.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Mengeinheitsgruppenpflege per Datendrehscheib</p>
        </td>
        <td>
          <p>Wird dieser Schalter auf „unterdrücken“ gestellt, so wird die Mengeneinheitsgruppe nicht durch den Import per Datendrehscheibe geändert. Alle anderen Änderungen werden übernommen.</p>
          <p>Innerhalb von privaten Prozeduren oder Anwendungen muss dieser Schalter berücksichtigt werden, wenn dieser ausgewertet werden soll. Da A.eins dies nicht automatisch macht.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Einzelhandels Kennzeichen</p>
        </td>
        <td>
          <p>Wird dieses Kennzeichen auf Ja gesetzt, so werden die Statistiken des Artikels mit an Terres übermittelt. Dies gilt auch für Artikel die nicht über Terres bezogen worden sind.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Bestell und Fakturiersperre per Datendrehscheibe</p>
        </td>
        <td>
          <p>Mit diesem Feld kann eingestellt werden, ob die Bestell- oder Fakturiersperre eines Artikels durch die Datendrehscheibe abgeändert werden darf. Die Einstellung im Artikelstamm wirkt sich auf alle Artikel des Stammes aus. Diese Globalen Einstellungen können am <a href="../artikel/registerkarte_markt.md">Artikel</a> überschreiben werden.</p>
          <table>
            <tbody>
              <tr>
                <th><strong>Ausprägung</strong></th>
                <th><strong>Bedeutung</strong></th>
              </tr>
              <tr>
                <td>Durchführen</td>
                <td>Mit dieser Einstellung werden die Kennzeichen Bestell- und Fakturiersperre durch die Datendrehscheibe abgeändert.</td>
              </tr>
              <tr>
                <td>Fakturiersperre unterdrücken</td>
                <td>Mit dieser Einstellung wird nur das Kennzeichen Bestellsperre durch die Datendrehscheibe abgeändert. Das Kennzeichen Fakturiersperre wird nicht durch die Datendrehscheibe verändert:</td>
              </tr>
              <tr>
                <td>Bestellsperre unterdrücken</td>
                <td>Mit dieser Einstellung wird nur das Kennzeichen Fakturiersperre durch die Datendrehscheibe abgeändert. Das Kennzeichen Bestellsperre wird nicht durch die Datendrehscheibe verändert.</td>
              </tr>
              <tr>
                <td>Beide unterdrücken</td>
                <td>Mit dieser Einstellung werden die Kennzeichen Bestell- und Fakturiersperre nicht durch die Datendrehscheibe abgeändert.</td>
              </tr>
            </tbody>
          </table>
        </td>
      </tr>
    </tbody>
  </table>
</div>
