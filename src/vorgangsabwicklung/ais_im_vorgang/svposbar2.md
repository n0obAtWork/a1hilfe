# SVPOSBAR2

<!-- source: https://amic.de/hilfe/svposbar2.htm -->

Auf der SVPOSBAR2 Maske wird nach jeder Eingabe eines Wertes das zu aktualisieren AIS aufgerufen. Folgende IDs werden in Abhängigkeit des Feldes an das Makro übergeben werden.

\*Die Maskenfelder werden hier in FRZ zugeordnet.

| Maskenfeld | Übergebene IDs | Nummer | Typ |
| --- | --- | --- | --- |
| FN_LagerNummer\* | ID_LAGERNUMMER | | Maskenfeld |
| FN_Menge\* | ID_MENGE | | Maskenfeld |
| FN_ArtikelNummer\* | ID_ARTIKELNUMMER | | Maskenfeld |
| FN_Preis\* | ID_PREIS | | Maskenfeld |
| FN_RabattEingabe\* | ID_RABATT | | Maskenfeld |

Benötigte JVARS

| JAVR | Funktion | Bedeutung |
| --- | --- | --- |
| VORGANGHANDLE | Lesend | Mit dieser JAVR wird der aktuelle Handle des Vorgangs übergeben |
| WAPOSITIONHANDLE | Lesend<br> | Mit dieser JVAR wird der Handle der Warenposition übergeben. |
| ID | Lesend<br> | Mit dieser JVAR wird die Nummer der ID übergeben |
| FELDNAME | Lesend | Diese JVAR enthält den Namen des aufrufenden Feldes. Das Feld kann aber auch eine Funktion sein. Es wird der Feldname aus der Spalte Maskenfeld von der Tabelle drüber an das Makro übermittelt. |
