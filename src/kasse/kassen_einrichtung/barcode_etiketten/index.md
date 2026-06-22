# Barcode-Etiketten

<!-- source: https://amic.de/hilfe/kassewaagenetiketten.htm -->

Es besteht die Möglichkeit, an der A.eins-Kasse über einen Strichcode neben der Artikelerkennung auch den Preis sowie die Menge direkt ins A.eins-System zu übernehmen.

Solche Barcodes werden „InStoreBarcodes“ genannt und sind ausschließlich zur hausinternen Verwendung gedacht.

Anwendungsbeispiele:

• Etikett einer Waage mit Artikelnummer und Gewicht

• Etikett eines Warenausgabeautomaten

• Barcode eines Warenlieferscheins

Der Aufbau eins solchen Etiketts könnte so aussehen:

| Position | Beschreibung |
| --- | --- |
| 1 - 2 | 21 (Konstante, die dem System mitteilt, dass hier eine Sonderbehandlung erfolgen soll; nämlich, dass es sich um einen Waagenartikel handelt.) |
| 3 – 7 | Artikelnummer (wie in den Stammdaten hinterlegt) |
| 8 – 12 | Gesamtpreis (Hier steht der durch die Waage ermittelte Preis drin, der durch A.eins übernommen werden soll.) |
| 13 | Prüfziffer |

Voraussetzungen:

Einige Voraussetzungen müssen jedoch alle erfüllen:

• Der Barcode darf nicht als EAN-Barcode in der Datenbank enthalten sein. Dies sollte in der Regel der Fall sein, wenn der InStoreBarcode mit der Ziffer 2 beginnt. Es findet keine programmatische Prüfung auf Kollisionen statt.

• Die Artikelnummer im Barcode muss im System pro Lager eindeutig sein (d.h. in unserem Beispiel müssen alle Artikelnummern von Barcodeartikeln 5-stellig sein)

• Der EAN-Code für Nicht-In-Store-Artikel muss im System eindeutig sein (im Artikelstamm)

Steuerparameter:

Der Steuerparameter [472 - Datenübernahme an Kasse aus Etikett](../../../firmenstamm/steuerparameter/kasse_daten_aus_strichcode/datenuebernahme_an_kasse_aus_etikett_spa_472.md) muss eingeschaltet sein.

Soll eine gegebene Mengeneinheit auch als Preismengeneinheit übernommen werden, so muss der Steuerparameter [772 –](../../../firmenstamm/steuerparameter/kasse_daten_aus_strichcode/preismengeneinheit_aus_mengeneinheit_uebernehmen_spa_772.md) Preismengeneinheit aus Mengeneinheit übernehmen gesetzt sein.

<p class="siehe-auch">Siehe auch:</p>

- [Barcode-Schema-Einrichter](./barcode_schema_einrichter.md)
