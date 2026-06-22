# Beispiel einer Abwicklung im Modul Strecke (Mappe)

<!-- source: https://amic.de/hilfe/_vorkassestrecke.htm -->

Um die Vorkasse in der Strecke zu benutzen sollten Sie sich erstmals mit der [Streckenerfassung](../../zusatzprogramme/streckenerfassung/index.md) generell auseinandersetzten. Die für die Vorkasse benötigten Funktionen der Streckenerfassung sind [STRECKE] für die Strecke und [VMAPP] für das [Streckenprofil](../../zusatzprogramme/streckenerfassung/profile/index.md#ProfileStreckenerfassung).

**Formulare oder Reporte die ausgedruckt werden sollen werden im Streckenprofil hinterlegt.**

Um mit der Vorkasse in der Strecke[STRECKE] zu starten wird mit der Taste F8 eine neue Strecke angelegt.

Als erstes wird ein Angebot in der Streckenerfassungsmaske angelegt. Dies passiert in der obersten Tabelle. Hier kann das Angebot direkt erfasst werden, oder es wird in der Datentabelle im Feld Belegnummer ausgewählt wobei das Feld Klasse auf 100 gesetzt werden muss.

Als nächstes wird der Kontrakt der Strecke hinzugefügt, dazu wird in das Feld Klasse 660 eingetragen sowie die Unterklasse und die Belegnummer eingetragen. Jetzt ist der Kontrakt der Strecke hinzugefügt worden.

Als nächstes wird in der Strecke ein Auftrag über die Kontraktmenge erzeugt.

Nachdem der Auftrag erstellt worden ist kann jetzt der Ladeschein erstellt werden. Dazu klicken Sie bitte den Auftrag an drücken dann wieder die rechte Maustaste jetzt wählen Sie Vorkasse/ Auftrag aus jetzt können Sie den Ladeschein erstellen. Nachdem der Ladeschein erstellt worden ist, wird dieser auch in der Strecke zu sehen sein.

Ladeschein Erzeugung

| Maskenfelder | Bedeutung |
| --- | --- |
| Kontrakt | Gewählter Kontrakt |
| Kunde | Kontraktkunde |
| Artikel | Kontraktartikel |
| Kreditlimit | Hier wird das aktuelle Kreditlimit aufgeteilt nach den einzelnen Faktoren angezeigt. Des Weiteren wird die Belastung des Kreditlimits durch den neu zu erstellenden Lieferschein mit einberechnet und angezeigt. |
| Abw. Lieferanschrift | Hier kann eine abweichende Lieferanschrift angegeben werden. |
| Kontraktpreis | Preis des Kontraktes |
| Auftrag | Nummer des gewählten Auftrages |
| Auftrags Menge | Auftrags Menge |
| Offene Menge | Noch zu liefernde Menge |
| Originale Menge | Original Menge des Auftrags wird beim Erstellen des ersten Ladescheins gesetzt |
| Mögliche Menge | Gibt an wie viel Menge anhand des Kreditlimits plus Eingangszahlungen geliefert werden kann. Die Menge kann durch drücken der Taste F5 geändert werden. |
| Preisaufschlag | In diesem Feld wird der prozentuale Aufschlag für Rohwarenlieferungen angezeigt. Dieser kann durch drücken der Taste Ctrl+F5 geändert werden. |
| Lieferdatum | Hier kann ein alternatives Planlieferdatum eingetragen werden. |

Damit der Kunde seine Ware am Lagerabholen kann muss ein Ladeschein geschrieben werden. Der Ladeschein wird in das Kreditlimit mit ein berechnet wenn der SPA [Ladeschein ins Kreditlimiteinberechnen(SPA 695)](../../firmenstamm/steuerparameter/vorgangsbearbeitung_allg/ladeschein_ins_kreditlimit_einberechnen_spa_695.md) auf ja gestellt wird.

In der Maske haben Sie jetzt noch die Möglichkeit, eine alternative Lieferanschrift anzugeben, die zuliefernde Menge abzuändern oder ein anderes Lieferdatum einzutragen.

Über die Tasten Kombination Ctrl+F9 „***Freistellungen Drucken***“ wird der Ladeschein mit der möglichen Menge erzeugt. Der Ladeschein wird gedruckt und kann dann an aus dem Formulararchiv an das Lager über die Methode „***Senden an***“ versendet werden.

Ist die Auftragsmenge kleiner als die mögliche Menge, so wird die Auftragsmenge genommen. 

Auf der Maske werden unten in der Datentabelle alle erstellten Ladescheine zu dieser Strecke angezeigt.
