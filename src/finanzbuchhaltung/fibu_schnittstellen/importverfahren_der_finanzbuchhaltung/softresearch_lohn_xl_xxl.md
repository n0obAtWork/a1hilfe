# Softresearch Lohn-XL/XXL

<!-- source: https://amic.de/hilfe/softresearchlohnxlxxl.htm -->

Hauptmenü > Abschlussarbeiten > DATEV / Import / Export > Import > Funktion **F9** Import Starten > Funktion **F4** Importdatei lesen

Direktsprung [FIIM]

Bei dieser Schnittstelle handelt es sich um den Import der Lohndaten aus der Software Lohn-XL/XXL. Es handelt sich bei diesen Buchungen um reine Sachkontenbuchungen. Die Lohnsoftware bietet diverse Exportschnittstellen an. In A.eins sind die Schnittstellen F_SR11 und F_SR13 Implementiert.

Satzaufbau F_SR11

| Stelle | Länge | Format | Bedeutung |
| --- | --- | --- | --- |
| 1 | 1 | | Buchungskennzeichen S oder H |
| 2 | 6 | Linksbündig | Kontonummer |
| 8 | 6 | Linksbündig | Gegenkonto |
| 14 | 12 | Linksbündig | Kostenträger (wird ignoriert) |
| 26 | 10 | Linksbündig | Kostenstelle |
| 36 | 12 | Rechtsbündig (12.2) | Betrag |
| 48 | 6 | MMJJJJ | Monat/Jahr |
| 54 | 8 | TTMMJJJJ | Tag/Monat/Jahr |
| 62 | 25 | Linksbündig | Buchungstext |
| 87 | 6 | Linksbündig | Personalnummer (wird ignoriert) |
| 93 | 3 | Linksbündig | Lohnartnummer (wird ignoriert) |
| 96 | 9 | Linksbündig | Einheit (Stunde/Tage) (wird ignoriert) |
| 105 | 3 | Linksbündig | Währungsbezeichnung DM oder EUR |
| 108 | 19 | | Leer |
| 127 | 2 | | CR/LF |

Satzaufbau F_SR13

| Stelle | Länge | Format | Bedeutung |
| --- | --- | --- | --- |
| 1 | 3 | | Buchungsnummer |
| 4 | 25 | | Buchungstext |
| 29 | 10 | TT.MM.JJJJ | Tag/Monat/Jahr |
| 39 | 1 | | Buchungskennzeichen S oder H |
| 40 | 8 | Rechtsbündig | Kontonummer |
| 48 | 8 | Rechtsbündig | Gegenkonto |
| 56 | 9 | Rechtsbündig 9.2 | Betrag |
| 65 | 3 | Linksbündig | Währungskennzeichen DM oder EUR |
| 68 | 2 | | CR/LF |
