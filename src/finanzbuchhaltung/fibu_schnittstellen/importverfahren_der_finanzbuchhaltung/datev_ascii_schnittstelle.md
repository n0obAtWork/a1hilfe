# DATEV ASCII-Schnittstelle

<!-- source: https://amic.de/hilfe/datevasciischnittstelle.htm -->

Hauptmenü > Abschlussarbeiten > DATEV / Import / Export > Import > Funktion **F9** ***Import Starten*** > Funktion **F4** ***Importdatei lesen***

Diese Schnittstelle steht zur Verfügung, wenn eine DATEV Lizenz vorhanden ist.

Es existiert im DATEV-Lohnprogramm die Möglichkeit Daten im einfachen CSV-Format auszugeben. Dabei werden die Daten nicht in den üblichen DATEV-Formaten **KNE**(Kontonummernerweiterung) bzw. **OBE** (Ordnungsbegriffserweiterung) geliefert, sonder in einem einfachen ASCII-Format.

Beim Einspielen der Daten wird die Periode anhand des Belegdatums bestimmt.

Sind für das Gegenkonto in den Stammdaten die Steuerklasse und der Steuerschlüssel hinterlegt und bei „Sperre Steuerschlüssel“ der Wert „Fest“ hinterlegt, so werden diese Werte für diesen Buchungssatz herangezogen und die Steuer wird errechnet. Dabei hängt es von der Steuerklasse ab, ob der Betrag in der Exportdatei als Nettobetrag (bei Steuerklasse 1 oder 101) oder als Bruttobetrag (bei Steuerklasse 2 oder 102) interpretiert wird.

Beispiel:

Für das Konto 1755 ist die Steuerklasse 2 hinterlegt. In der Importdatei steht der Betrag 14,06 €. Es wird folgender Buchungssatz gebildet:

<table class="AMICOlavsTabelle" style="BORDER-COLLAPSE: collapse" cellspacing="0" cellpadding="0" border="0"><tbody><tr style="HEIGHT: 13.65pt"><td style="HEIGHT: 13.65pt; WIDTH: 11.5%; PADDING-BOTTOM: 0pt; PADDING-TOP: 0pt; PADDING-LEFT: 5.4pt; PADDING-RIGHT: 5.4pt" valign="top" width="11%">4100</td><td style="HEIGHT: 13.65pt; WIDTH: 9.28%; PADDING-BOTTOM: 0pt; PADDING-TOP: 0pt; PADDING-LEFT: 5.4pt; PADDING-RIGHT: 5.4pt" valign="top" width="9%">an</td><td style="HEIGHT: 13.65pt; WIDTH: 15.94%; PADDING-BOTTOM: 0pt; PADDING-TOP: 0pt; PADDING-LEFT: 5.4pt; PADDING-RIGHT: 5.4pt" valign="top" width="15%">1755</td><td style="HEIGHT: 13.65pt; WIDTH: 37.62%; PADDING-BOTTOM: 0pt; PADDING-TOP: 0pt; PADDING-LEFT: 5.4pt; PADDING-RIGHT: 5.4pt" valign="top" width="37%">14,06</td><td style="HEIGHT: 13.65pt; WIDTH: 25.68%; PADDING-BOTTOM: 0pt; PADDING-TOP: 0pt; PADDING-LEFT: 5.4pt; PADDING-RIGHT: 5.4pt" valign="top" width="25%">12,12</td></tr><tr style="HEIGHT: 13.65pt"><td style="HEIGHT: 13.65pt; WIDTH: 11.5%; PADDING-BOTTOM: 0pt; PADDING-TOP: 0pt; PADDING-LEFT: 5.4pt; PADDING-RIGHT: 5.4pt" valign="top" width="11%"><span style="COLOR: black"></span>&nbsp;</td><td style="HEIGHT: 13.65pt; WIDTH: 9.28%; PADDING-BOTTOM: 0pt; PADDING-TOP: 0pt; PADDING-LEFT: 5.4pt; PADDING-RIGHT: 5.4pt" valign="top" width="9%"><span style="COLOR: black"></span>&nbsp;</td><td style="HEIGHT: 13.65pt; WIDTH: 15.94%; PADDING-BOTTOM: 0pt; PADDING-TOP: 0pt; PADDING-LEFT: 5.4pt; PADDING-RIGHT: 5.4pt" valign="top" width="15%">1775</td><td style="HEIGHT: 13.65pt; WIDTH: 37.62%; PADDING-BOTTOM: 0pt; PADDING-TOP: 0pt; PADDING-LEFT: 5.4pt; PADDING-RIGHT: 5.4pt" valign="top" width="37%"><span style="COLOR: black"></span>&nbsp;</td><td style="HEIGHT: 13.65pt; WIDTH: 25.68%; PADDING-BOTTOM: 0pt; PADDING-TOP: 0pt; PADDING-LEFT: 5.4pt; PADDING-RIGHT: 5.4pt" valign="top" width="25%">1.94</td></tr></tbody></table>

Satzaufbau

Jede Zeile enthält einen Datensatz und die einzelnen Werte sind durch Semikolon getrennt. Abgeschlossen werden die Zeilen mit CR/LF:  
    

Beispieldaten:

40000 H;;4001;200607;;3007;1711;;;;;"Aushilfslohn"  
800 H;;4012;200607;;3007;1711;;;;;"Pausch.Lohnsteuer"  
11200 H;;4031;200607;;3007;1711;;;;;"Gesetzl.Soz.Abgaben AG"

Die Felder haben folgende Bedeutung:

| Feld | Besonderheiten |
| --- | --- |
| Umsatz | Beinhaltet zwei nachkommastellen ohne Dezimalpunkt. Enthält S bzw. H also:  
800 H ⇨ 8,00 Haben | |
| Frei |   
    
 | |
| Gegenkonto |   
    
 | |
| Belegfeld1 | Hier steht Jahr und Periode in der Form YYYYPP.  
Die Jahrnummer wird ins Datum übernommen.  
Beispiel: 200607  
    
 | |
| Frei |   
    
 | |
| Datum | Belegdatum in der Form TTMM. Beispiel: 3007  
    
 | |
| Konto | Hauptkonto  
    
 | |
| KostFeld1 | Kostenstelle. Muss in A.eins so existieren.  
    
 | |
| KostFeld2 | Wird nicht ausgewertet  
    
 | |
| KostMenge | Wird nicht ausgewertet  
    
 | |
| Frei |   
    
 | |
| Buchungstext | Buchungstext wird dem Gegenkonto zugeordnet  
    
 | |
