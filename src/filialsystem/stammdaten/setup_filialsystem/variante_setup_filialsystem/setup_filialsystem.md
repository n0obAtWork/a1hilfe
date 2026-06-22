# Setup Filialsystem

<!-- source: https://amic.de/hilfe/maske_SetupFilialsystem.htm -->

| **Felder** | | |
| --- | --- | --- |
| Betrieb | Nummer der Betriebsstätte, danach dessen Bezeichnung. |
| | *Einrichtung Filialsystem* | *Aktiv unter SQL Remote* |
| Publikationen | Eingerichtete Publikationen die für die unter der im Feld Betrieb angegebenen Betriebsstätte. | Publikationen, die unter SQL Remote aktiv / eingetragen sind.<br>( Diese Informationen findet man in **scview** unter Publikationen ). |
| Remote User | Nummer und Bezeichnung der angeschlossenen Filialen, wie sie im Filialsystem von A.eins eingerichtet sind. | Zeigt die unter **scview** angegebenen SQL Remotebenutzer. |
| Subscriptions | Nummer und Bezeichnung der im Filialsystem von A.eins eingerichteten Subskriptionen. | SQL-Remote Benutzername und Subskription unter **scview** |
| Auswahl Betrieb | Zeigt die Liste der angeschlossenen Betriebsstätten nach Nummer für die im Feld Betrieb angegebene Betriebsstätte.<br>Sie wird für einige Funktionen aus der Funktionsbox benötigt. |

| **Funktionen** | |
| --- | --- |
| Neuaufbau Replikation | Eine bestehende Replikation zu einem Kommunikationspartner wird gestoppt und neu aufgebaut. Alle nicht übertragenen oder verarbeiteten Nachrichten gehen verloren. Notoperation, um eine Synchronisation zwischen 2 Betriebsstätten unter Inkaufnahme von Datenverlust zu erzwingen.<br>**Der Kommunikationspartner muss die gleiche Funktion ausführen.** |
| Komplett Setup | Inkrementelles Setup auf alle Remote User und Publikationen. Alle Publikationen, Publisher, Remote User, Remote Subscriptions werden wie in der Filialeinrichtung vereinbart dem SQL Remote System hinzugefügt, im SQL Remote System geändert oder aus SQL Remote System entfernt. |
| Komplett deinstallieren | Entfernt alle SQL Remote Objekte aus der Datenbank. |
| Setup Publikationen | Inkrementelles Setup auf alle Publikationen. Alle Publikationen werden wie in der Filialeinrichtung vereinbart dem SQL Remote System hinzugefügt, im SQL Remote System geändert oder aus SQL Remote System entfernt. **Der Publisher wird angelegt, falls er nicht existiert.** Anwendung: Veränderungen an Publikationen werden in das SQL Remote System übertragen (z.B. neue Tabelle replizieren). |
| Setup Remote User | Der ausgewählte Remote User wird wie in der Filialeinrichtung vereinbart dem SQL Remote System hinzugefügt, im SQL Remote System geändert oder aus SQL Remote System entfernt. Weiterhin werden die Subskriptionen vorgenommen und gestartet. |
| Remote User de-installieren | Beendet für die ausgewählte Betriebsstätte die Möglichkeit, SQL Remote-Nachrichten von dieser Datenbank zu empfangen. |
| Publikation entfernen | Entfernt den Publisher und alle Publikationen aus dem SQL Remote System. |
