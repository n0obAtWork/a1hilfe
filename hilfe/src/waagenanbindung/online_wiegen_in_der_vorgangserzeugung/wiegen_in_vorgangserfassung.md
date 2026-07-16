# Wiegen in Vorgangserfassung

<!-- source: https://amic.de/hilfe/_wiegenimvorgang_normalware.htm -->

Bei der Erfassung eines Vorganges (z.B. Lieferschein) ist es zunächst notwendig für die Position einen Artikel anzugeben. Ohne Eingabe eines Artikels kann nicht gewogen werden, denn der ist unbedingt notwendig für die Anlage eines Waagedatensatzes.  
    
Danach kann die Funktion ***Wiegen*** aus der Option Box aufgerufen werden.  
Es wird ein neuer Waagedatensatz angelegt. Die Inhalte der Felder Kunde, Artikel, Lager und Lagerplatz werden in den Waagedatensatz mit übernommen. Wurde vorm Wiegen auch eine Partie für die Position hinterlegt, dann wird diese auch übertragen. Bei Angabe mehrerer Partien wird die erste Partie übernommen.  
Dann wird eine automatische Ampelwiegung durchgeführt. Das gewogene Gewicht wird in das Feld Menge der Positionszeile übernommen und validiert. Die Id des neu angelegten Waagedatensatzes wird in die Warenbewegung übergeben.  
Um eine zweite Wiegung durchzuführen, wählt man erneut die Funktion ***Wiegen*** im Vorgang an. Die Differenz der beiden Wiegungen wird in das Feld Menge übernommen und validiert. Der Waagedatensatz wird auf den Status ‚mit Vorgang’ gesetzt.

Um eine Rückwiegung durchzuführen, ohne den Vorgang zu öffnen, gibt es jetzt eine eigene Variante (Offene Waagenbelege z.B. unter der Anwendung Lieferschein Bearbeiten **[LIB]**). Hier kann in der Auswahlliste ein Datensatz markiert und durch Ausführen der Funktion ***Rückwiegung*** wird die Menge im Vorgang automatisch gesetzt.  
Beim Speichern des Lieferscheines wird dessen VorgangsId in den erzeugten Waagedatensatz eingetragen, so dass die Verbindung der beiden Datensätze hergestellt ist.

Wird der Vorgang (Lieferschein) zwischen der ersten und zweiten Wiegung verlassen/gespeichert, dann gibt es in der OnlineWaage Datensätze mit dem Status ‚erste Wiegung’ und einer VorgangsId.  
Da diese Datensätze von der OnlineWaage aus nicht mehr geändert werden dürfen (die Änderung würde nicht in den Lieferschein übertragen werden) werden sie dort nur noch geschützt angezeigt (wie Datensätze mit dem Status ‚mit Vorgang’).
