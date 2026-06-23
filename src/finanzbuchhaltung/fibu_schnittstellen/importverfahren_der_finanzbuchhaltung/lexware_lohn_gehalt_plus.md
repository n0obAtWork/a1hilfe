# Lexware Lohn & Gehalt Plus

<!-- source: https://amic.de/hilfe/lexwarelohngehaltplus.htm -->

Hauptmenü > Abschlussarbeiten > DATEV / Import / Export > Import > Funktion **F9** Import Starten > Funktion **F4** ***Importdatei lesen***

Direktsprung ****[FIIM]****

Bei dieser Schnittstelle handelt es sich um den Import der Lohndaten aus der Software "Lexware Lohn und Gehalt Plus". Es handelt sich hierbei um reine Sachkontenbuchungen. Den Export der Buchungsdaten findet man in dieser Software im Menü unter:

Datei -> Exportieren -> ASCII -> Buchungsliste

Dort kann man noch einige Optionen einstellen. Da die in A.eins vorgegebene Schnittstelle davon ausgeht, dass die Kostenstellen mit importiert werden sollen, so muss bei Buchungsliste "Aufgeteilt nach Kostenstellen" eingetragen sein.

Beim Einspielen der Daten wird die Periode anhand des Belegdatums bestimmt.

Sind für das Gegenkonto in den Stammdaten die Steuerklasse und der Steuerschlüssel hinterlegt und bei „Sperre Steuerschlüssel“ der Wert „Fest“ hinterlegt, so werden diese Werte für diesen Buchungssatz herangezogen und die Steuer wird errechnet. Dabei hängt es von der Steuerklasse ab, ob der Betrag in der Exportdatei als Nettobetrag (bei Steuerklasse 1 oder 101) oder als Bruttobetrag (bei Steuerklasse 2 oder 102) interpretiert wird.

Beispiel:

Für das Konto 1755 ist die Steuerklasse 2 hinterlegt. In der Importdatei steht der Betrag 14,06 €. Es wird folgender Buchungssatz gebildet:

| 4100 | An | 1755 | 14,06 | 12,12 |
| --- | --- | --- | --- | --- |
| | | 1775 | | 1.94 |

<p class="just-emphasize">Satzaufbau</p>

In der Ersten Zeile der zu importierenden Datei stehen die Feldbezeichnungen getrennt durch ein Semikolon. Abgeschlossen ist die Zeile mit CR/LF:  
    

Belegdatum;Belegnummer;Buchungstext;Buchungsbetrag;Währung;Sollkonto;Habenkonto;Kostenstelle 1

Diese Zeile wird ignoriert. Danach kommen die Daten getrennt durch Semikolon:  
    

31.01.2003;LG03030001;Lohn;2.208,00;EUR;4110;1755;  
31.01.2003;LG03030002;Gehalt;16.600,00;EUR;4120;1755;  
31.01.2003;LG03030003;Auszubildendenvergütung;285,00;EUR;4120;1755;
