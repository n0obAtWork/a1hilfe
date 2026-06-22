# Der Datenbankdienst

<!-- source: https://amic.de/hilfe/derdatenbankdienst.htm -->

Bei neuen Systemen muss der Datenbankdienst eingerichtet werden, hierbei ist im Parameterbereich der Control Managers folgendes zu beachten:

    
\-n aeins -c80M -ti 240 -tl 3600

| Parameter | Bedeutung |
| --- | --- |
| \-n | gibt den Maschinennamen an (eng=....)  
 |
| \-c | gibt die Hauptspeicherbereich an, der genutzt werden darf |
| \-ti  
 | Ist die Zeit in Minuten bis ein User aus dem System geloggt wird |
| \-tl | gibt die Lebenszeit an, die verstreichen muss, bis ein nicht mehr existenter User aus dem System geworfen wird. |
