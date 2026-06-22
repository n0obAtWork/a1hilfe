# SLEEP

<!-- source: https://amic.de/hilfe/sleep.htm -->

Syntax

SLEEP [RANDOM] nnnn;

Purpose

Wartet nnnn Milisekunden, bevor das nächste Statement ausgeführt wird.

Anwendung

Befehlszeile, Kommandodatei

Berechtigung

Alle Anwender

Beschreibung

Dieses Kommando wartet lediglich einige Milisekunden, bis dass nächste Statement ausgeführt wird. Entweder gibt man direkt an, wie viele Milisekunden es sein sollen oder zusätzlich noch das Schlüsselwort RANDOM. Dann wird eine Zufallszahl erzeugt, die mit der angebebenen zahl multipliziert wird. Dies ist nützlich, wenn man automatische Tests in Mehrbenutzerumgebungen durchführen will und eine asynchrone Verarbeitung erreichen will.

Beispiel

```text
SLEEP 1000 // Wartet 1 Sekunde
```
