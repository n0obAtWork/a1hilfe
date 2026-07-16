# Menü-Anmeldungen

<!-- source: https://amic.de/hilfe/menanmeldungen.htm -->

Administration > Menü > Menü-Anmeldungen

oder Direktsprung [**MENUA**]

In dieser technischen Variante werden die Menü-Anmeldungen an das Aeins-System angezeigt.

Jede Aeins-Laufzeit-Instanz bekommt eine weltweit eindeutige GUID zugewiesen, anhand derer die Systeme die Aeins-Laufzeit-Instanz eindeutig bestimmen können.

Eine Aeins-Laufzeit-Instanz besitzt diverse Umgebungsparameter:

| Menü-Anmeldungen | |
| --- | --- |
| Bedienerid | Die Id des Bedieners der Aeins-Instanz. |
| Kurzname | Der Kurzname des Bedieners zur einfacheren Identifikation. |
| Anmeldung | Der Datenbank-Server-Zeitpunkt, an dem das Aeins-System den Anwender an das Hauptmenü weitergeleitet hat. |
| Status | ***Eigen*** : Ihre eigene Instanz!<br> <br>Folgende Ausprägungen werden jeweils beim Aufbau der Variante ermittelt:<br>***Abgemeldet***: Die Instanz ist nicht mehr mit dem Datenbank-Server verbunden.<br>***Aktiv***: Die Instanz ist aktiv mit dem Datenbank-Server verbunden. |
| Connection | Die technische Verbindungsidentifikation der Aeins-Instanz mit dem Datenbank-Server |
| Host | Der Hostname des Rechners auf dem die Aeins-Instanz läuft. |
| Prozess-ID | Die Windows-PID des Prozesses. |
| **Menü** | [***ActiveX***](./menue_favoriten/menue_favoriten_sortierung.md)***:*** Technische Basis des verwendeten Haupt-Menüs ist ActiveX<br>(A.eins-Standard-Hauptmenü)<br>[***A1Net***](./menue_anmeldungen.md#A1NetMenü)***:*** Technische Basis des verwendeten Haupt-Menüs ist ActiveX/.NET<br>(Aktivierbar über A.eins-Parameter: menu=false)<br>***Externes Menü:*** In der Entwicklung befindendes Haupt-Menü. |
| Menü-Version | Technische Versions-Nummer des verwendeten Menüs. |
| Programm-Version | Programm-Version der laufenden Aeins-Instanz. |
| Instanz | Identifikation der Aeins-Instanz. |
| Windows-User | Zur leichteren Identifikation, wenn z.B. mehrere Verbindungen mit dem gleichen Kurznamen da sind. |

Funktionen stehen in dieser Version keine zur Verfügung.
