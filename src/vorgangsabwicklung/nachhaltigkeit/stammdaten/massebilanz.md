# Massebilanz

<!-- source: https://amic.de/hilfe/_nachhaltigkeit_stamm_massebilanz.htm -->

Hauptmenü > Wareneinkauf > Nachhaltigkeit > Bewegungsübersicht > Variante Massebilanz

Direktsprung **[NAART]**

Die Massebilanz enthält die zugeordneten Warenbewegungen, den jeweiligen Vortrag und CO2-Wert, welche in den jeweiligen Report angezeigt werden.

Für die Massebilanz muss ein eigenständiger Nummernkreis zur Verfügung gestellt werden. Dieser muss dann in die Option „NUMMERNKREIS_MASSEBILANZ“ eingetragen werden.

Dafür wechselt man zum Direktsprung [OPT] und richtet mit F8 einen neuen Eintrag ein. Auf der Maske Optionen wählt man per F3 den Eintrag NUMMERNKREIS_MASSEBILAN aus und in das Feld Wert trägt man den gewünschten Nummernkreis ein.

Die vorhandenen Nummernkreis kann man unter dem Direktsprung [NKS] einsehen und auch neue Nummernkreise einrichten.

Damit die Bewegungen von Nichtrohware - oder Rohwarebelegen einer Massebilanz zugeordnet werden können, müssen die Artikel und Nuts-Nummer der jeweiligen Bewegung in der Massebilanz eingerichtet sein. Dafür benötigt man die Lagernummer, die Nuts-Nummer und die Artikelnummer. Bei fehlender Einrichtung einer Artikel- Lager- Nutsnummer in der gewünschten Massebilanz findet diese bei der Zuordnung der Massebilanz zu einer Bewegung automatisch statt und wird im Fehlerprotokoll als Ereignis protokolliert.

Des Weiteren muss beim Artikel das Gewicht eingerichtet sein. Dies richtet man auf der Artikelstammmaske unter [ARS] ein, indem man auf dem Tabreiter Allgemein in Gewich/Grundmengeneinheit das Feld ausfüllt. Zuletzt muss der zugehörige Kunde ein Nachhaltigkeitszertifikat besitzen, oder in der Bewegung muss ein passendes Zertifikat angegeben werden. In dem Zertifikat muss der Artikelnummer und die Nuts-Nummer eingerichtet sein. Ohne diese Angaben ist der Massebilanzabgang bzw. Massebilanzzugang immer mit dem Wert 0 angegeben.

| Feld | Beschreibung |
| --- | --- |
| Nummer | Nummer der Massebilanz |
| Bezeichnung | Bezeichnung für die Massebilanz |
| Festgeschrieben | Solange die Massebilanz nicht festgeschrieben ist, können Warenbewegungen hinzugefügt und entfernt werden.  
Über der Funktion „Festschreiben“ wird die Massebilanz geschlossen. Änderungen sind dann nicht mehr möglich. |
| Netto / Bruttomengen | Einstellung, ob die Rohwarenmengen Netto oder Brutto gerechnet werden sollen. |

Netto/Brutto-Unterscheidung ist nur relevant für Rohwarebelege.

Folgende Felder stehen für die Vorträge zur Verfügung.

| Feld | Beschreibung |
| --- | --- |
| Lager | Lager des Artikels, **Pflichtfeld** |
| Nuts-Nummer | Anbauland des Artikels, **Pflichtfeld** |
| Artikel | Artikelnummer des Eintrags, **Pflichtfeld** |
| Vortrag | Wert wird in der Massebilanzauswertung als Vortrag eingerechnet |
| CO2 | Wert wird in der Massebilanzauswertung als Vortrag eingerechnet |
| Bemerkung | Bemerkung für den Eintrag |

Es ist empfehlenswert alle nachhaltigen Warenbewegungen eine Massebilanz zuzuordnen, damit bei der Berechnung von nachhaltigen Beständen die Bestände stimmen.
