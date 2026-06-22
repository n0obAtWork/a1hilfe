# Kunden-Artikel Preisliste

<!-- source: https://amic.de/hilfe/_KundenArtikelPreisliste.htm -->

Hauptmenü > Preise/Konditionen > Preise > Preisliste EK/VK

Oder Direktsprung [PL]

In der Auswahlliste für Preisliste EK/VK gibt es eine Variante Kunden-Artikel Preisliste.  
In dieser kann man sich für einen oder mehrere Kunden und einen oder mehrere Artikel den jeweiligen aktuellen Preis für den per Datum angegebenen Tag anzeigen lassen. Im Auswahlbereich können die Artikelnummer, Lagernummer und Kundennummer sowie das Tagesdatum festgelegt werden. Aus Performancegründen sind in der Standardeinstellung die Kundennummer mit 10000 und die Artikelnummer mit 100 vorbelegt. Die Angabe des Tagesdatums kann auch relativ, zum Beispiel durch die Eingabe von ‚heute‘ oder ‚heute+1‘ etc., erfolgen. Die Daten werden nach Kundennummer, Artikelnummer, Lagernummer und, falls vorhanden, Kontraktnummer sortiert angezeigt.  
Mit Hilfe des Reports KundenArtikelPreisliste kann die gewünschte Selektion auch ausgedruckt werden.

Sowohl die Auswahlliste als auch der Report arbeiten mit der Prozedur aw_preis.  
Diese ermittelt die Preise aus den Kontrakten, die für den Kunden und Artikel existieren, sowie mit Hilfe der Prozedur PreisVektor den Grundpreis **für den ausgewählten Tag.** Handelt es sich um einen Kontraktpreis werden die Kontraktnummer und die Kontraktbezeichnung angezeigt, ansonsten bleiben diese beiden Felder leer.

Der ausgewiesene Preis für einen Artikel ohne Kontraktberücksichtigung wird ermittelt, indem zunächst geprüft wird, ob ein **individueller Preis** für den Artikel mit dem Kunden vereinbart ist. Andernfalls wird der für den Kunden gültige **Listenpreis** ausgewiesen. Andere bei der Preisfindung in Vorgängen ermittelbaren Preise, wie zum Beispiel **Partiepreise** oder **mengenabhängige Preise** und **Aktionspreise,** **können in dieser Auswahlliste nicht ausgewiesen werden.**

Die Inhalte der Felder werden aus folgenden Relationen geholt:

| **Feld** | **Relation** |
| --- | --- |
| Artikel | Artikel |
| Artikelbezeichnung | Artikel |
| Lagernummer | Artikel |
| Kundennummer | Kundenstamm |
| Kundenname | Kundenstamm |
| Preis | Mit Hilfe der Prozedur PreisVektor ermittelter Grundpreis beziehungsweise Kontraktpreis |
| Kontraktnummer | Kontraktstamm |
| Kontraktbezeichnung | Kontraktstamm |
| Mengeneinheit | Mit Hilfe der me_grupnummer (Artikelstamm) aus mengeinhgruppe (me_nummerVerkauf) / Kontraktpreis |
| Preiseinheit | Mit Hilfe der Prozedur PreisVektor / Kontraktpreis |
| Gültig am | Datum, zu dem der ermittelte Preis bestimmt wurde |

Die Prozedur PreisVektor berücksichtigt bei der Berechnung des Preises:

1\. Individualpreise (artiindivpreis), wenn im Kundenstamm die Preisklassen-Nummer für Verkaufs-Individualpreise (PrIndklnummervk) und die Preisgruppe für Verkaufs-Individualpreise (artiindpreisgrvk) im Artikel größer 0 sind.

2\. Artikellistenpreise (artilistenpreis) mit Hilfe der entsprechenden Preisliste (preislisnummer) aus der Relation artilprmatlink, die mit der Preisklassen-Nummer für Verkaufs-Listenpreise (preisklnummervk) aus dem Kundenstamm und der Preismatrix für Verkaufs-Listenpreise (preismatnummervk) aus dem Artikel ermittelt wurde.

3\. Nur bei Listenpreisen: Rabatte (artirabattsatz) mit Hilfe der Rabattklassen-Nummer für Standard-Verkaufs-Rabatte (rabklnummervk) aus dem Kundenstamm und der Verkaufs-Rabattgruppe (artirabgrupvk) des Artikels.
