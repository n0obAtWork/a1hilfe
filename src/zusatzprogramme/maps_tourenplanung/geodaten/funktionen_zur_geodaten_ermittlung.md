# Funktionen zur Geodaten-Ermittlung

<!-- source: https://amic.de/hilfe/funktionenzurgeodatenermittlun.htm -->

Auswahlliste (einzelne Punkte)

Kunden > Menü Karte > Geodaten ermitteln

Zunächst einmal lassen sich in den Anwendungen Anschriften und Kunden die Geodaten für die gewählte Anschrift bzw. die Hauptanschrift des gewählten Kunden mit der Funktion „Geodaten ermitteln“ im gewählten Webservice abfragen und in die Anschrift eintragen. Bitte beachten Sie, dass hierbei je nach Anbieter Kosten anfallen können.

**Menüfunktion (initial)**

Stammdatenpflege > Anschriften > offene Geodaten ermitteln

Aus dem Menü lassen sich initial Anschriften mit Geodaten versehen. Eine Prozedur, die im Mandantenstamm festgelegt wird, ermittelt hier die Anschriften, die mit GeoDaten versehen werden sollen und fragt für diese die GeoDaten bei dem eingestellten Anbieter ab.

Die Prozedur wird im [Mandantenstamm](../../../firmenstamm/firmenkonstanten/mandantenstamm.md#MND_GeoDaten) eingepflegt

Bitte beachten Sie, dass dies u.U. sehr lange dauern kann und ggf. Kosten je nach Anbieter nach sich ziehen kann.

**Event**

A.eins kann im Zeitplandienst die Menüfunktion aufrufen und somit z.B. in der Nacht alle neuen Anschriften nachpflegen.
