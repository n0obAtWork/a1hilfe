# Wiegen F5

<!-- source: https://amic.de/hilfe/_waage_wiegen.htm -->

Mit der Funktion ***Wiegen*** wird eine Maske geöffnet, die im [Bedienerstamm](../../../firmenstamm/firmenkonstanten/bedienerwesen_bediener_bedienerklassen_und_erfasser/bedienerstamm/index.md) auf der [Registerkarte Waage](../../../firmenstamm/firmenkonstanten/bedienerwesen_bediener_bedienerklassen_und_erfasser/bedienerstamm/bedienerstamm_pfleger.md#Registerkarte_Waage) hinterlegten Kombinationen von Terminal( [Waagentermina l](../../waagenterminals/waagenterminal_uebersicht.md)) und Prozess( [Waagenvorlagen](../prozess_einrichten/index.md) ) anzeigt. An jeder Schaltfläche ist ein Prozess aus dem Bedienerstamm hinterlegt. Wird die Schaltfläche betätigt, so wird die Waagenmaske mit dem jeweiligen Prozess (Terminal, Prozess) gestartet.

Ist im [Bedienerstamm](../../../firmenstamm/firmenkonstanten/bedienerwesen_bediener_bedienerklassen_und_erfasser/bedienerstamm/index.md) nur eine aktive Zuordnung zwischen Terminal und Prozess eingetragen, so wird die Waagenmaske gleich geöffnet, wenn die Funktion Wiegen aufgerufen wird.

Die Maske ist in drei Bereiche unterteilt.

1. [Eingangswiegungen](./wareneingang_wiegung_rohwareneingang_f6_sf6.md) kann Normalware wie Rohware sein

2. [Ausgangswiegungen](./warenausgang_wiegung_rohwarenausgang_f7_cf7.md) kann Normalware wie Rohware sein

3. [Lohnwiegungen](./lohn_schuettwiegung_f8.md) / [Lagerumbuchungen](../../../vorgangsabwicklung/vorgangsbearbeitung_allgemein/vorgangsklassen_in_a_eins/lagerumbuchung.md)

Im Bereich Eingangs - Ausgangswiegungen werden jeweils maximal 12 Kombinationen angezeigt. Im Bereich Lohnwiegungen / Lagerumbuchungen werden für jede Art maximal 3 Kombinationen angezeigt.

Wurde im Terminal( [Waagenterminal](../../waagenterminals/waagenterminal_uebersicht.md) ) ein Bild hinterlegt, so wird dieses Bild auf der Schaltfläche angezeigt. Ist kein Bild hinterlegt worden, so steht als Text in der Schaltfläche „Wiegen“.

Wird in der Hofliste **kein Wiegesatz** markiert und die Funktion ***Wiegen*** wird aufgerufen, so werden alle Möglichkeiten die im [Bedienerstamm](../../../firmenstamm/firmenkonstanten/bedienerwesen_bediener_bedienerklassen_und_erfasser/bedienerstamm/index.md) hinterlegt worden sind auf der Maske angezeigt.

Wurde in der Hofliste **ein Wiegesatz** markiert und die Funktion ***Wiegen*** wird aufgerufen, so werden nur all die Funktionen angezeigt, die zu dem markierten Wiegesatz kompatibel sind. Wird dann eine Schaltfläche gedrückt und dieser Schaltfläche ist ein anderes [Waagenterminal](../../waagenterminals/waagenterminal_uebersicht.md) zugeordnet als das [Waagenterminal](../../waagenterminals/waagenterminal_uebersicht.md) des Wiegesatz, so wird das [Waagenterminal](../../waagenterminals/waagenterminal_uebersicht.md) des Wiegesatzes mit dem an der Schaltfläche hinterlegen [Waagenterminal](../../waagenterminals/waagenterminal_uebersicht.md) überschrieben.

Private Funktion

Es besteht die Möglichkeit diese Mechanik zu privatisieren. Dabei wird nicht mehr die Auswahlmaske aufgerufen, sondern direkt die Waagenmaske. Die Waagenmaske wird dann mit dem im Bedienerstamm hinterlegten Prozess an der Position X gestartet. Für unterschiedliche Bediener können unterschiedliche Prozesse an der Position X im Bedienerstamm hinterlegt sein. Es wird immer der jeweilige Prozess des Bedieners aufgerufen.

Wurde ein Wiegesatz in der Hofliste ausgewählt und der an der Position X hinterlegte Prozess ist nicht kompatibel zu dem ausgewählte Wiegesatz in der Hofliste. So wird die Waagenmaske nicht aufgerufen.

Beispiel: ^jpl OwaageAufrufer 3
