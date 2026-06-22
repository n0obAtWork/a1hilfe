# Nachhaltigkeitseinstellungen (SPA 844)

<!-- source: https://amic.de/hilfe/_SPA_844.htm -->

In diesem Steuerparameter können Optionen für die Nachhaltigkeit eingestellt werden.

Zur Einstellung stehen verschiedene Typen zur Verfügung.

| Typ | Wert |
| --- | --- |
| MANDSERREFRESH | Das Aktualisieren der Nachhaltigkeitswerte erfolgt durch den Mandantenserver. Standardmäßig erfolgt eine Abarbeitung, auch wenn kein Wert eingetragen ist. Nur wenn der Wert „0“ (deaktiviert) eingetragen wird, erfolgt keine Abarbeitung durch den Mandantenserver. |
| STATUSMANUELL | Hiermit kann eingestellt werden das beim reinitialisieren der Werte durch den Mandantenserver oder über das JPP-Objekt, der Status auch gesetzt werden soll, wenn er manuell gesetzt wurde.  
Damit der manuelle Wert ersetzt wird, muss der Steuerparameterwert auf „0“ (deaktiviert) gesetzt werden. |
| THGMANUELL | Hiermit kann eingestellt werden, dass beim Reinitialisieren der Werte durch den Mandantenserver oder über das JPP-Objekt, die THG-Werte auch gesetzt werden soll, wenn sie manuell gesetzt wurden.  
Damit die manuellen Werte ersetzt werden, muss der Steuerparameterwert auf „0“ (deaktiviert) gesetzt werden. |
| ANBAULANDMANUELL | Hiermit kann eingestellt werden, dass beim Reinitialisieren der Werte durch den Mandantenserver oder über das JPP-Objekt, das Anbauland auch gesetzt werden soll, wenn es manuell gesetzt wurde.  
Damit der manuelle Wert ersetzt wird, muss der Steuerparameter auf „0“ (deaktiviert) gesetzt werden. |
| REFRESHZERTIFIKAT | Hiermit kann eingestellt werden, dass beim Reinitialisieren der Werte durch den Mandantenserver oder über das JPP-Objekt, das Zertifikat neu ermittelt wird. Standardmäßig werden die Zertifikatwerte nur aktualisiert.  
Damit das Zertifikat neu ermittelt wird, muss der Steuerparameterwert auf „0“ (deaktiviert) gesetzt werden. |
| MANDSERREFRESHSECONDS | Hier kann eingetragen werden, in welchem Intervall der Mandantenserver die „Aktualisierungsfunktion“ der Nachhaltigkeit aufruft. Standardmäßig wird die Funktion alle 15 Sekunden aufgerufen. |
| MASSEBEZOGENEWERTE | Hier kann mit dem Wert „1“ eingestellt werden, dass statt der THG-Werte die massebezogenen Werte für Zwischenprodukte angegeben werden.  
Dadurch erfolgt auf der Massebilanz nicht die Berechnung der massebezogenen Werte, sondern die angegebenen Werte werden direkt verwendet. |
| SCHWELLENWERTTHGMASSE | Hier kann ein Schwellenwert angegeben werden, mit dem auf der Bewegungsübersicht die Positionen markiert werden, die unter beziehungsweise über diesem Wert liegen. |
| REFRESHJAHRE | Hier kann die Anzahl der Jahre eingetragen werden, für die die Aktualisierung durchgeführt werden soll. Default mäßig wird das aktuelle Jahr und die letzten zwei Jahre davor aktualisiert.  
Wer alle Jahre davor haben will, muss hier z.B. den Wert 100 eintragen.  
Beachtet werden sollte, das als Jahr das Kalenderjahr zählt. Wird also der Wert „0“ eingetragen, werden nur die Belege vom 01.01. des aktuellen Jahres bis heute beachtet. Deswegen sollte man immer mindestens den Wert „1“ eintragen. |
