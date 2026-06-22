# TSE-Pfleger

<!-- source: https://amic.de/hilfe/_tseauswahllistepfleger.htm -->

Eine TSE-Konfiguration zeichnet sich durch eine **TSE ID** und dem **Gültig Ab** aus. **Die TSE-ID** wird vom System vergeben. Das Feld **Gültig Ab** gibt das Datum an, ab welchem die Konfiguration gelten soll.

Mehrere TSE-Konfigurationen zur gleichen ***TSE-ID*** sind möglich.

Sie werden durch **F5** und **Speichern unter** ermöglicht.

Erläuterungen zur Behandlung des ***Laufwerks*** bei Client-Transaktionen: A.eins versucht die richtige TSE selbst zu finden. Falls das nicht gelingt, wird versucht diese automatisch zu „mappen“, wenn bei **Manueller Host** eine Freigabe eingetragen ist.

Durch den automatischen Suchalgorithmus muss auf den Clienten selbst das in „Laufwerk“ hinterlegte Laufwerk nicht zwingend den gleichen Laufwerksbuchstaben haben.

<details>
<summary>Kopfdaten des TSE Pflegers</summary>

| Feld | Beschreibung |
| --- | --- |
| TSE-ID | Gibt die TSE-ID der Konfiguration an. |
| Gültig ab | Gibt an, ab wann die TSE-Konfiguration gültig ist. |
| Aktiv-Datum | Hier wird das **Gültig ab** der zurzeit maßgeblichen TSE-Einstellung angezeigt.  
Hinweis:  
Das beantwortet die Frage „Welche Konfiguration würde zum jetzigen Zeitpunkt vom System herangezogen?  
Ist nur eine Konfiguration zur „TSE-ID“ vorhanden, stimmen **Aktiv-Datum** und **Gültig Ab** überein. |
| Status/Verfügbarkeit | Gibt an, ob die TSE aus Sicht des aktuellen Arbeitsplatzes verfügbar ist.  
Wenn **nein** **aktiviert ist**, dann gibt es eine textuelle Erläuterung.  
Wenn **ja** aktiviert ist, wird **Datum und Uhrzeit** des letzten Zugriffs gemäß TSE-Spezifikation angezeigt. |
| Hardware-Host | Name/IP des Hosts zum Zeitpunkt der Ersteinrichtung.  
(Je nach Ausstattung können mehrere IPs aufgelistet werden) |
| Dieser Rechner | Name/IP des aktuellen Arbeitsplatzrechners  
(Je nach Ausstattung können mehrere IPs aufgelistet werden). |
| Bezeichnung | Frei wählbare Bezeichnung der TSE. |
| Laufwerk | Bei der Erstinstallation zugewiesener Windows-Laufwerksbuchstabe. (A-Z sind theoretisch denkbar) |
| Lizenz | Der Label ist je nach Situation farbig:  
**Grün:** wenn der „TSE-Seriennummer“ eine „A.eins-TSE-Lizenz“ zugeordnet werden konnte.  
**Rot:** wenn der „TSE-Seriennummer“ keine „A.eins-TSE-Lizenz“ zugeordnet werden konnte.  
**Schwarz**: Keine TSE-Seriennummer verfügbar. (z. B., weil im „Laufwerk“ keine TSE liegt bzw. das „Laufwerk“ gar nicht vorhanden ist.)  
    
Dahinter finden sich Angaben zur A.eins TSE-Lizenznummer und dem A.eins TSE-Gültigkeits-Bis-Datum. |
| Manueller Host | Hier kann ein UNC-Pfad hinterlegt werden, den ein A.eins-Client zum automatischen **Net Use** heranziehen kann. |
| Client-ID | Gibt die Client-ID der TSE an. |
| Kassenzuordnungen | Gibt an, an welche Kasse diese TSE angebunden ist. |

</details>

<details>
<summary>Register Zugang USB</summary>

| Feld | Beschreibung |
| --- | --- |
| Credential-Seed | Anmeldeinformationen der TSE |
| Admin-Pin | Wird von A.eins bei der ersten Initialisierung festgelegt. |
| Admin-Puk | Wird von A.eins bei der ersten Initialisierung festgelegt. |
| Tim-Admin-Pin | Wird von A.eins bei der ersten Initialisierung festgelegt. |
| Erläuterungen zu nachfolgenden Infos | \- ***HasPassedSelftest***: Die TSE hat einen internen Selbsttest ausgeführt.  
\- ***IsErsInterfaceActive***: Gibt an, ob die TSE-Transaktionen ausführen kann, oder nicht.  
\- ***HasValidTime***: Gibt an, ob in der TSE ein Datum gesetzt ist.  
(Das Datum wird immer in *Greenwich Mean Time (UTC)* angegeben)  
\- ***isDecommisioned***: Zeigt an, ob die TSE stillgelegt wurde. |
| Infos… | Öffnet den Editor und zeigt Parameter der TSE an.  
(Diese Informationen können der Entwicklung/Support ggf. Aufschlüsse über gewisse Zustände der TSE direkt vor Ort geben). |

</details>

<details>
<summary>Status</summary>

| Feld | Beschreibung |
| --- | --- |
| Anzahl nicht behebbarer ECC-Fehler | Sollte die Anzahl nicht behebbarer ECC-Fehler über **0** sein, ist die offizielle Empfehlung des Herstellers, die TSE sofort auszutauschen! |
| Freie Ersatzblöcke | Gibt den prozentualen Anteil an freien Ersatzblöcken an. |
| Verbleibende Erase Counts | Prozentsatz der verbleibenden Löschvorgänge |
| Verbleibende Erase Counts (10 Jahre) | Prozentualer Anteil der verbleibenden Löschvorgänge, bis die zehnjährige Datenspeicherung nicht mehr gewährleistet werden kann. |
| Anzahl erfasster Transaktionen | Anzahl generierter Signaturen der TSE |

</details>

<details>
<summary>Transaktionen</summary>

| Feld | Beschreibung |
| --- | --- |
| Erfasste | Anzahl der erfassten Transaktionen. |
| Maximal-Gestartete | Maximal-Anzahl gestarteter Transaktionen. |
| gestartete | Anzahl gestarteter Transaktionen. |

Wenn gestartete Transaktionen vorhanden sind, werden diese in einer Tabelle in der Spalte **Gestartet** aufgelistet.

</details>

<details>
<summary>Speicher</summary>

| Feld | Beschreibung |
| --- | --- |
| Kapazität TSE-Speicher in Blöcken | Ebendies. |
| Momentan verwendeter TSE-Speicher in Blöcken. | Ebendies. |

</details>

<details>
<summary>Version</summary>

| Feld | Beschreibung |
| --- | --- |
| WormAPI | Versionsnummer der Swissbit-Programmschnittstelle, mit der A.eins mit dem Swissbit-System kommuniziert. |
| Hardware-Version | Hardware-Version der TSE. |
| Software-Version | Software-Version der TSE. |

</details>

<details>
<summary>Zeitverhalten</summary>

| Feld | Beschreibung |
| --- | --- |
| bis nächster Selbsttest | Zeit in Sekunden bis spätestens ein neuer Selbsttest notwendig ist.  
Die Automatisierung dieses zum ordnungsgemäßen Ablauf der TSE wird von A.eins übernommen. |
| Selbsttest ausgeführt | Im Betrieb ist alle 25 Stunden ein Selbsttest notwendig.  
Hier ist der aktuelle Status einsehbar. |
| CTTS-Interface aktiv | Gibt den aktuellen Zustand der TSE-Programmschnittstelle an. |
| Aktualisierungs-Intervall | Zeit in Sekunden in der die TSE-Programmschnittstelle eine vom normalen Transaktionsverlauf längstens erwartet.  
A.eins kümmert sich im Rahmen eines eigenständigen Hintergrundprozesses darum. |
| Valide Zeitsetzung | Zustand der aktuellen Zeitsetzung.  
Hier steht ***false,*** wenn die TSE längere Zeit nicht im Transaktionsbetrieb war. |
| max. Pause für Transaktion | Im ***Kassenbetrieb*** von A.eins von untergeordneter Bedeutung – hier nur zur Information. |
| bis nächste Zeit-Synchronisation | Zeitangabe in Sekunden bis zum nächsten Synchronisation-Timeout. |

</details>

<details>
<summary>TSE-Stammdaten</summary>

| Feld | Beschreibung |
| --- | --- |
| TSE-Seriennummer | Gibt die TSE-Seriennummer an. |
| TSE-Signatur | Gibt die TSE-Signatur an. |
| TSE-Zeitformat | Gib an, in welchem Zeitformat die TSE arbeitet. (Die Zeit wird in UTC verarbeitet) |
| TSE-Description | Erster Teil der BSI-Zertifizierungs-ID (***BSI-K-TR-NNNN***).  
Hinweis:  
Die TSE muss vom Bundesamt für Sicherheit in der Informationstechnik (BSI) zertifiziert worden sein. Das BSI vergibt für die Zertifikate den Namen: ***BSI-K-TR-NNNN-YYYY***.  
(***YYYY***) = Jahr der Zertifikatsvergabe der BSI.  
   
Sollte dieses Feld nicht gefüllt sein, ist es notwendig eine Kassensitzung mit der TSE zu eröffnen. |
| TSE-Encoding | Gibt an, mit welchem Encoding die TSE arbeitet. |
| TSE-Public Key | Gibt den Public Key der TSE an. |
| TSE-Zertifikat I | Gibt das erste Zertifikat der TSE an. |
| TSE-Zertifikat II | Gibt das zweite Zertifikat der TSE an. |

</details>

<details>
<summary>Funktionen des Auswahllisten Pflegers</summary>

| Feld | Beschreibung |
| --- | --- |
| Löschen **(F7)**, Speichern **(F8)**, Speichern unter…**(Shift+F9)** | Standardfunktionen |
| Export TAR … | Exportiert die TSE-Daten als „:tar“-Datei auf die eigene Festplatte. (Nur über Ansehen **(F6)** aufrufbar) |
| Transaktionen anzeigen … | Öffnet den Editor und zeigt alle von TSE geschrieben Daten an. (Nur über Ansehen **(F6)** aufrufbar) |
| Swissbit TSE-Selbsttest | Führt einen Selbsttest auf der TSE aus. (Nur über Ansehen **(F6)** aufrufbar) |
| Transaktionen schließen | Schließt alle gestarteten und nach der Kassensitzung nicht automatisch geschlossenen Transaktionen.  
(Nur über Ansehen **(F6)** aufrufbar, nach Abschluss der zugehörigen Kasse).  
Die so geschlossen Transaktionen werden in der Relation ***AcashTransactionsClosed*** protokolliert. |

</details>
