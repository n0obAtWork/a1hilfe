# Test-Wägungen sollen übergangen werden und keine Fehler verursachen

<!-- source: https://amic.de/hilfe/testwgungensollenbergangenwerd.htm -->

Falls die reguläre Waagen-Datei auch Testwägungen enthält, die nicht weiter verarbeitet werden sollen, aber auch keinen Fehlerabbruch herbeiführen dürfen, so ist im ScriptParameter ZI_DEFAULT der Wert 99 einzutragen.

Ferner ist sicherzustellen, dass der Bereich der Daten, an dem die Zielansprach-Kennung erwartet wird, bei Testdaten einen Wert enthält, der mit keiner gültigen Zielansprache verwechselt werden könnte.

Beispiel: CER = Cerea, FAK=Faktura, ___ (3 Leerzeichen)=Testwägung.
