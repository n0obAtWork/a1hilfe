# Preismatrix für Listenpreise

<!-- source: https://amic.de/hilfe/_listenPreisMatrix.htm -->

Preise / Konditionen > Konstanten der Preispflege > Preismatrixpflege

Oder Direktsprung [PRM]

Einem Artikel wird im Einkauf und im Verkauf jeweils eine Preismatrix zugeordnet, die an dieser Stelle gepflegt wird. Neben der identifizierenden Matrixnummer kann eine aussagekräftige Bezeichnung vergeben werden. Eine Preismatrix dient dazu, Kunden/Lieferanten mittels der dort zugeordneten [Preisklassen](./preisklasse_fuer_listenpreise.md) eine [Listenpreisdefinition](./definition_von_listenpreisen.md) zuzuordnen, mit deren Hilfe [Listenpreise](./index.md) zum Artikel und Kunden/Lieferanten bestimmt werden können.  
Es können mehrere Listenpreisdefinitionen angegeben werden und jeder Preisdefinition mehrere Preisklassen zugeordnet werden. Dabei darf jede Preisklasse jedoch nur einmal in der gesamten Matrix vorkommen. Eine besondere Bedeutung hat die Zuordnung der Preisklasse 0 zu einer Listenpreisdefinition: Wird während der Preisfindung in der Preismatrix keine Zuordnung der Preisklasse x (ungleich 0!) eines Kunden/Lieferanten zu einer Listenpreisdefinition gefunden, so wird, falls in der Matrix definiert, die Listenpreisdefinition mit der Zuordnung der Preisklasse 0 herangezogen. In diesem Fall bedeutet diese Zuordnung also etwa „alle Kunden/Lieferanten, deren Preisklasse nicht in der Matrix anders zugeordnet sind“. Daher darf die Preisklasse 0 auch nur alleine und nicht mit weiteren Preisklassen einer Listenpreisdefinition zugeordnet werden.  
Zur jeweils aktuellen Listenpreisdefinitionszeile in der oberen Tabelle wird die Zuordnung der Preisklassen in der unteren Tabelle vorgenommen.  
Die Reihenfolge der [Listenpreisdefinitionen](./definition_von_listenpreisen.md) einer Preismatrix bestimmt auch deren Reihenfolge im [Listenpreispfleger](../../artikelstamm_und_artikel/artikel/listenpreise_verkauf_und_einkauf.md). Diese Reihenfolge kann zu jeder Zeit durch Änderung der Sortierungsnummer angepasst werden.

Tabelle Listenpreise

| Spaltenname | Bedeutung |
| --- | --- |
| Position | Sortierungsnummer zur Festlegung der Reihenfolge |
| Nummer | Nummer der Listenpreisdefinition |
| Bezeichnung | Bezeichnung der Listenpreisdefinition |
| Währung | Währungskurztext der Listenpreisdefinition |
| Brutto | Bruttokennzeichen |
| Abweichende Steuergruppe | Nummer und Name der abweichenden Steuergruppe der Listenpreisdefinition |
| Klasse | Auflistung der zugeordneten Preisklassen der Listenpreisdefinition |

Tabelle zugeordnete Preisklassen zur aktiven Listenpreisdefinition

| Spaltenname | Spaltenname |
| --- | --- |
| Nummer | Nummer der Preisklasse |
| Bezeichnung | Bezeichnung der Preisklasse |
