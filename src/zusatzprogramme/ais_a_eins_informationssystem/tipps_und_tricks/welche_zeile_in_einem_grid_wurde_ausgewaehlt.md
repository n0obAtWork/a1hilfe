# Welche Zeile in einem Grid wurde ausgewählt?

<!-- source: https://amic.de/hilfe/welchezeileineinemgridwurdeaus.htm -->

Man hat in AIS die Möglichkeit Controlstrings in Grids zu definieren, so dass man eine Zeile anwählen kann und von dort aus Makros (ais_makro) oder VBA-Scripte (ais_vba) ausführen kann. Nun muss man wissen, welche Zeile in diesem Grid angewählt wurde. Diese Zeile wird in die JVAR AIS_V_GRIDZEILE geschrieben, unter VBA wäre der Syntax folgender:

```text
Zeile =
Aeins.JVARS_GET(7100, "AIS_V_GRIDZEILE " )
```
