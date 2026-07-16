# Gewinnung aus Dateiinhalt

<!-- source: https://amic.de/hilfe/_gewinnungausdateiinh.htm -->

Sollen die Kern-Daten aus dem Dateiinhalt gewonnen werden, sollte man sich zunächst vergegenwärtigen das Dateiinhalt etwas sehr abstraktes und möglicherweise etwas höchst Binäres ist …

Es muss also eine „Absprache“ geben, wie diese Daten aufzufinden sind.

Moderne Scanner-Systeme legen Ihren binären Informationen das Ergebnis einer OCR-Erkennung bei. Damit ist es möglich gezielt nach Mustern zu suchen. Wenn die Dokumente also bei Drucklegung bzw. Erzeugung entsprechend ausgelegt worden sind, ist mit hoher Wahrscheinlichkeit ein gewisser Widererkennungswert gesichert.

Somit wird eine Möglichkeit bereitgestellt, eine Startkennung, sowie eine optionale Ende-Kennung anzugeben. Diese werden dann dazu verwendet, das entsprechende „Schnipsel“ aus der binären Datei herauszufinden, um dann zur weiteren Verarbeitung verwendet werden zu können.

Hier bei gilt das die Start-Kennung und auch die Ende-Kennung – sobald eine angegeben – exakt übereinstimmen müssen. Es sei angemerkt das die Kennungen möglichst „eindeutig“ gewählt sein sollten, damit eine eindeutige Bestimmung überhaupt sinnvoll sein kann. A.eins kann leider nicht in jedem Falle das Muster bestimmen, weil es gar nicht die Ausgangsbelege erzeugt!

Ist keine Ende-Kennung angegeben, liest das System maximal 128 Bytes ein.
