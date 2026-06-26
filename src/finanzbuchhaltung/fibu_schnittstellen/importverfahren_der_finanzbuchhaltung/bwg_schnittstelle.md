# BWG-Schnittstelle

<!-- source: https://amic.de/hilfe/bwgschnittstelle.htm -->

Hauptmenü > Abschlussarbeiten > DATEV / Import / Export > Import > Funktion **F9** ***Import Starten*** > Funktion **F4** ***Importdatei lesen***

Direktsprung **[FIIM]**

Bei dieser Schnittstelle handelt es sich um den Import der Lohndaten aus der BWG-Software. Es handelt sich hierbei um reine Sachkontenbuchungen.

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

#### Satzaufbau der Datei

Jede Zeile ist 128 Bytes lang und endet mit CR/LF. Die Daten stehen mit einer festen Länge hintereinander.

| Stelle | Länge | Format | Bedeutung |
| --- | --- | --- | --- |
| 1 | 8 | Rechts/vorl. Null | Übergabekonto |
| 9 | 15 | Rechts/vorl. Null | Betrag Soll in Cent |
| 24 | 15 | Rechts/vorl. Null | Betrag Haben in Cent |
| 39 | 8 | Rechts/vorl. Null | Gegenkonto |
| 47 | 8 | TTMMJJJJ | Übergabedatum |
| 55 | 10 | Links | Kostenstelle |
| 65 | 4 | Rechts/vorl. Null | Personalnummer |
| 69 | 11 | Links | Projektnummer |
| 80 | 8 | Rechts/vorl. Null | Stunden |
| 88 | 1 | Rechts/vorl. Null | Sollhaben immer 1 |
| 89 | 38 | | Filler |
| 127 | 2 | | CR/LF |

Die Felder Personalnummer, Projektnummer und Stunden werden nicht übernommen. Das Übergabedatum wird als Belegdatum verwendet.
