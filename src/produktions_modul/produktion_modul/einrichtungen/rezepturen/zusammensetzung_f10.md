# Zusammensetzung F10

<!-- source: https://amic.de/hilfe/_zusammensetzungf10.htm -->

<div class="table-wrapper">
  <table>
    <tbody>
      <tr>
        <td colspan="3">
          <p><strong>Felder</strong></p>
        </td>
      </tr>
      <tr>
        <td colspan="2">
          <p>Rezepturgruppe</p>
        </td>
        <td>
          <p>Rezepturgruppe zu der die Rezeptur gehören soll</p>
        </td>
      </tr>
      <tr>
        <td colspan="2">
          <p>Rezepturnummer</p>
        </td>
        <td>
          <p>Nummer der Rezeptur für die die Zusammensetzung angezeigt wird.</p>
        </td>
      </tr>
      <tr>
        <td colspan="2">
          <p>Mengenanteil Hauptprodukt</p>
        </td>
        <td>
          <p>Nur sichtbar und eingebbar bei NzuM-Produktion</p>
        </td>
      </tr>
      <tr>
        <td colspan="2">
          <p>Wertanteil Hauptprodukt</p>
        </td>
        <td>
          <p>Nur sichtbar und eingebbar bei NzuM-Produktion</p>
        </td>
      </tr>
      <tr>
        <td>
          <p><b>Tabellenfelder</b></p>
        </td>
        <td colspan="2">
          <p>Nur für NzuM-Produktion und Vermahlung sichtbar</p>
        </td>
      </tr>
      <tr>
        <td colspan="2">
          <p>Abgang/Zugang</p>
        </td>
        <td>
          <p>Über die F3-Auswahl kann zwischen Zugang und Abgang für die einzelne Komponente gewählt werden.</p>
        </td>
      </tr>
      <tr>
        <td colspan="2">
          <p>Anteil Abgang</p>
        </td>
        <td>
          <p>In diesem Feld ist der Anteil für den Abgang einzugeben, wenn beim Feld ‚Abgang/Zugang‘ Abgang gewählt wurde.</p>
        </td>
      </tr>
      <tr>
        <td colspan="2">
          <p>Anteil Zugang</p>
        </td>
        <td>
          <p>In diesem Feld ist der Anteil für den Zugang einzugeben, wenn beim Feld ‚Abgang/Zugang‘ Zugang gewählt wurde.</p>
        </td>
      </tr>
      <tr>
        <td colspan="2">
          <p>Wertanteil Zugang</p>
        </td>
        <td>
          <p>Nicht sichtbar für Vermahlung</p>
        </td>
      </tr>
      <tr>
        <td>
          <p><b>Summen</b></p>
        </td>
        <td colspan="2">
          <p>Nur für NzuM-Produktion und Vermahlung sichtbar</p>
        </td>
      </tr>
      <tr>
        <td colspan="2">
          <p>Anteile Abgang</p>
        </td>
        <td>
          <p>Anzeigefeld</p>
        </td>
      </tr>
      <tr>
        <td colspan="2">
          <p>Anteile Zugang</p>
        </td>
        <td>
          <p>Anzeigefeld</p>
        </td>
      </tr>
      <tr>
        <td colspan="2">
          <p>Wertanteile Zugang</p>
        </td>
        <td>
          <p>Anzeigefeld, nur bei NzuM-Produktion sichtbar</p>
        </td>
      </tr>
    </tbody>
    <tbody>
      <tr>
        <td></td>
        <td></td>
        <td></td>
      </tr>
    </tbody>
  </table>
</div>

Die Zusammensetzung einer Rezeptur kann sowohl aus Artikel als auch aus weiteren Rezepten bestehen. In der Tabelle erkennt man Rezepte daran, dass in der Spalte „Artikel-Rezeptnr“ die Rezeptgruppe und -variante steht, außerdem ist das Feld in diesem Fall farblich hervorgehoben.

In der Auswahlliste der Artikel unterscheiden sich Rezepte von Artikel darin, dass Rezepte bei „Rezept“ „Ja“ und bei „Variante“ ein Wert ungleich 0 steht, Artikel haben dagegen in diesen Feldern immer den Wert „Nein“ bzw. 0.

Hier kann je Rezeptposition festgelegt werden, ob es sich um eine Wertposition (wird bei Mengenrechnung nicht berücksichtigt), um eine Pauschalposition (wird unabhängig von der Produktmenge berücksichtigt) oder um eine Position mit Fixpreis (Preisfindung unabhängig von der unter **[SPA]** eingestellten Methode) handelt.

Hinweis

Es ist bei der Rezeptureingabe im Mehrlagerbetrieb auf die korrekte Lagerzuordnung zu achten. Es könnte erwünscht oder aber unerwünscht sein, dass Produkt und Komponenten von verschiedenen Lagern zugeordnet werden.

### Details F5

Diese Funktion ist nur sichtbar und aufrufbar, wenn eine Zeile in der Tabelle angewählt wurde.  
Hier kann man den Schwund in Prozent und die Mengeneinheit Anteil angeben.

### Preise F11

Diese Funktion ist nur aufrufbar, wenn in der Spalte Fixpreis für eine Komponente „Ja“ eingetragen wurde.

Hier ist die Preispflege für die einzelnen Komponenten möglich.

### Rezept auflösen F6

Ist in der Zusammensetzung der Rezeptur ein Unterrezept enthalten kann man sich die Komponenten des Unterrezeptes mit anzeigen lassen.
