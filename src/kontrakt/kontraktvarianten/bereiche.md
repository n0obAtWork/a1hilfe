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

<p class="just-emphasize">Variantenbereich [![](../../ImagesExt/image8_148.png)](./bereiche.md)</p>

Im Variantenbereich werden alle allgemeinen Informationen zum Bereich hinterlegt.

<div class="table-wrapper">
  <table>
    <tbody>
      <tr>
        <td colspan="2">
          <p><strong>Variantenbereich</strong></p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Lfd. Nummer in Variante</p>
        </td>
        <td>
          <p>Die lfd. Nummer bestimmt die Reihenfolge (aufsteigend nach Nummer) im Ausdruck.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Formularbereich</p>
        </td>
        <td>
          <p>Hier wird der Bezug zum Formular (siehe “Formulareinrichter”) hergestellt.</p>
          <p>Damit wird im Programmablauf sichergestellt, dass die hier gemachten Angaben sich (z.B.) auf die “Artikelposition” des Formulars beziehen. Welche Informationen aus der “Artikelposition” ausgedruckt werden, ist im Formular selbst hinterlegt.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Bezeichnung</p>
        </td>
        <td>
          <p>Dies ist wieder ein freier Text zur besseren Beschreibung eines Bereiches.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Maximale Anzahl im Druck</p>
        </td>
        <td>
          <p>Gibt an, wie oft der Bereich maximal gedruckt werden soll. Wenn eine 0 eingegeben wird, so wird der Bereich bis zu maximal 50 Zeilen gedruckt.</p>
          <p>Wird eine Zahl größer als 0 eingegeben, so wird der Bereich genauso oft gedruckt, wie die eingegebene Anzahl vorgibt.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Maximale Anzahl Folgezeilen</p>
        </td>
        <td>
          <p>Maximale Anzahl von Folgezeilen, bei der Eingabe von 0 wird auch keine Folgezeile gedruckt.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Bereichsüberschrift</p>
        </td>
        <td>
          <p>Der auszudruckende Bereich kann eine Überschrift erhalten.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Beschriftung 1. Zeile</p>
        </td>
        <td>
          <p>Beschriftung für die erste Zeile des Bereiches. Sie ersetzt praktisch die Bereichsüberschrift, wenn die Bereichsbeschreibung (z.B. “Qualität”) in der ersten Spalte des Formulars erfolgt.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Pflicht</p>
        </td>
        <td>
          <p>Wird der Bereich als Pflicht gekennzeichnet, kann er im Kontraktstamm nicht abgeschaltet werden.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Optimierung Bereichsende</p>
        </td>
        <td>
          <p>Mit dieser Option kann die Optimierung am Bereichsende festgelegt werden.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Bereich nicht drucken</p>
        </td>
        <td>
          <p>Falls der Bereich nicht gedruckt werden soll, kann dies hier festgelegt werden.</p>
        </td>
      </tr>
      <tr>
        <td>
          <p>Textbaustein aktiv</p>
        </td>
        <td>
          <p>Kennzeichnet, ob der Bereich im Kontraktstamm automatisch ausgewählt sein soll, oder nicht.</p>
        </td>
      </tr>
    </tbody>
  </table>
</div>

<p class="just-emphasize">Textbaustein [![](../../ImagesExt/image8_148.png)](./bereiche.md)</p>

Die als Festtext definierten Bereiche werden später bei der [Kontrakterfassung](../kontraktstammdaten/index.md) unter ***Textbausteine*** zur Korrektur und Ergänzung angeboten. Hier wird also der Text soweit erfasst, dass er weitgehend in den individuellen Kontrakt übernommen werden kann, dort können dann Ergänzungen vorgenommen werden. Siehe dazu auch den Abschnitt “[Kontraktstammdaten](../kontraktstammdaten/index.md)”.

Bei Festtexten ist es möglich Platzhalter einzubinden. Diese werden später in der Kontrakterfassung abgefragt. Zum definieren eines Platzhalters kann folgender Text an der Position eingegeben werden.

```text
Hier befindet sich ein Platzhalter: #0$$$$$$$$$$$$$$$
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

<p class="just-emphasize">Private Itembox [![](../../ImagesExt/image8_148.png)](./bereiche.md)</p>

Für die Festtextbereiche lassen sich private Itemboxes hinterlegen. Die Maske zum bearbeiten der Itemboxes lässt sich über die Funktion ***Private Itembox*** aufrufen.

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

<p class="just-emphasize">Standardwerte [![](../../ImagesExt/image8_148.png)](./bereiche.md)</p>

Auf dieser Maske können die Standardwerte für einzelne Platzhalterfelder festgelegt und geändert werden. Für jede Klasse kann der Standardwert unterschiedlich definiert sein.

Das farblich markierte Feld zeigt immer den aktuellen Platzhalter an. Zum Ändern eines anderen Platzhalters wird ein Doppelklick auf den entsprechenden Platzhalter gemacht. Dieser wird dann mit folgender Farbe gekennzeichnet.

<div class="table-wrapper">
  <table>
    <tbody>
      <tr>
        <td></td>
      </tr>
    </tbody>
  </table>
</div>

<p class="just-emphasize">Textbausteinwerte [![](../../ImagesExt/image8_148.png)](./bereiche.md)</p>

Auf dieser Maske können die gelernten Texte der Textbausteine korrigiert werden. Angezeigt werden immer die Texte des aktuell farblich markierten Platzhalters. Zum Wechseln des Platzhalters wird ein Doppelklick auf den entsprechenden Platzhalter gemacht. Dieser wird dann mit folgender Farbe gekennzeichnet.

<div class="table-wrapper">
  <table>
    <tbody>
      <tr>
        <td></td>
      </tr>
    </tbody>
  </table>
</div>
