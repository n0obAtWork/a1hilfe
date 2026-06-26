# DATEV ASCII-Schnittstelle

<!-- source: https://amic.de/hilfe/datevasciischnittstelle.htm -->

Hauptmenü > Abschlussarbeiten > DATEV / Import / Export > Import > Funktion **F9** ***Import Starten*** > Funktion **F4** ***Importdatei lesen***

Diese Schnittstelle steht zur Verfügung, wenn eine DATEV Lizenz vorhanden ist.

Es existiert im DATEV-Lohnprogramm die Möglichkeit Daten im einfachen CSV-Format auszugeben. Dabei werden die Daten nicht in den üblichen DATEV-Formaten **KNE**(Kontonummernerweiterung) bzw. **OBE** (Ordnungsbegriffserweiterung) geliefert, sonder in einem einfachen ASCII-Format.

Beim Einspielen der Daten wird die Periode anhand des Belegdatums bestimmt.

Sind für das Gegenkonto in den Stammdaten die Steuerklasse und der Steuerschlüssel hinterlegt und bei „Sperre Steuerschlüssel“ der Wert „Fest“ hinterlegt, so werden diese Werte für diesen Buchungssatz herangezogen und die Steuer wird errechnet. Dabei hängt es von der Steuerklasse ab, ob der Betrag in der Exportdatei als Nettobetrag (bei Steuerklasse 1 oder 101) oder als Bruttobetrag (bei Steuerklasse 2 oder 102) interpretiert wird.

Beispiel:

Für das Konto 1755 ist die Steuerklasse 2 hinterlegt. In der Importdatei steht der Betrag 14,06 €. Es wird folgender Buchungssatz gebildet:

<div class="table-wrapper">
  <table>
    <tbody>
      <tr>
        <td>
          <p>4100</p>
        </td>
        <td>
          <p>an</p>
        </td>
        <td>
          <p>1755</p>
        </td>
        <td>
          <p>14,06</p>
        </td>
        <td>
          <p>12,12</p>
        </td>
      </tr>
      <tr>
        <td></td>
        <td></td>
        <td>
          <p>1775</p>
        </td>
        <td></td>
        <td>
          <p>1.94</p>
        </td>
      </tr>
    </tbody>
  </table>
</div>

#### Satzaufbau

Jede Zeile enthält einen Datensatz und die einzelnen Werte sind durch Semikolon getrennt. Abgeschlossen werden die Zeilen mit CR/LF:  
    

Beispieldaten:

40000 H;;4001;200607;;3007;1711;;;;;"Aushilfslohn"  
800 H;;4012;200607;;3007;1711;;;;;"Pausch.Lohnsteuer"  
11200 H;;4031;200607;;3007;1711;;;;;"Gesetzl.Soz.Abgaben AG"

Die Felder haben folgende Bedeutung:

<div class="table-wrapper">
  <table>
    <tbody>
      <tr>
        <td>
          <p><strong>Feld</strong></p>
        </td>
        <td colspan="2">
          <p><strong>Besonderheiten</strong></p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Umsatz</p>
        </td>
        <td>
          <p>Beinhaltet zwei nachkommastellen ohne Dezimalpunkt. Enthält S bzw. H also:</p>
          <p>800 H ⇨ 8,00 Haben<br><br></p>
        </td>
        <td></td>
      </tr>
      <tr>
        <td>
          <p>Frei</p>
        </td>
        <td></td>
        <td></td>
      </tr>
      <tr>
        <td>
          <p>Gegenkonto</p>
        </td>
        <td></td>
        <td></td>
      </tr>
      <tr>
        <td>
          <p>Belegfeld1</p>
        </td>
        <td>
          <p>Hier steht Jahr und Periode in der Form YYYYPP.</p>
          <p>Die Jahrnummer wird ins Datum übernommen.</p>
          <p>Beispiel: 200607<br><br></p>
        </td>
        <td></td>
      </tr>
      <tr>
        <td>
          <p>Frei</p>
        </td>
        <td></td>
        <td></td>
      </tr>
      <tr>
        <td>
          <p>Datum</p>
        </td>
        <td>
          <p>Belegdatum in der Form TTMM. Beispiel: 3007<br><br></p>
        </td>
        <td></td>
      </tr>
      <tr>
        <td>
          <p>Konto</p>
        </td>
        <td>
          <p>Hauptkonto<br><br></p>
        </td>
        <td></td>
      </tr>
      <tr>
        <td>
          <p>KostFeld1</p>
        </td>
        <td>
          <p>Kostenstelle. Muss in A.eins so existieren.<br><br></p>
        </td>
        <td></td>
      </tr>
      <tr>
        <td>
          <p>KostFeld2</p>
        </td>
        <td>
          <p>Wird nicht ausgewertet<br><br></p>
        </td>
        <td></td>
      </tr>
      <tr>
        <td>
          <p>KostMenge</p>
        </td>
        <td>
          <p>Wird nicht ausgewertet<br><br></p>
        </td>
        <td></td>
      </tr>
      <tr>
        <td>
          <p>Frei</p>
        </td>
        <td></td>
        <td></td>
      </tr>
      <tr>
        <td>
          <p>Buchungstext</p>
        </td>
        <td>
          <p>Buchungstext wird dem Gegenkonto zugeordnet<br><br></p>
        </td>
        <td></td>
      </tr>
    </tbody>
  </table>
</div>
