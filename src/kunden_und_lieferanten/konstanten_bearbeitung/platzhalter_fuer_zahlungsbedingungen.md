# Platzhalter für Zahlungsbedingungen

<!-- source: https://amic.de/hilfe/platzhalterfrzahlungsbedingung.htm -->

| Nr. | Bezeichnung | Nr. | Bezeichnung |
| --- | --- | --- | --- |
| 1 | Betrag netto | 2 | Betrag brutto |
| 3 | Steuerbetrag | 4 | Skontierfähiger Betrag |
| 5 | Skontobetrag | 6 | Skontosatz |
| 7 | Skontotage | 8 | Skontodatum |
| 9 | Valutadatum | 10 | Zieltage |
| 11 | Zahlbetrag | 17 | Plan- / Lieferdatum |
| 18 | Währungstext | 19 | Ext. Kundennummer |
| 20 | BLZ | 21 | Bankkontonummer |
| 22 | Bezugsdatum | 23 | Zielverlängerungstage |
| 24 | BIC | 25 | IBAN |
| 26 | Bankbezeichnung | 27 | GläubigerID |
| 28 | Mandat | 29 | SEPA Ausführungsdatum |

Um einen Platzhalter in den Text einzufügen, muss der gewünschte Platzhalter in geschweiften Klammern geschrieben werden. Der Aufbau ist {Nummer, Länge, Kommastellen}.

Ist der Wert kürzer als die in den Zahlungsbedingungen angegebene Länge, so wird der Wert mit Leerzeichen aufgefüllt. Dieses Verhalten lässt sich mit dem [SPA 1148 „Leerzeichen bei Zahlungsbedingungstext entfernen“](../../firmenstamm/steuerparameter/optionen_warenwirtschaft/leerzeichen_bei_zahlungsbedingungstext_entfernen_spa_1148.md) abstellen.

Den numerischen Werten (z.B. Betrag brutto) wird automatisch ein Tausendertrennzeichen hinzugefügt, außer die angegebene Länge ist zu kurz. Reicht die Länge nicht aus, um den numerischen Wert darzustellen, wird im Zahlungsbedingungstext statt des Wertes „\*“ angezeigt.

**Beispiel:**

```text
Ausgabe des Bruttobetrages {2,10,2}
Ausgabe des Wahrungstextes {18}
Ausgabe der Zielverlängerungstage {23,2,0}
```
