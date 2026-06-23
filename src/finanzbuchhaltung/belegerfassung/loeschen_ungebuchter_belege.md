# Löschen ungebuchter Belege

<!-- source: https://amic.de/hilfe/lschenungebuchterbelege.htm -->

Es können nur Belege gelöscht werden, die noch nicht verbucht wurden. Für bereits gebuchte Belege muss ein Stornobeleg erstellt werden. Es gibt mehrere Stellen im Programm, an denen die Belege wieder gelöscht werden können:

Hauptmenü > Finanzbuchhaltung > Erfassung > Belegerfassung 

Direktsprung **[FIBE]**

oder

Hauptmenü > Finanzbuchhaltung > Primanota > Primanota 

Direktsprung **[PRIMA]**

oder

Hauptmenü > Finanzbuchhaltung > Buchungen / Journal > Standardvorgänge Fibu > Variante „ungebuchte Belege“ 

Direktsprung **[FISV]**

und überall dort, wo die [Einzelbeleganzeige](../op_verwaltung/einzelbeleganzeige.md) aufgerufen werden kann.

Bereits erfasste oder aus der Warenwirtschaft übertragene Belege können wieder gelöscht werden, falls festgestellt wird, dass diese falsch oder doppelt erfasst wurden. Vor dem Löschen werden folgende Prüfungen durchgeführt.

1. Wird der Datensatz bearbeitet? Wenn dies der Fall ist, so erscheint folgende Meldung und der Datensatz wird nicht gelöscht. **„Satz kann nicht gelöscht werden, da von &lt;Bedienerkürzel> in Bearbeitung!“**

2. Bestimmte automatisch erstellte Belege dürfen nicht gelöscht werden. Zu diesen Belegen gehören Skontobuchungen und Kursdifferenzen. Es erscheint dann die Meldung:

    
**„Kursdifferenzbuchung &lt;Belegnummer> kann nicht gelöscht werden!“  
**bzw.  
**„Skontobuchung &lt; Belegnummer> kann nicht gelöscht werden!“**

Wenn diese Belege falsch sind, so muss die Auszifferung zurückgesetzt werden. Die Belege werden dann automatisch gelöscht bzw. wenn sie schon gebucht waren, werden automatisch Stornobelege erstellt.

3. Ist der Beleg bereits gebucht bzw. schon für das Buchen vorgesehen, kann er nicht mehr gelöscht werden. Man muss diesen Beleg dann Stornieren. Es erscheint in diesem Fall die Meldung  
**„Beleg &lt; Belegnummer > kann nicht gelöscht werden, da bereits gebucht bzw. in Vorbereitung!“**

4. Wurde der Beleg bereits durch den automatischen Zahlungsverkehr beglichen, also ein Scheck gedruckt bzw. eine DTA-Überweisung durchgeführt? Diese Prüfung ist nur möglich, wenn der Zahlungsbeleg nicht nach dem erfolgten DTA gelöscht wurde.  
Um ein versehentliches doppeltes Bezahlen zu vermeiden, wird das Löschen mit der Meldung "Dieser Beleg ist schon durch den automatischen Zahlungsverkehr beglichen worden. Er kann nicht mehr gelöscht werden!" nicht gestattet. Gegebenenfalls kann im Zahlungsverkehr das Druckkennzeichen gelöscht werden, man muss aber dann selbst dafür sorgen, dass dieser Beleg nicht doppelt bezahlt wird.  
Bei Belegen, die nicht gebucht werden konnten, weil sie Fehlerhaft sind, erscheint an Stelle der Meldung eine Abfrage:  
    

**„ACHTUNG! Dieser Beleg ist schon durch den automatischen Zahlungsverkehr beglichen worden.  
Wollen sie ihn trotzdem löschen?“**

5. Zinsbelege, die durch OpenTrans verarbeitet wurden, werden können hier nicht gelöscht werden. Sie müssen in der Zinsabrechnung storniert werden. Es erscheint eine entsprechende Meldung

6. Bei Jahreswechselbelegen wird separate nachgefragt, da hier immer zu beachten ist, dass sowohl die Abschluss- als auch die Eröffnungsbuchung zu löschen ist.

7. In der Anlagenbuchhaltung zugewiesene Belege können erst auf Nachfrage gelöscht werden.  
    

**„Die zugehörigen Einträge in der Anlagenkartei wurden bereits fortgeschrieben und werden daher nicht mit gelöscht.**

**Wollen Sie den Beleg &lt;Belegnummer> trotzdem löschen?“**

Sind alle Tests ohne Probleme durchgeführt worden, wird der Beleg gelöscht. Dabei werden alle eventuell vorhandenen Auszifferungen automatisch zurückgesetzt und der Beleg aus eventuell vorhandenen Mahnungen, Zahlungsbelegen und Zahl- bzw. Mahnvorschlagslisten gelöscht. Belege, die aus der Warenwirtschaft übertragen wurden, werden in der Warenwirtschaft zurückgesetzt, so dass sie dort wieder in dem Zustand sind, als wären sie nicht übertragen worden.

<p class="just-emphasize">Löschen im Batchmode</p>

In den Standardvorgängen Finanzbuchhaltung – Direktsprung **[FISV]** – ist die Funktion zum Löschen von Belegen in der Variante „ungebuchte Belege“ integriert. Hier kann man mehrere Belege gleichzeitig markieren und dann löschen. Die Abfrage, ob gelöscht werden soll erscheint nur einmal zu Beginn. Danach werden die Belege nacheinander abgearbeitet. Belege die nicht verarbeitet werden können, werden am Ende aufgelistet. Hier kann es vorkommen, dass Belege, die in der Einzelanwahl gelöscht werden können, abgewiesen werden. Dabei handelt es sich immer um die Sonderfälle, bei denen noch eine Nachfrage nötig gewesen wäre. Zum Beispiel erscheint statt der Abfrage für in der Anlagenbuchhaltung zugewiesene Belege (s.o.) dann am Ende folgende Meldung:

**„In der Anlagenkartei wurde der zugehörige Eintrag bereits fortgeschrieben. Beleg &lt;Belegnummer> wird hier nicht gelöscht.“**

Wenn man diesen Beleg jedoch einzeln markiert, ist er wieder per Nachfrage löschbar.
