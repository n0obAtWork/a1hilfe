# Einrichtung des externen Kassendisplays

<!-- source: https://amic.de/hilfe/einrichtungdesexternenkassendi.htm -->

Die Einrichtung erfolgt in der Kassenverwaltung. Jeder logischen Kasse ist ein Kassensystem zugeordnet. Dieses Kassensystem beschreibt die Hardwareeinheit. Technisch gesehen sind alle der Einstellungen dem Kassensystem zugeordnet.

Auf der Registerkarte „Geräte“ findet sich im unteren Bereich ein Rahmen mit der Überschrift [Hardware externes Display](../kassenverwaltung_logische_kasse.md#Log_Kasse_Geraete_ExtDisplay). Hier richten Sie ein Bildschirmdisplay ein.

Auf der Registerkarte „Anzeige“ findet sich eine Tabelle mit Feldern, die zur Anzeige eingerichtet werden können.

Diese Tabelle mit Anzeigefeldern ist einem Anzeigeschema zugeordnet, dass Sie in dem Feld „Schema“ auswählen müssen. Sie können auch mit der Funktion „Neues Anzeigeschema“ ein neues Schema erstellen.

**Hinweis:**

Bitte beachten Sie, dass die Angaben des Schemas stets für alle Anzeigen gelten, die das gleiche Schema verwenden!!!

<div class="table-wrapper">
  <table>
    <tbody>
      <tr>
        <td colspan="2">
          <p><strong>Registerkarte Anzeige Name</strong></p>
          <p>Folgende Namen können eingerichtet werden</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>BON</p>
        </td>
        <td>
          <p>Hier wird der laufende Bon dargestellt</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>LDC</p>
        </td>
        <td>
          <p>In dieses Feld wird das Währungskennzeichen geschrieben. *)</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>LDT</p>
        </td>
        <td>
          <p>Text der Zeilendisplayzeile *)</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>LDV</p>
        </td>
        <td>
          <p>Wert der Zeilendisplayzeile *)</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>LINEDISPLAY</p>
        </td>
        <td>
          <p>Zeilendisplay – Hier werden Daten der o.g. Werte zusammen dargestellt.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>SCREEN</p>
        </td>
        <td>
          <p>Diese Konfiguration dient der Größendimensionierung des Fensters</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>QRCODE</p>
        </td>
        <td>
          <p>QR-Code für AnyBill, wenn die Lizenz vorhanden ist. Diese Einrichtung ist nur bei entsprechender Lizenz verfügbar.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>SUMME</p>
        </td>
        <td>
          <p>Hier wird die Summe des Bons angezeigt.</p>
        </td>
      </tr>
    </tbody>
  </table>
</div>

(\*) Diese Zeilen werden nicht auf dem ScreenDisplay dargestellt. Die dienen der Füllung und Ausrichtung in einem Feld namens LINEDISPLAY. Wird beispielsweise das Währungskennzeichen (LDC) linksbündig angegeben, so wird dies links neben den Wert (LDV) geschrieben.

Wird kein Zeilendisplay und kein Feld mit dem Namen LINEDISPLAY verwendet, so können diese Felder entfallen.

<div class="table-wrapper">
  <table>
    <tbody>
      <tr>
        <td colspan="2">
          <p><strong>Positionen</strong></p>
          <p>In den nachfolgenden Spalten werden die Positionen der Objekte eingerichtet. Alle Angaben sind in Pixel anzugeben.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>TOP</p>
        </td>
        <td>
          <p>Position relativ vom oberen Rand</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>LEFT</p>
        </td>
        <td>
          <p>Position vom linken Rand</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>HEIGHT</p>
        </td>
        <td>
          <p>Höhe des Objekts</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>WIDTH</p>
        </td>
        <td>
          <p>Breite des Objekts</p>
        </td>
      </tr>
    </tbody>
  </table>
</div>

<div class="table-wrapper">
  <table>
    <tbody>
      <tr>
        <td colspan="2">
          <p><strong>Feldtyp</strong></p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Head</p>
        </td>
        <td>
          <p>Es wird in der Textbox stets der obere Teil des Textes angezeigt. Bei mehrzeiligen Texten wird der untere Teil abgeschnitten. Der zuvor gegebene Text wird gelöscht.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Tail</p>
        </td>
        <td>
          <p>Es wird in der Textbox stets der untere Teil des Textes angezeigt. Bei mehrzeiligen Texten wird der obere Text abgeschnitten. Der zuvor gegebene Text wird gelöscht.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>AddUp</p>
        </td>
        <td>
          <p>Es wird der Text im oberen Teil der Textbox hinzugefügt. Der zuvor gegebene Text bleibt darunter erhalten.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>AddDn</p>
        </td>
        <td>
          <p>Es wird der Text im unteren Teil der Textbox hinzugefügt. Der zuvor gegebene Text bleibt darüber erhalten.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>LineDisplay</p>
        </td>
        <td>
          <p>Diese Textbox verhält sich wie ein Zeilendisplay, zeigt also Text, Währung und Betrag in zwei Zeilen an.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>LineDisplayText</p>
        </td>
        <td>
          <p>Für die Anzeige eines Textes im Zeilendisplay muss dieser Typ eingerichtet sein.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>LineDisplayValue</p>
        </td>
        <td>
          <p>Für die Anzeige eines Betrages im Zeilendisplay muss dieser Typ eingerichtet sein.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>LineDisplayCurrency</p>
        </td>
        <td>
          <p>Für die Anzeige eines dreistelligen Währungskennzeichens im Zeilendisplay muss dieser Typ eingerichtet sein.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Screen</p>
        </td>
        <td>
          <p>Dieser Feldtyp gibt an, dass hier Angaben zur Größe und der Position des Anzeigefensters eingetragen werden.</p>
        </td>
      </tr>
    </tbody>
  </table>
</div>

<div class="table-wrapper">
  <table>
    <tbody>
      <tr>
        <td colspan="2">
          <p><strong>Ausrichtung</strong></p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Links</p>
        </td>
        <td>
          <p>Die Anzeige erfolgt linksbündig in der Textbox</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Zentriert</p>
        </td>
        <td>
          <p>Die Anzeige erfolgt zentriert in der Textbox</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Rechts</p>
        </td>
        <td>
          <p>Die Anzeige erfolgt rechtsbündig in der Textbox</p>
        </td>
      </tr>
    </tbody>
  </table>
</div>

<p class="just-emphasize">Font</p>

Geben Sie hier den Namen der zu verwendenden Schriftart an. Bitte bedenken Sie, dass Sie für den optischen Eindruck eines Zeilendisplays oder eines ASCII-Bons eine nicht-proportionale Schriftart wählen sollten, bei der alle Zeichen die gleiche Breite haben. Anderenfalls könnte die Anzeige ungewohnt ungeordnet aussehen.

<p class="just-emphasize">Fontsize</p>

Geben Sie hier die Schriftgröße ein

<p class="just-emphasize">Text</p>

Hier können Sie einen Text für einen Test bzw. die Initialisierung des Displays eingeben
