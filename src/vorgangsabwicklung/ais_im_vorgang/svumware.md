# SVUMWARE

<!-- source: https://amic.de/hilfe/svumware.htm -->

Auf der SVWARE Maske wird nach jeder Eingabe eines Wertes das zu aktualisieren AIS aufgerufen. Folgende IDs werden in Abhängigkeit des Feldes an das Makro übergeben werden.

| Maskenfeld | Übergebene IDs | Nummer | Typ |
| --- | --- | --- | --- |
| LagerNummerAbg$ | ID_LAGERNUMMER_ABG | 1801 | Maskenfeld |
| ArtikelIdAbg$ | ID_ARTIKELID_ABG | 1807 | Maskenfeld |
| LagerPlatzAbg$ | ID_LAGERPLATZ_ABG | 1802 | Maskenfeld |
| LagerNummerZug$ | ID_LAGERNUMMER_ZUG | 1804 | Maskenfeld |
| ArtikelIdZug$ | ID_ARTIKELID_ZUG | 1808 | Maskenfeld |
| LagerPlatzZug$ | ID_LAGERPLATZ_ZUG | 1805 | Maskenfeld |
| PreisEinh$ | ID_PREISEINHEIT | 1078 | Maskenfeld |
| PreisEinh_Z$ | ID_PREISEINHEIT | 1078 | Maskenfeld |
| ME_Nummer$ | ID_ME_NUMMER | 1108 | Maskenfeld |
| ME_Nummer_Z$ | ID_ME_NUMMER | 1108 | Maskenfeld |
| ME_NummerPreis$ | ID_ME_NUMMERPREIS | 1077 | Maskenfeld |
| ME_NummerPreis_Z$ | ID_ME_NUMMERPREIS | 1077 | Maskenfeld |
| ZusatzInfos$ | ID_ZUSATZINFO | 1353 | Maskenfeld |
| ZusatzInfo_Z$ | ID_ZUSATZINFO | 1353 | Maskenfeld |
| ZusatzInfos2$ | ID_ZUSATZINFO2 | 1358 | Maskenfeld |
| ZusatzInfo2_Z$ | ID_ZUSATZINFO2 | 1358 | Maskenfeld |
| Preis$ | ID_PREIS | 1000 | Maskenfeld |
| Preis_Z$ | ID_PREIS | 1000 | Maskenfeld |
| Menge$ | ID_MENGE | 1001 | Maskenfeld |
| Menge_Z$ | ID_MENGE | 1001 | Maskenfeld |
| Netto$ | ID_NETTO | 1003 | Maskenfeld |
| Netto_Z$ | ID_NETTO | 1003 | Maskenfeld |
| V_LGUBuchTyp$ | ID_LGU_BUCHTYP | 4500 | Maskenfeld |

Benötigte JVARS

| JAVR | Funktion | Bedeutung |
| --- | --- | --- |
| VORGANGHANDLE | Lesend | Mit dieser JAVR wird der aktuelle Handle des Vorgangs übergeben |
| UMBUCHUNGHANDLE | Lesend  
 | Mit dieser JVAR wird der Handle der Umbuchung übergeben. |
| ID | Lesend  
 | Mit dieser JVAR wird die Nummer der ID übergeben |
| FELDNAME | Lesend | Diese JVAR enthält den Namen des aufrufenden Feldes. Das Feld kann aber auch eine Funktion sein. Es wird der Feldname aus der Spalte Maskenfeld von der Tabelle drüber an das Makro übermittelt. |
