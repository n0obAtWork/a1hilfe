# Kassensystemverwaltung (Hardware)

<!-- source: https://amic.de/hilfe/_kass_einr_hardw.htm -->

Jeder logischen Kasse ist ein Kassensystem zugeordnet. Dieses Kassensystem beschreibt die Hardwareeinheit.

Durch diese Trennung ist es möglich eine komplette Hardwareeinheit (Kassensystem) mit ihrer hardwarespezifischen Einrichtung auszutauschen und an einem definierten Arbeitsplatz (logische Kasse) mit seinen Regeln und Spezifikationen einzusetzen.

| Kassensystem-Kopfdaten |
| --- |
| Kassensystemnummer | Nummer des Hardware-Systems |
| Bezeichnung | Bezeichnung des Kassensystems |
| Anlagedatum | Anzeige des Datums der Anlage dieses Kassensystems |
| Änderung am | Anzeige des Datums der letzten Änderung |

| Kassensystem-Drucker |
| --- |
| Rechnungsdrucker | Drucker, auf dem Rechnungsbelege (keine Bons) gedruckt werden. |

| Kassensystem-Schublade. | |
| --- | --- |
| Bezeichnung Schubladentyp | |
| Anschlusstyp | Eine Schublade kann am Drucker angeschlossen werden. |
| Anschluss ist | |
| • Port | z.B. COM1 oder LPT1 |
| • Verbindungsparameter | z.B. 9600,n,8,1 (Baud, Parity, Data-, Stopbits) |
| • Puffergröße Eingang | Ist 1024 |
| • Puffergröße Ausgang | Ist 1024 |
| Druckertyp (enth. Steuerseq) | |
| • Typ | Normal oder Win7/2008 |
| • Druckertyp | Druckertyp aus den Druckertypen, die mit dem Direktsprung [DRT] gepflegt werden. |
| • Druckertyp-Bezeichnung | Anzeige der Bezeichnung des Druckertyps |
| Steuersequenz | Hier wird, wenn vorhanden eine Steuersequenz zu diesem Druckertyp angezeigt. Ist noch keine Steuersequenz zu dem Druckertyp hinterlegt, an dem Sie die Schublade anschließen wollen, so muss diese hier eingetragen werden. Diese gilt dann für alle Kassensysteme, die für den Anschluss der Schublade diesen Druckertyp verwenden. |
| | | |

| Kassensystem-Display  
Anzeige auf einem zweizeiligen Kundendisplay |
| --- |
| Displaytyp | Texteintrag um welches Kassendisplay es sich handelt |
| Steuersequenz | Das Display verfügt typerweise über 20 Zeichen pro Zeile. Eine längere Zeichenkette würde in der 2. Zeile fortgesetzt, die Daten nach einer kürzeren Zeichenkette an der aktuellen Stelle fortgesetzt werden. Um zu steuern, welche Zeile die Daten darstellen soll, werden Steuersequenzen aus nicht druckbaren Zeichen verwendet. Diese werden hier hexadezimal angegeben. |
| • 1\. Zeile | Steuersequenz, die dem Display angibt, dass die nachfolgenden Zeichen in der 1. Zeile dargestellt werden sollen  
z.B. 1B3D020C0B |
| • 2\. Zeile | Steuersequenz, die dem Display angibt, dass die nachfolgenden Zeichen in der 1. Zeile dargestellt werden sollen  
z.B. 1F4218 |
| Anschlusstyp | |
| • Generic | Die bisherigen Einstellungen erlaubten COM-LPT und Dateisystem in einem Feld einzugeben. (Diese Einstellung wird als Generic bezeichnet).  
Beginnt die Zielbezeichnung mit „c“ oder „C“, so wird angenommen, dass es sich um einen COM-Port handelt |
| • COM | Für spätere Verwendungen geplant |
| • LPT | Für spätere Verwendungen geplant |
| • Dateisystem | Für spätere Verwendungen geplant |
| • TCPIP | Bei TCPIP kann ein „Serial-Device-Server“ zum Einsatz kommen, um ein Display mit einer seriellen Schnittstelle an diesem im Netzwerk an einer Ethernet-Schnittstelle zu betreiben. Hier sind die IP-Adresse und der Port des Servers einzugeben. Die Verbindungsparameter für die Kommunikation zwischen Server und Display müssen in diesem selbst konfiguriert werden.  
z.B. „192.168.1.12:950“  
siehe auch [unterstützte Hardware](../unterstuetze_hardware/serial_device_server.md) |
| Display Device | Angaben zum Anschluss |
| • Port | COM-Port  
z.B. COM1 |
| • Einstellungen | Baudrate 9600, 8 Datenbits, keine Parität und 1 Stopbit  
z.B. 9600,8,n,1 |
| • Puffergröße Eingang | Ist 1024 |
| • Puffergrüße Ausgang | Ist 1024 |

| Kassensystem-EC-Cash-Verfahren  
Bargeldlose Zahlung mit EC-Karte mit nachfolgend beschriebenen Optionen |
| --- |
| • Lastschrift | Hier werden die Daten für die Lastschrift aus dem Kartenleser gewonnen. Eine manuelle Eingabe der Daten ist nicht vorgesehen. |
| • Lastschrift auch manuelle Eingabe | Hier können neben der Erfassung der Daten für die Lastschrift aus dem Kartenleser auch manuelle Eingaben gemacht werden. |
| • Bezahlterminal verwenden | Aktiviert die nachfolgenden Einstellungen zum Bezahlterminal |

| Kassensystem-EC-Cash-Einstellungen Bezahlterminal  
Verwendung eines Bezahlterminals mit den nachfolgenden Optionen. |
| --- |
| • Manuelle Bedienung | Hier wird der Betrag manuell in das Terminal eingegeben, es erfolgt keine Kommunikation mit A.eins. In A.eins wird lediglich die Abwicklung der Bezahlung bestätigt. |
| • Ansteuerung über Makro | Ein Makro steuert das Bezahlterminal über die ZVT-700-Schnittstelle an. A.eins wartet auf eine Rückmeldung vom Bezahlterminal, ob der Bezahlvorgang erfolgreich war.  
Siehe hierzu auch Liste der unterstützten Geräte. [Bezahlterminals](../vorbelegung_kassennummer.md)  
Siehe auch [Bezahlterminal-Ansteuerung](../../../zusatzprogramme/ehemalige_addins_uebersicht/n_a/bezahlterminal_metis/index.md) |
| Steuerungsmakro | Name des Makros, das von AMIC_BZT_MUSTER abgeleitet ist und die Ansteuerung des Bezahlterminals vornimmt. |

<p class="siehe-auch">Siehe auch:</p>

- [Geräte einrichten](./geraete_einrichten/index.md)
- [Druckereinrichtung](./druckereinrichtung/index.md)
