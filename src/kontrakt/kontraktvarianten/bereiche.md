# Bereiche

<!-- source: https://amic.de/hilfe/_bereichekontraktvari.htm -->

In einem Kontraktschreiben bestehen verschiedene Bereiche, so z.B. ein Abschnitt, in dem textlich die Qualitäten beschrieben werden, ein anderer, in dem auf Stammdaten zugegriffen wird (Artikeltext), ein weiterer, in dem andere Parameter mit Rechenfunktionen (z.B. Paritäten) ausgedruckt werden sollen.

Die Reihenfolge des Ausdrucks und z.T. auch der Umfang werden hier bestimmt.

Auf der Maske werden alle Bereiche einer Kontraktvariante dargestellt. Folgende Funktionen stehen zur Verfügung, wobei die Funktionen teilweise nur beim Variantentyp „Festtext“ zur Verfügung stehen.

[Variantenbereich](./bereiche.md#ktr_vari_Variantenbereich)

[Textbaustein](./bereiche.md#ktr_vari_bereich_festtext)

[Private Itembox](./bereiche.md#ktr_vari_privateitembox)

[Standardwerte](./bereiche.md#ktr_vari_standardwerte)

[Textbausteinwerte](./bereiche.md#ktr_vari_textbausteinwerte)

Variantenbereich [![](../../ImagesExt/image8_148.png)](./bereiche.md)

Im Variantenbereich werden alle allgemeinen Informationen zum Bereich hinterlegt.

| Variantenbereich |
| --- |
| Lfd. Nummer in Variante | Die lfd. Nummer bestimmt die Reihenfolge (aufsteigend nach Nummer) im Ausdruck. |
| Formularbereich | Hier wird der Bezug zum Formular (siehe “Formulareinrichter”) hergestellt.  
Damit wird im Programmablauf sichergestellt, dass die hier gemachten Angaben sich (z.B.) auf die “Artikelposition” des Formulars beziehen. Welche Informationen aus der “Artikelposition” ausgedruckt werden, ist im Formular selbst hinterlegt. |
| Bezeichnung | Dies ist wieder ein freier Text zur besseren Beschreibung eines Bereiches. |
| Maximale Anzahl im Druck | Gibt an, wie oft der Bereich maximal gedruckt werden soll. Wenn eine 0 eingegeben wird, so wird der Bereich bis zu maximal 50 Zeilen gedruckt.  
Wird eine Zahl größer als 0 eingegeben, so wird der Bereich genauso oft gedruckt, wie die eingegebene Anzahl vorgibt. |
| Maximale Anzahl Folgezeilen | Maximale Anzahl von Folgezeilen, bei der Eingabe von 0 wird auch keine Folgezeile gedruckt. |
| Bereichsüberschrift | Der auszudruckende Bereich kann eine Überschrift erhalten. |
| Beschriftung 1. Zeile | Beschriftung für die erste Zeile des Bereiches. Sie ersetzt praktisch die Bereichsüberschrift, wenn die Bereichsbeschreibung (z.B. “Qualität”) in der ersten Spalte des Formulars erfolgt. |
| Pflicht | Wird der Bereich als Pflicht gekennzeichnet, kann er im Kontraktstamm nicht abgeschaltet werden. |
| Optimierung Bereichsende | Mit dieser Option kann die Optimierung am Bereichsende festgelegt werden. |
| Bereich nicht drucken | Falls der Bereich nicht gedruckt werden soll, kann dies hier festgelegt werden. |
| Textbaustein aktiv | Kennzeichnet, ob der Bereich im Kontraktstamm automatisch ausgewählt sein soll, oder nicht. |

Textbaustein [![](../../ImagesExt/image8_148.png)](./bereiche.md)

Die als Festtext definierten Bereiche werden später bei der [Kontrakterfassung](../kontraktstammdaten/index.md) unter Textbausteine zur Korrektur und Ergänzung angeboten. Hier wird also der Text soweit erfasst, dass er weitgehend in den individuellen Kontrakt übernommen werden kann, dort können dann Ergänzungen vorgenommen werden. Siehe dazu auch den Abschnitt “[Kontraktstammdaten](../kontraktstammdaten/index.md)”.

Bei Festtexten ist es möglich Platzhalter einzubinden. Diese werden später in der Kontrakterfassung abgefragt. Zum definieren eines Platzhalters kann folgender Text an der Position eingegeben werden.

```text
Hier
befindet sich ein Platzhalter: #0$$$$$$$$$$$$$$$
```

Der Start eines Platzhalters wird mit „#0“ definiert. Die Länge wird dann mit der Anzahl „$“ festgelegt, die Startposition zählt dabei dazu.

Desweiteren ist es möglich einen Platzhalter als Pflichtfeld zu kennzeichnen. Dafür wird statt den „#0“ ein „#1“ verwendet.

| Position | Zeichen | Bedeutung |
| --- | --- | --- |
| 1 | # | Start des Platzhalters, nach diesem Zeichen muss eine Ziffer kommen. |
| 2 | 0 | Zweite Position eines Platzhalters. Es muss sich um eine Ziffer handeln, da es sonst nicht als Platzhalter erkannt wird. |
| 2 | 1 | Zweite Position eines Platzhalters. Wenn diese Position eine 1 ist, handelt es sich um eine Pflichteingabe. Beim Speichern eines |
| 3..n | $ | Fest |
| 3..n | § | Variable |

Private Itembox [![](../../ImagesExt/image8_148.png)](./bereiche.md)

Für die Festtextbereiche lassen sich private Itemboxes hinterlegen. Die Maske zum bearbeiten der Itemboxes lässt sich über die Funktion Private Itembox aufrufen.

Die Maske ist in zwei Bereiche aufgeteilt. Im oberen Bereich befinden sich drei Datentabellen, im unterem ist der Textbaustein mit den Platzhaltern aufgebaut.

Für die Datentabelle „Liste der Itemboxes“ stehen folgende Felder zur Verfügung.

| Feld | Beschreibung |
| --- | --- |
| Itembox | Name der zu verwendenden Itembox |
| Zeile | Zeile im Textbaustein |
| Position | Platzhalterposition in der Zeile |

Zeile und Position können hierbei von Hand eingegeben werden oder per Doppelklick auf den Platzhalter.

Felder der Datentabelle „Zielfelder“.

| Feld | Beschreibung |
| --- | --- |
| Feldname | Name des Rückgabefeldes. Die Namen können über eine Itembox ausgewählt werden, sie beziehen sich dabei auf die „RETURN“ Werte der aktuell ausgewählten Itembox aus der Datentabelle „Liste der Itemboxes“. |
| Zeile | Zeile im Textbaustein |
| Position | Platzhalterposition in der Zeile |

Zeile und Position können hierbei von Hand eingegeben werden oder per Doppelklick auf den Platzhalter.

Felder für die Datentabelle „Kontraktklassen“.

| Feld | Beschreibung |
| --- | --- |
| Klasse | Kontraktklasse für die die Itembox gelten soll. |
| Bezeichnung | Bezeichnung der Kontraktklasse |

Die Itembox kann durch diese Einträge nur für bestimmte Kontraktklassen gültig sein. Wird keine Kontraktklasse angegeben, so gilt die Itembox für alle Klassen. Sollte jedoch eine weitere Itembox für eine bestimmte Klasse gültig sein, wird diese der Itembox ohne Eingrenzung vorgezogen.

Um kenntlich zu machen welche Platzhalterpositionen schon in den Zielfeldern und der Itembox benutzt werden, werden die Felder eingefärbt.

Hierbei gilt folgendes:

| Nur Itembox |
| --- |
| Itembox und Zielfeld |
| Nur ein Zielfeld |
| Mehr als ein Zielfeld |

Standardwerte [![](../../ImagesExt/image8_148.png)](./bereiche.md)

Auf dieser Maske können die Standardwerte für einzelne Platzhalterfelder festgelegt und geändert werden. Für jede Klasse kann der Standardwert unterschiedlich definiert sein.

Das farblich markierte Feld zeigt immer den aktuellen Platzhalter an. Zum Ändern eines anderen Platzhalters wird ein Doppelklick auf den entsprechenden Platzhalter gemacht. Dieser wird dann mit folgender Farbe gekennzeichnet.

| |
| --- |

Textbausteinwerte [![](../../ImagesExt/image8_148.png)](./bereiche.md)

Auf dieser Maske können die gelernten Texte der Textbausteine korrigiert werden. Angezeigt werden immer die Texte des aktuell farblich markierten Platzhalters. Zum Wechseln des Platzhalters wird ein Doppelklick auf den entsprechenden Platzhalter gemacht. Dieser wird dann mit folgender Farbe gekennzeichnet.

| |
| --- |
