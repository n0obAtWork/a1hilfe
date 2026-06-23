# Inventur

<!-- source: https://amic.de/hilfe/_cescannerinventur.htm -->

Vorbereitungsschritte

- Als erstes müssen die Scancodes für die Inventur mit dem Lagerverwaltungssystem eingerichtet werden. Dies sind IV [-106] für Inventur Start undIVENDE[-107] für Inventur Ende. Des Weiteren sind die AI-Zuordnungen für den Scancode einzutragen. (Beispiel mit Partie) Soll die Inventur ohne Partie erfasst werden, so kann die Zeile mit dem AI-Code 10 weggelassen werden. Mit dem Feld „Optional“ kann gesteuert werden, ob die AI in einem Erfassungsblock erfasst werden muss.
- Eine [Inventur](../../../../abschluesse_inventur/inventur/index.md) muss eröffnet worden sein.
- Es müssen zwei Scancodes im EAN 128 Codiert erstellt werden IV und IVENDE
- Aus den Scannertabellen sollten die Daten abgeschlossener Inventuren gelöscht werden bevor die Erfassung einer neuen Inventur gestartet wird.

| AI | **Application Identifier** | **Gruppe** | **Typ** | **Optional** |
| --- | --- | --- | --- | --- |
| \-30 | Mengeneingabe per Hand | 2 | | Nein |
| \-6 | UPC-A Code | 1 | | Nein |
| \-5 | EAN-Code 8 | 1 | | Nein |
| \-4 | EAN-Code 13 | 1 | | Nein |
| 1 | EAN Nummer der Handelseinheit | 1 | | Nein |
| 10 | Partie(Charge) | 3 | | Nein / Ja |
| 30 | Menge in Stück (EAN128) | 2 | | Nein |
| 3100 | Nettogewicht in Kilogramm (0 Nachkomma) (EAN128) | 2 | | Nein |
| 3101 | Nettogewicht in Kilogramm (EAN128) | 2 | | Nein |
| 3102 | Nettogewicht in Kilogramm (EAN128) | 2 | | Nein |
| 3103 | Nettogewicht in Kilogramm (EAN128) | 2 | | Nein |

Abarbeitung der Inventur

Nachdem die Scancodes für die Inventur angelegt und die AI-Zuordnung eigerichtet worden sind kann jetzt die Erfassung mit dem Scanner erfolgen.

| Erfassung |
| --- |
| IV für den Beginn einer Inventur |
| Artikel mittels Scancode |
| Menge per Hand |
| Eventuell eine Partie mittels Scancode |
| IVENDE für das Ende eines Inventurblockes. |

In einem Inventurblock können mehrere Positionen erfasst werden.

Sind alle Daten erfasst worden, so wird die Aufnahme der Inventur mit IVENDE beendet. Es empfiehlt sich die komplette Inventuraufnahme mit dem Scanner in mehrere Blöcke aufzuteilen.

Nach dem Scannen des Befehls IVENDE werden die Daten in die Inventur übernommen. Sobald das System anfängt die Daten zu übertragen wird in der Relation TCPIP_Scanner_Inventuruebernahme eine Zuordnung zwischen der erfassten Scannerposition und der Position in der Inventur hergestellt.

Diese Zuordnungen werden in der Anwendung „TCPIP Scanner“ in der Variante „Scanner Inventur Abgleicher“ dargestellt.
