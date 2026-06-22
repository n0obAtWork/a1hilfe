# Geschäftsjahr falsch

<!-- source: https://amic.de/hilfe/geschftsjahrfalsch.htm -->

Nach dem Wechsel von 1999 zu 2000 kann dieser Fehler auftreten. Man prüfe unter Direktsprung [OSQL] das folgende Statement:

SELECT AMIC_Jahrnummer_zu_Datum(‘01.01.00‘)

wird jetzt nicht „2000“ ausgegeben sondern „.....“, so muss folgendes Statement abgesetzt werden:

SET OPTION nearest_century=50.

Anschließend sollte das erste Statement den gewünschten Wert liefern. Andernfalls muss ein dbupgrade durchgeführt werden -> Hotline.
