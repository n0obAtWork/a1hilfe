# Materialverbrauch Produktion

<!-- source: https://amic.de/hilfe/_lvs20_ProdVerbrauch.htm -->

Wenn Material aus der Bereitstellungszone im LVS für die Produktion entnommen wird, so muss die Menge in der Bereitstellungszone reduziert werden. Dazu gibt es verschiedene Strategien. Die Auswahl hängt von der Frage ab, ob Restmaterialien (teilentleerte Kisten/Paletten) evtl. wieder ins Lager zurückgebracht werden sollen.

Szenario1: Restmaterialien ins Lager

Sobald die Produktion beendet ist, wird in der Schnittstelle der Verbrauch des Materials und der entsprechende Ladeträger angegeben. So kann gerechnet werden: Menge auf dem Ladeträger Minus Verbrauch = neuer Bestand Ladeträger.

Eine Anzeige muss alle Ladeträger mit Material darauf anzeigen, die in der Bereitstellungszone ohne Allokation stehen und dem Bediener anbieten, diese mit einem Fahrauftrag ins Lager zu versehen.

Diese Lösung stellt die Anforderung ans Produktionssystem, die entnommene Menge pro Ladeträger(Palette) zurückzumelden.

Szenario2: Material verbleibt in der Bereitstellungszone

Sobald das Material in der Bereitstellungszone ankommt wird in einem Makro das Material vom ankommenden Ladeträger auf einen Sammelladeträger in der Produktion gebucht.

Beim Eingang der Ende-Meldung wird die verbrauchte Menge von Sammel-Ladeträger ([Typ Linie](../einrichtung_lvs.md#LVS_Einrichtung_LTT) – [eingerichtet in SPA 1037](../einrichtung_lvs.md#LVS_Einrichtung_SPA)) abgebucht. Die Materialien werden stets vollständig verbraucht oder durch eine 1:1-Produktion wieder im LVS angemeldet.
