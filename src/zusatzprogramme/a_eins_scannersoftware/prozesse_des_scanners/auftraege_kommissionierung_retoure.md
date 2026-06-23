# Aufträge (Kommissionierung / Retoure )

<!-- source: https://amic.de/hilfe/_cescannerauftrag.htm -->

| **Vorgangsfunktions Übersicht** |
| --- |
| Kommissionierung Start |
| Daten einscannen |
| Kommissionierung Ende |

<p class="just-emphasize">Besonderheiten des Aufträge welcher mit einem Scanner bearbeitet werden soll.</p>

In einem Auftrag welcher mit einem Scanner bearbeitet wird, darf es keine Partieverteilung geben. Dies bedeutet, wenn ein Artikel mehrere Partien in dem Auftrag, muss für jede Artikel/Partie Kombination eine eigene Warenposition angelegt werden.

<p class="just-emphasize">Erstellen eines AMIC Etikettendruck Dokuments für den Auftrag</p>

Um eine Kommissionierung mit dem Scanner durchzuführen, kann in der ersten Variante der Auftragsbearbeitung ein AMIC Etikettendruck Dokument ausgedruckt und bearbeitet werden, welches die benötigten Scancodes und eine Liste der Artikel enthält.

Auf dem Report befinden sich drei feste EAN 128 Codes. Es existiert eine Vorlage für den AMIC Etikettendruck (Scanner Auftragsbearbeitung) wenn diese Vorlage in die privaten AMIC Etikettendruck Dokumente übernommen werden soll, so muss die AMIC Etikettendruck Dokumente ID gleich bleiben, da ansonsten das AMIC Etikettendruck Dokument nicht aufgerufen werden kann.

<p class="just-emphasize">Die benötigten Scancodes</p>

- Der erste mit der Auftragsnummer ist der Start Code
- Mit dem Scan Code „STORNO“ kann die zuletzt gescannte Position gelöscht werden.
- Mit „AUENDE“ wird die Auftragsbearbeitung abgeschlossen.

<p class="just-emphasize">Ablauf</p>

- Als erstes wird der Startscancode erfasst wie z.B. AU 55. Beim Startscancode muss immer zwischen dem AU und der Vorgangsnummer ein Leerzeichen stehen. Nach dem der Startscancode erfasst worden ist, werden im unteren Teil des Scanner Bildschirmes alle Position des Auftrags angezeigt. Enthält der Auftrag mehr als neun Positionen so kann mit den Pfeil hoch und runter Tasten geblättert werden.
- Jetzt kann eine Position aus dem Auftrag eingescannt werden. Die Suche, der Position im Auftrag funktioniert so. Wird nur der Artikel erfasst, so wird der erste Artikel mit dem erfassten EAN Code genommen, dies bedeute, dass wen ein Artikel öfters in der Liste vorkommt so wird nur der eine gebucht. Wird jetzt eine Partie zu diesem Artikel erfasst, so wird jetzt in der Kommissionierungsliste nach der Kombination Artikel und Partie gesucht. Zur noch weiteren Eingrenzung kann der Lagerplatz mit dem AI-Code 97 erfasst werden. Die Menge kann entweder per Hand eingegeben oder eingescannt werden. Es kann pro Position im Auftrag nur eine Partie geben. Der Scanner unterstützt beim Erfassen keine Partieverteilung.
- Nach dem abarbeiten der Positionen wird AUENDE eingescannt.

<p class="just-emphasize">Gebinde</p>

Für das Erfassen des Gebindes gibt es mehrere Einstellungsmöglichkeiten. In dem Standardfall kann nur die Ergebnismengeneinheit angegeben werden. Wird der Schalter Gebinde Faktor aus Vorgang auf ja gestellt, so wird die eingegebene Menge als Anzahl genommen und der Gebinde Faktor aus dem Auftrag gelesen. Des Weiteren kann an der Mengeneinheit selbst noch einmal hinterlegt werden, ob die Mengenangabe als Gesamt Menge oder als Gebinde Anzahl gewertet werden soll.

<p class="just-emphasize">Retourenaufträge</p>

Retouren Aufträge sind Aufträge mit einer negativen Menge. Die Retoruen Aufträge können mit dem Scanner genauso abgearbeitet werden wie die „normale“ Auftragsbearbeitung. Da es schwierig ist eine negative Mengeneingabe auf den Scanner durch zuführen, wird die Menge positive erfasst. Beim Abschluss eines Retouren Auftrages werden die Eingegebenen Menge mit -1 multipliziert. Dies bewirkt, dass es die Mengen als negative Mengen berechnet werden. Des Weiteren unterscheidet sich der Scancode des Retouren Auftrages von dem des Auftrages.

| Scancode Auftrag | Scancode Auftrag Retoure |
| --- | --- |
| AU | AUR |
| AUENDE | AURENDE |

Die Einrichtung des Retouren Auftrags wird an dieser Stelle [beschrieben](../anwendung_scanner_in_aeins/scanner_scancodes_bearbeiten_modus/standard_einstellungen_scancodes.md).

<p class="just-emphasize">Maskenfelder die Einfluss auf den Beleg haben stehen [hier](../anwendung_scanner_in_aeins/einrichtung_des_scanners_an_der_zentral_datenbank/server_starten.md)</p>
