# ImportVorgTextPosition

<!-- source: https://amic.de/hilfe/importvorgtextposition.htm -->

#### Positionstext

In dieser Relation werden Textpositionen hinterlegt, die entweder vor oder nach einer Position in den Beleg eingefügt werden können.

| Feld | Bedeutung |
| --- | --- |
| UebernahmeID | Übernahmeid aus der Relation ImportVorgPosition |
| SatzId | SatzId aus ImportVorgPosition |
| PositionId | Positionsid aus ImportVorgPosition |
| Zeilenzaehler | Zähler der Textzeile |
| Texttyp | Texttyp<br>0. Positionstext |
| TextPosition | Beim TextTyp 0 (Positionstext) an welche Stelle soll der Text geschrieben werden<br>0 Vor der Position<br>1 Nach der Position |
| VorgText | Inhalt des Textes |
| IVP_GUID | Guid der Position aus der Relation ImportVorgPosition |

Hier muss als Schlüssel Beziehung zur Tabell ImportVorgPosition die ÜbernahmeId, SatzId und PositionId genommen werden
