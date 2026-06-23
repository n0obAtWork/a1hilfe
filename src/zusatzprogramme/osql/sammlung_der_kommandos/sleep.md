# SLEEP

<!-- source: https://amic.de/hilfe/sleep.htm -->

<p class="just-emphasize">Syntax</p>

SLEEP [RANDOM] nnnn;

<p class="just-emphasize">Purpose</p>

Wartet nnnn Milisekunden, bevor das nächste Statement ausgeführt wird.

<p class="just-emphasize">Anwendung</p>

Befehlszeile, Kommandodatei

<p class="just-emphasize">Berechtigung</p>

Alle Anwender

<p class="just-emphasize">Beschreibung</p>

Dieses Kommando wartet lediglich einige Milisekunden, bis dass nächste Statement ausgeführt wird. Entweder gibt man direkt an, wie viele Milisekunden es sein sollen oder zusätzlich noch das Schlüsselwort RANDOM. Dann wird eine Zufallszahl erzeugt, die mit der angebebenen zahl multipliziert wird. Dies ist nützlich, wenn man automatische Tests in Mehrbenutzerumgebungen durchführen will und eine asynchrone Verarbeitung erreichen will.

<p class="just-emphasize">Beispiel</p>

```text
SLEEP 1000 // Wartet 1 Sekunde
```
