# Private Scannerprozedur (SPA 801)

<!-- source: https://amic.de/hilfe/_SPA_801.htm -->

Hier kann zu der IP-Adresse eines Scanners eine private Abarbeitungsprozedur angegeben werden, die eigene entwickelte Module aufruft.

Des Weiteren gibt es im A.eins System schon vorgefertigte Module die mit der Prozedur [CallScannerModule](http://www.amic.de/ihilfe/XMLDocuments/iAeins/html/M_SQL_FUNCTION_CallScannerModule_4_660c634e.htm) aufgerufen werden. Der Steuerparameter 801 muss wie folgt eingerichtet werden, um ein Modul aufzurufen.

| aktiv | IP Adresse | Private Prozedur |
| --- | --- | --- |
| Private Prozedur | z.B. 192.168.241.50 | CallScannerModule( oder eine Private Prozedur) |
