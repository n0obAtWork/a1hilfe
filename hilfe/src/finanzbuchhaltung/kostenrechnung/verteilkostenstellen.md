# Verteilkostenstellen

<!-- source: https://amic.de/hilfe/verteilkostenstellen.htm -->

Hauptmenü \> Kostenrechnung \> Kostenstellenstamm \> Verteilkostenstellen

Direktsprung **[VKST]**

Die Kosten einer Verteilkostenstelle verteilen sich auf verschiedene andere Kostenstellen. In diesem Pfleger kann nun die Verteil-Kostenstelle den einzelnen Kostenstellen prozentual zugeordnet werden. Dies kann bestimmten Zeiträumen zugeordnet werden, so dass die prozentuale Einteilung sich im Geschäftsjahresverlauf ändern kann (mit den Funktionen Neuer Gültigkeitsbereich, Neue Periode).

Um eine Kostenstelle als Verteilkostenstelle einzurichten, müssen folgende Eingaben gemacht werden:

| | Beschreibung |
| --- | --- |
| Verteilkostenstellen<br><br> | Nummer der Verteilkostenstelle |
| Bezeichnung | Bezeichnung der Kostenstelle (sprechende und eindeutige Namen erleichtern hier die spätere Suche (Bsp.: KFZ-KI-QM-12345).<br> |
| Matchcode | Kurzbezeichnung der Kostenstelle<br> |
| Erfassungssperre<br><br> | Diese Sperre gilt für die Belegerfassung der Finanzbuchhaltung. Steht diese auf „Ja“, so kann diese Verteilkostenstelle dort nicht mehr verwendet werden. Auch ist es nicht mehr möglich diese Kostenstelle erneut in einer Verteilkostenstelle oder in den Kostenstellengruppen zu verwenden. Ist sie bereits in irgendeiner Verteilkostenstelle eingetragen, so erscheint die Meldung:<br>![](../../ImagesExt/image8_680.png)<br>Die hier angesprochenen Arbeitsschritte müssen manuell durchgeführt werden.<br>Wird in einem Beleg eine gesperrte Kostenstelle verwendet - dies ist z.B. dann möglich, wenn die Sperre erst nach der Verwendung der Kostenstelle gesetzt wurde -, so wird der Beleg nicht gebucht. Es erscheint die Meldung „**Kostenstelle … ist gesperrt!**“ im Buchungsprotokoll.<br> |
| Verteilung 100% | Bei Anwahl dieses Punktes wird überprüft, ob die hier eingegebene Verteilung 100% ergibt. Ist das Kennzeichen „Manuell änderbar“ gesetzt, so wird auch bei Änderung der Werte in der Belegerfassung auf 100% geprüft.<br> <br> |
| Manuell änderbar<br><br> | Bei Einstellung dieser Option können bei der Belegerfassung noch Einstellungen vorgenommen werden.<br> |
| Kostenstelle<br><br> | Hier werden die Kostenstellen erfasst, auf die sich die Verteilkostenstelle aufteilt. Mit F3 kann hierbei aus den im Kostenstellenstamm erfassten ausgewählt werden. Die Bezeichnung wird dann mit der aus dem Kostenstellenstamm belegt. Die Anteile werden dann in der gewünschten Prozentzahl eingegeben.<br> |
| Periode<br><br> | Periode, in der die Kostenstelleneinstellungen gelten. Es gibt eine Standard-Einstellung, die solange gilt, wie keine extra Perioden eingerichtet sind. Man kann hier also z.B. für Saisongeschäfte andere Verteilungen angeben.<br> |
| Gültigkeitsbereich | Datum, ab dem die Kostenstelleneinstellungen gelten sollen.<br> |

**Löschen der Verteilkostenstellen**

Wenn Verteilkostenstellen gelöscht werden, werden sie nicht sofort physikalisch gelöscht, sondern so wie bereits bei den Kostenstellen erwähnt, als gelöscht gekennzeichnet. Diese gelöschten Kostenstellen sind dann für die Belegerfassung gesperrt, erscheinen aber trotzdem – soweit sie bebucht sind – auf den Auswertungen.

Bevor eine Verteilkostenstelle jedoch als gelöscht gekennzeichnet werden kann, muss vorher getestet werden, ob sie nicht noch verwendet wird.

- Wird die Kostenstelle noch als Vorbelegung für die Belegerfassung im Sachkontenstamm verwendet?
- Ist sie als Kostenstelle für die automatischen Buchungen der Mahngebühren/Zinsen im Mahnsatz hinterlegt?
- Ist sie als Fehlerkostenstelle im Mandantenstamm hinterlegt?
- Wird sie in der Relation ARTIKOSTSTGRUPPE verwendet?
- Wird sie als Kostenstelle für die automatischen Buchungen der Zinsrechnung in den Zinsgruppen verwendet?
- Wird sie als Kostenstellenvorbelegung in den Wechselkosten verwendet?
- Ist diese Kostenstelle einer Verteilkostenstelle zugeordnet? Bei dieser Prüfung wird unterschieden, ob die Verteilkostenstelle bereits gelöscht worden ist oder nicht. Bei gelöschten Verteilkostenstellen erfolgt ein Hinweis darauf mit einer Abfrage, ob  
tatsächlich gelöscht werden soll. Bei Verwendung in nicht gelöschten Verteilkostenstellen wird das Löschen nicht durchgeführt.

- Wird diese Kostenstelle in den Periodischen Buchungen hinterlegt?
- Bei Kostenstellen, die bereits bebucht wurden, erfolgt ein Hinweis mit Abfrage, ob tatsächlich gelöscht werden soll.

Wurden diese Tests durchlaufen, wird die Kostenstelle als gelöschte Kostenstelle gekennzeichnet. Alle so gekennzeichneten Kostenstellen finden sich in der Variante „Gelöschte...“ wieder. Dort stehen dann – nachdem man eine Kostenstelle markiert hat – zwei Funktionen zur Verfügung.

1. ***Wiederherstellen***  
Hier öffnet sich dann die Maske mit den Daten dieser Verteilkostenstelle und nach erneutem **F7** wird die Kostenstelle wieder als nicht gelöscht gekennzeichnet.

2. ***endgültig löschen***  
Die Kostenstelle wird ohne weitere Prüfung physikalisch gelöscht. Bei bereits bebuchten Kostenstellen führt dies auch dazu, dass sie nicht mehr auf Auswertungen erscheinen.
