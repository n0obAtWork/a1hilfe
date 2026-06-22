# Vorgangsimport vor Import prüfen (SPA 1131)

<!-- source: https://amic.de/hilfe/_SPA_1131.htm -->

Ist der Steuerparameter deaktiviert, so werden nur jene Positionen importiert, die gültig sind. Fehlerhafte Zeilen werden auf fehlerhaft gesetzt und nicht importiert. Meldungen finden sich im Fehlerprotokoll. Es kann so zu unvollständigen Belegen kommen.

Ist der Steuerparameter aktiviert und der Import-Parameter useCS=1, wird der Import nicht durchgeführt, wenn dieser bei der Plausibilitätsprüfung Fehler enthält. Auf diese Weise können keine teilweisen Importe eines Beleges erfolgen.
