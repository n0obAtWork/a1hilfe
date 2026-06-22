# Lagerwechsel in der Online Waage

<!-- source: https://amic.de/hilfe/_90_38897.htm -->

Beim Ändern der Lagernummer innerhalb eines Waagensatzes wurde der Lagerwechsel nicht übernommen, wenn das Feld Lagernummer nicht aktiv verlassen wurde. Dies trat insbesondere dann auf, wenn der Benutzer den Cursor noch im Feld hatte und direkt F11 – Wiegung abschließen ausführte. In solchen Fällen blieb der Artikel weiterhin auf dem ursprünglichen Lager gespeichert. Dadurch kam es zu Inkonsistenzen zwischen: dem Lager des Artikels (z.B. weiterhin Lager 1) und dem Lager, das im Waagensatz hinterlegt war (z.B. Lager 2). Bei der anschließenden Vorgangserzeugung wurde das Lager des im Waagensatz hinterlegten Artikels verwendet – und nicht das Lager des Waagensatzes selbst. Da der Artikel durch das nicht verlassene Lagernummernfeld nicht aktualisiert wurde, blieb dessen ursprüngliches Lager bestehen und bestimmte somit das Lager des erzeugten Vorgangs. Das Systemverhalten wurde angepasst: Der Lagerwechsel des Artikels wird nun auch dann korrekt übernommen, wenn das Feld Lagernummer nicht aktiv verlassen wurde. Wird der Vorgang über F11 – Wiegung abschließen abgeschlossen, wird der Artikel zuverlässig auf das Lager des Waagensatzes aktualisiert. Auswirkung: Verhindert Inkonsistenzen zwischen Artikel- und Waagensatzlager. Stellt sicher, dass Vorgänge immer mit dem tatsächlich gültigen Lager erzeugt werden.

Releasenote Kategorie:

Ticket: 751599[38897]

Version: 9.0.2502.9

Datum:

Anwendung: Hofliste

Variante: Hofliste

Funktion/Report: Wiegung abschließen

[Weitere Informationen](http://www.amic.de/hilfe/_waage_wiegungabschliessen.htm)

Tags:

Releasenote, 9.0.2502.9, 38897, 751599
