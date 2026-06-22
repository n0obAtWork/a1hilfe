# BWG-Schnittstelle

<!-- source: https://amic.de/hilfe/bwgschnittstelle.htm -->

Hauptmenü > Abschlussarbeiten > DATEV / Import / Export > Import > Funktion **F9** ***Import Starten*** > Funktion **F4** ***Importdatei lesen***

Direktsprung **[FIIM]**

Bei dieser Schnittstelle handelt es sich um den Import der Lohndaten aus der BWG-Software. Es handelt sich hierbei um reine Sachkontenbuchungen.

Beim Einspielen der Daten wird die Periode anhand des Belegdatums bestimmt.

Sind für das Gegenkonto in den Stammdaten die Steuerklasse und der Steuerschlüssel hinterlegt und bei „Sperre Steuerschlüssel“ der Wert „Fest“ hinterlegt, so werden diese Werte für diesen Buchungssatz herangezogen und die Steuer wird errechnet. Dabei hängt es von der Steuerklasse ab, ob der Betrag in der Exportdatei als Nettobetrag (bei Steuerklasse 1 oder 101) oder als Bruttobetrag (bei Steuerklasse 2 oder 102) interpretiert wird.

Beispiel:

Für das Konto 1755 ist die Steuerklasse 2 hinterlegt. In der Importdatei steht der Betrag 14,06 €. Es wird folgender Buchungssatz gebildet:

<table class="AMICOlavsTabelle" style="BORDER-COLLAPSE: collapse" cellspacing="0" cellpadding="0" border="0"><tbody><tr style="HEIGHT: 13.65pt"><td style="HEIGHT: 13.65pt; WIDTH: 11.5%; PADDING-BOTTOM: 0pt; PADDING-TOP: 0pt; PADDING-LEFT: 5.4pt; PADDING-RIGHT: 5.4pt" valign="top" width="11%">4100</td><td style="HEIGHT: 13.65pt; WIDTH: 9.28%; PADDING-BOTTOM: 0pt; PADDING-TOP: 0pt; PADDING-LEFT: 5.4pt; PADDING-RIGHT: 5.4pt" valign="top" width="9%">an</td><td style="HEIGHT: 13.65pt; WIDTH: 15.94%; PADDING-BOTTOM: 0pt; PADDING-TOP: 0pt; PADDING-LEFT: 5.4pt; PADDING-RIGHT: 5.4pt" valign="top" width="15%">1755</td><td style="HEIGHT: 13.65pt; WIDTH: 37.62%; PADDING-BOTTOM: 0pt; PADDING-TOP: 0pt; PADDING-LEFT: 5.4pt; PADDING-RIGHT: 5.4pt" valign="top" width="37%">14,06</td><td style="HEIGHT: 13.65pt; WIDTH: 25.68%; PADDING-BOTTOM: 0pt; PADDING-TOP: 0pt; PADDING-LEFT: 5.4pt; PADDING-RIGHT: 5.4pt" valign="top" width="25%">12,12</td></tr><tr style="HEIGHT: 13.65pt"><td style="HEIGHT: 13.65pt; WIDTH: 11.5%; PADDING-BOTTOM: 0pt; PADDING-TOP: 0pt; PADDING-LEFT: 5.4pt; PADDING-RIGHT: 5.4pt" valign="top" width="11%"></td><td style="HEIGHT: 13.65pt; WIDTH: 9.28%; PADDING-BOTTOM: 0pt; PADDING-TOP: 0pt; PADDING-LEFT: 5.4pt; PADDING-RIGHT: 5.4pt" valign="top" width="9%"></td><td style="HEIGHT: 13.65pt; WIDTH: 15.94%; PADDING-BOTTOM: 0pt; PADDING-TOP: 0pt; PADDING-LEFT: 5.4pt; PADDING-RIGHT: 5.4pt" valign="top" width="15%">1775</td><td style="HEIGHT: 13.65pt; WIDTH: 37.62%; PADDING-BOTTOM: 0pt; PADDING-TOP: 0pt; PADDING-LEFT: 5.4pt; PADDING-RIGHT: 5.4pt" valign="top" width="37%"></td><td style="HEIGHT: 13.65pt; WIDTH: 25.68%; PADDING-BOTTOM: 0pt; PADDING-TOP: 0pt; PADDING-LEFT: 5.4pt; PADDING-RIGHT: 5.4pt" valign="top" width="25%">1.94</td></tr></tbody></table>

Satzaufbau der Datei

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
