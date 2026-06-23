# Technische Information zur Konfiguration des Thales-Terminals

<!-- source: https://amic.de/hilfe/technischeinformationzurkonfig.htm -->

Es gibt im Terminal zweierlei Konfigurationen, die separat einzustellen sind:

1. [Die Einstellung, wie das Terminal mit dem Netzanbieter kommunizieren soll.](./technische_information_zur_konfiguration_des_thales_terminal.md)

Diese Einstellung ist i.d.R. zwischen ISDN und LAN einzustellen.

2. [Die Einstellung, wie das Terminal mit der Kasse kommunizieren soll.](./technische_information_zur_konfiguration_des_thales_terminal.md)

Diese Einstellung ist möglich zwischen COM-Port(RS232), USB und LAN. A.eins unterstützt jedoch nur COM und LAN.

Bitte beachten Sie, dass in der Regel die Kommunikation zwischen dem Terminal und A.eins konfiguriert werden muss. Die Einstellung entnehmen Sie bitte dem Handbuch des Terminals. Auf der Webseite [http://www.easycash.de/anleitungen.html](http://www.easycash.de/anleitungen.html) finden Sie Bedienungsanleitungen verschiedener gängiger Terminals.

Hier die Hinweise zu dem von uns getesteten Terminal Thales Artema Hybrid und sind ohne Gewähr:

- Die Voreinstellung des Konfigurationspassworts des getesteten Terminals Thales Artemis Hybrid ist 111111.

###### Einstellung der Verbindung zum Netzanbieter

Verwaltung > Service > Service-Funktionen > DFÜ-AUSWAHL > LAN (für LAN)

###### Einstellung der Kommunikation mit der A.eins-Schnittstelle

- Verwaltung > Service > Service-Funktionen > Kassenprotokoll > ZVT > aktiv
- Verwaltung > Service > Service-Funktionen > Kassenprotokoll > Schnittstelle > LAN *(Für LAN-Betrieb)*
- Verwaltung->Service > Service-Funktionen > DFÜ-Parameter > DHCP > JA *(für DHCP)*
- Verwaltung > Service > Service-Funktionen > DFÜ-Parameter > IP-Adresse *(zum Ansehen bzw. setzen der IP-Adresse wenn kein DHCP)*
