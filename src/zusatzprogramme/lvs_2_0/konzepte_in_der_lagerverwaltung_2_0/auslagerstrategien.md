# Auslagerstrategien

<!-- source: https://amic.de/hilfe/_lvs20_auslagerstrat.htm -->

Je Vorgangsunterklasse lässt sich eine Auslagerstrategie festlegen. In der zugehörigen Prozedur werden auch die Mengen für mögliche Über- bzw. Unterlieferungen festgelegt. Durch diese „Kulanz“ bei der Auslieferung kann eine unnötig häufige Kommissionierung verhindert werden.

Hier gibt es mehrere Möglichkeiten, die allesamt mehr oder weniger streng das Prinzip FIFO (First In First Out) bzw. bei Beteiligung von Partien mit Gültigkeitsdatum FEFO (First Expire First Out) berücksichtigen.

| **Auslagerstrategien** |
| --- |
| **Wert** | **Bezeichnung** | **Beschreibung** |
| 1 - FIFO Only | Strenges FIFO | Hier wird die Ware streng nach FIFO ausgelagert. Dabei entstehen unter Umständen viele Kommissionierungen. |
| 2 - FIFO GFO | Greatest First out | Bei der Allokation werden die größten Paletten zuerst allokiert. Es entsteht nur noch am Ende der Liste ein Kommissionierungsbedarf. |
| 3 - One Charge Only | Die ganze Lieferung darf nur aus einer Charge bestehen | Die Lieferung der Position darf nur aus einer Charge bestehen. Ist also die eigentlich nach FIFO höher priorisierte Partie nicht in der gewünschten menge verfügbar, so wird die nächste gesucht, die in ganzer Menge verfügbar ist. |
| 4 - FIFO GFO/SFO | Greatest First out Smallest First Out | Wie 2, jedoch wird für die Kommissionierung darauf geachtet, dass möglichst Anbruchpaletten zuerst aufgebraucht werden. Das spart Platz, bedeutet aber u.U. einen höheren Kommissionierungsaufwand. |
