# ImportVorgPositionPartie

<!-- source: https://amic.de/hilfe/importvorgpositionpartie.htm -->

In dieser Relation werden Informationen der Partie(n) einer Position abgelegt. Eine Partie, die hier eingetragen ist, jedoch im System noch nicht existiert, wird angelegt werden.

| Feld | Bedeutung |
| --- | --- |
| IVP_GUID | Guid der dazugehörigen Position der Relation  
ImportVorgPosition |
| Zaehler | Partiezähler |
| PartieId | PartieId |
| PartieNummer | Partienummer  
Ist die Partienummer gesetzt und die Partiebezeichnung wird mit der Kombination  
Partienummer und Partiebezeichnung nach der Partie gesucht. Wenn nur die Partienummer gesetzt worden ist wird nach der Partienummer gesucht  
Existiert mehr als eine Partie zu einer Partienummer wird immer die erste Partie gewählt |
| PartieBezeichnung | Ist nur die Partiebezeichnung angegeben worden, und zu dieser Partie wurde keine aktive Partie gefunden, so wird eine neue Partie angelegt.  
Sind Partienummer und Partiebezeichnung angegeben, so wird die Partie nach dieser Kombination gesucht. |
| PartieAbDatum | Wird bei Neuanlage einer Partie ausgewertet und als Partieabdatum gesetzt |
| PartieBisDatum | Wird bei Neuanlage einer Partie ausgewertet und als Partiebisdatum gesetzt |
| Menge | Menge der Partie |
| ME | Mengeneinheit der Partie |
