# Einrichterparameter zum Vorgang als Stapel

<!-- source: https://amic.de/hilfe/einrichterparameterzumvorganga.htm -->

Die Einrichterparameter umfassen momentan folgende Bereiche:

| Parameter | |
| --- | --- |
| Druckauswahlfenster | Mit diesem EPA kann eingestellt werden, ob ein Druckauswahlfenster Ja/Nein angezeigt wird oder nicht. |
| Druckvorbelegung | Für die Knöpfe D1 und D2 kann festgelegt werden, ob der Fokus beim Druckfenster auf Ja oder auf Nein steht. |
| Vorgangsklasse | Mit diesem EPA wird eingestellt, welche Standardvorgangsklasse beim öffnen vorgegeben wird. |
| Zielvorgangsklasse bei automatischer Umwandlung | Die Zielvorgangsklasse steuert die Möglichkeit, aus einem Beleg der Klasse 400 einen Belege der Klasse 600 zu machen, also einen Auftrag in einen Lieferschein umzuwandeln. Wird die Zielvorgangsklasse leer gelassen, so werden keine Umwandlungsbelege angezeigt. |
| Vorgangsunterklasse | Im Standardfall (Knopf OK) wird diese Unterklasse als Vorbelegung genommen. |
| Vorgangsklasse, bei der keine Preise gezogen werden | |
| Preise nachladen aus der Liste bei Umwandlung und 0 Preisen | |
| Itembox Kunde | In diesem EPA muss die ITEM Boxen festgelegt werden, die zum Anzeigen der Auswahlinformationen genutzt werden sollen.  
IB_KU ist die Vorbelegungen den Kunden. |
| Itembox Artikel | In diesem EPA muss die ITEM Boxen festgelegt werden, die zum Anzeigen der Auswahlinformationen genutzt werden sollen.  
IB_ARTIKEL_NU ist die Vorbelegungen den Artikelfall. |
| Itembox Zusatz | In diesem EPA muss die ITEM Boxen festgelegt werden, die zum Anzeigen der Auswahlinformationen genutzt werden sollen. |
| Listenpreisklassenliste 1 (mit Komma trennen) | An dieser Stelle wird mit Komma getrennt, eingetragen, welche Kundenpreisklassen welche Preise (1 bis 4) zu sehen bekommen. |
| Listenpreisklassenliste 2 (mit Komma trennen) | An dieser Stelle wird mit Komma getrennt, eingetragen, welche Kundenpreisklassen welche Preise (1 bis 4) zu sehen bekommen. |
| Listenpreisklassenliste 3 (mit Komma trennen) | An dieser Stelle wird mit Komma getrennt, eingetragen, welche Kundenpreisklassen welche Preise (1 bis 4) zu sehen bekommen. |
| Listenpreisklassenliste 4 (mit Komma trennen) | An dieser Stelle wird mit Komma getrennt, eingetragen, welche Kundenpreisklassen welche Preise (1 bis 4) zu sehen bekommen. |
| Bestellte Menge anzeigen in Klassen (mit Komma trennen) | |
| Bestellte Menge abfragen in Klassen (mit Komma trennen) | |
| Gebindefaktor -2- anzeigen in folgenden Klassen (z.B. 200,300) | |
| Gebindefaktor -2- abfragen in folgenden Klassen (z.B. 200,300) | |
| Letzter VK Maskenfeld anzeigen in folgenden Klassen (s.o.) | |
| Preis pro anzeigen in folgenden Klassen (s.o.) | |
| Preis pro abfragen in folgenden Klassen (s.o.) | |
| Gebindefaktor -1- Anzeigen in folgenden Klassen (s.o.) | |
| Gebindefaktor -1- Abfragen in folgenden Klassen (s.o.) | |
| Gebindemengenfeld anzeigen in folgenden Klassen (s.o.) | |
| Gebindemengenfeld abfragen in folgenden Klassen ( s.o.) | |
| Mengeneinheit anzeigen in folgenden Klassen (s.o.) | |
| Mengeneinheitsnummer anzeigen in folgenden Klassen (s.o.) | |
| Preis anzeigen in folgenden Klassen (s.o.) | |
| Preis abfragen in folgenden Klassen (s.o.) | |
| Preismengeneinheit anzeigen in folgenden Klassen (s.o.) | |
| Preismengeneinheit abfragen in folgenden Klassen (s.o.) | |
| Zusatz anzeigen in folgenden Klassen (s.o.) | |
| Zusatz abfragen in folgenden Klassen (s.o.) | |
| Position anzeigen in folgenden Klassen (s.o.) | |
| Artikelnummer anzeigen in folgenden Klassen (s.o.) | |
| Prozedur zur Berechnung des Letzten VK Preises (leer Standard) | Um auch abweichende LtzVK Anzeigen realisieren zu können, kann hier die Prozedur angegeben werden, die den letzten VK anzeigen soll. Die Prozedur hat die Parameter KundId und ArtikelId, und muss einen „result set mit einem Datensatz“ mit dem Feld WabewPreis zurückliefern. |
| Liste der Artikelnummern für Textänderungen zugelassen | |
| Artikelgruppe, für die doppelte Erfassung erlaubt ist (muss > 0 sein) | |
| Unterklasse für D1 Knopf (Default) | Hier kann die Unterklassen zum den Knopf D1 festgelegt werden. |
| Unterklasse für D2 Knopf | Hier kann die Unterklassen zum den Knopf D2 festgelegt werden. |
| Name des Addonfeldes, in dem der Auftragsbestand im Lieferschein eingetragen wer | Wird die Funktion Auftrag in Lieferschein genutzt, so kann noch angegeben werden, in welchem Feld im Lieferschein (WabewAddon) die Originalmenge hinterlegt werden soll, so dass bei evtl. Mengenänderungen die alten Mengen noch auf dem Lieferschein mit angedruckt werden können. |
| Liste der ME-Nummern, bei denen im Umw. Fall die Menge zu 0 wird (mit Komma tr.) | |
| Vorgangsklassenliste, für die zusätzlich Leergut abgefragt werden soll | |
| Preiseinheit abfragen in folgenden Klassen (s.0.) | |
| [Sortiervariante der Warenpositionen](./sortiervariante_der_warenpositionen_format_vorgpossort.md) | Dieser Einrichterparameter erlaubt eine Auswahl von Sortiermöglichkeiten innerhalb derer die Belegpositionen sortiert werden.  
Als Besonderheit besteht die Möglichkeit, die Sortierung nach EAN (3) vorzunehmen. Wird diese Sortierung gewählt, so werden statt der EAN Nummern im Vorgang die Sortierpositionen aus der MSA Liste in das Feld EAN im Vorgang eingetragen.  
Ein weiterer Sonderfall stellt hier das Leergut dar, für Leergut wird der EAN Code aus dem Artikelstamm in den Vorgang übernommen, so dass entschieden werden kann, ob das Leergut am Anfang oder am Ende des Beleges eingetragen werden soll. |
| Im Umwandlungsfall eine ¨polnische¨ Korrekturrechnung erstellen | |
| Darstellung Wertartikel | |
| Artikel in die MSA-Liste übernehmen | |
| Preise in die MSA-Liste übernehmen | |
| Anzeige des Bruttowertes nach dem Speichern (Ja/Nein) | Nach Abschluss eines Beleges wird standardmäßig der Bruttowert angezeigt, diese Anzeige kann hiermit unterdrückt werden.  
 |

<p class="siehe-auch">Siehe auch:</p>

- [Sortiervariante der Warenpositionen (Format VORGPOSSORT)](./sortiervariante_der_warenpositionen_format_vorgpossort.md)
