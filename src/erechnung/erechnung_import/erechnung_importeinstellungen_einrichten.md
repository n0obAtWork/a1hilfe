# eRechnung Importeinstellungen einrichten

<!-- source: https://amic.de/hilfe/erechnungimporteinstellungenei.htm -->

In der Anwendung eRechnung **[XRE],** Variante ***Import-Vorgänge*** hat die Funktion Importeinstellungen bearbeiten.

Hier richten Sie die Importeinstellungen der eRechnung ein.

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
          <p>Fehlerbehandlung HTML</p>
        </td>
        <td>
          <p>Gibt an, ob eine nicht erfolgreiche Erstellung einer HTML-Visualisierung als Fehler gelten soll (Default <strong>Ja</strong>)</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Fehlerbehandlung Kunde</p>
        </td>
        <td>
          <p>Gibt an, ob eine nicht erfolgreiche Findung eines Kunden/Lieferanten als Fehler gelten soll (Default <strong>Nein</strong>)</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Fehlerbehandlung Validierung</p>
        </td>
        <td>
          <p>Gibt an, ob eine nicht erfolgreiche Validierung eines Imports als Fehler gelten soll (Default <strong>Ja</strong>)</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Fehlerfunktion</p>
        </td>
        <td>
          <p>Gibt eine Datenbankfunktion an, die die Fehlermeldungen eines Imports aufnehmen und z. B. per E-Mail weiterleiten soll.</p>
          <p>Als Eingabeparameter wird die ImportId gegeben.</p>
          <p>Als Vorlage kann hier die ausgelieferte Funktion „AMIC_DEMO_XRE_ImportFehlerFunc“ dienen.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Kundenfindungsfunktion</p>
        </td>
        <td>
          <p>Gibt eine Datenbankfunktion an, die aus den importierten Daten einen Kunden/Lieferanten ermitteln soll.</p>
          <p>Als Eingabeparameter wird die ImportId gegeben, als Ausgabe wird die KundId des Kunden/Lieferanten erwartet.</p>
          <p>Als Vorlage kann hier die ausgelieferte Funktion „AMIC_STD_XRE_ImportKundensuche“ dienen.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Belegflusspostfach Warenwirtschaft</p>
        </td>
        <td>
          <p>Standardbelegflusspostfach für eRechnungsimporte im Bereich Warenwirtschaft</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Belegflusspostfach Finanzbuchhaltung</p>
        </td>
        <td>
          <p>Standardbelegflusspostfach für eRechnungsimporte im Bereich Finanzbuchhaltung</p>
        </td>
      </tr>
    </tbody>
  </table>
</div>
