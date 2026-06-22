# ZollPositionen in Zollausfuhr

<!-- source: https://amic.de/hilfe/zollpositioneninzollausfuhr.htm -->

Hierbei handelt es sich um das Fenster zur Bearbeitung der, in der Ausfuhr enthaltenen, Positionen, die keine Leergutartikel sind (Artikelstamm.Artistamtyp ungleich 5).

Im unteren Teil der Ansicht befindet sich eine Tabelle zur Anzeige und Bearbeitung aller zur Position gehörigen Packstücke. Die Packstück-Zeilen werden beim Anlegen des Ausfuhrvorgangs durch die im Modul „Formularzuordnung/Vorgangsunterklasse“ [FRZ] der Unterklasse des Quellbelegs zugeordnete Datenbankprozedur im Feld „DB-Prozedur für Packstücke“ vorbelegt. Ist dort keine Prozedur hinterlegt, so wird die Vorbelegung durch die exemplarische Datenbankprozedur „Zoll_Packstuecke“ vorgenommen. 

Die einzelnen Zeilen können hier auch manuell überschrieben werden.

| **Parameter** | **Bedeutung** |
| --- | --- |
| Artikel | Artikelnummer und Artikelbezeichnung der Position. |
| Lager | Lagernummer und Lagerbezeichnung der Position. |
| Warennummer | Zollwarennummer, vorbelegt aus dem Artikelstamm. |
| Ursprungsbundesland | Das Ursprungsbundesland gibt an, aus welchem Bundesland stammt oder ob der Artikel seinen Ursprung im Ausland hat. |
| Verfahren | Der Verfahrenscode gibt an unter welchen Voraussetzungen der ausgewählte Artikel versendet wird. Der Verfahrenscode wird mit ‚10‘ (Endgültige Ausfuhr) vorbelegt. |
| Vorgang. Verfahren | Der Code für das vorangehende Verfahren ist hier zu erfassen. Das vorangehende Verfahren wird mit ‚00‘ (ohne vorrangehendes Verfahren) vorbelegt. |
| Nat. Zusatzverfahren | Optionales nationales Zusatzverfahren |
| Eigenmasse | Hierbei handelt es sich um das Eigengewicht der Position in kg. Der Wert wird aus dem Positionsgewicht der zugehörigen Warenposition mittels der Angaben „Gewicht pro Grundmengeneinheit“ im zugehörigen Artikelstamm vorbelegt. |
| Statistische Menge | Nur bei dafür laut Zollwarennummer vorgesehenen Artikeln: Anzugebende statistische Menge in der geforderten Mengeneinheit. |
| Rohmasse | Die Summe der Rohmassen aller Packstücke zur Position. Diese kann nicht kleiner als die Eigenmasse der Position sein. |
| Nettopreis | Der Nettowert der Warenposition |
| Statistischer Wert | Der zollrechtlich anzugebende statistische Wert |
| Nettopreis | Der Nettowert der Warenposition |
| Positionsvermerk | Vermerk für die Position |

<p class="just-emphasize">Packstücke</p>

| **Parameter** | **Bedeutung** |
| --- | --- |
| Art | Zollcode für die Packstückart (F3) |
| Bezeichnung | Das Feld Bezeichnung zeigt die genaue Beschreibung des bei dem Feld „Art“ angegebenen Kürzels an |
| Anzahl | Hierbei handelt es sich um die Anzahl der Packstücke dieser Packstückart |
| Stammartikel | Verweis auf einen zum Packstück passenden Leergut-Artikelstamm |
| Zeichen/Nummer | Dieses Feld beschreibt die auf dem Packstück aufgebrachte Beschriftung an |
| Rohmasse | Hier wird das Gesamtgewicht des in diesem Packstück enthaltenen Anteils der Position angezeigt. Beispiel: 3 Paletten á 300 kg mit derselben Aufschrift ergeben ein Gesamtgewicht von 900 kg für diese Packstück-Zeile |
