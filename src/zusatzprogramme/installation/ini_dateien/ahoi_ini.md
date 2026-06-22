# Ahoi.INI

<!-- source: https://amic.de/hilfe/ahoiini.htm -->

Steuerdatei zum Aufbau der Datenverbindung und zur Datenbank.

Die im Windows Verzeichnis hinterlegte Datei kann verschiedenste Abschnitte enthalten. Jeder Abschnitt repräsentiert einen „Mandanten“ also eine separate Zugriffsschiene zu einer Datenbank.

| Abschnitte | |
| --- | --- |
| AeinsRoot | Hier sollten nur zwei Punkte oder die AeinsRoot eingetragen sein, also ..  
    
 |
| Database_Connect | Hier wird eingetragen, wo die Datenbank zu finden ist, im Mehrplatzfall als  
eng=aeins;dbf=d:\\aeins\\daten\\awed.db  
oder im Einplatzfall als  
dbf=c:\\aeins\\daten\\xxx.db |
| User | Hier kann die Vorbelegung des Usernamens eingetragen sein |
| Passwort | Hier kann die Vorbelegung des Kennwortes eingetragen sein |

Mandantenabschnitt :

| [mandant] | |
| --- | --- |
| Version | Versionsnummer des letzten Updates, hieran wird entschieden, ob es den losgehen soll mit einem Datenbankupdate. |
| Lizenznehmer | Lizenzinformationen |
| Seriennummer | Die Seriennummer |
| Lizenz | Die Lizenznummer |
