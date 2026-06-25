# Ausführen von Skripten zulassen

<!-- source: https://amic.de/hilfe/_SkripteZulassen.htm -->

Die Steuerung des Replikationssystems erfolgt teilweise über Powershell-Skripte. Dies bedeutet dafür zu sorgen, dass diese Skripte auf dem System auch ausgeführt werden dürfen. Dies erreicht man am einfachsten durch das Setzen des „ExecutionPolicy“-Wertes über die Powershell.

Dazu rufen Sie bitte eine Powershell mit Administrator-Rechten auf und geben folgenden Befehl ein:

```powershell
Set-ExecutionPolicy unrestricted
```

Hiermit wird das Ausführen von Skripten erlaubt.
