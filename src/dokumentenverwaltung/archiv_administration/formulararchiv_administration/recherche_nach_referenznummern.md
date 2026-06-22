# Recherche nach Referenznummern

<!-- source: https://amic.de/hilfe/_recherchenachreferen.htm -->

Die Aufgabe der Funktion „Recherche“ ist es, anhand einer vorgegebenen Referenznummer zu versuchen, aus dem System die anderen Daten zu ermitteln.

Man hat beispielsweise über einen Import einen Beleg ins Formulararchiv transportiert und weiß zu diesem zunächst nur die Referenznummer. Die Idee ist nun, dass es schon weitere Verwendung dieser Referenznummer im System geben könnte, und in einem solchen Falle sollen dann die Daten zur Vervollständigung des Formulararchiveintrages herangezogen werden.

Inhaltlich ist also das Vorhandensein einer Referenznummer unabdinglich, des Weiteren darf die Belegklasse noch nicht festgelegt sein. Die Belegklasse ist der Erkenner, dass der Eintrag schon möglicherweise behandelt wurde.

Nun ist die Suchstrategie folgende:

• Vorgangsstamm anhand von Referenznummer, im Falle eines Treffers sind somit Belegnummer, Kundennummer und Vorgangsklasse mittelbar. Die Belegklasse des Formulararchiv-Eintrages wird die Vorgangsbelegklasse.

• FibuvorgStamm, falls gefunden, sind somit Fibu-Belegnummer, Kontonummer resp. Kundennummer ermittelt. Belegklasse wird 7000.

• Kontraktstamm, falls gefunden, wird die Kontraktnummer zur Belegnummer, der Kunde wird über eine weitere Datenrecherche gewonnen. Belegklasse wird 7500

• OWaage, falls gefunden, wird die Wiegenummer zur Belegnummer, Kunde übernommen und Belegklasse 8000

Als letztes wird der Belegtyp-Text konventionsgemäß ermittelt und festgeschrieben.
