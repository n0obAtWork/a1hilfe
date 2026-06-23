# Neue Partie anlegen F8

<!-- source: https://amic.de/hilfe/_waage_neuepartieanlegen.htm -->

Mit dieser Funktion hat man die Möglichkeit, nach Eingabe eines Artikels direkt eine neue Partie anzulegen und in die Waagenmaske zu übernehmen. Es öffnet sich das Fenster zum Neuanlegen einer Partie, in dem die notwendigen Angaben (wie z.B. Gültigkeiten) gemacht werden müssen.

Die Vorbelegung für die Partiebezeichnung wird über den Einrichterparameter „Vorbelegung der Partiebezeichnung“ bestimmt. Dieser EPA kann folgende Werte annehmen:

- **Anlieferungsnummer**
- **Artikeltext**: Die Artikelbezeichnung
- **Automatisch als Jahr (2st.) und Vertragsnr.**: Die zweistellige Jahrnummer und die sechsstellige Vertragsnummer. Sollte die Vertragsnummer kürzer sein, wird sie linksseitig mit Nullen aufgefüllt. Wenn sie länger ist, werden nur die letzten sechs Stellen verwendet.
- **per Makro**: Es wird das Makro ausgeführt, dass im Einrichterparameter „Makro zur Vorbelegung der Partiebezeichnung“ eingetragen ist. Das Makro muss die ermittelte Partiebezeichnung in der LDB-Variablen LDB_TRANSFER$VC ablegen. Aus technischen Gründen ist es momentan nicht möglich, einzelne Hochkommata in der erzeugten Partiebezeichnung zu verwenden.
