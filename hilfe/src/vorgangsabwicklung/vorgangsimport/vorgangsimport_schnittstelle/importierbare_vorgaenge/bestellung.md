# Bestellung

<!-- source: https://amic.de/hilfe/bestellung1.htm -->

Bei der Bestellung gibt es eine Besonderheit. Wenn in der Relation [ImportVorgStamm](http://www.amic.de/ihilfe/XMLDocuments/iAeins/html/T_Datenmodell_ImportVorgstamm.htm) kein Lieferant zugeordnet worden ist und der Steuerparameter 883 auf „Ja“ steht, so wird bei der Belegerzeugung sofern für den Artikel ein Lieferant eingerichtet worden ist, der erste Lieferant automatisch zugeordnet. Bei der Zuordnung wird die Aktuelle Position als gelöscht gekennzeichnet und es wird eine neue Position mit der gleichen ÜbernahmeId und einer neuen SatzId erzeugt. Ist für den Lieferanten noch eine nicht verarbeitete Bestellung offen, so wird diese Position zu der anderen Bestellung hinzugefügt.
