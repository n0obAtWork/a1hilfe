# Benutzerinformation

<!-- source: https://amic.de/hilfe/_systeminfobenutzerinfo.htm -->

<details>
<summary>Server</summary>

| Feldname | Beschreibung |
| --- | --- |
| Name | Name des Datenbankservers<br> |
| RequestLogFile | Hier wird der Name der Datei zur Anforderungsprotokollierung angezeigt, wenn eine existiert.<br> |
| Technische Verbindungen | Anzahl aller technischen Verbindungen die zurzeit zur Datenbank bestehen<br> |
| Unterschiedliche Benutzer | Anzahl der unterschiedlichen Benutzer (ein Benutzer kann mehrere Verbindungen zu Datenbank haben wird hier aber nur einmal gezählt)<br> |

</details>

 

<details>
<summary>Benutzerinformation</summary>

| Kopfdaten | Beschreibung |
| --- | --- |
| Name | Kürzel des aktuellen Bedieners |
| Eigene Id | Die Id des aktuellen Bedieners |

Hier erhält man Informationen zu den Benutzern in der Datenbank. Man kann die Verbindung eines Benutzers durch Doppelklick trennen oder allen Benutzern eine Meldung schicken.

| Felder |
| --- |
| Name | Kürzel des aktuellen Bedieners<br> |
| Eigene Id | Die Id über die der aktuelle Bediener zurzeit verbunden ist<br> |
| Id | Id über die der Bediener verbunden ist<br> |
| Benutzer | Kürzel des Bedieners<br> |
| DBName | Name der Datenbank<br> |
| Nodeadress | IP-Adresse des Rechners über den der Benutzer mit der Datenbank verbunden ist.<br> |
| Commlink | Verbindungsart z.B. TCPIP<br> |
| LastRegTime | Registrierte Zeit der letzten Anforderung durch den Benutzer<br> |
| Schreib | Anzahl der Schreibanforderungen<br> |
| Lese | Anzahl der Leseanforderungen<br> |
| Blocknr. | Falls die aktuelle Verbindung nicht blockiert ist, wird der Wert Null angezeigt. Sonst entspricht der Wert der Verbindungsnummer der Verbindung, die aufgrund eines Sperrenkonflikts blockiert ist.<br> |
| Connection | Art der Verbindung / Programm mit dem man sich mit der Datenbank verbunden hat<br> |

</details>
