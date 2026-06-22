# Systeminformationen

<!-- source: https://amic.de/hilfe/_systeminformation.htm -->

Hauptmenü > Systempflege > Update > Systeminformationen

oder Direktsprung **[SYSIN]**

oder das Fragezeichen (?) in der oberen Leiste anklicken und dann Systeminformationen auswählen

Hier findet man alle notwendigen Informationen zum System z.B. wer mit der Datenbank verbunden ist, welche Lizenz und welche Versionen verwendet werden, die Größe der Datenbank und vieles weiteres.

<p class="just-emphasize">Kopfdaten</p>

| Feldname | Beschreibung |
| --- | --- |
| Kunden-Bezeichnung | Name des Kunden / Mandanten.  
Entspricht dem Feld Name aus dem [Mandantenstamm](../../firmenstamm/firmenkonstanten/mandantenstamm.md)  
(Direktsprung **[MND]**)  
 |
| Bediener | Hier werden das Kürzel und der Name des aktuellen Bedieners angezeigt.  
Entspricht den Feldern Kurzname und Bedienername aus dem [Bedienerstamm](../../firmenstamm/firmenkonstanten/bedienerwesen_bediener_bedienerklassen_und_erfasser/bedienerstamm/index.md) (Direktsprung **[BD]**)  
 |
| Mandant | Hier sieht man welcher Mandant ausgewählt wurde  
Entspricht dem Feld Kurztext aus dem [Mandantenstamm](../../firmenstamm/firmenkonstanten/mandantenstamm.md)  
(Direktsprung **[MND]**)  
 |

<p class="just-emphasize">Register</p>

<details>
<summary>Allgemein</summary>

| Felder | Beschreibung |
| --- | --- |
| Datenbankserver | Hier wird angezeigt welche Datenbank auf welchem Rechner verwendet wird.  
 |
| Datenbanksoftware | Verwendete Datenbanksoftware und deren Versionsnummer z.B. SQL Anywhere  
 |
| Verbindungsparameter | Hier wird angezeigt welche Parameter für die Verbindung verwendet wurden.  
z.B.  
eng - EngineName  
dbn - DatabaseName  
links – CommLinks z.B. tcpip - Die Verbindung wird über das Kommunikationsprotokoll TCP/IP eingerichtet  
 |
| Datenbankgröße (GB) | Größe der Datenbank in Gigabyte  
 |
| Informationssystem | Verwendetes Informationssystem  
AIS – [A.eins Informationssystem](../../zusatzprogramme/ais_a_eins_informationssystem/index.md)  
 |
| Version der Daten | Version der internen Daten  
 |
| Version des Programms | A.eins Versionsnummer und Datum der Version  
 |
| Version AMIC-Etikettendruck | Aktuelle Versionsnummer AMIC-Etikettendruck . Beim Exportieren und Importieren der Reporte muss darauf geachtet werden, dass die Versionsnummer im Zielsystem mindesten so hoch ist, wie die im Quellsystem.  
 |
| Version Elster | Elster Versionsnummer  
Mehr zu [Elster](../../finanzbuchhaltung/umsatzsteuer/elster.md) |
| Version Crystal Report | Versionsnummer des verwendeten Crystal Report.  
Durch einen Klick auf den Knopf öffnet sich eine Maske mit den ODBC-Einstellungen.  
Mehr zu [Crystal Report](../../zusatzprogramme/crystal_report/index.md)  
 |
| PDF-Engine | Hier wird angezeigt welche Software zum Erzeugen von Pdf-Dateien verwendet wird.  
 |
| Dokument-Engine | Hier wird angezeigt welche Software zum Verarbeiten von Dokumenten verwendet wird.  
 |

| Felder in der Tabelle | Beschreibung |
| --- | --- |
| Sybase | Die Versions- und die Build-Nummer des Datenbankservers, der den Vorgang ausführt.  
 |
| Zeitpunkt | Der Zeitpunkt (Datum und Uhrzeit), an dem die Datenbank das erste Mal auf einem bestimmten Betriebssystem mit einer bestimmten Version der Software gestartet wurde.  
 |
| Betriebssystem | Das Betriebssystem, auf dem der Vorgang ausgeführt wurde.  
 |
| Details | Diese Spalte enthält Angaben über Befehlszeilenoptionen, mit denen der Datenbankserver gestartet wird, oder über die Fähigkeitsbits der Datenbank. Diese Angaben sind in erster Linie für den technischen Support gedacht.  
 |

</details>

 

<details>
<summary>Lizenz</summary>

Hier werden alle Lizenzen angezeigt.

| Felder in der Tabelle | Beschreibung |
| --- | --- |
| Spa | Steuerparameter der Lizenz  
 |
| Name | Name des Lizenzsteuerparameters  
 |
| Wert | Zeigt an, ob die Lizenz aktiv ist  
 |
| Gültig bis | Bei einem eingetragenen Datum ist die Lizenz bis zum angegebenen Datum gültig. Ist das Feld leer gilt die Lizenz unbefristet.  
 |
| Anzahl | Es existierten Lizenzen, welche Anzahl basiert sind. In solchen Fällen wird diese hier dargestellt.  
 |

Die Steuerparameter werden bei jedem Start von A.eins erneut gesetzt. In den Feldern „Lizenz gesetzt von Arbeitsplatz“ steht der Windows-User und der Zeitpunkt der letzten Änderung der Lizenzeinstellungen. Der Abgleich mit der Lizenzdatei erfolgt bei jedem Anmeldeversuch.

</details>

<details>
<summary>Konsistenz</summary>

Konsistenzprüfung der Datenbank

| Felder in der Tabelle | Beschreibung |
| --- | --- |
| Tabelle | Tabelle die geprüft wurde  
 |
| Letzte Prüfung | Zeitpunkt der letzten Prüfung für die entsprechende Tabelle  
 |
| Status | Status der letzten Prüfung  
 |

<p class="just-emphasize">Option</p>

Hier kann man die Datenbankoptionen und ihren Status einsehen.  
Aufgrund der Anzahl der Optionen wird hier nicht jede einzelne erläutert.

| Felder in der Tabelle | Beschreibung |
| --- | --- |
| Option | Hier kann man den Namen der Datenbankoption sehen  
 |
| Status | Hier wird der Status der Option angezeigt  
 |

<p class="just-emphasize">DB-Eigenschaften</p>

Hier kann man die Datenbank-Eigenschaften und ihren Status einsehen.

| Felder in der Tabelle | Beschreibung |
| --- | --- |
| Eigenschaft | Name der Datenbank-Eigenschaft  
 |
| Status | Aktueller Status der Eigenschaft  
 |
| Erläuterung | Kurze Erläuterung / Übersetzung der Eigenschaft  
 |

</details>

 

<p class="just-emphasize">Funktionen</p>

<details>
<summary>Funktionen der Systeminformationen</summary>

| Bezeichnung | Beschreibung |
| --- | --- |
| Konsistenzprüfung starten | Wird nur angezeigt, wenn der Tab Reiter Konsistenz aktiv ist.  
Konsistenzprüfung der Datenbank wird gestartet und in der Tabelle angezeigt. Die Prüfung kann einige Zeit in Anspruch nehmen.  
 |
| Benutzerinfo **(F9)** | Öffnet die Maske Benutzerinformation.  
 |
| Serverinfo **(F10)** | Öffnet die Maske Serverinformation.  
 |
| DB-MessageFile **(F6)** | Öffnet die Datei server.txt.  
Wo sich diese Datei befindet, ist in den optionalen Parametern **[OPT]** unter DBMESSAGEFILE festgelegt.  
 |
| Parameter | Hier steht die Verbindungsnummer.  
 |
| Sperren in der Datenbank **(F11)** | Öffnet die Auswahlliste Parameter.  
 |
| Aktualisieren **(F5)** | Aktualisiert die Daten auf dem aktuellen Tab Reiter.  
 |

</details>

 

<p class="siehe-auch">Siehe auch:</p>

- [Benutzerinformation](./benutzerinformation.md)
- [Serverinfo](./serverinfo.md)
- [Parameter](./parameter.md)
- [Sperren in der Datenbank](./sperren_in_der_datenbank.md)
