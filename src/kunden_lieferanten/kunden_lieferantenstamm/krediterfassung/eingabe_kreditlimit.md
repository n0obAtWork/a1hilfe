# Eingabe Kundenkredit

Hauptmenü > Stammdatenpflege > Kunden / Lieferanten > Kundenstamm

Direktsprung **[KU]**

Die Einrichtung der Kreditlimits geschieht in Abhängigkeit des Steuerparameters 503 – „Alle Kredite als Summe übernehmen“.

Die Eingabe des Kreditlimits kann von zweiten Seiten aus geschehen. Zum einen kann das Limit vom Pfleger für **Kunden-/Lieferantenstammdaten** aus eingegeben werden und zum anderen über den Pfleger der **Kreditvergabe**.

Im Pfleger für **Kunden-/Lieferantenstammdaten** ist lediglich ein Feld „Kreditlimit“ vorhanden, welches je nach Steuerparameter (s.o.) für eine Bearbeitung freigeschaltet ist oder nicht.

Wird hier ein Wert geändert und gespeichert, so wird in Folge dessen ein Abgleich mit dem Pfleger der Kreditvergabe durchgeführt. Zunächst wird der Kredittyp ermittelt zu dem das neue Kreditlimit dort angelegt werden soll. Vorhandene Einträge werden auf den Status „abgelaufen“ gesetzt und das neue Limit wird eingetragen.

Hier ein wichtiger Hinweis zum Kundenkreditlöschkennzeichen.

Die Anwendung **Kreditvergabe** dient zur Einrichtung und Kontrolle der Kreditlimits für den gewählten Kunden. Sie zeigt alle aktiven und inaktiven Kreditlimits in Sortierung nach der Limitart bzw. des Kreditversicherers an.

Es ist möglich mehrere Einträge zu einem Kreditversicherer zu erzeugen. Dies ermöglicht zum Beispiel Planung eines Limits in der Zukunft.

Die Gültigkeit eines Kreditlimits hängt vom Datum der Genehmigung und des „Gültig Bis“ - Datum ab und davon, ob das Kreditlimit mit in die Summierung einfließen soll.

Wird für einen Kunden ein Kreditlimit gespeichert, so wird dieses gleichzeitig in dem zugehörigen Feld im Kunden-/Lieferantenstamm aktualisiert. Das Feld im Kunden-/Lieferantenstamm enthält immer den zur Zeit der Betrachtung korrekten Wert.

Wie dieser Wert bestimmt wird, lässt sich mit dem Steuerparameters 503 – „Alle Kredite als Summe übernehmen“ – festlegen. Ist keine Summierung erwünscht, so ist das Feld „Kreditlimit“ in den Kunden-/Lieferantenstammdaten änderbar. Das hier direkt erfasste Kreditlimit wird dann beim Speichern in die Auflistung der aktiven Kreditlimits unter der als Standard angegeben Limitart übernommen.
