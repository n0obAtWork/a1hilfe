# Materialorder [LVSMO]

<!-- source: https://amic.de/hilfe/_lvs20_Materialorder.htm -->

Es gibt zwei Arten der Materialorder.

1. Vorgangsgebundene Materialordern

Diese Materialordern bilden 1:1 einen Vorgang und dessen Materialbedarf ab. Artikel und Partien befinden sich nebst den Referenzen zu den Vorgangspositionen in dieser Materialorder.

2. Ungebundene Materialordern

Diese Materialordern werden manuell oder über die Produktionsschnittstellle erstellt und enthalten in der Regel keine Referenzen auf Vorgangspositionen. Sie lassen sich mit dem Materialorder-Pfleger [LVSMO] erstellen.

| **Kopfdaten** |
| --- |
| **Feld** | **Beschreibung** |
| Nummer | Wird automatisch vom System vergeben |
| Ziel | Hier kann eine LVS-Lokalität ausgewählt werden (nicht empfohlen). |
| Linie | Auswahl einer Produktionslinie – In diesem Fall wird „ziel“ deaktiviert und mit dem Bereitstellungsbereich der Produktionslinie belegt. |

Es ist zu empfehlen, die EPA-Einstellung „Linie als Default-Quelle“ auf „ja“ eingestellt zu lassen. In diesem Fall wird der Cursor bei Start dieser Maske sofort in das Feld „Linie“ gesetzt.

| **Zeilendaten** |
| --- |
| **Wert** | **Anzeige** | **Beschreibung** |
| Liste | Ja | ListenNr |
| Position | Ja | Laufende Positionsnummer |
| Artikel | Nein | Artikelnummer aus dem Lager der im Kopf gewählten Lokalität |
| Artikelbezeichnung | Ja | Bezeichnung des Artikels |
| Partie | Nein | Partienummer |
| Partiebezeichnung | Ja | Bezeichnung der Partie |
| Menge/Anzahl | Ja | |
| ME | Nein | Mengeneinheit. Hier sollte eine LVS-Mengeneinheit gegeben werden. In der EPA-Einstellung „Mengeneinheit aus“ sollte LVS stehen. |
| Bezeichnung Mengeneinheit | Ja | |
