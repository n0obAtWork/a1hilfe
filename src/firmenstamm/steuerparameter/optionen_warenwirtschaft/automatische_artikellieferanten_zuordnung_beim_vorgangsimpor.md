# Automatische Artikellieferanten Zuordnung beim Vorgangsimport von Bestellungen (SPA 883)

<!-- source: https://amic.de/hilfe/_SPA_883.htm -->

Mit diesem Steuerparameter kann eingestellt werden, ob bei der Belegerzeugung einer Bestellung in der Anwendung Vorgangimport**[VIMP]** über die Funktion „Standarvorgang erzeugen“ der Hauptlieferant eines Artikels gezogen werden soll.

| Ausprägung | Bedeutung |
| --- | --- |
| Ja | Wird der Steuerparameter auf „Ja“ gestellt, so wird vor der Belegerzeugung geprüft, ob für diesen Artikel ein Lieferant in A.eins hinterlegt worden ist. Ist der Lieferant unterschiedlich zu dem im Stammsatz hinterlegten Lieferanten, so wird für diese Position ein neuer Stammsatz und eine neue Positionszeile erzeugt. Existiert zu diesem Lieferanten eine noch nicht eingespielte Bestellung, so wird diese Position dem noch nicht eingespielten Stammsatz zugeordnet.  
Existiert kein Lieferant zu diesem Artikel, so wird der Lieferant aus dem originalen Stammsatz beibehalten.  
Soll die Lieferantensuche trotz aktiven Steuerparameter für die Artikelposition nicht durchgeführt werden, so muss das Kennzeichen „KundenAenderungManuell“ im Stammsatz auf 1 gesetzt werden. |
| Nein | Bei der Vorgangserzeugung wird immer der Lieferant aus dem Stammsatz gewählt. |
