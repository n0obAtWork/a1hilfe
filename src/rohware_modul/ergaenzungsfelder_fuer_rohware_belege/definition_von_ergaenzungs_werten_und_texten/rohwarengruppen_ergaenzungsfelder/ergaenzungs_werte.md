# Ergänzungs-Werte

<!-- source: https://amic.de/hilfe/ergnzungswerte.htm -->

Der erste Block enthält die rohwarengruppenweit definierten Ergänzungsfelder, die eine ganze Zahl beinhalten können.

Die ( nicht änderbare ) Nummer der 1. Spalte bestimmt das Datenfeld (V_RohwareZFeldI1, V_RohwareZFeldI2 bzw. V_RohwareZFeldI3) der Relation V_Rohware, in der die korresponierenden Werte der Rohwaren-Belege gespeichert werden.

Die Werte der Spalte **‚Pos’** für Position bestimmt die Erfassungsposition des Feldes der Rohwarebearbeitungs-Teilmaske für die Ergänzungswerte wie auch der Korrekturmaske für Rohware-Waage-Belege. Es handelt sich hierbei um eine relative Positionsangabe, die Positionsangaben müssen also nicht lückenlos sein. Die Angabe ‚0’ für die Position bewirkt ein Unterdrücken des Feldes, es ist dann nicht definiert. Für die Berechnung der Abfrageposition eines Ergänzungsfeldes werden die ‚Pos’-Angaben aller rohwarengruppen- und schemaspezifischen Ergänzungsfelddefinitionen in aufsteigender Reihenfolge sortiert und die zugehörige Eingabefelder in dieser Reihenfolge erzeugt.

Die Angabe in der Spalte **‚Bezeichnung’** ist auf der Bearbeitungsmaske vor dem Eingabefeld wiederzufinden, hierdurch identifiziert der Anwender also ein Ergänzungsfeld.

In der Spalte **‚Verwend.’** wird festgelegt, ob die Felddefinition für den Bereich ‚Einkauf’, ‚Verkauf’ oder für beide Bereiche gelten soll.

Mit den Spalten **‚Min.Wert’** für Mindestwert und **‚Max.Wert’** für Maximalwert werden korrespondierende Eingaben verprobt und ggf. zurückgewiesen.

In der optionalen Spalte **‚Item-Box’** kann eine existierende Item-Box für die F3-gestütze Erfassung hinterlegt werden. Es ist jedoch darauf zu achten, dass der RETURN-Wert einer derartigen Item-Box eine ganze Zahl beinhaltet. Es wird jedoch kein Test durchgeführt, ob eine eingegebene Zahl auch per Item-Box auswählbar ist (kein Item-Check). Dadurch kann hier auch eine Item-Box als Vorschlagsliste aufgefasst werden, deren Werte aber nicht bindend sind.

Eine optionale Eingabe in der Spalte **‚Validierung: zugeh. Textquelle’** bewirkt zweierlei:

- In Verbindung mit einer in der Spalte ‚Item-Box’ angegeben Itembox wird bei einem über die Item-Box ausgewählten Eintrag hinter dem korrespondierenden Bearbeitungsfeld der Inhalt des zusätzlichen Item-Box-Rückgabewertes ausgegeben, wenn der zusätzliche Rückgabewert den angegeben Feldnamen trägt.
- In Verbindung mit einem in der Spalte <strong>‚Validierung: SQL-Text’</strong> angegeben Namen eines privaten SQL-Textes wird hinter dem korrespondierenden Bearbeitungsfeld der Inhalt des Ergebnisses ( Return-Wert ) der Ausführung des SQL-Textes ausgegeben, wenn die SELECT-Liste des SQL-Textes den hier angegeben Namen beinhaltet.

  Eine optionale Eingabe in der Spalte **‚Validierung: SQL_Text’** bewirkt die Ausführung eines hier angegeben privaten SQL-Textes und eine Zurückweisung der Eingabe eines Wertes, der zum Fehlschlagen der SQL-Text-Ausführung führt.

  Die mit dem Eingabewert korrespondierende Variable derartiger SQL-Texte ist immer ‚ErgWert’.

  Beispiel:

```sql
select
SpediNummer,SpediBezeich
 from SpediStamm
 where
SpediNummer = ':ErgWert'
```
