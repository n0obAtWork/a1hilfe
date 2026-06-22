# Wiegungen abschliessen

<!-- source: https://amic.de/hilfe/_waage_wiegungabschliessen.htm -->

Wiegungen kann man an zwei Stellen der Online-Waage abschließen:

In der Auswahlliste **F11**:

Man kann einen oder mehrere Datensätze mit Status 2te Wiegung markieren und abschließen. Es erscheint eine Abfrage, ob man die Wiegungen wirklich abschließen möchte.  
Wenn man keinen bestimmten Datensatz markiert hat und Wiegungen abschließen wählt, erscheint eine Abfrage, ob man wirklich alle Datensätze abschließen möchte. Bestätigt man dann mit Ja, werden alle Wiegungen in der Auswahlliste mit Status 2te Wiegung abgeschlossen.  
Sind Wiegungen dazwischen, die diesen Status nicht haben, erhält man eine Mitteilung, dass diese nicht abgeschlossen werden konnten.  
Der Status der Wiegungen, die abgeschlossen werden konnten, wird auf abgeschlossen gesetzt.

In der Waagen-Maske **F11**:

Hier kann man die in der Maske geöffnete Wiegung abschließen, wenn der Status 2te Wiegung ist.

Bei angeschlossener [Silo](../../../firmenstamm/siloverwaltung/index.md) / [Ladeträgerverwaltung](../../../firmenstamm/lagerverwaltungssystem/index.md) wird die Wiegemenge in die zugewiesenen Silo / Ladeträger gebucht. Ausführliche Informationen zu diesem Punkt ist in der Hilfe [Ladeträgerverwaltung an der Waage](../../ladetraegerverwaltung_an_der_waage.md) beschrieben.

Ist der Schalter „[Bei Restmengenüberschreitung Nettomenge aufteilen](../prozess_einrichten/registerkarte_rohware.md#RestmengenUeberschreitung)“ im Wiegeprozess auf der Registerkarte Rohware auf „Ja“ gestellt, wird bei Abschluss der Wiegung automatisch geprüft, ob die Nettowiegemenge nach Abzug der Qualitäten die Kontraktrestmenge nicht übersteigt. Ist dies der Fall, wird der Überschuss auf einen weiteren Kontrakt verteilt, falls einer vorhanden ist. Eine automatische Aufteilung auf mehr als zwei Kontrakte ist nicht vorgesehen.
