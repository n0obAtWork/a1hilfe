# LKW-Kennzeichen

<!-- source: https://amic.de/hilfe/lkwkennzeichen.htm -->

Das Kfz-Kennzeichen wird gegen den LKW-Stamm validiert und die zugehörige LKW_Nummer ermittelt. Der Parameter LKW_FEHLER_ABBR bestimmt, ob ein nicht gefundener LKW zur Abweisung des Importsatzes führt (Wert1=1) oder nicht (Wert1=0). Bei einem Fehler wird folgender Satz ins Fehlerprotokoll geschrieben: „LKW [...] fehlt in LKW_Stamm [...], Übern. #..., SatzId #..., Zl. #...“. Standardmäßig wird kein Fehlerabbruch durchgeführt.

(Zugehörige Positionsparameter: LKW_SAx)
