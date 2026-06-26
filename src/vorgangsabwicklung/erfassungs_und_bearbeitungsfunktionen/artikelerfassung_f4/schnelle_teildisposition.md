# Schnelle Teildisposition

<!-- source: https://amic.de/hilfe/_warenpositionerfassen_schnelleteildispo.htm -->

Wenn bei der Artikelerfassung noch kein Artikel ausgewählt wurde, besteht die Möglichkeit zu einer schnellen Teilumwandlung von Warenpositionen aus Quellbelegen. Nach Auswahl dieser Funktion wird wie bei der [Standard-Teildisposition](../../standard_teildisposition_mehrfachteildisposition/index.md) eine Artikelübersicht angezeigt. Wird ein Artikel ausgewählt, so wird dieser in die Artikelerfassungsmaske übernommen und es wird in den Teildispositionsmodus umgeschaltet. Dies ist daran zu erkennen, dass die maximal zur Disposition zur Verfügung stehende Menge neben der gewählten Menge angezeigt wird.

Wie gewohnt kann jetzt eine Warenposition erfasst werden. Die umgewandelte Menge wird erst bei Abschluss des Beleges von der Menge im Quellbeleg abgezogen. Durch eine normale Artikelauswahl wird der Teildispositionsmodus wieder verlassen. Weitere Einstellungen wie „Stornoprozentsatz“, „disponierbare Quellbelegklasse“ oder „lagerübergreifende Teildisposition“ können in der [Formularzuordnung](../../formularzuordnung/index.md) **[FRZ]** auf dem Register [Schnelle Teildisposition](../../formularzuordnung/schnelle_teildisposition.md) vorgenommen werden.

#### Einschränkungen

Im Gegensatz zur [Standard-Teildisposition](../../standard_teildisposition_mehrfachteildisposition/index.md#StandardTeildisposition) und [Mehrfachteildisposition](../../standard_teildisposition_mehrfachteildisposition/mehrfachteildisposition.md) sind bei der „Schnellen Teildisposition“ die erzeugten Belege, sobald sie abgeschlossen wurden, losgelöst von ihren Quellbelegen. Das heißt zum Beispiel, dass nachträgliche Korrekturen von Quellbelegen nicht in dem erzeugten Beleg berücksichtigt werden und dass bei der Stornierung von durch die „Schnelle Teildisposition“ erzeugten Belegen die Quellbelege nicht zurückgerechnet werden.

Des Weiteren können keine Warenpositionen mit mehr als einer Gebindezeile, bereits disponierte Warenpositionen oder Warenpositionen mit Ausprägungen umgewandelt werden.

Bei lagerübergreifender Teilumwandlung (einzustellen im [Register Schnelle Teildisposition](../../formularzuordnung/schnelle_teildisposition.md) der Formularzuordnung **[FRZ]**) müssen die umgewandelten Artikel auch in dem neu gewählten Lager existieren. Ebenso muss eine Partie oder ein Kontrakt für die Warenposition lagerübergreifend sein und der Steuerparameter [575 „Lager/Versandadr./Vertreter ignorieren“](../../../firmenstamm/steuerparameter/objektverwaltungswesen/lager_versandadresse_vertreter_ignorieren_spa_575.md) aus der Gruppe „Objektverwaltungswesen“ muss auf „Ja“ geschaltet sein.

Bei der Artikelauswahl für die Schnelle Teildisposition wird ein Feld „Ursprungsmenge“ angezeigt. In diesem Feld steht die ursprüngliche Menge des Quellbeleges. Da aber die Schnelle Teildisposition die Mengen in den Quellbelegen direkt abändert, werden anders korrigierte Mengen nur solange angezeigt, bis der Quellbeleg mit der Schnellen Teildisposition behandelt wurde. Sobald der Quellbeleg einmal mit der Schnellen Teildisposition behandelt wird, wird die Menge intern in einem extra Feld gespeichert, das dann angezeigt wird und auf das sich Korrekturen nicht auswirken.

#### Ausbuchen

Mit der Funktion „Schnelle Teildisposition ausbuchen“ kann festgelegt werden, dass die disponierte Position beim Abschluss des Beleges ausgebucht wird. Sollte die Position mehrfach disponiert und mindestens einmal als ausgebucht gekennzeichnet werden, wird die Position auch ausgebucht.

Zusätzlich wird auf der Positionserfassung per [Signalfeld](./signalfelder.md) angezeigt, ob die Position ausgebucht werden soll.
