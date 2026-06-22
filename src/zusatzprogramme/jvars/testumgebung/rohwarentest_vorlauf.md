# Rohwarentest Vorlauf

<!-- source: https://amic.de/hilfe/rohwarentestvorlauf.htm -->

Die Vorlaufroutine des Rohwarentests prüft die Usersetuproutine auf Funktionalität incl. der AktiverMandant Klausel -\* =========================================== und alles aus dem Speicher rauswerfen =========================================== /d %VM_TESTINSTALL%\\bin ====================================================================== Die Installationsprozedur ändert das INI Verhalten, auf jeden Fall hier zurückstellen ====================================================================== %vm_system%\\bin\\setini.exe Mandanten AktiverMandant %3 ahoi.ini
