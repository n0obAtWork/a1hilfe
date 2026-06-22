# Rechnung Lieferschein aus Ladeschein

<!-- source: https://amic.de/hilfe/_reliausladeschein.htm -->

Mit diesem Modul lässt sich ein Ladeschein in ein Lieferschein oder in eine Rechnung wandeln. Dazu werden die einzelnen Ladescheinpositionen in der Auswahlliste markiert.

Dann wird die Funktion ***Re./Lie. Aus Ladeschein*** aufgerufen.

<p class="just-emphasize">Bedeutung der Felder auf der Maske Ladeschein zu Rechnung oder Lieferschein.</p>

Informationsfeld

In diesem Feld werden Ausgaben angezeigt, die während der Konvertierung des Ladescheins zu einem Lieferschein oder einer Rechnung auftreten, wie z.B. „Es wurden n Aufträge zu den gewählten Ladescheinen gefunden“.

Box Aufträge

In der Box Aufträge werden alle Aufträge Angezeigt, die zu den ausgewählten Ladescheinen gehören. In der linken Tabelle werden die Auftragskopfdaten angezeigt. In der rechten Tabelle werden die Positionen zu dem ausgewählten Auftrag angezeigt. Beim Einstieg in die Maske werden immer die Positionen des ersten Auftrages angezeigt. Durch klicken auf einen anderen Auftrag werden die Positionen des Auftrage aktualisiert.

Wenn in dem Feld Auftrag eine 0 steht, so existiert zu diesem Ladeschein kein Auftrag.

Box Ladescheine

In der Box Ladescheine werden alle Ausgewählten Ladeschein samt aller Positionen angezeigt.

Funktionen

Die Funktionen Ladeschein zu Lieferschein und Ladeschein zu Rechnung haben die gleiche Funktionalität, außer das in Abhängigkeit der Funktion eine Rechnung oder ein Lieferschien erstellt wird.

Mit der Funktion Abbruch wird die Maske verlassen ohne eine Aktion durchzuführen.

Ablauf

Sind an einem Ladeschein unterschiedliche Aufträge beteiligt, so wird zu jedem Auftrag ein Lieferschein erzeugt. Die Positionen werden von dem jeweiligen Auftrag per Teildisposition in den Lieferschein übertragen. Konnten einzelne Positionen des Auftrages nicht komplett geliefert werden, so wird die Menge im Auftrag um die gelieferte Menge reduziert.

Besonderheiten

In der Anwendung [Vorgangsunterklasse](../../../formularzuordnung/index.md) **[FRZ]** für die Klasse 500 „Ladeschein“ auf der Registerkarte „[Sperren](../../../formularzuordnung/sperren.md)“ wird eingestellt, ob nach der Erzeugung des Lieferscheins oder der Rechnung der Ladeschein gesperrt oder Storniert werden soll.

Dis Standardeinstellung ist, dass der Ladeschein storniert wird.

Wird der Ladeschein gesperrt so bleiben alle Positionen im Ladeschein nach der Umwandlung erhalten und die Weiterverarbeitungssperre wird gesetzt.

Wenn die Einstellung auf Stornieren gestellt wird, so wird der Ladeschein storniert, wenn nach der Umwandlung zu einer Rechnung oder zu einem Lieferschein sich keine Positionen mehr im Ladeschein befinden.

Wurde einem Ladeschein der aus mehreren Aufträgen erstellt worden ist, ein Artikel ohne Auftragsbindung hinzugefügt, so bleibt der Ladeschein mit der Position nach der Lieferscheinerstellung erhalten. Die Position muss dann einem Lieferschien manuell hinzugefügt werden.

Vor dem Beenden der Erstellung des Lieferscheins werden zwei BAG Variablen gesetzt, damit im Nachlauf Makro auf die Relation ImportVorgStamm zugegriffen werden kann.

| JVARS BAG | Bedeutung |
| --- | --- |
| LATOLIUEBERNAHMEID | Aktuelle ÜbernahmeId |
| LATOLISATZID | Aktuelle SatzId |
