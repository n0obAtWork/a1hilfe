# Automatische Umrechnung

<!-- source: https://amic.de/hilfe/automatischeumrechnung.htm -->

Bei einer Rechnung in Fremdwährung werden die eingegebenen Beträge automatisch umgerechnet. Dabei kann es durch Rundungen zu einer Besonderheit kommen. Hat eine Rechnung mehrere Positionen und rechnet man jede einzelne Position von Fremdwährung in Euro um, so kann der Saldo dieses Beleges unter Umständen nicht auf null aufgehen. Beispiel:

| | USD | Kurs | EUR | Euro Gerundet |
| --- | --- | --- | --- | --- |
| Position 1 | 1.000,00 | 1,269300 | 787,8358… | 787,84 |
| Position 2 | 2.000,00 | | 1.575,6716… | 1.575,67 |
| Position 3 | 1.000,00 | | 787,8358… | 787,84 |
| Summe | 4.000,00 | | | 3.151,35 |

Rechnet man jedoch 4.000,00 USD zu dem Kurs um, so ergibt sich ein gerundeter Wert von 3.151,34. Diese Differenz wird dann auf das im Währungsstamm hinterlegte Ausgleichskonto gebucht. Es ergibt sich dann folgenden Buchungssatz:

| | USD | EUR | USD | EUR |
| --- | --- | --- | --- | --- |
| Personenkonto | 4.000,00 | **3.151,34** | | |
| an Erlöskonto | | | 1.000,00 | 787,84 |
| an Erlöskonto | | | 2.000,00 | 1.575,67 |
| an Erlöskonto | | | 1.000,00 | 787,84 |
| an Ausgleichskonto | | | 0,00 | **\-0,01** |

    
Diese Ausgleichsbuchung wird nur gemacht, wenn im Währungsstamm das Ausgleichkonto eingetragen ist. Ansonsten wird in die Hauptposition der Euro-Betrag als Summe der Gegenpositionen ermittelt:

| | USD | EUR | USD | EUR |
| --- | --- | --- | --- | --- |
| Personenkonto | 4.000,00 | **3.151,35** | | |
| an Erlöskonto | | | 1.000,00 | 787,84 |
| an Erlöskonto | | | 2.000,00 | 1.575,67 |
| an Erlöskonto | | | 1.000,00 | 787,84 |
