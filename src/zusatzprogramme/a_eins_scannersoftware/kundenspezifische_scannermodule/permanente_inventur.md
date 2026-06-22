# Permanente Inventur

<!-- source: https://amic.de/hilfe/_scanner_permanente_inventur.htm -->

Einrichtung der Permanenten Inventur

1. [Manuelle Einrichtung](../../../abschluesse_inventur/permanente_inventur/index.md)

Die Permanente Inventur kann auf zwei Arten durchgeführt werden.

1. Der Scanner befindet sich in keinem Vorgangsmodus, so wird per erfassten Artikel mit Menge ein Differenzbeleg für die Permanente Inventur erfasst.

2. Es wird ein Differenzbeleg für die Permanente Inventur mit dem Scanner eröffnet. Es wird ein Differenzbeleg mit allen erfassten Artikel erzeugt.

Die erfassten Differenzbelege werden automatisch angelegt.

Besonderheiten:

Bei der Permanenten Inventur ist darauf zu achten, dass Artikel immer Partie weise und oder Lagerplatz weise gezählt werden wenn diese vorhanden sind. Existieren Lagerplätze und oder Partien und der Artikel wird ohne Partien oder Lagerplätze gezählt, so wird der Artikelbestand erhöht, ohne dass eine richtige Zuordnung zur Partie oder zu dem Lagerplatz möglich ist.

Beispiel:

Der Artikel A befindet sich auf den Lagerplätzen LP1 und LP2. Jetzt wird der Artikel ohne Lagerplatz Zuweisung gezählt, dadurch wird die Menge des Artikels auf die erfasste Menge am Scanner korrigiert. Die korrigiert Menge wird nun dem Standardlagerplatz 0 zugewiesen und nicht dem zu zählenden Lagerplatz. Dadurch ändert sich die Artikelmenge auf dem Lagerplatz 0 um die eingegebene Menge.

Der Artikel B hat eine Partie Zuordnung von Partie1 und Partie2. Jetzt wird der Artikel ohne Partie gezählt. In diesem Fall wird die Partiemenge nicht mit korrigiert. Sondern nur die Artikelmenge. Partien können nur einzeln erfasst werden. Es existiert keine Partieverteilung.

Erfassung eines Artikels für die Permanente Inventur

1. Als erstes muss der zu zählende Artikel gescannt werden.

2. Jetzt kann zusätzlich eine Partie oder ein Lagerplatz angegeben werden

3. Als letztes wird die Menge angegeben. Nach der Mengenangabe wird ein Positionswechsel vorgenommen. Wurde am Scanner kein Block für die Permanente Inventur gestartet, so wird nach der Erfassung ein Vorgang für die Permanente Inventur geschrieben.

Anzeige auf dem Scanner:

In der Eingabezeile des Scanners zeigt das Wasserzeichen an was als nächstes erfasst werden soll.

| Text | Bedeutung |
| --- | --- |
| Artikel oder Erfassungsvorgang | Es kann ein Artikel erfasst werden, oder es wird ein Scannvorgang gestartet |
| Lagerplatz / Partie oder Menge eingeben | Nach der Artikeleingabe kann jetzt entschieden werden, ob ein Lagerplatz eine Partie oder eine Menge eingegeben werden. Bei der Mengeneingabe wird die Position als Abgeschlossen gesetzt und es kann wieder ein neuen Artikel angelegt werden |
| Partie oder Menge eingeben | Wurde der Lagerplatz eingegeben aber es wurde noch keine Partie erfasst. So kann jetzt noch die Partie oder die Menge erfasst werden. Wird die Menge erfasst, so wird die Position abgeschlossen. |
| Lagerplatz oder Menge eingeben | Wurde ein Partie erfasst, aber noch kein Lagerplatz, so kann jetzt ein Lagerplatz erfasst werden oder mit der Eingabe der Menge die Position abgeschlossen werden. |
| Menge eingeben | Es wurden ein Artikel, ein Lagerplatz und eine Partie erfasst. Jetzt muss nur noch die Mengen eingegeben werden um, diese Position abzuschließen. |

Wird ein Artikel mit dem Scanner erfasst so erscheint als erstes eine Übersicht im unteren Teil des Scanners

| Name | Bedeutung |
| --- | --- |
| Artikel | In diesem Feld wird der erfasste Artikel angezeigt. Artikelnummer und Artikelbezeichung |
| Partie | Wird eine Partie erfasst, so wird die Partienummer und Partiebezeichnung angezeigt |
| Lagerplatz | Sobald ein Lagerplatz eingegeben wurde wird hier die Lagerplatznummer angezeigt |
| Lager | Das Aktuelle Lager des Scanners |
| Bestandsmenge | Die Aktuelle Menge auf dem Lagerplatz |
| Neuemenge | Wenn die Menge eingegeben wurde, steht in diesem Feld die Korrekturmenge |

Wurde auf dem Scanner ein Differenzbeleg der Permanenten Inventur gestartet, so wird nach der Mengeneingabe eine Übersichtstabelle über alle erfassten Artikel angezeigt.

Zusätzliche Tastendrücke die in der Permanenten Inventur ausgewertet werden.

1. KeyUp

2. KeyDown

Mit Keyup und Keydown kann die untere Anzeige auf dem Scanner aktualisiert werden. Werden auf dem Scanner mehrere Position im Differenzbeleg der Permanente Inventur erfasst, so kann mit den Pfeiltasten gescrollt werden.

Sobald die Erfassung einer Permanenten Inventur Beendet ist, wird der Inventur Beleg Automatisch erstellt. Die erfassten Daten für die Permanente Inventur werden im [Vorgangsimport](../../../vorgangsabwicklung/vorgangsimport/index.md) gespeichert.
