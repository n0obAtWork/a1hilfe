# Zielansprache

<!-- source: https://amic.de/hilfe/zielansprache.htm -->

Die gelesene Zielansprache-Kennung wird in einen internen Wert umgesetzt. Wird keine Zielansprache erkannt, so zieht der Standard, der im Parameter ZI_DEFAULT hinterlegt ist. Ist im Parameter ZI_DEFAULT der Wert 99 hinterlegt, so geht das Programm davon aus, dass es sich bei dem Datensatz um eine Testwägung handelt und geht zum Schleifenanfang und zum nächsten Datensatz über.

Wird auch der Parameter ZI_DEFAULT nicht gefunden, so ist FAKTURA die Standard-Zielansprache.

(Zugehörige Positionsparameter: ZI_SAx)
