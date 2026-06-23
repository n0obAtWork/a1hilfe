# Zahlungen mit Karte

<!-- source: https://amic.de/hilfe/zahlungenmitkarte.htm -->

<p class="just-emphasize">Relevante SPA-Einstellungen</p>

[SPA 505 - Manuelle Erfassung von EC-Karten ?:](../../firmenstamm/steuerparameter/kasse_barverkauf/manuelle_erfassung_von_ec_karten_spa_505.md)

[SPA 579 - Gekennzeichnete EC Zahlung stornierbar](../../firmenstamm/steuerparameter/kasse_barverkauf/gekennzeichnete_ec_zahlung_stornierbar_spa_579.md)

Die Kombination dieser beiden SPA Einstellungen kann relevant werden, wenn zum Einzug der Zahlung ein separates Bankterminal benutzt wird. Wenn mittels dieses Terminals nicht die Möglichkeit besteht, erfolgte Zahlungen wieder rückgängig zu machen, so soll unbedingt in A.eins auch der SPA 576 -„Gekennzeichnete EC Zahlung stornierbar“ auf nein gestellt sein.

Dann nämlich gibt es in A.eins keine Möglichkeit, diese Zahlung zu revidieren (Funktion „Zahlungsweg stornieren“) oder den Beleg abzubrechen.

Bei Verwendung dieser SPA Kombination wird die Belegverarbeitung für die online –Transaktion am Bankterminal unterbrochen und wird erst nach Bestätigung, dass die Zahlung korrekt erfolgt ist, fortgesetzt.

Wird die Bestätigung verweigert, so wird der Zahlungsweg storniert und die Zahlung kann erneut eingegeben oder der Beleg abgebrochen werden.

Die Bestätigung soll also nicht voreilig gegeben werden! Wenn es doch passiert, hat man Folgendes zu tun:

1. Beleg abschließen, falls noch ein Restzahlungsbetrag aussteht, diesen Bar abwickeln.

2. Stornobeleg in Bar erfassen.

3. Dem Kunden wird der einzuziehende Betrag in Bar ausgezahlt.

4. In der Kassenzählung entsteht dann eine Differenz über diesen EC Betrag.

5. Zahlungseingang EC Cash gegen das Differenzenkonto ausgleichen.

| Einstellmöglichkeiten POS Kasse / Empfohlene Einstellungen |
| --- |
| | EC Cash Lastschrift mit Kartenleser ohne manuellen Eingriff \*\*\* | EC Cash Lastschrift mit Kartenleser oder wahlweise Eingabe der Daten \*\*\* | EC Cash Lastschrift ohne Kartenleser, nur Eingabe der Daten EC Cash Lastschrift ohne Kartenleser, nur Eingabe der Daten \*\*\* | EC Cash Pin über sep. Bankterminal<br>(nicht widerrufbar) | EC Cash Pin über sep. Bankterminal<br>(widerrufbar)<br> |
| SPA Manuelle Erfassung von EC-Karten | unterbunden | Manuell | Manuell | Kennz. ZA | Kennz. ZA |
| SPA Gekennzeichnete EC Zahlung stornierbar | \--- | \--- | \--- | Ja | Ja |
| Kasseneinstellung EC-Karte manuell erfassen | Nein | Nein | Ja | Nein | Nein |
| EPA Abschlussbestätigung beim Belegabschluss | Nur ZaMi | Nur ZaMi | Nur ZaMi | Nur ZaMi | Nur ZaMi |

\*\*\* Diese Vorgehensweise wird nicht mehr empfohlen – siehe [Lastschrift](./barcode_etiketten/index.md)
