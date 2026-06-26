# Materialorder [LVSMO]

<!-- source: https://amic.de/hilfe/_lvs20_Materialorder.htm -->

Es gibt zwei Arten der Materialorder.

1. Vorgangsgebundene Materialordern

   Diese Materialordern bilden 1:1 einen Vorgang und dessen Materialbedarf ab. Artikel und Partien befinden sich nebst den Referenzen zu den Vorgangspositionen in dieser Materialorder.

2. Ungebundene Materialordern

   Diese Materialordern werden manuell oder über die Produktionsschnittstellle erstellt und enthalten in der Regel keine Referenzen auf Vorgangspositionen. Sie lassen sich mit dem Materialorder-Pfleger [LVSMO] erstellen.

<div class="table-wrapper">
  <table>
    <tbody>
      <tr>
        <td colspan="2">
          <p><b>Kopfdaten</b></p>
        </td>
      </tr>
      <tr>
        <td>
          <p><b>Feld</b></p>
        </td>
        <td>
          <p><b>Beschreibung</b></p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Nummer</p>
        </td>
        <td>
          <p>Wird automatisch vom System vergeben</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Ziel</p>
        </td>
        <td>
          <p>Hier kann eine LVS-Lokalität ausgewählt werden (nicht empfohlen).</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Linie</p>
        </td>
        <td>
          <p>Auswahl einer Produktionslinie – In diesem Fall wird „ziel“ deaktiviert und mit dem Bereitstellungsbereich der Produktionslinie belegt.</p>
        </td>
      </tr>
    </tbody>
  </table>
</div>

Es ist zu empfehlen, die EPA-Einstellung „Linie als Default-Quelle“ auf „ja“ eingestellt zu lassen. In diesem Fall wird der Cursor bei Start dieser Maske sofort in das Feld „Linie“ gesetzt.

<div class="table-wrapper">
  <table>
    <tbody>
      <tr>
        <td colspan="3">
          <p><b>Zeilendaten</b></p>
        </td>
      </tr>
      <tr>
        <td>
          <p><b>Wert</b></p>
        </td>
        <td>
          <p><b>Anzeige</b></p>
        </td>
        <td>
          <p><b>Beschreibung</b></p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Liste</p>
        </td>
        <td>
          <p>Ja</p>
        </td>
        <td>
          <p>ListenNr</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Position</p>
        </td>
        <td>
          <p>Ja</p>
        </td>
        <td>
          <p>Laufende Positionsnummer</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Artikel</p>
        </td>
        <td>
          <p>Nein</p>
        </td>
        <td>
          <p>Artikelnummer aus dem Lager der im Kopf gewählten Lokalität</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Artikelbezeichnung</p>
        </td>
        <td>
          <p>Ja</p>
        </td>
        <td>
          <p>Bezeichnung des Artikels</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Partie</p>
        </td>
        <td>
          <p>Nein</p>
        </td>
        <td>
          <p>Partienummer</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Partiebezeichnung</p>
        </td>
        <td>
          <p>Ja</p>
        </td>
        <td>
          <p>Bezeichnung der Partie</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Menge/Anzahl</p>
        </td>
        <td>
          <p>Ja</p>
        </td>
        <td></td>
      </tr>
      <tr>
        <td>
          <p>ME</p>
        </td>
        <td>
          <p>Nein</p>
        </td>
        <td>
          <p>Mengeneinheit. Hier sollte eine LVS-Mengeneinheit gegeben werden. In der EPA-Einstellung „Mengeneinheit aus“ sollte LVS stehen.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Bezeichnung Mengeneinheit</p>
        </td>
        <td>
          <p>Ja</p>
        </td>
        <td></td>
      </tr>
    </tbody>
  </table>
</div>
