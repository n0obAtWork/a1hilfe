# Rohware-Preisfindung per Datenbankprozedur bestimmen

<!-- source: https://amic.de/hilfe/_rwwarepreisperdbproc.htm -->

Hauptmenü > Rohwarenabrechnung \> Rohwaren-Verwaltung > Bearbeiten > Abrechnungsschema > Merkmal-Definition

Direktsprung **[RWG]**

Hauptmenü > Administration \> Werkzeuge > SQL Textmanager

Direktsprung **[SQLM]**

Für die Bestimmung der Anfangspreise von [Lieferwarenpositionen](../vorgehensweise_bei_der_einrichtung_von_abrechnungsschemata_s.md#LPosDef) und [Sekundärwarenpositionen](../vorgehensweise_bei_der_einrichtung_von_abrechnungsschemata_s.md#WPosDef) in Rohwareabrechnungsschemata können Datenbankprozeduren eingesetzt werden.

Dabei ist zu beachten, dass ein derart ermittelter Preis für eine Preismengeneinheit der Position normiert zurückgegeben werden muss.

**Hinweis:** Es ist bei der Verwendung von Datenbankprozeduren unbedingt auf die Performance bei der Prozedurausführung zu achten, da die Preisfindung während der Erfassung oder Korrektur eines Rohware bei allen Eingaben durchgeführt wird, die preisrelevant sein könnten.

Es kann jeweils ein Prozedurname zur Ermittlung von

- Produkt-/Finalpreis
- Abschlagpreis
- Weltmarktpreis
- Mindestpreis

  jeweils getrennt nach Einkauf und Verkauf angegeben werden.

  Bei der jeweiligen Einstellung **Prozedurpreis 0,00 überschreibt Preis** \= **‚NEIN‘** erfolgt entsprechenden Fall eine Preisfindung ohne Prozedur ( Kontraktpreis, Partiepreis, Listen-/Fixpreis ).

  Die verwendeten Datenbankprozeduren müssen ein RESULT mit einem Attribut zurückliefern, dass praktischerweise vom Typ ‚numeric‘, oder ‚decimal sein sollte. Der Name des Ergebnisfeldes ist beliebig wählbar. Bei der Erfassung, Korrektur und/oder Abrechnung eines entsprechenden Beleges wird der Ergebniswert ermittelt und der gewünschte Anfangspreis in Abhängigkeit der Preis-Nachtragseinstellung, der 0-Prozedurpreiseinstellung der Positionseinrichtung und des Fix-Kennzeichens des Preises im Beleg überschrieben.  
    
Die Parameter der DB-Prozedur werden mittels festgelegter Parameternamen bestimmt. Diese sind mit DEFAULT-Werten in der Parameterliste zu versehen. Aus der [*Liste der möglichen Parameter*](../rohwareparameter_einrichten/rohwareparameter_uebersicht.md#RwDbProc_Parameter) müssen nur die tatsächlich benötigten deklariert werden.
