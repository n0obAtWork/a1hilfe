# SKRIPT FALSCH PARAMETRISIERT!

<!-- source: https://amic.de/hilfe/skriptfalschparametrisiert.htm -->

Beim Einlesen der Scriptparameter wurden Fehler festgestellt, die dazu führen würden, dass die ASCII-Daten nicht korrekt gelesen werden können. Nähere Informationen liefert das Fehlerprotokoll (Direktsprung: [FEHLP]).

Mögliche Fehler sind:

Einer der Parameter MEN_SA1 .. MEN_SA4 enthält in Wert1 oder Wert2 eine 0. Das könnte dazu führen, dass eine Mengenangabe nicht gelesen wird. In allen benutzten Satzarten müssen Wert1 und Wert2 ungleich 0 sein, außerdem müssen die betreffenden Parameter aktiv geschaltet sein.

Die Parameter ART_AUS_SORTx und SORT_AUS_ARTx derselben Satzart x stehen gleichzeitig auf 1. Dies führt zu dem unauflösbaren Widerspruch, dass nämlich die Artikelnummer aus der Sortennummern und die Sortennummer aus der Artikelnummer jeweils über eine Umsetztabelle ermittelt werden sollen. Selbstverständlich darf nur eine Konvertierung je Satzart erfolgen, so dass mindestens einer der Parameter auf 0 gesetzt werden muss.

Für eine Satzart x wurde folgende Konstellation erkannt: Die Artikelnummer wird nicht gelesen, (ART_AUS_SORTx&lt;>1) gleichzeitig soll die Artikelnummer nicht aus der Sortennummer ermittelt werden (im Parameter ART_SAx ist Wert1 oder Wert2 =0, oder der Parameter ist inaktiv geschaltet). Dies ist nicht zulässig, da die Artikelnummer zwingend benötigt wird – entweder eingelesen oder aus der Sortennummer konvertiert.
