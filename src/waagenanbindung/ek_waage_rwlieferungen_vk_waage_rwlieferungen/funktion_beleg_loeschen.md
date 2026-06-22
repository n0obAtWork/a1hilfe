# Funktion Beleg löschen

<!-- source: https://amic.de/hilfe/_rwwev_belegloeschen.htm -->

Mit der Funktion ***Beleg löschen*** kann man Rohwarenbelege löschen.  
Beim Löschen von Rohwarenbelegen wird man gefragt, ob man den zugehörigen Waagedatensatz auch löschen möchte. Diese Abfrage erscheint nur, wenn der Rohwarenbeleg nicht den Status erledigt hat, d.h. aus ihm noch keine Lieferung erzeugt wurde.  
 

Beim Löschen wird unterschieden zwischen Rohwarenbelegen, die eine VorgangsId der Bestandsführung enthalten und denen die keine enthalten.

Löschen von Rohwarenbelegen mit Bestandsführung aus der Waage:

Mit Hilfe der BestandsVorgId im Rohwarenbeleg, der gelöscht werden soll, wird der zugehörige Waagedatensatz gesucht. Wenn man die Abfrage, ob der Waagendatensatz auch gelöscht werden soll, mit Ja beantwortet, dann wird der Waagedatensatz auf den Status gelöscht gesetzt und die VorgangsId des Bestandsbeleges (gespeichert im Feld owaage_bvid) wird aus ihm entfernt. Der zugehörige Bestandsbeleg wird storniert.  
Falls das Stornieren des Bestandsbeleges fehlschlägt, erhält man eine Warnung, dass man den Beleg von Hand löschen muss. Außerdem gibt es einen Eintrag im Fehlerprotokoll.  
entscheidet man sich gegen das Löschen des Waagedatensatzes, dann wird dieser vom Status ‚mit Vorgang’ auf den Status ‚Abgeschlossen’ zurückgesetzt.

Löschen von Rohwarenbelegen ohne Bestandsführung aus der Waage:

Mit Hilfe der OwaageId im Rohwarenbeleg, der gelöscht werden soll, wird der zugehörige Waagedatensatz gesucht.  
Wenn man die Abfrage, ob der Waagendatensatz auch gelöscht werden soll, mit Ja beantwortet, dann wird der Waagedatensatz auf den Status gelöscht gesetzt.  
entscheidet man sich gegen das Löschen des Waagedatensatzes, dann wird dieser vom Status ‚mit Vorgang’ auf den Status ‚Abgeschlossen’ zurückgesetzt.
