# Ladeträgerverwaltung an der Waage

<!-- source: https://amic.de/hilfe/_waage_ladetraegerverwaltung.htm -->

Um die [Lagerverwaltung](../firmenstamm/lagerverwaltungssystem/index.md) bzw. die [Siloverwaltung](../firmenstamm/siloverwaltung/index.md) zu verwenden muss der [Steuerparameter 636](../firmenstamm/steuerparameter/optionen_global/lagerverwaltungssystem_spa_636.md) Lagerverwaltungssystem auf „Ja“ gestellt werden. Ist die Lager/Siloverwaltung auf aktiv gestellt worden, so wird auf der Registerkarte LVS/Silo/Kontrakt der Bereich Silo-/Lagerverwaltung angezeigt. Auf der Registerkarte Wiegung wird ein Schnellerfassungsfeld für das Silo bzw. für den Ladeträger eingeblendet.

Bevor das [Lagerverwaltungssystem](../firmenstamm/lagerverwaltungssystem/index.md) bzw. die [Siloverwaltung](../firmenstamm/siloverwaltung/index.md) an der Waage benutzt werden kann, muss diese Eingerichtet werden.

1. Anlage von ein oder mehrere [Ladeträgertypen](../firmenstamm/lagerverwaltungssystem/ladetraegertyp.md).

2. Anlage von [Ladeträger](../firmenstamm/lagerverwaltungssystem/ladetraeger.md) bzw. [Silos](../firmenstamm/lagerverwaltungssystem/ladetraeger.md)

3. Anlage von einer oder mehreren [Lokalitäten](../firmenstamm/lagerverwaltungssystem/lokalitaeten/index.md)

4. [Buchen](../firmenstamm/lagerverwaltungssystem/ladetraeger_buchungen.md) eins Silos / Ladeträgers auf eine [Lokalität](../firmenstamm/lagerverwaltungssystem/lokalitaeten/index.md)

<p class="just-emphasize">Vorbelegung des Ladeträgers / Silos und der Lokalität / Standort</p>

Nach der ersten erfolgreichen Bebuchung eines Ladeträgers / Silo werden sich diese Daten gemerkt. Diese Felder werden dann beim Erstellen einer neuen Wiegung mit den zuletzt Erfassten Daten vorbelegt.

<p class="just-emphasize">Ablauf</p>

Mit einer [Eingangswiegung](./waagenanbindung_online_waage/funktionen_in_der_auswahlliste/wareneingang_wiegung_rohwareneingang_f6_sf6.md) wird die gewogene Menge auf ein oder mehrere Ladeträger gebucht. Mit einer [Ausgangswiegung](./waagenanbindung_online_waage/funktionen_in_der_auswahlliste/warenausgang_wiegung_rohwarenausgang_f7_cf7.md) wird die gewogene Menge von einem oder mehreren Ladeträgern abgebucht. Dabei wird keine Prüfung gemacht, ob der Artikel sich auf dem Ladeträger befindet.

Es ist jetzt möglich eine Silo Buchung an der Waage zu einem bestimmten Zeitpunkt zu buchen. Dazu muss der [Einrichterparameter](../firmenstamm/einrichterparameter/online_waage_epa_owaage.md) „Belegnr bearbeiten“ auf „Ja“ gestellt werden. Wird in dem Feld Uhrzeit nicht eingetragen, so wird die Buchung um 0:01 des eingetragenen Datums durchgeführt. Die Bewegungszeit im Protokoll wird dann auf das entsprechende Datum gesetzt.

Mit der Funktion „Silo nachbuchen“ in der Auswahlliste ist es möglich Wiegungen, die nicht mehr rückgängig abgeschlossen werden können, da aus diesen schon ein Vorgang erzeugt worden ist, nachträglich in das Silo / den Ladeträger einzubuchen. Als Buchungszeit wird dann die Zeit der zweiten Wiegung genommen. Auch hier wird berücksichtigt, ob auf diesem Silo / Ladeträger schon eine [Leermeldung](../firmenstamm/siloverwaltung/silo_silobestand/leermeldung.md) erfolgt ist. Die Wiegung wird dann zeitlich richtig eingeordnet.

Einer Wiegung können ein oder mehrere Ladeträger / Silos zugeordnet werden. Soll der Wiegung nur ein Ladeträger oder Silo zugeordnet werden, so kann der Ladeträger / das Silo im Feld Silo / Träger, welches sich auf der Registerkarte Wiegung befindet, eingetragen werden. Wird in diesem Feld ein Eintrag gemacht, so wird das Feld Ladeträger / Silo in der Datentabelle im Bereich Silo-/Lagerverwaltung auf der Registerkarte LVS/Silo/Kontrakt mit diesem Wert vorbelegt.

<p class="just-emphasize">Aufteilen der Wiegemenge auf mehrere Ladeträger / Silos</p>

Soll die Wiegung auf mehrere Ladeträger / Silos verteilt werden, so wird die Aufteilung im Bereich Silo-/Lagerverwaltung auf der Registerkarte LVS/Silo/Kontrakt durchgeführt. In der Datentabelle können dann mehrere Ladeträger / Silos ausgewählt und zugeordnet werden.

Jedem dieser Ladeträger / Silo muss dann manuell eine Menge zugeordnet werden. Hierbei ist zu beachten, dass die Gesamtmenge aller Positionen in der Datentabelle nicht größer als die Wiegemenge sein kann. Wird die Menge bei der Eingabe überschritten, so wird diese in Rot dargestellt. Ist die gesamt eingegebene Menge kleiner als die Wiegemenge, so wird keine Meldung ausgegeben.

Die Menge wird immer in der Mengeneinheit des Artikels auf den Ladeträger oder auf das Silo gebucht. Dies bedeutet, wenn die Mengeneinheit der Wiegung sich von der Mengeneinheit des Artikels unterscheidet, so wird die zu buchende Menge in die Mengeneinheit des Artikels umgerechnet.

<p class="just-emphasize">Lokalität / Standort</p>

In dem Feld Lokalität / Standort wird die Lokalität oder der Silostandort eingetragen. Ist die eingetragene Lokalität oder der eingetragene Standort unterschiedlich zu einer Lokalität oder einem Standort von einem der aufgelisteten Ladeträger / Silos so wird beim Bebuchen des Ladeträgers / Silos der Ladeträger / Silo auf diese Lokalität / Standort umgelagert. Dabei ist zu beachten, wenn die eingegebene Lokalität oder der eingegebene Standort sich auf einem anderen Lager befindet als das aktuelle Lager der Lokalität des Ladeträgers oder der aktuelle Standort des Silos so wird eine technische Lagerumbuchung erzeugt. Hierbei ist zu beachten, dass alle Artikel die sich auf dem Ladeträger / Silo befinden müssen auch auf dem Ziellager angelegt worden sein.

Die Verschiebung des Ladeträgers / Silo auf eine andere Lokalität kann in den [Waagenprozessen](./waagenanbindung_online_waage/prozess_einrichten/registerkarte_silo.md) an und ausgeschaltet werden.

<p class="just-emphasize">Einbuchen der Wiegungsmenge in ein Silo / Ladeträger</p>

Durch die Funktion [Wiegung abschließen](./waagenanbindung_online_waage/funktionen_auf_der_waagenmaske/wiegungen_abschliessen.md) wird eine neue Position an dem Ladeträger / Silo hinzugefügt. Die Position erhält entweder die komplette Menge oder die jeweilige Menge aus der Datentabelle. Dabei wird der Ladeträger / Silo auf die neue Lokalität oder auf den neuen Standort umgebucht, wenn dieser nicht der eigenen Lokalität oder dem eigenen Standort entspricht. Vor der Buchung der Position wird noch einmal geprüft, ob die zu Buchende Menge größer ist als die Wiegemenge, ist dies der Fall, so wird das Abschließen der Wiegung verhindert.

Wird eine Wiegung für ein Silo nacherfasst, und in der Zwischenzeit wurde für das Silo eine [Leermeldung](../firmenstamm/siloverwaltung/silo_silobestand/leermeldung.md) erzeugt, so bucht das System diese Wiegung hinter die Leermeldung mit dem aktuellen Datum ein.

<p class="just-emphasize">Ausbuchen einer Wiegungsmenge aus einem Silo / Ladeträger</p>

Wird eine Wiegung mit der Funktion [Rückgängig Abschließen](./waagenanbindung_online_waage/funktionen_auf_der_waagenmaske/abschliessen_rueckgaengig.md) wieder geöffnet, so werden alle Positionen von den Ladeträgern / Silos gelöscht, die dieser Wiegung zugeordnet worden sind. Ist in der Zwischenzeit auf dem Ladeträger eine [Leermeldung](../firmenstamm/siloverwaltung/silo_silobestand/leermeldung.md) erfolgt, so wird das Entfernen der Position vor die Leermeldung gebucht. Die bestehende Leermeldung wird angepasst, und die auszubuchende Menge wird neu berechnet.

Sollte sich eine Position durch das [Rückgängig Abschließen](./waagenanbindung_online_waage/funktionen_auf_der_waagenmaske/abschliessen_rueckgaengig.md) einer Wiegung nicht löschen lassen, so kann die Position manuell gelöscht werden. Wie dies funktioniert, ist in der Hilfe zum [Lagerverwaltungssystem](../firmenstamm/lagerverwaltungssystem/ladetraeger_buchungen.md) beschrieben worden.

Des Weiteren besteht die Möglichkeit den Ladeträger/Silo per [Leermeldung](../firmenstamm/siloverwaltung/silo_silobestand/leermeldung.md) komplett leer zumachen.
