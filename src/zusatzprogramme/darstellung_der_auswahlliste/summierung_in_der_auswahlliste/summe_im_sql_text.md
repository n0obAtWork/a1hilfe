# Summe im SQL-Text

<!-- source: https://amic.de/hilfe/summeimsqltext.htm -->

Damit die Summe über eine Spalte gebildet wird, muss man das Schlüsselwort SUM in die Fieldanweisung schreiben:

```text
FIELD Betrag,ZahlVorBetrag,N2,20,SUM
```

Es wird dann dieses Feld aufsummiert und sowohl im Tiptext, als auch in der unteren Tabelle neben den Auswahlbedingungen, dargestellt.

Man kann aber auch Formeln angeben

```text
FIELD
Betrag,ZahlVorBetrag,N2,20,SUM=(ZahlVorBetrag*(3-Zahlvorsollhaben*2))
```

Die Syntax der Formeln entspricht der SQL-Syntax.

Soll das Format vom Format der aktuellen Spalte Abweichen, so kann man noch zusätzlich mit dem Schlüsselwort SUMFORMAT ein abweichendes Format angeben.

```text
FIELD
Betrag,ZahlVorBetrag,N2,20,SUM=(ZahlVorBetrag*(3-Zahlvorsollhaben*2)),SUMFORMAT=S2
```
