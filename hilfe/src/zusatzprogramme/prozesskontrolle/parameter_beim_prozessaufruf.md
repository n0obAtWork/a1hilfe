# Parameter beim Prozessaufruf

<!-- source: https://amic.de/hilfe/parameterbeimprozessaufruf.htm -->

Die Parameter des Webaccess.wsf Prozesses sind wie folgt zu verstehen:

| process | Name der EXE Datei, die überwacht werden soll, im A.eins Fall handelt es sich hier um die Aeins32.exe |
| --- | --- |
| Idleloops | Schleifenzähler, der angibt wie lange gewartet und geprüft werden soll, bis ein Prozess aus dem Speicher entfernt wird, wenn keine CPU Zeit verbraucht worden ist. Die Zeit, die der Prozess dann stillsteht, bevor er abgebrochen wird, berechnet sich aus : Sleeptime \* IdelLoops in Sekunden |
| Wait | Diese Zeit, gemessen in Sekunden, gibt an, wie lange das System warten soll, bis mit der Prozessüberwachung begonnen wird. Hier handelt es sich um die vorzugebende Startzeit eines Prozesses, bis er im Speicher zur Verfügung steht. Standardmäßig sollte hier eine 5 angegeben werden |
| Forever | Es kann angegeben werden, das der Überwachungsprozess ewig läuft (/forever=1) oder aber nach Beendigung aller zu überwachender Objekte dann selber auch anhalten soll (/forever=0) |
| Sleeptime | Die Anzahl von Sekunden, die gewartet wird, bevor eine neue Zeitmessung erfolgen soll. Bei der Prozessüberwachung wird zunächst eine Zeit gemessen, dann wird gewartet (und zwar sleeptime in Sekunden) um wieder eine CPU Zeitmessung vorzunehmen. Ist während dieser beiden Messungen keine CPU Zeit verbraucht worden, so wird der Prozess zum ersten mal als "nicht arbeitend" gekennzeichnet, folgt nun eine &lt;idleloops> malige Kennzeichnung dieses Prozesses als "nicht arbeitend" und zwar direkt hintereinander, dann wird der Prozess aus dem System entfernt. |
| | |

Beispiel einer Mandantenserverüberwachung wäre:

Webaccess.wsf /process=aeins32.exe /sleeptime=5 /wait=5 /idleloops=50 /forever=1

**ACHTUNG:** die Leerzeichen zwischen den Parametern sind zwingend vorgeschrieben!

Zur Kompletten Einrichtung einer Mandantenserverumgebung muss zunächst der AeinsCrtrl Prozess gestartet werden, dieses erfolg üblicherweise über die Autostart Funktion, in der folgender Aufruf eingetragen werden muss :

E:\\Perl\\bin\\perl.exe e:\\Aeins\\batch\\AeinsCtrl.pl -log -re:\\Aeins -w 20

In der zugehörigen AeinsCtrl.INI (aus dem Windows Verzeichnis) muss ein passender Mandantenserverstart eingetragen werden , wie z.B.:

[Default]

[Mansi]

start=+2

BatchDirectory=e:\\aeins\\user\\batch

job=mansi.bat

name=MANSI

In der Userser Directory \\aeins\\user\\batch muss dann folgende Datei mit dem Namen mansi.bat existieren :

cd /d e:\\aeins\\bin

aeins32.exe welcome MANSI Autostart=START user=MAND MS_OBERGRENZE=10000 passwort=AMIC amicconf_section=&lt;mandant>
