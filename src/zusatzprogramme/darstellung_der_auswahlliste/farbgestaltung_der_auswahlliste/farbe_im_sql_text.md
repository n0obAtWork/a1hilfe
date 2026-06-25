# Farbe im SQL-Text

<!-- source: https://amic.de/hilfe/farbeimsqltext.htm -->

Im SQL-Text kann pro FIELD – Zeile die Farbe angeben. Dabei gibt es verscheiden Schlüsselwörter, die alle gleichzeitig in einer FIELD - Zeile stehen können

**STYLE=ITALIC  
**Die gesamte Spalte wird kursive dargestellt.

**STYLE=BOLD  
**Die gesamte Spalte wird in Fettschrift dargestellt. Soll eine Spalte sowohl Fett als auch Kursiv dargestellt werden, so muss man die FIELD - Zeile wie folgt darstellen:

```text
FIELD ZahlBankZEmpf,ZahlBankZEmpf,char,20,STYLE=BOLD,STYLE=ITALIC
```

**FGCOLOR=RED  
**Die Vordergrundfarbe der Spalte wird rot. Man kann folgende Farben verwenden:

BLAU

GRÜN

TÜRKIS

ROT

MAGENTA

GELB

WEISS

SCHWARZ

GRAU

Ist der Name der Farbe nicht bekannt, so erscheint diese Farbe nicht.

**BGCOLOR=MAGENTA**

Die Hintergrundfarbe wird mit diesem Schlüsselwort gesetzt. Die möglichen Farben sind dieselben wie für die Vordergrundfarben.

**COLOR=(bankcolor,1=ROT/WEISS,2=GELB/SCHWARZ,...,9=BLAU/TÜRKIS)  
**Dies ist das Schlüsselwort, dass dafür sorgt, dass einzelne Zellen farblich unterschieden werden können. Ein Anwendungsbeispiel ist z.B., dass man bei Kunden, die ihr Kreditlimit überschritten haben, den Saldo Rot darstellt. Mit diesem Schlüsselwort sollte sparsam umgegangen werden, da es sich auf die Performance der Darstellung auswirkt. Man sollte auch nur einzelne Spalten einfärben und nicht für jede FIELD – Anweisung die Farbgestaltung wiederholen um die gesamte Zeile einzufärben.

Hinter dem Schlüsselwort COLOR folgt eine Variable Anzahl von Parametern, die in Klammer stehen müssen.

Der erste Parameter - in dem obigen Beispiel **bankcolor** – ist das Feld bzw. die Funktion, mit deren Hilfe der Wert bestimmt wird. Als Rückgabe wird eine Zahl zwischen 0 und 9 erwartet. 0 bedeute, dass die Zelle nicht eingefärbt wird. Wird eine Zahl größer 0 zurückgeliefert, so wird diese Zelle in der Farbe dargestellt, die als weitere Parameter angegeben wurde. Liefert **bankcolor** zum Beispiel 2 zurück würde dann im obigen Beispiel der Vordergrund Gelb und die Hintergrundfarbe schwarz dargestellt.

Im SQL-Text kann **bankcolor** so aussehen:

```text
.
.
SQL select if Saldo < Kreditlimit then 1 else 0 endif as bankcolor, …
.
.
```
