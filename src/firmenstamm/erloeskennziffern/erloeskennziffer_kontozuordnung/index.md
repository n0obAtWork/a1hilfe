# Erlöskennziffer / Kontozuordnung

<!-- source: https://amic.de/hilfe/_erlskennzifferkontoz.htm -->

Hauptmenü > Administration > Erlöskennziffern > Erlöskennziffer/Kontozuordnung

oder Direktsprung **[EKZZ]**

Hier erfolgt die Verknüpfung der Elemente

- Erlöskennziffer
- Gültigkeit der Eintragungen
- Steuerschlüssel
- Erlösklasse
- Steuergruppe
- Buchungsklasse

mit den Konten der Finanzbuchhaltung. Hier kann man die Bearbeitung wie bei der normalen Stammdatenpflege Datensatz für Datensatz vornehmen oder aber ganze Gruppen von Datensätzen gleichzeitig ändern. Für die gleichzeitige Bearbeitung der Datensätze kann man unter „gültig ab“ in den Feldern Steuerschlüssel, Erlösklasse bzw. Steuergruppe einen Haken setzen.

![](../../../ImagesExt/image8_30.jpg)

Setzt man z.B. beim Steuerschlüssel den Haken, so werden in der Datentabelle alle möglichen Kombinationen für Erlösklasse und Steuergruppe angezeigt. In der so entsehenden Übersicht kann man schnell erkennen, wenn Konten falsch zugeordnet sind. Die Felder rechts von den Haken geben die Sortierungsreihenfolge an. Sie wird immer in der Reihenfolge gesetzt, in der man die Haken setzt.

Die Schlüsselfelder Steuerschlüssel, Erlösklasse und Steuergruppe links sind auch im Ändern-Fall aktiv, wobei man jedoch nicht die Werte ändert, sondern die anzuzeigenden Daten auswählen kann.

<details>
<summary>Felder der Erlöskennziffer / Kontozuordnung:</summary>

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
          <p>EKZ Nummer</p>
        </td>
        <td>
          <p>Die Erlöskennziffer, die im Artikel hinterlegt ist.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Gültig ab</p>
        </td>
        <td>
          <p>Mit Hilfe der Angabe eines Datums hat man die Möglichkeit zukünftige Änderungen der Konten für die Kombination aus EKZ Nummer, Erlösklasse, Steuerschlüssel und Buchklasse vorab in die Datenbank einzupflegen um dann zum entsprechenden Datum Buchungen auf den richtigen Konten zu erhalten.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Steuerschlüssel</p>
        </td>
        <td>
          <p>Es ist möglich, Erlöse nach steuerlichen Gesichtspunkten zu differenzieren (Verprobung Umsatzsteuervoranmeldung). Die Definition der Steuerschlüssel erfolgt bekannt­lich im Rahmen der Firmenkonstanten unter dem Punkt Steuerschlüssel. Der Steuerschlüssel wird im Artikelstamm hinterlegt. Der Steuerschlüssel 0 (Null) hat für das Erlöskennzifferwesen DEFAULT-Funktion. Daher sollte er nicht als 0,00 % Steuer definiert werden.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Erlösklasse</p>
        </td>
        <td>
          <p>Kundenspezifische Zuordnung.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Steuergruppe</p>
        </td>
        <td></td>
      </tr>
      <tr>
        <td>
          <p>Buchklasse</p>
        </td>
        <td>
          <p>Per Buchklasse werden unterschiedliche Buchungstypen auf verschiedene Erlös- und Aufwandskonten gelenkt. Buchklassen sind in A.eins festgelegt und können nicht geändert oder erweitert werden.<br><br></p>
          <p>Bedeutung der Buchklasse:</p>
          <p>&nbsp;0: DEFAULT Fehlwert (alles andere)</p>
          <p>&nbsp;1: Normal-Buchungen (Waren-Ein- oder -Verkauf)</p>
          <p>&nbsp;2: Erlös-/Aufwandsschmälerungen</p>
          <p>&nbsp;Hierunter laufen folgende Warenpositionsmechanismen:</p>
          <ul>
            <li>sämtliche Zu- / Abschläge</li>
            <li>sämtliche Rabatte<br>3. Frachten<br>5. Gutschriften<br>Hierunter fallen die Vorgangsklassen 800 und 1800.<br>&nbsp;Gutschriften lassen sich somit buchungstechnisch anders<br>&nbsp;behandeln als Storno-Belege.<br>Die Buchungsmaschine arbeitet bzgl. der Buchungsklasse nach folgender Logik:<br>5 -&gt; 1 -&gt; 0<br>3 -&gt; 1 -&gt; 0<br>2 -&gt; 1 -&gt; 0<br>1 -&gt; 0<br>0<br>Beispiel:<br>Ist für die Buchklasse 5 (Gutschrift) eine explizite Kontenzuordnung definiert, so wird diese verwendet. Falls nicht, wird eine solche in Buchklasse 1 gesucht. Endet auch dort die Suche ohne Erfolg, so findet die Fehlwerteinstellung 0 Anwendung.</li>
          </ul>
        </td>
      </tr>
      <tr>
        <td>
          <p>Erlöskonto</p>
        </td>
        <td>
          <p>Erlöskonto, auf dem die Verkäufe verbucht werden sollen.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Aufwandskonto</p>
        </td>
        <td>
          <p>Aufwandskonto, auf dem die Einkäufe verbucht werden sollen</p>
        </td>
      </tr>
    </tbody>
  </table>
</div>

</details>

<details>
<summary>Felder des Kontoregister</summary>

Diese Felder werden nur angezeigt, wenn der Steuerparameter ([Steuerparameter 720](../../steuerparameter/optionen_warenwirtschaft/mengenbuchung_bei_fibu_uebertrag_spa_720.md) „Mengenbuchung bei dem Übertrag in die Finanzbuchhaltung“) entsprechend gesetzt ist. Das Bestands-Erlöskonto und das Bestands-Aufwandskonto kann im Sachkontenstamm fest einem zugehörigen Erlös, oder Aufwandskonto zugewiesen werden. Es ist dann hier nicht mehr änderbar, sondern nur im [Sachkontenstamm](../../../finanzbuchhaltung/stammdaten_der_fibu/sachkonten.md#ZugehörigesStatistikkonto).

<div class="table-wrapper">
  <table>
    <tbody>
      <tr>
        <td colspan="2">
          <p><strong>Bestandskonten für Mengenbuchung beim Übertrag in die Finanzbuchhaltung</strong></p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Erlöskonto</p>
        </td>
        <td>
          <p>Konto auf dem die Mengen für den Verkauf gebucht werden sollen</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Aufwandskonto</p>
        </td>
        <td>
          <p>Konto auf dem die Mengen für den Einkauf gebucht werden sollen</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Erlössammelkonto</p>
        </td>
        <td>
          <p>Gegenkonto zum Erlöskonto</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Aufwandssammelkonto</p>
        </td>
        <td>
          <p>Gegenkonto zum Aufwandskonto</p>
        </td>
      </tr>
    </tbody>
  </table>
</div>

</details>

Die Bestandsbewertungskonten werden auf der Maske nur angezeigt, wenn der zugehörige Einrichterparameter auf **Ja** steht.

Diese Konten werden für Buchungen von Werten aus der [permanenten Inventur](../../../abschluesse_inventur/permanente_inventur/fibu_buchung_der_permanenten_inventuren.md) verwendet.

<div class="table-wrapper">
  <table>
    <tbody>
      <tr>
        <td colspan="2">
          <p><strong>Bestandsbewertungskonten</strong></p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Zugangskonto</p>
        </td>
        <td>
          <p>Konto für die Buchung des SOLL-Bestandes</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Abgangskonto</p>
        </td>
        <td>
          <p>Konto für die Buchung des IST-Bestandes</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Inventurkonto</p>
        </td>
        <td>
          <p>Konto für die Buchung der Bestandsdifferenz</p>
        </td>
      </tr>
    </tbody>
  </table>
</div>

<p class="just-emphasize">Einrichterparameter</p>

| Einrichterparameter | Beschreibung | Vorbelegung |
| --- | --- | --- |
| Sollen die Bestandsbewertungskonten auch angezeigt werden? | Bei ‚Ja‘ werden die Felder auf der Maske eingeblendet. | Nein |

<p class="siehe-auch">Siehe auch:</p>

- [Erlöskennziffer Kontozuordnung bei Steuersatzänderung](./erloeskennziffer_kontozuordnung_bei_steuersatzaenderung.md)
