# Kundennummer (Waagendatenimport-/-export)

<!-- source: https://amic.de/hilfe/kundennummerwaagendatenimporte.htm -->

Kann die Kundennummer nicht gelesen werden, so wird sie auf 0 gesetzt. Belege, die unberechtigt mit der Kundennummer 0 belegt werden, können später nicht in Vorgänge umgewandelt werden!

Steht der ScriptParameter TANKKTE_KUNDx auf 1, so wird die gelesene Kundennummer als Tankkartennummer interpretiert. Anschließend wird über die Relation KundenTankKarte der zugehörige Kunde ausgewählt. Ein Fehler an dieser Stelle bewirkt einen Eintrag im Fehlerprotokoll (und die Abweisung des Importes dieses Datensatzes): "DBFehler b. Kunden-Ermittl. f. Tankkarte [...], Datei [...], Übern. #..., Zl. #..."

Die Kundennummer wird zusammen mit EK-/VK-Kennzeichen bzw. Vorgangsklasse gegen den Kundtyp validiert. Ein Fehler an dieser Stelle bewirkt einen Eintrag im Fehlerprotokoll (und die Abweisung des Importes dieses Datensatzes): „KundNr o. Typ falsch [...], Datei [...], Übern. #..., Zl. #...“.

(Zugehörige Positionsparameter: KU_SAx)
