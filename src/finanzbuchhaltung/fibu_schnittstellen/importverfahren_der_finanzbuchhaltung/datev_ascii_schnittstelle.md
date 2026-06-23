# DATEV ASCII-Schnittstelle

<!-- source: https://amic.de/hilfe/datevasciischnittstelle.htm -->

Hauptmenü > Abschlussarbeiten > DATEV / Import / Export > Import > Funktion **F9** ***Import Starten*** > Funktion **F4** ***Importdatei lesen***

Diese Schnittstelle steht zur Verfügung, wenn eine DATEV Lizenz vorhanden ist.

Es existiert im DATEV-Lohnprogramm die Möglichkeit Daten im einfachen CSV-Format auszugeben. Dabei werden die Daten nicht in den üblichen DATEV-Formaten **KNE**(Kontonummernerweiterung) bzw. **OBE** (Ordnungsbegriffserweiterung) geliefert, sonder in einem einfachen ASCII-Format.

Beim Einspielen der Daten wird die Periode anhand des Belegdatums bestimmt.

Sind für das Gegenkonto in den Stammdaten die Steuerklasse und der Steuerschlüssel hinterlegt und bei „Sperre Steuerschlüssel“ der Wert „Fest“ hinterlegt, so werden diese Werte für diesen Buchungssatz herangezogen und die Steuer wird errechnet. Dabei hängt es von der Steuerklasse ab, ob der Betrag in der Exportdatei als Nettobetrag (bei Steuerklasse 1 oder 101) oder als Bruttobetrag (bei Steuerklasse 2 oder 102) interpretiert wird.

Beispiel:

Für das Konto 1755 ist die Steuerklasse 2 hinterlegt. In der Importdatei steht der Betrag 14,06 €. Es wird folgender Buchungssatz gebildet:

| 4100 | an | 1755 | 14,06 | 12,12 |
| --- | --- | --- | --- | --- |
| | | 1775 | | 1.94 |

<p class="just-emphasize">Satzaufbau</p>

Jede Zeile enthält einen Datensatz und die einzelnen Werte sind durch Semikolon getrennt. Abgeschlossen werden die Zeilen mit CR/LF:  
    

Beispieldaten:

40000 H;;4001;200607;;3007;1711;;;;;"Aushilfslohn"  
800 H;;4012;200607;;3007;1711;;;;;"Pausch.Lohnsteuer"  
11200 H;;4031;200607;;3007;1711;;;;;"Gesetzl.Soz.Abgaben AG"

Die Felder haben folgende Bedeutung:

| Feld | Besonderheiten |
| --- | --- |
| Umsatz | Beinhaltet zwei nachkommastellen ohne Dezimalpunkt. Enthält S bzw. H also:<br>800 H ⇨ 8,00 Haben<br><br> | |
| Frei | <br><br> | |
| Gegenkonto | <br><br> | |
| Belegfeld1 | Hier steht Jahr und Periode in der Form YYYYPP.<br>Die Jahrnummer wird ins Datum übernommen.<br>Beispiel: 200607<br><br> | |
| Frei | <br><br> | |
| Datum | Belegdatum in der Form TTMM. Beispiel: 3007<br><br> | |
| Konto | Hauptkonto<br><br> | |
| KostFeld1 | Kostenstelle. Muss in A.eins so existieren.<br><br> | |
| KostFeld2 | Wird nicht ausgewertet<br><br> | |
| KostMenge | Wird nicht ausgewertet<br><br> | |
| Frei | <br><br> | |
| Buchungstext | Buchungstext wird dem Gegenkonto zugeordnet<br><br> | |
