# Serial-Device-Server

<!-- source: https://amic.de/hilfe/_hardw_kass_serdevserver.htm -->

Ein Serial-Device-Server ist eine relativ kleine Hardware, die an ein Netzwerkkabel angeschlossen wird und einen oder mehrere serielle Anschlüsse (RS232 / V.24) zur Verfügung stellt. Diese Geräte können mittels eines Treibers als virtueller COM-Port auf einem oder mehreren Rechnern eingerichtet werden. Die Verbindungsparameter der seriellen Schnittstelle werden direkt im Gerät mittels Software oder Treibereinstellungen konfiguriert.

Die Daten selbst werden vom Treiber oder von einer Software an eine Netzwerkadresse und einen zugehörigen Netzwerkport gesendet, so dass diese dann seriell ausgegeben werden.

In A.eins sind beide Verwendungsmöglichkeiten (virtueller COM-Port und Netzwerk-Port) möglich. Siehe [Kassensystemeinstellung Displayeinstellungen](../kassensystemverwaltung_hardware/index.md#Kassensystem_Display)

#### MOXA NPort 5110/EU V2.0

Ist bei uns für die Entwicklung der Ansteuerung per IP verwendet worden. Netzwerkport ist 950.
