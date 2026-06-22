# Allgemeiner Steuerparameter für die Reklamation (SPA 1036)

<!-- source: https://amic.de/hilfe/_SPA_1036.htm -->

In diesem Steuerparameter können Einstellungen für die Reklamation vorgenommen werden.

| Wert | Bedeutung |
| --- | --- |
| \- | |
| Nummernkreis Reklamation | Die Reklamationsnummer wird über einen Nummernkreis bestimmt, welcher hier eingetragen wird. |
| Vorgangsunterklasse Reklamierer | Unterklasse des Vorgangs welcher beim Reklamierer erzeugt wird. |
| Makro Vorgang erzeugen Reklamierer | Makro zum erzeugen des Vorgangs beim Reklamierer. (Makro muss die komplette Neuanlage übernehmen und die V_Id in den Reklamationstamm eintragen.) |
| Angebotsunterklasse | Alt |
| Auftragsunterklasse | Alt |
| Vorgangsklasse Reklamierer | Vorgangsklasse des Vorgangs welcher beim Reklamierer erzeugt wird. |
| Vorgangsklasse Verursacher | Vorgangsklasse des Vorgangs welcher beim Verursacher erzeugt wird. |
| Vorgangsunterklasse Verursacher | Unterklasse des Vorgangs welcher beim Verursacher erzeugt wird. |
| Makro Vorgang erzeugen Verursacher | Makro zum erzeugen des Vorgangs beim Verursacher. (Makro muss die komplette Neuanlage übernehmen und die V_Id in den Reklamationstamm eintragen.) |
| SQL-Prozedur Vorgang Reklamierer | Prozedur welche vor dem Erzeugen des Vorgangs ausgeführt wird. Bei Rückgabe ungleich „“ wird der Vorgang nicht erzeugt. („Reklamation_SqlMuster“ dient als Muster) |
| SQL-Prozedur Vorgang Verursacher | Prozedur welche vor dem Erzeugen des Vorgangs ausgeführt wird. Bei Rückgabe ungleich „“ wird der Vorgang nicht erzeugt. („Reklamation_SqlMuster“ dient als Muster) |
