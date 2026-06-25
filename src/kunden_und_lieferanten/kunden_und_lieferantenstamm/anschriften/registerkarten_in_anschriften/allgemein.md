# Allgemein

<!-- source: https://amic.de/hilfe/_anschallgemein.htm -->

| Feld | Beschreibung |
| --- | --- |
| Adresstyp | |
| Person/Firma (Sanktion) | Typ der Angaben dieser Anschrift siehe [Verbotslisteneinrichtung](../../verbotslisten/stammdatenpflege.md)<br> |
| Kurzbezeichnung | |
| Anrede | |
| Vorname | |
| Name | |
| Zusatz1 | Namenszusatz |
| Straße | |
| PLZ/Ort zur Straße | |
| Ortsteil | |
| Postfach | |
| PLZ/Ort zum Postfach | Postleitzahl und Ort, die dem Postfach zugeordnet sind<br> |
| Staat | |
| Ort | |
| Anbauland | Anbauland das bei Nachhaltigkeitsvorbelegung gezogen werden soll |
| Telefon | |
| FAX | |
| Mobiltelefon | |
| e-Postbrief | |
| Partner1 | |
| Partner2 | |
| E-Mail | In dieser Tabelle können diverse Mailadressen hinterlegt werden. In der ersten Spalte gibt man den Bereich an, für welchen die Mailadresse gültig sein soll:<br><ul><li>&nbsp;&nbsp;&nbsp; 1 = Standard e-Mail1</li><li>&nbsp;&nbsp;&nbsp; 2 = Standard e-Mail2</li><li>&nbsp;&nbsp;&nbsp; 3 = Avise (nur Hauptanschrift)</li><li>&nbsp;&nbsp;&nbsp; 4 = Mahnung (nur Hauptanschrift)</li><li>&nbsp;&nbsp;&nbsp; 5 = Zinsabrechnung (nur Hauptanschrift)<br>Für Avise, Mahnung und Zinsabrechnung kann zusätzlich angegeben werden, ob der Belegversand mit oder statt Belegdruck, oder gar nicht geschehen soll.<br>&nbsp;<br>Zusätzliche Bereiche können über das Anwendungsformat „af_mailtyp“ erfasst werden. Zum Lesen der Daten existiert die SQL-Funktion „Mailadresse“. Mit folgendem Statement erhält man die unter dem Bereich Avise erfasste Mailadresse der Kundenhauptanschrift:<br>&nbsp;<br>&nbsp;</li></ul> <code>select Mailadresse(AdressIdHauptadr,3) from Kundenstamm where KontoNummer=10111</code> |
| | |
