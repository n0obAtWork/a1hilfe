# Liste offene Aufträge/Bestellungen – auftrli.rpt

<!-- source: https://amic.de/hilfe/listeoffeneauftrgebestellungen.htm -->

Mit diesem Report kann man sich alle offenen Aufträge/Bestellungen anzeigen lassen.  
Im Auswahlbereich kann man die Kundennummer/Lieferantennummer., die Auftragsnummer/Bestellnummer, das Auftragsdatum/Bestelldatum, das geplante Lieferdatum und den Vertreter eingrenzen.

Die Datensätze werden dem Vorgangstamm entnommen und erst nach der Kundennummer (Feld Kundnummer) und dann nach der Auftragsnummer (Feld V_NumNummer) sortiert ausgegeben.  
Hinter der Kundennummer wird in einem Feld die zur Kundennummer (über KundIdZuord) gehörige Bezeichnung aus dem Kundenstamm angegeben. Dahinter werden, wenn gefüllt, noch die Felder AdressVorname, AdressZeile1 und AdressStrasse zur Versandadresse aus dem Vorgangstamm (V_VersAdressId) angezeigt. Dabei handelt es sich immer um die Versandadresse des ersten Datensatzes aus dem Vorgangstamm der zu diesem Kunden gefunden wurde.
