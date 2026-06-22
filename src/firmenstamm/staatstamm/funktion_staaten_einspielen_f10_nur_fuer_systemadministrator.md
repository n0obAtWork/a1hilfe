# Funktion Staaten einspielen F10 (nur für Systemadministratoren)

<!-- source: https://amic.de/hilfe/_funktionstaateneinsp.htm -->

Die Funktion Staaten einspielen ergänzt Staaten in der Relation Staatstamm mit Hilfe der Relation Amic_Staatstamm, die von Amic mit ausgeliefert wird.  
Die Funktion wird nur ausgeführt, wenn im Staatstamm für alle Staaten das Feld ISO-Code gefüllt ist. Falls nicht erscheint eine Meldung, dass die Daten zunächst gepflegt werden müssen. Es werden nur Staaten aus Amic_Staatstamm in den Staatstamm übertragen deren ISO-Code noch nicht im Staatstamm vorkommt.

Existiert ein ISO-Code dort noch nicht wird geprüft, ob es die Staatnummer schon gibt. Falls ja, wird eine neue Nummer vergeben. Danach wird noch überprüft, ob der Inhalt des Feldes StaatPostKurz noch nicht existiert, da auf dem Feld ein ‚unique index‘ liegt. Ist dies der Fall, dann wird der Datensatz aus Amic_Staatstamm in Staatstamm übernommen, falls nicht, dann erhält der Anwender eine Meldung, dass der Datensatz nicht übernommen werden kann.

    
Nach dem Durchlauf aller Datensätze erhält der Anwender eine Meldung, dass die Verarbeitung abgeschlossen ist.
