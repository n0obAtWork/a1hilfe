# Scanner im Marktbereich

<!-- source: https://amic.de/hilfe/_scanner_markt.htm -->

Folgende Module sind für den Marktbereich verfügbar.

- Eingangslieferschein
- Bestellung
- Inventur
- [Permanente Inventur](./permanente_inventur.md)

Starten eines Vorganges mit dem Scanner

Die Scannervorgänge können per Menü auf dem Scanner aufgerufen werden. Dazu wird bei aktiver Scanner Software die Taste F1 gedrückt.

Folgende Punkte können ausgewählt werden

| Menü Punkt | Bedeutung |
| --- | --- |
| 1. Inventur | Startet ein Inventurerfassung |
| 2. Bestellung | Startet eine Bestellerfassung |
| 3. Eingangslieferschein | Startet ein Eingangslieferschein |
| 4. Permanente Inventur | Startet die Erfassung für einen Beleg der Permanenten Inventur |
| 5. Abbruch | Damit wird das Menü verlassen |
| 6. Reset | Damit wird der Scanner zurückgesetzt. Die erfassten Daten im Vorgangsimport werden für den Scanner auf gelöscht gesetzt |

Um einen dieser Vorgänge zu starten kann entweder die Zahl eingegeben werden, oder mit den Pfeiltasten Hoch oder Runter wird im Menü der jeweilige Punkt ausgewählt und mit Enter bestätigt.

Die einzelnen Vorgänge des Scanners können natürlich auch per Scancode gestartet werden.

Beenden eines Vorganges mit dem Scanner

Um ein Scanvorgang abzuschließen kann die Taste F2 des Scanners gedrückt werden. Dadurch wird der aktuelle Scanvorgang geschlossen.

Auch hier können die einzelnen Vorgänge mit per Scancode beendet werden.

Meldungen die beim Starten oder Beenden eines Vorgangs auf dem Scanner erscheinen können

| Meldung | Bedeutung |
| --- | --- |
| Der erfasste Befehl:<br>z.B. Inventur Ende<br>passt nicht zum aktuellem Vorgang des Scanners: Bestellung<br>Scannung wird verworfen | Dies bedeutet, dass mittels Scancode probiert wurde eine Inventur zu beenden, obwohl der sich der Scanner in der Bestellerfassung befindet. Mit den Pfeil Tasten nach oben oder unten wird die eigentliche Ansicht wieder geladen. |
| Es existiert kein offener Vorgang am Scanner. Der erfasste Befehl z.B. Bestellung Enden kann nicht verarbeitet werden. | Dies bedeutet, dass probiert wurde ein Vorgang am Scanner abzuschließen, obwohl kein offener Scanvorgang vorhanden ist. |
| Der Scanner befindet sich gerade im z.B. Eingangslieferschein Modus. Zum Starten des Vorgangs z.B. Inventur bitte IV erneut Scannen.' | Dies bedeutet, dass probiert wurde einen neuen Vorgang auf dem Scanner zu eröffnen, obwohl noch ein Vorgang offen ist. Durch erneutes erfassen des Scancodes oder Auswahl im Menü wird der alte Vorgang abgebrochen und der neue gestartet. |

Besonderheit

Ist kein Erfassungsvorgang auf dem Scanner gestartet, so befindet sich der Scanner Automatisch im Modus der [Permanenten Inventur](../../../abschluesse_inventur/permanente_inventur/index.md). Die Besonderheit dabei ist, dass pro gescannten Artikel ein Differenzbeleg für die Permanente Inventur geschrieben wird.
