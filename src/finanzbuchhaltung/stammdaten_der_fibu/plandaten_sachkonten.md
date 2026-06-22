# Plandaten Sachkonten

<!-- source: https://amic.de/hilfe/plandatensachkonten.htm -->

Hauptmenü > Finanzbuchhaltung > Stammdaten > Sachkonten > Funktionen ***Plandaten*** und ***Plandatenübernahme***

Direktsprung **[SKS]**

Für jedes Sachkonto können Plandaten je Periode angelegt werden. Diese werden in der Tabelle Kontosummen mit den Periodenwerten abgespeichert und stehen so für Auswertungen zur Verfügung.

Manuelle Übernahme pro Konto

Diese Funktionalität erreicht man über die Funktion ***Plandaten*** **F10**:

![](../../ImagesExt/image8_542.png)

Die Periodeneinteilung entspricht der Vorgabe im Firmenstamm. Je Periode werden die Planwerte eingegeben. Bei gleichen Werten genügt die Eintragung in der ersten Periode und Auslösung der Funktion ***Periodenwerte aus 1. Periode***.  
Die Plandaten des Vorjahres können mittels ***Vorjahreswerte übernehmen*** übernommen werden.

Automatische Übernahme für alle Konten

Diese Funktionalität erreicht man über die Funktion ***Plandaten übernehmen*** **SH+F10**.

![](../../ImagesExt/image8_543.png)

| | Beschreibung |
| --- | --- |
| Planzahlen/Ist-Zahlen<br> <br> | Sollen die Planzahlen oder die Ist Zahlen als Grundlage verwendet werden?<br> |
| Aus dem Jahr<br> | Hier gibt man das Jahr an, das als Grundlage dienen soll.<br> |
| Für das Jahr<br> | Für dieses Jahr werden die Planzahlen neu generiert.<br> |
| Oberkonten-Planzahlen erstellen<br> | Die Werte für Oberkonten ergeben sich bekanntlich aus den Werten der Sachkonten. Deswegen existiert hier die Möglichkeit, die Planzahlen gleich für die Oberkonten mit zu generieren. Die Verteilung wird dann anhand der Struktur der Oberkonten vorgenommen.<br> |
| Rundung<br> | Bei Planzahlen sind im Allgemeinen kleine Beträge nicht von Bedeutung. Hier kann man angeben, wie genau die Daten übernommen werden sollen. Werden Werte größer 0 angegeben, so bezieht sich die Rundung auf die Nachkommastellen, bei Werten kleiner 0 werden die Zahlen vor dem Komma gerundet. Gibt man beispielsweise als Rundungsfaktor –2 an, so werden die Werte auf voll 100 Euro gerundet:<br>123456789,123456 => 123456800,00<br>In der Zeile unter dem Eingabefeld für die Rundung, wird immer gezeigt, wie sich der eingegebene Wert auswirkt.<br> |

Nach dem Starten mit **F9** erscheint noch einmal eine Sicherheitsabfrage:

![](../../ImagesExt/image8_544.png)

Hier ist zu beachten, dass evtl. manuell erfasste Planzahlen überschrieben werden.
