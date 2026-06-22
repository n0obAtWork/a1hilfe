# Lagerumbuchung

<!-- source: https://amic.de/hilfe/_vimp_lagerumbuchung.htm -->

Um einen Lagerumbuchung / Lagerplatzumbuchung mit dem Vorgangsimport in das A.eins System einzuspielen müssen folgende Regeln beachtet werden und mindestens folgende Felder gefüllt werden.

Besonderheiten

Für eine Positionszeile in der Lagerumbuchung müssen zwei Zeilen in der Relation ImportVorgPosition angelegt werden. Damit das System weiß, welches die Abgangs- und welches die Zugangszeile ist, werden die Zeilen per Positionsklammer und „TypAbgangZugang“ geklammert.

Die Positionsklammer beschreibt die Stelle der Warenposition in der Lagerumbuchung. Es empfiehlt sich in diesem Feld die PositionId der Abgangsposition einzutragen. Mit dem Feld TypAbgangZugang wird beschreiben, ob es sich um eine Zugangs- oder Abgangszeile handelt.

1 ist die AbgangsZeile

2 ist die ZugangsZeile

Gebinde

Um eine Gebinde Position anzulegen muss die Gebindemengeneinheit in dem Feld „ME“ und die Gebindeanzahl muss im Feld „Menge“ in der Relation ImportVorgPosition gespeichert werden.

Partie

Existiert zu einer Warenposition nur eine Partie so kann diese direkt mit in der Relation ImportVorgPosition gespeichert werden.

Es werden bislang nur die Kombination aus Partiebezeichnung und oder Partienummer geprüft.

Wird nur die Partiebezeichnung gespeichert, so wird mit dieser Partiebezeichnung eine neue Partie angelegt, wenn diese nicht vorhanden ist.

Sollen mehrere Partien zu einer Warenposition angelegt werden, so müssen diese Partien in der Tabelle [ImportVorgPositionPartie](../vorgangsimport_anwendung/funktionen_des_vimp_pflegers.md#Tab_ImportVorgPositionPartie) gespeichert werden. Auch hier gilt die Kombination zwischen Partienummern und Partiebezeichnung. Eine Verprobung zwischen der Partiemenge und der Positionsmenge findet nicht statt.
