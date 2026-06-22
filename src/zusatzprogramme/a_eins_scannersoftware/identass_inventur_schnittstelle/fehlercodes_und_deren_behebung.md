# Fehlercodes und deren Behebung

<!-- source: https://amic.de/hilfe/_identassfehlercodes.htm -->

Hier finden sich die Fehlercodes und eventuelle Lösungsmöglichkeiten.

| Fehlercode | Fehlertext | Behebung |
| --- | --- | --- |
| ohne | Fehler beim Öffnen der Verbindung! (An error message cannot be displayed because an optional resource assembly containing it cannot be found) | Dieser Fehler bezieht sich auf die IP-Verbindung WLAN/Netzwerk) zwischen dem MDE (Scanner) und Multilink.  
Unter Umständen ist der Aufbau der TCP/IP-Verbindung fehlgeschlagen, weil gerade kein WLAN zur Verfügung stand.  
Bei dauerhaftem auftreten sollte die Netzwerkverbindung zwischen MDE (Scanner) und Multilink-Rechner geprüft werden (WLAN-Einrichtung/Verschlüsselung/ggf. Router/Windows-Firewall auf Multilinkrechner).  
Das Terminal sollte im eingeschalteten Zustand auf einen Ping reagieren, Multilink auf eine Telnet-verbindung auf Port 8591. Darüber lässt sich die Netzwerkverbindung mit Hilfe eines weiteren Rechners im Netzwerk grob testen. |
| \-50000 | Beim Ausführen der Prozedur IdentassscannerInventur ist ein Fehler aufgetreten | Der Fehler tritt bei nicht spezifizierten Fehlern auf. |
| \-50001 | Der Scanner mit der Identifikation %s wurde im Bedienerstamm nicht gefunden. Ordnen Sie dem Scanner bitte einem Bediener zu. | Die ScannerId oder Scannerip muss im Bedienerstamm einem Bediener zugeordnet werden. Die ScannerId wird im Feld Name extern gespeichert. Ist kein Bediener zugeordnet wird der Bediener -99 und die Lagernummer 0 gewählt. |
| \-50002 | In den Steuerparameter mit der Nummer 802 wurde keine Inventurgruppe für den Scanner %s hinterlegt. Es wird die Inventurgruppe 0 gewählt. | In den Steuerparameter kann die Inventurgruppe für den Scanner hinterlegt werden. Ist keine Inventurgruppe hinterlegt worden so wird die Inventurgruppe 0 gewählt. |
| \-50003 | Es wurde kein offener Inventurstichtag zu der Inventurgruppe %s gefunden. Eröffnen Sie bitte eine Inventur. | Alle erfassten Daten werden in der Tabelle MDE Übergabe gespeichert. Egal welche Steuerparameter Einstellung gewählt wurde. |
| \-50004 | Es kann keine Inventurbelegnummer zum Inventur Stichtag %s und der Inventurgruppe %s erzeugt werden. | Die Inventurgruppe muss dem Inventurstichtag zugewiesen werden.  
Die erfassten Daten werden in die Tabelle MDEUebergabe gespeichert. |
| \-50005 | Zu der Artikel EAN %s wurde kein Artikel im Lager %s gefunden. | Der Artikel wurde nicht im Lager des Benutzers gefunden.  
Der gescannte Artikel wird in der Tabelle MDEUebergabe gespeichert. Auch dann, wenn nur die Tabelle Inventurbelege beschreiben werden soll. Aus der Tabelle MDEUebergabe kann der Artikel dann in die Inventur geschrieben werden. |
| \-50006 | Zu der Artikel EAN %s wurden mehrere gültige Artikel gefunden. | Zu dieser Artikel EAN wurden mehrere Artikel gefunden. Ein EAN Code sollte auf einen Artikel zeigen.  
Der gescannte Artikel wird in der Tabelle MDEUebergabe gespeichert. Auch dann, wenn nur die Tabelle Inventurbelege beschreiben werden soll. Aus der Tabelle MDEUebergabe kann der Artikel dann in die Inventur geschrieben werden. |
| \-50007 | Der Artikel %s hat die Inventurgruppe %s und gehört nicht in die Inventurgruppe des Scanners %s. | Der gescannte Artikel wird in der Tabelle MDEUebergabe gespeichert. Auch dann, wenn nur die Tabelle Inventurbelege beschreiben werden soll. Aus der Tabelle MDEUebergabe kann der Artikel dann in die Inventur geschrieben werden. |
| \-50008 | Zu der Partiebezeichnung %s und dem Artikel %s wurde keine gültige Partie gefunden. | Zu der erfasste Partiebezeichnung und dem Artikel konnte keine Partiezugeordnet werden.  
Der gescannte Artikel wird in der Tabelle MDEUebergabe gespeichert. Auch dann, wenn nur die Tabelle Inventurbelege beschreiben werden soll. Aus der Tabelle MDEUebergabe kann der Artikel dann in die Inventur geschrieben werden. |
| \-50009 | Zu der EAN wurde kein gültiger Artikel gefunden. | Die gescannte EAN verweist auf keine gültigen Artikel im System. |
