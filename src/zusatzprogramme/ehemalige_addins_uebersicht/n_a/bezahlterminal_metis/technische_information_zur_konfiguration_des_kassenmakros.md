# Technische Information zur Konfiguration des Kassenmakros

<!-- source: https://amic.de/hilfe/technischeinformationzurkonfig1.htm -->

Das Kassenmakro ist für jede Kasse einzeln zu definieren. Als Beispiel dient AMIC_BZT_MUSTER.

Dort finden Sie auch die Beispiele für die Abmeldung eines Terminals und die Initiierung des Kassenschnitts.

Erfolgt die Kommunikation mit dem Terminal per LAN, so wird hier die IP-Adresse angegeben. Diese darf sich beim Terminal nicht per DHCP ändern.

Bitte kontaktieren Sie hierzu den Administrator des Netzwerkes bzw. weisen Sie in Absprache mit dem Netzwerkadministrator eine feste IP-Adresse zu. Die Hilfe dazu bietet die Bedienungsanleitung des jeweiligen Terminals.

Im Fall von LAN wird ebenso ein Port angegeben. Dieser ist von Hersteller zu Hersteller unterschiedlich. Im Protokoll ist der Port 20007 vorgesehen, der Hersteller Thales z.B. verwendet nach eigenen Angaben jedoch 22000, Ingenico den Port 5577.

Hinweis:

Einige Terminals weisen der Erfahrung nach zuweilen Kommunikationsprobleme zu A.eins auf. Dadurch kann es vorkommen, dass A.eins auf Rückmeldungen vom Terminal erwartet, jedoch nicht bekommt. In diesem Fall würde A.eins dauerhaft warten. Um diesen Wartemodus abbrechen zu können, muss der Parameter „SHOWABORT“ auf TRUE gesetzt werden.

Der Parameter Terminalname wurde ersetzt durch den Parameter Authentifizierung:

Parameter AUTHENTIFIZIERUNG: TRUE / FALSE Standard: TRUE.

Legt fest, ob das Terminal LogOn beim Start benötigt. THALES_0001 entspricht TRUE / INGENICO_0001 entspricht FALSE

Weitere Einstellungen finden Sie unter [Technisches Umfeld](./index.md)
