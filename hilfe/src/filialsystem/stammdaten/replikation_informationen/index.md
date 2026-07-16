# Replikation-Informationen

<!-- source: https://amic.de/hilfe/replikationinformationen.htm -->

Hauptmenü > Filialsystem > Stammdaten > Übersicht Replikation

oder Direktsprung [**RINFO**]

Dieser Dialog stellt Informationen zur Replikation zur Verfügung.

Die angezeigten Informationen werden alle 3 Sekunden aktualisiert.

| **Felder** | |
| --- | --- |
| Zentrale | [Replikationsadressen Zentrale](../betriebsstaetten_filialen/variante_betriebsstaetten/replikationsadressen.md#replikationsadressen_zentrale) |
| Server | Datenbank-Server **auf** Computer |
| DB-File | Datenbank-Property „File“ |
| DB-Logname | Datenbank-Property „Logname“ |
| Server-Zeit | Die Zeit des Datenbank-Servers zu Referenzzwecken. |
| Datenbank | Datenbank-Property „alias“ |
| Publisher | Sofern vorhanden der „Current Publisher“ des laufenden Mandanten.<br>Während dieses Feld betreten ist, besteht mit F6 oder der Funktion „***DBRemote-Log ansehen***“ die Möglichkeit, das DBRemote-Logfile des Publishers in einem Editor zu öffnen |
| Publisher-Address | Zeigt die Adresse des Publishers an. Ein Doppelklick öffnet, sofern es sich um das Nachrichtensystem „FILE“ handelt, den Dateiexplorer an dieser Adresse. |
| **Remote-User** | Ein Mandant kann ein oder mehrere Replikations-Partner (abgebildet über die Remote-User des Datenbanksystems) haben. Hier finden sich Information dazu. |
| Remote-User | Name des Remote-Users.<br>Doppelklick auf den Namen öffnet ein weiteres Fenster mit Informationen zu der für diesen Benutzer eingerichteten Replikation.<br>Während dieses Feld betreten ist, besteht mit F6 oder der Funktion „***DBRemote-Log ansehen***“ die Möglichkeit, das DBRemote-Logfile des Remote-Users in einem Editor zu öffnen. |
| Nachrichtensystem | Zeigt das verwendete Nachrichtensystem. (FILE, http, usw.) |
| Adresse | Die Adresse, an die SQL Remote-Nachrichten gesendet werden sollen.<br>Ein Doppelklick öffnet, sofern es sich um das Nachrichtensystem „FILE“ handelt, den Dateiexplorer an dieser Adresse. |
| Konsolidiert | Zeigt an, ob dem Benutzer "CONSOLIDATE"-Berechtigungen (Y) oder "REMOTE"-Berechtigungen (N) erteilt wurden. |
| Empfangene Nachrichten | Anzahl der empfangenen Nachrichten |
| Gesendete Nachrichten | Anzahl der gesendeten SQL Remote-Nachrichten |
| Letzter Empfang | Zeitpunkt des letzten Empfangs von Nachrichten des Remote-User |
| Letzte Sendung | Zeitpunkt der letzten Sendung von Nachrichten des Remote-User |
| **Event** | Das Datenbank-Event welches den Nachrichten-Austausch des Mandanten mit den definierten Replikations-Partnern technisch abwickelt.<br>Das Datenbank-Event ist fest vorgegeben: [**dbrexp_schedule**](../setup_filialsystem/variante_setup_filialsystem/dbrexp_event_erstellen.md) |
| eingerichtet | Dbrexp_schedule ist als Objekt in der Datenbank vorhanden |
| aktiviert | Dbrexp_schedule ist als Event aktiviert |
| Aktiv | Dbrexp_schedule ist aktiv (Ja) oder inaktiv (Nein) |
| Nächster Auslösezeitpunkt | Zeitpunkt der nächsten Auslösung des Events dbrexp_schedule. |

| **Funktionen** | |
| --- | --- |
| DBRemote-Log ansehen **F6** | Sichtbar wenn Focus im Feld Publisher oder einem der Felder von Remote-User liegt. Öffnet, sofern vorhanden die gleichnamige Logdatei des DBRemote-Agenten |

<p class="siehe-auch">Siehe auch:</p>

- [Replikationsmonitor – Subskriptionen / Publikationen](./replikationsmonitor_subskriptionen_publikationen.md)
