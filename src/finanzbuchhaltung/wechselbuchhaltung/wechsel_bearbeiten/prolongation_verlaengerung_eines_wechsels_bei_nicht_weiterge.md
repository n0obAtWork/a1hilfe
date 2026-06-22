# Prolongation / Verlängerung eines Wechsels bei nicht weitergebebenen Wechseln

<!-- source: https://amic.de/hilfe/prolongationverlngerungeineswe.htm -->

Hauptmenü \> Finanzbuchhaltung \> Mahn-/Zahl-/Zinswesen \> Wechselbuchhaltung > Wechsel bearbeiten

Direktsprung **[WEB]**

Kann der Bezogene am Verfalltag die Wechselsumme nicht bezahlen, dann besteht die Möglichkeit der Prolongation, also der Verlängerung der Zahlungsfrist. Dazu muss man in der Anwendung **Wechsel bearbeiten** den Wechsel auswählen und mit F5 ändern. Dort steht einem die Funktion **F9** für **Prolongation** zur Verfügung

![](../../../ImagesExt/image8_751.png)

Es wird im Wechselstamm ein neuer Wechsel mit dem ehemaligen Verfallsdatum als Ausstellungsdatum hinterlegt. Es erfolgt jedoch ***keine Buchung in der FiBu****!* 

Der alte Wechsel wird als verlängert gekennzeichnet. Folgende Prolongationsstati sind implementiert:

<table class="AMICOlavsTabelle" style="WIDTH: 49.74%; BORDER-COLLAPSE: collapse" cellspacing="0" cellpadding="0" width="49%" border="0"><tbody><tr><td style="WIDTH: 7.38%; PADDING-BOTTOM: 0pt; PADDING-TOP: 0pt; PADDING-LEFT: 5.4pt; PADDING-RIGHT: 5.4pt" valign="top" width="7%">0</td><td style="WIDTH: 36.82%; PADDING-BOTTOM: 0pt; PADDING-TOP: 0pt; PADDING-LEFT: 5.4pt; PADDING-RIGHT: 5.4pt" valign="top" width="36%">Originalwechsel nicht verlängert</td><td style="WIDTH: 55.8%; PADDING-BOTTOM: 0pt; PADDING-TOP: 0pt; PADDING-LEFT: 5.4pt; PADDING-RIGHT: 5.4pt" valign="top" width="55%">Gültig</td></tr><tr><td style="WIDTH: 7.38%; PADDING-BOTTOM: 0pt; PADDING-TOP: 0pt; PADDING-LEFT: 5.4pt; PADDING-RIGHT: 5.4pt" valign="top" width="7%">1</td><td style="WIDTH: 36.82%; PADDING-BOTTOM: 0pt; PADDING-TOP: 0pt; PADDING-LEFT: 5.4pt; PADDING-RIGHT: 5.4pt" valign="top" width="36%">Originalwechsel verlängert</td><td style="WIDTH: 55.8%; PADDING-BOTTOM: 0pt; PADDING-TOP: 0pt; PADDING-LEFT: 5.4pt; PADDING-RIGHT: 5.4pt" valign="top" width="55%">Verfallener Wechsel aus Status 0</td></tr><tr><td style="WIDTH: 7.38%; PADDING-BOTTOM: 0pt; PADDING-TOP: 0pt; PADDING-LEFT: 5.4pt; PADDING-RIGHT: 5.4pt" valign="top" width="7%">2</td><td style="WIDTH: 36.82%; PADDING-BOTTOM: 0pt; PADDING-TOP: 0pt; PADDING-LEFT: 5.4pt; PADDING-RIGHT: 5.4pt" valign="top" width="36%">Verlängerter (neuer) Wechsel</td><td style="WIDTH: 55.8%; PADDING-BOTTOM: 0pt; PADDING-TOP: 0pt; PADDING-LEFT: 5.4pt; PADDING-RIGHT: 5.4pt" valign="top" width="55%">(folgt auf Status 0 oder 3)</td></tr><tr><td style="WIDTH: 7.38%; PADDING-BOTTOM: 0pt; PADDING-TOP: 0pt; PADDING-LEFT: 5.4pt; PADDING-RIGHT: 5.4pt" valign="top" width="7%">3</td><td style="WIDTH: 36.82%; PADDING-BOTTOM: 0pt; PADDING-TOP: 0pt; PADDING-LEFT: 5.4pt; PADDING-RIGHT: 5.4pt" valign="top" width="36%">Erneut verlängert</td><td style="WIDTH: 55.8%; PADDING-BOTTOM: 0pt; PADDING-TOP: 0pt; PADDING-LEFT: 5.4pt; PADDING-RIGHT: 5.4pt" valign="top" width="55%">Verfallener Wechsel aus Status 2</td></tr></tbody></table>
