# Lagerumbuchung

<!-- source: https://amic.de/hilfe/_waage_lagerumbuchung.htm -->

Für die Lagerumbuchung ist es nötig, sich ein [Wiegeprozess](./prozess_einrichten/index.md) einzurichten mit dem Wiegetyp Lagerumbuchung und der passenden Vorgangsklasse und -unterklasse.  
    

Diese Vorlage ist dann in der Waagenmaske auszuwählen. Anstelle des Kundeneingabefeldes erscheinen zwei Felder für das Ziellager und den Ziellagerplatz.  
Es sind Lager und Lagerplatz für den Abgang und den Zugang anzugeben. Die Lagerplatzangabe für das Ziel ist nicht zwingend notwendig. Man erhält aber bei der Prüfung einen Hinweis, wenn man ihn nicht angegeben hat. Bei keiner Eingabe wird die Lagerplatznummer des Abganges auch für den Zugang verwendet. Weitere Pflichtangaben sind die Artikelnummer und mindestens die erste Wiegung. Steht für den ausgewählten Artikel die Partiezuordnung auf ‚immer mit Partie’, dann wird man durch die Prüfung beim Abschließen der Wiegung gezwungen eine Partie anzugeben.  
Hat man eine Partie angegeben, dann wird diese Nummer in den Zu- und Abgang übernommen.

Unter dem Direktsprung [LGU] für Lagerumbuchung findet man nach der Vorgangserzeugung den zugehörigen Datensatz.

Zu beachten ist:

Die Lagerplatzabfrage für den Abgang ist auf der Maske nur aktiv, wenn der Einrichterparameter ‚[Lagerplatzabfrage aktiv](./funktionen_auf_der_waagenmaske/einrichterparameter_in_der_waage.md)’ entsprechend gesetzt ist.
