# Server

<!-- source: https://amic.de/hilfe/server.htm -->

In der vierten Varianten werden die Einrichtungen für Server vorgenommen, dies gilt für den Zentral Mandant sowie für den Unter Mandant. Die Anzeige in der Auswahlliste ist nur für den Zentral Mandant wichtig. Jeder Server der Verbunden ist wird grün Angezeigt. Server die keine Verbindung haben werden Rot angezeigt.

Funktionen für den Zentralmandanten

Dateivorbereitung Zentral: Mit der Funktion wird der Untermandant angelegt der Server angelegt und die Proxy Tabelle eingerichtet.

Event Anlagen: Mit der Funktion wird der Event angelegt, der die Daten aus der zwischen Relation in die jeweiligen Untermandanten verteilt. Das Ereignis ruft die Prozedur mms_transfer_speicher_evt() auf. Der Name des Ereignisses ist hole_mms_daten_aus_dem_speicher.

Private Trigger löschen: Mit der Funktion können alle Trigger gelöscht werden, die auf den Artikel Tabellen wirken.

Trigger Anlegen: Mit der Funktion werden alle Trigger angelegt, die auf den Artikel Tabellen wirken

Funktionen für den Untermandanten

Dateivorbereitung Zentral: Bereitet den Untermandanten vor diese legt den Trigger an der auf der Relation MMS_Transfer wirkt.

Event Anlagen: Hier wird der Ereignis angelegt, der das Abändern von einzelnen Relationen vornimmt. Das bedeutet fehlt eine Spalte oder mehrere bei einer Tabelle in der Zieldatenbank, so ändert das Ereignis die Tabelle ab und spielt danach die Daten neu ein.

| Masken Felder | Bedeutung |
| --- | --- |
| Zentralmadant | Hier kann eingestellt werden, ob der Server der Zentral oder Untermandant ist |
| Remotelogin | Hier wird das Remote Login eingegeben |
| RemotePWD | Hier wird das Remote Passwort eingegeben |
| Klasse | Hier wird die ODBC Klasse eingegeben entweder SAODBC oder ASAODBC. SAODBC für einen Connection String oder die ASAODBC für eine ODBC Verbindung, die auf dem Server läuft. Es ist zu empfehlen eine SAODBC Verbindung zu benutzen. |
| individuellername | Hier bitte mmsxml eintragen |
| Proxy Server | Hier wird der Name des Untermandanten eingetragen |
| Proxy Tabelle | Hier wird der Alias Name für die Proxy Tabelle auf dem Zentral Server eingetragen.<br>Achtung jede Proxy Tabelle hat ein eigene Namen.<br>z.B.:<br>mms_transfer_1 wird abgebildet auf Server 1<br>mms_transfer_2 wird abgebildet auf Server 2<br>usw. |
| Proxy Tabelle Alias | Hier wird der Alias Name für den Proxy Server eingegeben, dies ist ganz wichtig, wenn nicht alle Artikel an alle Untermandanten übertragen werden sollen. Dieser Name wird für die privaten Views benötigt.<br>z.B. Server 1 hat als Alias UM1 |
| Verbindungseingabe | Hier wird entweder die ODBC Verbindung eingetragen oder die Verbindungsconnection. Beispiel eines Connection Strings<br>Driver=sql anywhere 10;eng=ServerName;links=tcpip;dbn=DatenbankName; |
| Ziel Tabelle | |
