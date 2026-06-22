# Buchungstyp (KtrBuchTyp)

<!-- source: https://amic.de/hilfe/buchungstypktrbuchtyp.htm -->

Wird eine Ware gebucht, so wirkt diese Buchung auf einen bestimmten Bestand.

Um unterscheiden zu können, auf welche Bestände eine Buchung wirken soll, gibt es den Buchungstyp.

Der Buchungstyp findet sich in der Tabelle Warenbewegung als Feld KtrBuchTyp.

| Buchungstypen  
 |
| --- |
| 0 | Eigenwarebuchung | Diese Buchung verändert nur die Bestände der Eigenware |
| 1 | Vorverkauf | Diese Buchung verändert Fremdware |
| 2 | Voreinkauf | Diese Buchung verändert Fremdlager |
| 3 | Einlagerung | Diese Buchung verändert Fremdware |
| 4 | Kommission | Diese Buchung verändert Fremdlager |

Buchungstyp im Kontrakt

Der Buchungstyp eines Kontrakts bestimmt Quelle und Ziel einer Buchung (z.B. Eigenware zu Fremdware). Eine nachträgliche Änderung des Buchungstyps ist deshalb nicht möglich, da damit alle bereits erfolgten Buchungen und Bestände im Zusammenhang mit diesem Kontrakt ebenfalls geändert werden müssten, die Zusammenhänge aber nicht überall ersichtlich sind.
