# Eingangslieferscheine \ Bestellung \Lieferschein erfassen

<!-- source: https://amic.de/hilfe/_cescannerelerfassung.htm -->

| **Vorgangsfunktion Übersicht** |
| --- |
| Eingang Start |
| Kundenauswahl |
| Daten einscannen |
| Eingang Ende |

<p class="just-emphasize">Erstellen eines neuen Beleges.</p>

• Als erstes wird der Start Scan Code EL der im EAN 128 verschlüsselt ist eingescannt.

• Danach kann entweder über die Kundensuchmaske auf dem Scanner oder das Einscannen des ILN Codes der Kunde bestimmt werden. Die Kunden ILN muss im Kundenstamm dafür richtig hinterlegt sein. Die ILN kann auch in der NVE verschlüsselt sein z.B. (0034006900000010048). Wichtig dabei ist nur, dass nach der 00 eine 3 folgt. Wenn der Kunde richtig erkannt worden ist, so steht dieser in der zweiten Zeile des Scanners.

• Jetzt kann der Artikel oder die Partie oder das MHD oder Lagerplatz eingescannt werden. Der Artikel kann im EAN 8, EAN13, UPCA, UPCE, oder im EAN 128 Code als AI 01 verschlüsselt werden. Die Partie wird mit dem AI-Code 10 verschlüsselt, das MHD wird mit dem AI-Code 15 und der Lagerplatz wird mit dem AI-Code 97 des EAN 128 Codes verschlüsselt.

• Dann kann die Menge eingegeben oder eingescannt werden. Die AI sind -30 für Handeingabe und 30, 37, 3100, 3101, 3102 … für die eingescannten Mengen

• Als letztes wird ELENDE der im EAN 128 verschlüsselt ist eingescannt.

<p class="just-emphasize">Gruppenzuordnung in der AI-Liste</p>

• Der Artikel hat immer die Gruppennummer 1

• Die Menge hat immer die Gruppennummer 2

• Die Partie hat immer die Gruppennummer 3

• Das MHD hat die Gruppennummer 4

• Für den Lagerplatz und das Speichern von Feldern im AO eine beliebige Gruppe verwendet werden.

<p class="just-emphasize">Preise</p>

Damit die Preise gezogen werden muss im Kundenstamm eine Preisklasse eingerichtet werden im Artikel muss die richtige Preismatrix hinterlegt werden, die die Preisklasse des Kunden enthält.

<p class="just-emphasize">Partie</p>

Wird eine Partiebezeichnung mit dem AI-Code 10 erfasst und diese existiert noch nicht, dann wird einen neue Partie mit dieser Bezeichnung angelegt.

<p class="just-emphasize">Gebinde</p>
