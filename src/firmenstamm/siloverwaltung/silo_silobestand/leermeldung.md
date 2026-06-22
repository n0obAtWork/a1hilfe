# Leermeldung

<!-- source: https://amic.de/hilfe/_leermeldung.htm -->

Mit der Funktion ***Leermeldung*** wird ein Silo leergemeldet. Die Leermeldung wird über die Waage abgebildet, d.h. es wird erst automatisch ein Waagenbeleg erzeugt mit der Differenzmenge zu 0. Nach dem der Waagenbeleg erzeugt worden ist, wird das Silo leergemeldet. Der Waagenbeleg bekommt dann den Status abgeschlossen.

Soll eine Silobewegung in ein Leergemeldeten Silosatz nachgebucht werden, so wird auch eine neue Leermeldung für diesen Silosatz erzeugt. Der aktuelle Silo bestand wird dadurch nicht verändert.

Soll bei der Leermeldung die Differenzmenge auf ein Schwundsilo gebucht werden, so wird für das Schwundsilo ebenfalls ein Waagenbeleg erzeugt. Dieser bebucht das Silo. Der Status des Waagensatzes steht dann auch auf abgeschlossen.

Damit eine Leermeldung durchgeführt werden kann, müssen für jedes Lager die Leermeldungs Waagenprozesse eingerichtet sein.

Des Weiteren muss für jedes [Lager](../../uebersicht_lager_und_lagerplatz/lagerstamm_lgs.md) ein Lagerkunde zugewiesen sein. Dieser Kunde wird für den Waagenbeleg benötigt.

Leermeldungen können sich dann in der Variante „Silobuch“ mit der Aktivität „Bestandsmeldung Leermeldung“ gefiltert werden.

Meldungen und deren Bedeutung

| Meldung | Bedeutung |
| --- | --- |
| Es konnte kein Artikel auf dem Silo … an der Position … gefunden werden. | Dies bedeutet, dass das Silo schon leergemeldet worden ist, oder das die Ausgewählte Position in der Silobestands Maske leer ist. |
| Leermeldung kann nicht durchgeführt werden. Sa es kein Prozess gibt um eine Waagenbuchung durchzuführen, welche die Menge im Silo auf 0 bringt. | Dies bedeutet, dass die [Waagenprozesse](../../../waagenanbindung/waagenanbindung_online_waage/prozess_einrichten/registerkarte_silo.md) für die Leermeldung nicht richtig eingerichtet worden sind.<br>Folgende Prozesse müssen für jedes Lager eingerichtet sein<br>1. Leermeldung Abgang<br>2. Leermeldung Zugang |
| Es wurde das Schwundsilo … im Prozess … angegeben, Es konnte kein Prozess mit dem Prozesstyp Leermeldung Schwundsilobuchung für die Schwundbuchung gefunden werden. Leermeldung wird nicht durchgeführt. | Wenn diese Meldung erscheint ist in dem [Waagenprozess](../../../waagenanbindung/waagenanbindung_online_waage/prozess_einrichten/registerkarte_silo.md) ein Schwundsilo angegeben worden. Aber kein Prozess für die Schwundsilo buchung wurde eingerichtet.<br>Es muss ein [Waagenprozess](../../../waagenanbindung/waagenanbindung_online_waage/prozess_einrichten/registerkarte_silo.md) für die Schwundbuchung des jeweiligen Silo Lagers eingerichtet werden. |
| Dem Lager … wurde kein Kunde zugewiesen. Waagenbeleg wird nicht erzeugt. | Dem [Lager](../../uebersicht_lager_und_lagerplatz/lagerstamm_lgs.md) muss noch ein Lagerkunde zugewiesen werden. Dieser wird für die Erstellung der Waagenbelege benötigt. |
| Das Anlegen des Waagenbeleges für die Leermeldung des Silos … hat nicht funktioniert.<br>oder<br>Das Anlegen des Waagensatzes für das Schwundsilo … hat nicht funktioniert. | Beim Erzeugen eines Waagenbeleges ist ein Fehler aufgetreten.<br>1. Artikel ist nicht auf dem Silolager<br>2. Dem Lager des Silos ist kein Kunde zugewiesen<br>3. Es existiert keine Wiegemenge<br>4. Es existiert keine Lagernummer an der Lokalität<br>5. Es konnte kein Waagenprozess gefunden werden zu dem Lager des Silos.<br>6. Das Silo konnte nicht gefunden werden<br>7. Das Silo steht auf keiner Lokalität<br>8. Dem Lagerkunden ist keine Währung zugeordnet worden.<br>9. Im [Waagenprozess](../../../waagenanbindung/waagenanbindung_online_waage/prozess_einrichten/registerkarte_silo.md) ist kein [Wiegetyp](../../../waagenanbindung/waagenanbindung_online_waage/prozess_einrichten/index.md) oder kein [Waagenterminal](../../../waagenanbindung/waagenterminals/waagenterminal_uebersicht.md) angegeben worden. |
| Es konnte kein Waagenbeleg erstellt werden | Das Anlegen eines Waagenbeleges per Prozedur (OWaageInsert) hat nicht funktioniert. |
| Der Status der Wiegung … steht nicht auf abgeschlossen. Leermeldung wird nicht durchgeführt. | Es konnte zwar ein Waagenbeleg angelegt werden. Aber dieser kann nicht abgeschlossen werden.<br>Lösung<br>1. Der Waagenbeleg wird in der Hofliste bearbeitet und auf dann mit [Wiegung abschließen](../../../waagenanbindung/waagenanbindung_online_waage/funktionen_auf_der_waagenmaske/wiegungen_abschliessen.md) abgeschlossen. Danach muss die Leermeldung noch einmal aufgerufen werden. Dabei wird kein neuer Waagenbeleg angelegt sondern das Silo nur Leergemeldet. Ist aber für den [Waagenprozess](../../../waagenanbindung/waagenanbindung_online_waage/prozess_einrichten/registerkarte_silo.md) ein Schwundsilo eingerichtet worden, so wird dieses nicht bebucht.<br>2. Die Waage und der Prozess müssen so eingerichtet werden, dass das Abschließen der Wiegung funktioniert. Der erzeugte Waagenbeleg muss gelöscht werden und die Leermeldung muss erneut durchgeführt werden. |
| Die Wiegung … konnte nicht auf dem Silo … gefunden werden. | Das Abschließen der Wiegung hat funktioniert, aber beim Abschließen wurde das Silo nicht bebucht. |
