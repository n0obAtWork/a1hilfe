# Registerkarte Schnittstelle / ME

<!-- source: https://amic.de/hilfe/_waagenprofil_reg_schnitt.htm -->

Hier wird der technische Anschluss eines Wägesystems abgehandelt.

| Schnittstelle / ME | |
| --- | --- |
| Typ | Typ des Waagen-Anschlusses<br>[Mögliche Anschlussarten](./registerkarte_schnittstelle_me.md#Typ_moeglicheAnschlussarten) |
| Knopf „Editieren“ | Wird die Anschlussart „XML“ verwendet, so ist dieser Knopf aktiviert und ermöglicht das Editieren eines im Archiv abgelegten XML-Dokuments.<br>Im Neu-Fall sorgt dieser Knopf für die Erstellung eines XML-Templates, welches dann auch zum Editieren geöffnet wird. |
| XML kopieren von… | Wird die Anschlussart „XML“ verwendet, hat man an dieser Stelle die Möglichkeit, ein bereits in einem anderen Waagenprofil verwendetes XML-Dokument in das gerade bearbeitete Waagenprofil zu übernehmen. |
| Parameter | Siehe [Parameter](./registerkarte_schnittstelle_me.md#Waagenprofil_parameter) |
| Prozedur | Name der SQL-Prozedur zur Auswertung der Wiegeergebnisse |
| Zusatzversuche | Maximal-Anzahl der Wiederholungsversuche<br>Standard ist 0, das bedeutet mindestens eine Wiegung |
| Status | Verfügbarkeit des ausgewählten Ports.<br>Sind die Angaben unter Port und Parameter gemacht worden kann man nun mit der Funktion „Test Port“ die Einstellungen testen.<br>Es erfolgt eine technische Status-Überprüfung im Rahmen des Möglichen und das Ergebnis wird hier repräsentiert.<br>Die Prüfung bedeutet noch nicht, dass man Kontakt zur Waage hat, sondern nur, dass die technischen Möglichkeiten auch so gehalten sind, dass eine Kontaktaufnahme physikalisch sinnvoll wäre …<br>Gibt es hier einen „Status“ „NICHT OK“, dann sollte geprüft werden, ob z.B. der verlangte COM-Port am administrierenden Host auch vorhanden ist, oder im Falle des UDP-Systems auch der angegebene Host kontaktierbar ist. |
| ME-Nummer | Hier hinterlegt man die Grundmengen-Einheit der Waage |
| Host/IP | Durch die Angabe eines expliziten Hostnamens kann bewirkt werden, dass das Waagenprofil nur an dem hier angegebenen Host zur Verfügung steht |
| Hostport | Hier wird der Port des Hosts hinterlegt |
| Knopf „Öffnen“ | Sofern Host oder IP angegeben wurden, kann man hiermit eine http-Verbindung zu dieser Adresse öffnen.<br>Standardbrowser wird verwendet |
| Flusskontrolle | Einige COM-Port-Waagen benötigen eine „Flusskontrolle“. Eine mir bekannte ist „Escape“. Bei Bedarf wird das Format implementiert und freigegeben.<br>In aller Regel kann man hier die Grundeinstellung „Keine“ verwenden! |
| Protokoll | Hier kann ein ausführliches Logging für DEBUG-Zwecke aktiviert werden. |
| | |
| **Zusatz** | |
| Zusatztext | Ist eine Waage über Waagenprofil angebunden, so hat man hier die Möglichkeit, einen Zusatztext anzugeben. Dieser Zusatztext wird bei der Vorgangserzeugung zusätzlich als Position erzeugt |
| Übernahme bis | [Entscheidet, wie weit der Zusatztext fortgereicht werden soll](./registerkarte_schnittstelle_me.md#Uebernahme_bis) |
| Zusatz aktiv | Schaltet die Übernahme des Zusatztextes in den Vorgang an oder ab |
| | |
| **Wiegetaktungen** | |
| Maximal Gewicht | In diesem Feld wird das Maximalgewicht eingetragen, welches mit der Waage gemessen werden kann. |
| Minimal Gewicht | In diesem Feld wird das Minimalgewicht eingetragen, welches mit der Waage gemessen werden kann. |
| Taktung | Hier wird das Taktgewicht der Waage eingerichtet |
| | |
| Version | Eine frei wählbare Versionskennung des Waagenprofils. Diese wird explizit bei den AMIC-Mustervorlagen bei möglichen Änderungen/Verbesserungen am Profil hochgesetzt. Der Anwender kann es gut zu eben diesem Zwecke mitbenutzen |
| Autor | Der Autor des Waagenprofils |
| Anmerkung | Anmerkungen zum Waagenprofil |
| Archiv-Referenz | Archiv Belegreferenz |
| Waagenbild | Hier kann ein Bild hinterlegt werden, welches sich in der Kund.lib oder in der Aein5.lib befindet. Das Bild wird dann im Vorschau Fenster angezeigt. Dieses Bild ist dann auch in der Online-Waage zu sehen |

<p class="just-emphasize">Anschlussarten ( Typ )</p>

Mögliche Anschlussarten sind:

| Wert | Bezeichnung |
| --- | --- |
| 0 | anderer |
| 1 | COM1 |
| 2 | COM2 |
| 3 | COM3 |
| 4 | COM4 |
| 5 | COM5 |
| 6 | COM6 |
| 7 | COM7 |
| 8 | COM8 |
| 9 | COM9 |
| 10 | COM10 |
| 100 | UDP |
| 101 | JPL |
| 102 | TCP |
| 201 | ECOM1 |
| 202 | ECOM2 |
| 203 | ECOM3 |
| 204 | ECOM4 |
| 1000 | DEMO |

<p class="just-emphasize">COM-Ports</p>

COM1, COM2, COM3, COM4, …

Die wohl einschlägig bekannten COM-Ports am Host.

Aeins unterstützt auch weitere, diese lassen sich dann mit Wahl „anderer“ und Eingabe im nachfolgenden Feld erreichen:

<p class="just-emphasize">UDP</p>

Diesen Port wählen zum Anschluss eines Wägesystems, das über Netzwerk UDP-Protokoll mit dem Aeins-Host kommunizieren soll.

<p class="just-emphasize">JPL</p>

Zur Anbindung von weiteren Speziallösungen. Dieses können neben Hersteller-DLL’n auch Infrarot- oder USB-Anschlüsse sein. Allerdings bedarf es bei Nichtvorhandensein eines passenden Waagenprofils einer individuellen Sotwareanpassung durch AMIC. 

<p class="just-emphasize">TCP</p>

Anschluss einer Waage per TCPIP

<p class="just-emphasize">ECOM1, ECOM2, ECOM3, ECOM4</p>

Extendet Com-Ports

<p class="just-emphasize">DEMO</p>

Dient als Entwicklerunterstützung und bei nicht Vorhandensein eines physikalischen Gegenparts als Möglichkeit, eine Waage zu simulieren.

<p class="just-emphasize">Parameter</p>

<p class="just-emphasize">COM-Ports</p>

Die Verbindungsparameter zur COM-Schnittstelle, in aller Regel also so etwas ähnliches wie z.B. „“2400,E,7,1“ oder auch „9600,E,8,1“

: BPS

N : Parität

8 : Datenbits

1 : Stoppbits

Die Ermittlung dieser Daten ist ein wesentlicher Punkt beim Anschluss eine Wägesystems. Die Parameter müssen in aller Regel 100% stimmen, sonst kann die Kommunikation nicht mit der Waage funktionieren!

Man ermittelt diese Daten vom Hersteller der Waage, oder schaut in bestehender Fremdsoftware nach. Manchmal befinden sich diese Daten auch in Dokumentationen, oder stehen ganz einfach auf Gehäuse oder Terminal. Hin- und wieder gibt es auch Erfahrungswerte; manchmal hilft probieren…

<p class="just-emphasize">UDP</p>

Konfigurieren des UDP-Anschlusses mit IP-Adresse Hostname, UDP-Port.

Ist zum Beispiel das UDP-Wägesystem mit der IP 192.168.241.60 im Netzwerk zu erreichen, dann trägt man hier

 192.168.241.60, 187

ein. ( 187 ist der Standard-UDP-Port für das Netscale-Wägesystem )

Unterstützt das Netzwerk DNS, kann man selbstverständlich auch statt der IP den FQDN des Wiege-Hosts angeben.

<p class="just-emphasize">JPL</p>

Hier erfolgt die Parametrisierung des jeweiligen JPL-Ports.

Angegeben wird der Name des jeweiligen JPL-Modules.

Ein JPL-Modul muss die Phasen Init, Anforderung und Exit unterstützen.

<p class="just-emphasize">DEMO</p>

Siehe dazu die Ausführungen unter COM-Ports.

<p class="just-emphasize">Übernahme bis</p>

Entscheidet, wie weit der Zusatztext fortgereicht werden soll.

| Wert | Bezeichnung | Bedeutung |
| --- | --- | --- |
| 0 | Immer übernehmen | Der Zusatztext wird für alle Vorgangsklassen übernommen. |
| 1 | Angebot | Der Zusatztext wird nur bis zur Klasse Angebot übernommen. |
| 2 | Auftrag | Der Zusatztext wird nur bis zur Klasse Auftrag übernommen. |
| 4 | Rechnung | Der Zusatztext wird nur bis zur Klasse Rechnung übernommen. |
