# Systeminformationen

<!-- source: https://amic.de/hilfe/_systeminformation.htm -->

Hauptmenü > Systempflege > Update > Systeminformationen

oder Direktsprung **[SYSIN]**

oder das Fragezeichen (?) in der oberen Leiste anklicken und dann Systeminformationen auswählen

Hier findet man alle notwendigen Informationen zum System z.B. wer mit der Datenbank verbunden ist, welche Lizenz und welche Versionen verwendet werden, die Größe der Datenbank und vieles weiteres.

### Kopfdaten

| Feldname | Beschreibung |
| --- | --- |
| Kunden-Bezeichnung | Name des Kunden / Mandanten.<br>Entspricht dem Feld Name aus dem [Mandantenstamm](../../firmenstamm/firmenkonstanten/mandantenstamm.md)<br>(Direktsprung **[MND]**)<br> |
| Bediener | Hier werden das Kürzel und der Name des aktuellen Bedieners angezeigt.<br>Entspricht den Feldern Kurzname und Bedienername aus dem [Bedienerstamm](../../firmenstamm/firmenkonstanten/bedienerwesen_bediener_bedienerklassen_und_erfasser/bedienerstamm/index.md) (Direktsprung **[BD]**)<br> |
| Mandant | Hier sieht man welcher Mandant ausgewählt wurde<br>Entspricht dem Feld Kurztext aus dem [Mandantenstamm](../../firmenstamm/firmenkonstanten/mandantenstamm.md)<br>(Direktsprung **[MND]**)<br> |

### Register

<details>
<summary>Allgemein</summary>

| Felder | Beschreibung |
| --- | --- |
| Datenbankserver | Hier wird angezeigt welche Datenbank auf welchem Rechner verwendet wird.<br> |
| Datenbanksoftware | Verwendete Datenbanksoftware und deren Versionsnummer z.B. SQL Anywhere<br> |
| Verbindungsparameter | Hier wird angezeigt welche Parameter für die Verbindung verwendet wurden.<br>z.B.<br>eng - EngineName<br>dbn - DatabaseName<br>links – CommLinks z.B. tcpip - Die Verbindung wird über das Kommunikationsprotokoll TCP/IP eingerichtet<br> |
| Datenbankgröße (GB) | Größe der Datenbank in Gigabyte<br> |
| Informationssystem | Verwendetes Informationssystem<br>AIS – [A.eins Informationssystem](../../zusatzprogramme/ais_a_eins_informationssystem/index.md)<br> |
| Version der Daten | Version der internen Daten<br> |
| Version des Programms | A.eins Versionsnummer und Datum der Version<br> |
| Version AMIC-Etikettendruck | Aktuelle Versionsnummer AMIC-Etikettendruck . Beim Exportieren und Importieren der Reporte muss darauf geachtet werden, dass die Versionsnummer im Zielsystem mindesten so hoch ist, wie die im Quellsystem.<br> |
| Version Elster | Elster Versionsnummer<br>Mehr zu [Elster](../../finanzbuchhaltung/umsatzsteuer/elster.md) |
| Version Crystal Report | Versionsnummer des verwendeten Crystal Report.<br>Durch einen Klick auf den Knopf öffnet sich eine Maske mit den ODBC-Einstellungen.<br>Mehr zu [Crystal Report](../../zusatzprogramme/crystal_report/index.md)<br> |
| PDF-Engine | Hier wird angezeigt welche Software zum Erzeugen von Pdf-Dateien verwendet wird.<br> |
| Dokument-Engine | Hier wird angezeigt welche Software zum Verarbeiten von Dokumenten verwendet wird.<br> |

| Felder in der Tabelle | Beschreibung |
| --- | --- |
| Sybase | Die Versions- und die Build-Nummer des Datenbankservers, der den Vorgang ausführt.<br> |
| Zeitpunkt | Der Zeitpunkt (Datum und Uhrzeit), an dem die Datenbank das erste Mal auf einem bestimmten Betriebssystem mit einer bestimmten Version der Software gestartet wurde.<br> |
| Betriebssystem | Das Betriebssystem, auf dem der Vorgang ausgeführt wurde.<br> |
| Details | Diese Spalte enthält Angaben über Befehlszeilenoptionen, mit denen der Datenbankserver gestartet wird, oder über die Fähigkeitsbits der Datenbank. Diese Angaben sind in erster Linie für den technischen Support gedacht.<br> |

</details>

 

<details>
<summary>Lizenz</summary>

Hier werden alle Lizenzen angezeigt.

| Felder in der Tabelle | Beschreibung |
| --- | --- |
| Spa | Steuerparameter der Lizenz<br> |
| Name | Name des Lizenzsteuerparameters<br> |
| Wert | Zeigt an, ob die Lizenz aktiv ist<br> |
| Gültig bis | Bei einem eingetragenen Datum ist die Lizenz bis zum angegebenen Datum gültig. Ist das Feld leer gilt die Lizenz unbefristet.<br> |
| Anzahl | Es existierten Lizenzen, welche Anzahl basiert sind. In solchen Fällen wird diese hier dargestellt.<br> |

Die Steuerparameter werden bei jedem Start von A.eins erneut gesetzt. In den Feldern „Lizenz gesetzt von Arbeitsplatz“ steht der Windows-User und der Zeitpunkt der letzten Änderung der Lizenzeinstellungen. Der Abgleich mit der Lizenzdatei erfolgt bei jedem Anmeldeversuch.

</details>

<details>
<summary>Konsistenz</summary>

Konsistenzprüfung der Datenbank

| Felder in der Tabelle | Beschreibung |
| --- | --- |
| Tabelle | Tabelle die geprüft wurde<br> |
| Letzte Prüfung | Zeitpunkt der letzten Prüfung für die entsprechende Tabelle<br> |
| Status | Status der letzten Prüfung<br> |

### Option

Hier kann man die Datenbankoptionen und ihren Status einsehen.  
Aufgrund der Anzahl der Optionen wird hier nicht jede einzelne erläutert.

| Felder in der Tabelle | Beschreibung |
| --- | --- |
| Option | Hier kann man den Namen der Datenbankoption sehen<br> |
| Status | Hier wird der Status der Option angezeigt<br> |

### DB-Eigenschaften

Hier kann man die Datenbank-Eigenschaften und ihren Status einsehen.

| Felder in der Tabelle | Beschreibung |
| --- | --- |
| Eigenschaft | Name der Datenbank-Eigenschaft<br> |
| Status | Aktueller Status der Eigenschaft<br> |
| Erläuterung | Kurze Erläuterung / Übersetzung der Eigenschaft<br> |

</details>

 

### Funktionen

<details>
<summary>Funktionen der Systeminformationen</summary>

| Bezeichnung | Beschreibung |
| --- | --- |
| Konsistenzprüfung starten | Wird nur angezeigt, wenn der Tab Reiter Konsistenz aktiv ist.<br>Konsistenzprüfung der Datenbank wird gestartet und in der Tabelle angezeigt. Die Prüfung kann einige Zeit in Anspruch nehmen.<br> |
| Benutzerinfo **(F9)** | Öffnet die Maske Benutzerinformation.<br> |
| Serverinfo **(F10)** | Öffnet die Maske Serverinformation.<br> |
| DB-MessageFile **(F6)** | Öffnet die Datei server.txt.<br>Wo sich diese Datei befindet, ist in den optionalen Parametern **[OPT]** unter DBMESSAGEFILE festgelegt.<br> |
| Parameter | Hier steht die Verbindungsnummer.<br> |
| Sperren in der Datenbank **(F11)** | Öffnet die Auswahlliste Parameter.<br> |
| Aktualisieren **(F5)** | Aktualisiert die Daten auf dem aktuellen Tab Reiter.<br> |

</details>

 

<p class="siehe-auch">Siehe auch:</p>

- [Benutzerinformation](./benutzerinformation.md)
- [Serverinfo](./serverinfo.md)
- [Parameter](./parameter.md)
- [Sperren in der Datenbank](./sperren_in_der_datenbank.md)
