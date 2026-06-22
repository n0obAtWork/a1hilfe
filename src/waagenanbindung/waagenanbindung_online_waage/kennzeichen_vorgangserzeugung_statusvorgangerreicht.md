# Kennzeichen Vorgangserzeugung (statusVorgangerreicht)

<!-- source: https://amic.de/hilfe/_waage_kennzeichenvorgangerzeugung.htm -->

In der Relation owaage gibt es ein Feld, welches anzeigt, ob für einen Datensatz schon einmal ein Vorgang erzeugt wurde (statusVorgangerreicht = 1). Default ist 0.

Das Feld wird per Update Trigger auf 1 gesetzt, wenn sich der Status eines Waagedatensatzes von 4 (abgeschlossen) auf 5 (mit Vorgang) ändert.

Hat man in der Waage Belege mit Hilfe der Funktion [Wiegungen raffen](./funktionen_auf_der_waagenmaske/wiegungen_abschliessen.md) erzeugt, dann werden diese Belege wie folgt gelöscht:

Der geraffte Beleg wird echt gelöscht (verschwindet ganz aus der Datenbank); die Original-/Ursprungsbelege werden vom Status ‚gerafft gelöscht’ auf ‚abgeschlossen’ zurückgesetzt.

Existiert zu der Wiegung ein Eintrag in der Siloverwaltung / Lagerverwaltungssystem so kann durch das Setzen des [Steuerparameters 925](../../firmenstamm/steuerparameter/waagensteuerung/allgemeiner_steuerparameter_fuer_die_waage_spa_925.md) mit der Option „SILOPOSLOESCHENBEILOESCHEWIEGUNG“ eingestellt werden, ob die Position aus dem Silo / Ladeträger ausgebucht werden soll. Ist eine Position schon als gelöscht markiert worden, und die dazu gehörige Position im Ladeträger / Silo wurde noch nicht ausgebucht, so kann mit der Funktion Löschen die Position vom Ladeträger / Silo gebucht werden.
