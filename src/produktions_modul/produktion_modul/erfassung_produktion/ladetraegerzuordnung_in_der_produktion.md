# Ladeträgerzuordnung in der Produktion

<!-- source: https://amic.de/hilfe/_ladetraegerzuordProduktion.htm -->

Mit der Funktion Ladeträgerzuordnung in der Produktion können nachträglich dem Produkt und den einzelnen Komponenten einer Produktion / Vermahlung Ladeträger zugewiesen werden.

Achtung:

Die N zu M Produktion wird in diesem Modul nicht unterstützt.

#### Voraussetzungen:

1. In dem [Lagerstamm](../../../firmenstamm/uebersicht_lager_und_lagerplatz/lagerstamm_lgs.md) muss an den jeweiligen Lägern ein Kunde hinterlegt sein.

2. Es müssen [Waagenprofile](../../../waagenanbindung/waagenanbindung_online_waage/prozess_einrichten/index.md) für den [Produktionszugang](../../../waagenanbindung/waagenanbindung_online_waage/prozess_einrichten/registerkarte_silo.md) sowie für den [Produktionsabgang](../../../waagenanbindung/waagenanbindung_online_waage/prozess_einrichten/registerkarte_silo.md) angelegt werden.

3. Der Steuerparameter [Lagerverwaltungssystem 636](../../../firmenstamm/steuerparameter/optionen_global/lagerverwaltungssystem_spa_636.md) muss auf Ja gestellt sein.

#### Funktionsweise des Moduls:

In der ersten Zeile des Grids wird entweder das Produkt oder der Ausgangsartikel einer Vermahlung dargestellt. Alle weiteren Zeilen sind die Komponenten der Produktion / Vermahlung. Hier besteht die Möglichkeit jeder Position ein Ladeträger zuzuordnen. Mit der Funktionalität „Starte Zuordnung“ werden die zugeordneten Ladeträger mit den Positionsdaten bebucht.

#### Löschen / Einfügen von im Grid

Es können keine „Stamm“ Positionen aus dem Grid gelöscht werden. Bei aufgeteilten Positionen kann nur die angefügte Position gelöscht werden. Des Weiteren ist das Einfügen von neuen Positionen nicht möglich.

#### Position Aufteilen

Es besteht die Möglichkeit eine Produktionsposition auf mehrere Ladeträger aufzuteilen. Dazu wird die neue Menge in die zu teilende Position eingetragen. Das Programm legt dann automatisch eine neue Position des gleichen Artikels mit der Differenzmenge an. Diese Position kann dann mit „STRG-SHIFT-ENTF“ wieder gelöscht werden. Die Differenz wird dann automatisch wieder auf die eigentliche Position addiert.

#### Buchen von Position in das Lagerverwaltungssystem

Das Buchen der einzelnen Position in das Lagerverwaltungssystem übernimmt die [Waage](../../../waagenanbindung/waagenanbindung_online_waage/index.md). Beim Ausführen der Funktion „Starte Zuordnung“ wird für jede Position die eine Ladeträgerzuordnung hat, ein neuer [Waagensatz](../../../waagenanbindung/waagenanbindung_online_waage/mustervorlage_in_der_waage.md) mit den jeweiligen Informationen der Position angelegt. Dieser Waagesatz wird automatisch „[Abgeschlossen](../../../waagenanbindung/waagenanbindung_online_waage/funktionen_auf_der_waagenmaske/wiegungen_abschliessen.md)“. Durch diese Aktion wird der jeweilige Ladeträger bebucht.

Wird in der [Waage](../../../waagenanbindung/waagenanbindung_online_waage/index.md) einer dieser Datensätze über die Funktionalität „[Abschließen Rückgängig](../../../waagenanbindung/waagenanbindung_online_waage/funktionen_auf_der_waagenmaske/abschliessen_rueckgaengig.md)“ wieder eröffnet, so verschwindet die Zuordnung zwischen der Produktion und dem Ladeträger.

Achtung:

Beim erneuten „[Abschließen](../../../waagenanbindung/waagenanbindung_online_waage/funktionen_auf_der_waagenmaske/wiegungen_abschliessen.md)“ der Wiegung über die Waage, wird die Zuordnung zu dieser Position nicht mehr hergestellt.

Sind keine [Waagenprofile](../../../waagenanbindung/waagenanbindung_online_waage/prozess_einrichten/index.md) für den [Produktionszugang](../../../waagenanbindung/waagenanbindung_online_waage/prozess_einrichten/registerkarte_silo.md) sowie für den [Produktionsabgang](../../../waagenanbindung/waagenanbindung_online_waage/prozess_einrichten/registerkarte_silo.md) angelegt worden, so legt das System automatisch die beiden Profile an. Dazu werden je nach Buchungsrichtung jeweils ein vorhandenes Warenausgangs / Wareneingangsprofil kopiert. Um die automatisch angelegten Profile einfacher zu identifizieren, bekommen diese folgende Bezeichnung „Automatische Anlage bei Produktionsbuchung (Lager …).

Wurde für die ausgewählte Produktion / Vermahlung schon eine Zuordnung vorgenommen, so werden die Ladeträger beim Betreten der Maske angezeigt. Wird jetzt für eine der dargestellten Positionen ein anderer Ladeträger ausgewählt, so diese Position wird aus dem Lagerverwaltungssystem ausgebucht und die dazugehörige Wiegung erhält den Status gelöscht. Für den neu eingetragenen Ladeträger wird wieder ein neuer Datensatz angelegt.

Beim Stornieren der Produktion / Vermahlung werden die dazugehörigen Waagenbelegen, sowie die dazugehörigen Einträge zum Lagerverwaltungssystem nicht gelöscht.
