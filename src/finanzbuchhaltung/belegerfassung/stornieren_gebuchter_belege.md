# Stornieren gebuchter Belege

<!-- source: https://amic.de/hilfe/stornierengebuchterbelege.htm -->

Ist ein Beleg, der Fehlerhaft ist, bereits gebucht, so kann er nicht mehr gelöscht werden. Er muss dann storniert werden. Es gibt verschieden Stellen im Programm, an denen für einen Beleg automatisch ein Stornobeleg erstellt werden kann.

    
Hauptmenü > Finanzbuchhaltung > Buchungen / Journal > Standardvorgänge Fibu > Variante „gebuchte Belege“ 

Direktsprung **[FISV]**

und überall dort, wo die [Einzelbeleganzeige](../op_verwaltung/einzelbeleganzeige.md) aufgerufen werden kann. Dort lässt sich für den Stornobeleg unter dem [Einrichterparameter](../../firmenstamm/einrichterparameter/einzelbeleganzeige_epa_fikinfoe.md) „Darf ein Stornobeleg geändert werden?“ einstellen, ob er im Nachhinein geändert bzw. gelöscht werden darf. Die stornierten Belege werden als storniert gekennzeichnet, damit sie nicht versehentlich ein zweites Mal storniert werden.

Vor der automatischen Erstellung der Stornobelege werden vom Programm einige Tests durchgeführt.

1. Es können nur bestimmte Belegarten storniert werden. Technische Belegarten, die durch die Auszifferung entstehen (z.B. Interne Umbuchungen oder Teilzahlungsbelege) werden ggf. durch das Zurücksetzen der Auszifferung automatisch storniert.

2. Ist der Beleg eventuell ausgeziffert so muss die Auszifferung zurückgesetzt werden. Man muss dies vor der Erstellung bestätigen.

3. Ist die Periode des Belegs offen? Wenn dies nicht der Fall ist, wird man aufgefordert die Periode anzugeben, in die der Stornobeleg gestellt werden soll.

4. Rohwarenbelege dürfen nicht storniert werden, wenn der Rohwarenbeleg selbst bereits weiterverarbeitet wurde.

5. Bei Kunden, die ihre Belege mittels OpenTrans erhalten dürfen Zinsbelege nicht mehr storniert werden.

6. Da das Fibu-Übertragskennzeichen bei Warenwirtschaftsbelegen wieder zurückgesetzt wird, muss man erst bestätigen, dass dies auch gewollt ist.

7. Bereits einmal stornierte Warenwirtschaftsbelege können nicht versehentlich ein zweites Mal storniert werden.

8. Ist der Beleg von einem anderen Benutzer gesperrt, so wird er nicht storniert.

Sind alle Tests erfolgreich durchgeführt worden, so wird ein Beleg erstellt, der bis auf das Vorzeichen des Betrags dem Originalbeleg entspricht.

Stornieren im Batchmode

In den Standardvorgängen Finanzbuchhaltung – Direktsprung **[FISV]** – ist die Funktion zum Stornieren von Belegen in der Variante „gebuchte Belege“ integriert. Hier kann man mehrere Belege gleichzeitig markieren und dann in einem Rutsch stornieren. Die Abfrage, ob die markierten Belege storniert werden sollen, erscheint nur einmal zu Beginn und zwar mit folgendem Hinweis:

**Achtung, sollten in dieser Auswahl Belege existieren, die aus der Warenwirtschaft importiert wurden,**

**so werden diese wieder als nicht übertragen gekennzeichnet.**

**Sollen alle &lt;Anzahl> Belege storniert werden?**

Danach werden die markierten Belege nacheinander abgearbeitet. Belege die nicht verarbeitet werden können, werden am Ende aufgelistet. Hier kann es vorkommen, dass Belege abgewiesen werden. Dabei handelt es sich immer um die Sonderfälle, bei denen noch eine Nachfrage nötig gewesen wäre (siehe oben). Diese Belege müssen dann einzeln markiert werden.
