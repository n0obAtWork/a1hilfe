# Wiegen in Rohwareerfassung

<!-- source: https://amic.de/hilfe/_wiegenimvorgang_rohware.htm -->

Bei der Erfassung einer Rohwarenlieferung ist es zunächst notwendig einen Artikel und einen Kunden anzugeben. Ohne Eingabe eines Artikels und Kundens kann nicht gewogen werden, denn diese Angaben sind unbedingt notwendig für die Anlage eines Waagedatensatzes.  
    

Die Funktion ***Wiegen*** ist in der Option Box nur anwählbar, wenn der Cursor im Feld Menge steht.  
Wird diese Funktion aufgerufen, dann wird ein neuer Waagedatensatz angelegt und die erste Wiegung als Ampelwiegung ausgeführt. Das gewogene Gewicht wird in das Feld Menge übernommen und die Id des neu angelegten Waagedatensatzes an den Rohwaredatensatz übergeben.  
Um eine zweite Wiegung durchzuführen wählt man erneut die Funktion ***Wiegen*** an. Die Differenz der beiden Wiegungen wird in das Feld Menge übernommen. Der Waagedatensatz wird auf den Status ‚mit Vorgang’ gesetzt.  
Beim Speichern des Rohwarenbeleges wird dessen VorgangsId in den erzeugten Waagedatensatz eingetragen, so dass die Verbindung der beiden Datensätze hergestellt ist.
