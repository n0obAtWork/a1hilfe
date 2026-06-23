# Einrichtung der Inventur mit dem Lagerverwaltungssystem

<!-- source: https://amic.de/hilfe/_cescannerlvsinventur.htm -->

<p class="just-emphasize">Bedingungen für die Inventur</p>

1. Alle Inventurgruppen mit einem Artikel im LVS müssen den gleichen Stichtag und das gleiche Erfassungsdatum haben.

2. Alle Artikel im LVS müssen einer Inventurgruppe zugeordnet sein.

3. Es muss im Inventurstamm ein Nummernkreis für die Inventur hinterlegt worden sein.

4. Es muss ein LVS Verarbeitungsmodul je Scanner gestartet sein

<p class="just-emphasize">Vorbereitungsschritte</p>

- Als erstes müssen die Scancodes für die Inventur mit dem Lagerverwaltungssystem eingerichtet werden. Dies sind LVSIV [-147] für Inventur Start und LVSIVENDE[-148] für Inventur Ende. Des Weiteren sind die AI-Zuordnungen für den Scancode einzutragen.

| AI | **Application Identifier** | **Gruppe** | **Typ** | **Optional** |
| --- | --- | --- | --- | --- |
| \-30 | Mengeneingabe per Hand | 2 | | Nein |
| \-6 | UPC-A Code | 1 | | Nein |
| \-5 | EAN-Code 8 | 1 | | Nein |
| \-4 | EAN-Code 13 | 1 | | Nein |
| 1 | EAN Nummer der Handelseinheit | 1 | | Nein |
| 10 | Partie(Charge) | 3 | | Nein |
| 30 | Menge in Stück (EAN128) | 2 | | Nein |
| 3100 | Nettogewicht in Kilogramm (0 Nachkomma) (EAN128) | 2 | | Nein |
| 3101 | Nettogewicht in Kilogramm (EAN128) | 2 | | Nein |
| 3102 | Nettogewicht in Kilogramm (EAN128) | 2 | | Nein |
| 3103 | Nettogewicht in Kilogramm (EAN128) | 2 | | Nein |

- Um die Inventur zu beginnen werden im Inventurstamm [IVS] neue Inventuren für die einzelnen Inventurgruppen eröffnet.
- Danach werden alle Inventuren unter Inventurvorbereitung ausgewählt und der Haken bei Vortrag der Ladeträger mit Bestand gesetzt. 
- Jetzt prüft das System ob die Bedingungen für alle Inventurgruppen eingehalten worden sind. Ist dies der Fall, so werden die Daten in die Inventur eingespielt. Es werden dabei die Tabellen InventurBeleg, InventurBelegPartie, InventurBelegPartieLVS gefüllt. Des Weiteren werden in der Protokolltabelle LVS_LE_PositionBewegung zwei Einträge je Ladeeinheit, LadeeinheitsPosition gemacht, die den Bewegungsstatus 1 für Menge herunternehmen und den Bewegungsstatus 2 für den Inventurvortag bekommen. Wird die Inventur gelöscht, so bekommt das Feld den Bewegungsstatus die 3 für Menge herunternehmen und die 4 für den Inventurvortrag.

Nachdem die Vorbereitung erfolgreich abgeschlossen ist, kann jetzt mit der Erfassung begonnen werden.

Während der Inventur können die Kisten gewogen, geleert und gefüllt werden. Während beim Wiegen und Leeren nur die Menge in der Inventur abgeändert wird, wird hingegen beim Kisten Füllen ein neuer Eintrag in die Inventur geschrieben. Jeder Datensatz der abgeändert oder neu hinzugefügt wird, bekommt in der Tabelle InventurBelegPartieLVS ein Bearbeitungskennzeichen und die TCPIP Adresse des jeweiligen Scanners.

 Das Bearbeitungskennzeichen kann zwei Ausprägungen annehmen.

| Bearbeitungskennzeichen in InventurBelegPartieLVS | Status |
| --- | --- |
| 1 | Ist gerade in Bearbeitung. |
| 2 | Bearbeitung ist abgeschlossen |

<p class="just-emphasize">Beispiel</p>

| Scannung | Bedeutung |
| --- | --- |
| LVSIV | Startet die LVS Inventur auf dem Scanner |
| LEEREN 911 | Startet das Leeren auf der Lokation 911 |
| 9750 | Der Ladeträger 50 |
| LEERENENDE | Entleert den Ladeträger und schreibt eine 0 Menge in die Inventur; setzt das Bearbeitungskennzeichen in Inventurbelegpartielvs auf 1 |
| KEYLEFT | Zeigt in der Datenanzeige unten im Scanner den Ladeträger 50 mit der Menge 0 an. |
| WIEGEN 911 | Startet das Wiegenkommando |
| 9751 | Der Ladeträger 51 |
| 1250 | Das Gewicht |
| WIEGENENDE | Schreibt das Gewicht in den Ladeträger und schreibt die Menge in die Inventur; setzt das Bearbeitungskennzeichen in Inventurbelegpartielvs auf 1 |
| KEYLEFT | Zeigt in der Datenanzeige unten im Scanner den Ladeträger 50 mit der Meng 0 an und den Ladeträger 51 mit der Menge 1250 an. |
| LVSIVENDE | Beendet die Inventur Erfassung auf dem Scanner und setzt das Bearbeitungskennzeichen in Inventurbelegpartielvs auf 2. |

Es wird die Itembox IB_CE_Inventur_LVS benutzt.

Zur Auswertung wurden zwei Standard Reporte vorbereitet. (Differenzen-Liste LVS und Zählliste Inventur LVS).

Im ersten Report wird der Bestand, der über den Inventurvortrag eingespielt wurde, der Zählmenge gegenüber gestellt. Im Auswahlbereich des Reportes kann entschieden werden ob alle Datensätze angezeigt werden sollen, oder nur die Datensätze, die eine Mengenveränderung erhalten haben.

Der Zweite Report kann als Zählliste verwendet werden. Hier werden alle Ladeträger samt Ladeeinheiten angezeigt, die per Inventurvortrag in die Inventur eingespielt worden sind.
