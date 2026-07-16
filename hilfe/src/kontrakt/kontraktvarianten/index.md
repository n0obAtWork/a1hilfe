# Kontraktvarianten

<!-- source: https://amic.de/hilfe/_kontraktvarianten.htm -->

Hauptmenü > Kontraktverwaltung > Kontraktvarianten

oder Direktsprung **[KTVA]**

Als Kontrakt-Variante wird die Steuerung für die Druck-Aufbereitung optisch identisch aufzubereitender Kontrakte bezeichnet.

Insbesondere ergibt sich aus ihr die Reihenfolge der verschiedenen darzustellenden Daten (Relationen), die einen Kontrakt beschreiben.

Kontraktvarianten können einem Kontrakt für die Aufgaben

- Kontraktdruck
- Kontrakterledigungsschreiben
- Kontraktstorno

zugeordnet werden. Dabei kann es sich (jeweils) um eine Variante für alle Kontrakte handeln, in diesem Fall ist z.B. das Kontraktbestätigungsschreiben für alle Kontrakte gleich, oder man unterscheidet Kontrakte inhaltlich (z.B. soll das Bestätigungsschreiben für einen Futtermittelkontrakt anders aussehen als für einen Futtermischungskontrakt). Im letzten Fall würde man dann mindestens zwei Varianten anlegen, die den unterschiedlichen Anforderungen gerecht würden. Mit der Einbindung der Variantennummer in die Kontraktstammdaten bekommt der Kontrakt also zusätzliche inhaltliche Bedeutung. Um diese Bedeutung auch bei der Neuerfassung von Kontrakten sicherzustellen, empfiehlt es sich, Musterkontrakte für jede Variante anzulegen.

Zu beachten ist, dass Varianten nur in Zusammenhang mit der eigentlichen Formulareinrichtung gesehen werden können.

Der optische Aufbau eines Schreibens wird in zwei Programmbereichen eingeteilt:

1. Im Formulareinrichtungsprogramm wird die generelle Optik des Formulars bestimmt. Hierbei handelt es sich um die quasi unveränderlichen Elemente des Kontraktdrucks, z.B. die Position der Anschrift, des Datums, aber auch des generellen Ausdrucks von Qualitäten, Paritäten, etc.

2. Die von Kontrakt zu Kontrakt unterschiedlichen Merkmale, wie vereinbarte Qualitäten, Paritäten, etc. werden hier, in den Varianten, erfasst. Außerdem wird die Reihenfolge des Ausdrucks bestimmt.

Folgende Felder stehen auf der Erfassungsmaske zur Verfügung.

| Feld | Beschreibung |
| --- | --- |
| Kontrakt-Varianten-Nummer | Nummer der Variante |
| Kontrakt-Varianten-Typ | Typ der Kontrakt-Variante, d. h. ob es sich um eine Variante für den Kontraktdruck, den Erledigungsdruck, den Stornodruck, Andienung etc. handelt:<br>1. Kontrakt<br>2. Kontrakt-Storno<br>3. Kontrakt-Erledigung<br>4. Andienung<br>5. Freistellung<br>6. Andienung mit Freistellung<br>7. Kontrakt-Erinnerung<br>Hiermit wird dann gesteuert, welches Formular bei der Auslösung z.B. des Erledigungsschreibens gewählt wird. |
| Bezeichnung | Vergabe eines freien Textes zur Beschreibung der Variante. |
| Formular-Zuordnung | Im Formulareinrichter sind (möglicherweise) für unterschiedliche Anforderungen Standards festgelegt worden. Das für den Ausdruck dieser Kontraktvariante gewünschte Grundformular wird hier ausgewählt. Anschließend werden für die eingerichteten Bereiche dieses Grundformulars, z.B. den Bereich “Qualitäten”, die variablen Druckpositionen festgelegt. |

Über den Optionboxeintrag „[Bereiche](./bereiche.md)“ lassen sich die einzelnen Bereiche der Kontraktvariante einrichten.

<p class="siehe-auch">Siehe auch:</p>

- [Bereiche](./bereiche.md)
- [Kontrakt-Variante Exportieren](./kontrakt_variante_exportieren/index.md)
- [Varianten kopieren](./kontrakt_variante_exportieren/varianten_kopieren.md)
