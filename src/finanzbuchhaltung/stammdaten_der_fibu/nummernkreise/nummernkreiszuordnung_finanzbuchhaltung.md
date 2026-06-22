# Nummernkreiszuordnung Finanzbuchhaltung

<!-- source: https://amic.de/hilfe/nummernkreiszuordnungfinanzbuc.htm -->

Hauptmenü > Administration > Nummernkreise \> Fibu-Vorgangszuordnung

Direktsprung **[NKF]**.

Die Belegnummernvergabe erfolgt in der Fibu analog zur Ware über sogenannte Nummernkreise. Zur allgemeinen Einrichtung der Nummernkreise gehört die Einrichtung der Nummernkreise, der Zählkreise und der Gültigkeiten. Die Beschreibung zur Einrichtung dieser Stammdaten findet man im allgemeinen Bereich **Stammdaten**. Für die Finanzbuchhaltung gibt es einen weiteren Pfleger "Fibu-Vorgangszuordnung", in dem für die einzelnen Belegarten pro Bedienerklasse Einstellungen vorgenommen werden können.

| | Beschreibung |
| --- | --- |
| Bedienerklasse | Hier muss eine Bedienerklasse eingetragen sein, wie sie in A.eins hinterlegt ist. Eine Auswahl mit **F3** ist möglich.<br><br> |
| Belegart | Hier wird die Belegart eingetragen. Mögliche Belegarten sind.<br>• ZA Zahlungsverkehr Banken<br>• AR Ausgangsrechnung<br>• AG Ausgangsgutschrift<br>• ER Eingangsrechnung<br>• EG Eingangsgutschrift<br>• SO Sonstige Belege<br>• RP Restposten<br>• SK Skonto<br>• AB Ausbuchungen<br>• WE Wechselerfassung<br>• KD Kursgewinn/Kursverlust<br>• JW Jahreswechsel<br>• EB Eröffnungsbuchung<br>• IU Interne Umbuchung<br>• KU Kostenträgerumbuchung<br>• SE Scheckeinreicher<br>• ZU Zinsumbuchung<br>• KO [Kostenobjektumbuchung](../../kostenrechnung/kostenobjekte/kostenobjekte_einrichtung.md#Kostenobjektumbuchung)<br>Die Belegarten SK, KD, IU und ZU existieren nur als automatische Buchungen.<br><br> |
| Erfassungsform<br><br> | Man kann bei bestimmten Belegarten noch unterscheiden, ob die Einstellungen für die automatisch erzeugten Belege (z.B. bei gebuchten Mahngebühren oder die Zahlungsbelege beim automatischen Zahlungsverkehr) oder für die manuelle Erfassung gelten sollen.<br>Will man also z.B. im automatischen Zahlungsverkehr die Zahlungsbelege buchen, so muss ein Satz eingetragen sein, bei dem automatisch eingetragen ist.<br>Bedienerklasse, Belegart und Erfassungsform sind der eindeutige Schlüssel, über den später bestimmt wird, welche Einstellungen verwendet werden.<br> |
| [Eindeutigkeit](./eindeutigkeit.md)<br><br> | Hier wird angezeigt, wie später die Prüfung auf Eindeutigkeit vorgenommen werden soll. Die Erfassung erfolgt separat unter „***Eindeutigkeit* F9**“. Die Eindeutigkeit ist nicht von der Bedienerklasse, sondern lediglich von der Belegart und der Erfassungsform abhängig.<br> |
| Nummernkreis | Hier wird der Nummernkreis hinterlegt, der bei der Belegerfassung/automatischen Erstellung von Belegen verwendet werden soll. Nummernkreise in der Finanzbuchhaltung sind nur 4-stellig. Eine Auswahl mit **F3** ist möglich.<br> |
| Nummernkreis nicht änderbar | Ist hier der Haken gesetzt, ist bei der Belegerfassung bzw. bei den automatischen Buchungsmodulen der Nummernkreis nicht änderbar. Bei Bankbuchungen, bei denen man für jede Bank einen eigenen Nummernkreis verwendet, sollte hier kein Häkchen gesetzt sein.<br> |
| Belegnummer nicht änderbar | Die vorgeschlagene Nummer ist nicht änderbar, wenn hier ein Häkchen gesetzt ist. Bei "manueller Nummernvergabe" ist hier kein Häkchen zu setzen.<br> |
