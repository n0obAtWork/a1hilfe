# Stoffstromdaten-Druckpositionen

<!-- source: https://amic.de/hilfe/_stoffstromdruckpositionen.htm -->

Zur Darstellung von Stoffstromdaten auf Vorgangsdruck-Formularen stehen im Modul zur [Formulareinrichtung](../formulareinrichtung_und_zuordnung/formulareinrichtung/einrichtung_f6.md) für den Druckbereich ‚<em>Warenposition‘</em> Formulardruckpositionen zur Verfügung.

- **6601 ID_STOFFSTROM_ART  
**Nummer der Stoffstromart aus dem Anwendungsformat <em>‚af_stofstart‘</em><strong></strong>

- **6602 ID_STOFFSTROM_TEXT  
**Bezeichnung des Bestandteils aus der Artikelbestandteil-Liste

- **6603 ID_STOFFSTROM_ANTEIL  
**Anteil des Stoffstroms (Prozent oder Menge/Grundmengeneinheit des Artikels)

- **6604 ID_STOFFSTROM_MENGE  
**Berechnete Stoff-Menge zur Stoffstromart

- **6605 ID_STOFFSTROM_MEANTEIL  
**Mengeneinheitsnummer des Stoffstromanteils (0 = %)

- **6606 ID_STOFFSTROM_METEXT_ANTEIL  
**Text zur Mengeneinheit des Stoffstromanteils (%, kg/hl …)

- **6607 ID_STOFFSTROM_MEMENGE  
**Mengeneinheitsnummer der Stoffstrom-Menge

- **6608 ID_STOFFSTROM_METEXT_MENGE  
**Kurztext zur Mengeneinheit der Stoffstrom-Menge

- **6609 ID_STOFFSTROM_HERKUNFT  
**Herkunftskennzeichen der Stoffstrom-Werte  
- 0: aus Artikelstamm  
- 10: Anteil manuell  
- 20: Menge manuell  
    

**Wichtig bei der Einrichtung der Druckpositionen ist die Angabe des Wertes für ‚*Parameter‘ in der*** [***Detail-Maske***](../formulareinrichtung_und_zuordnung/formulareinrichtung/einrichtung_f6.md#Knopf_Detail) ***zur Druckposition.***Der Parameter gibt an, für welchen Stoffstrom-Bestandteil aus der [Artikelstamm-Zusammensetzungsliste](../../artikelstamm_und_artikel/parameter_des_artikelstamms/zusammensetzung.md) der zu druckende Wert zu ermitteln ist. Dazu wird intern eine Liste der Teilmenge der Bestandteile geführt, die als Stoffstrombestandteil gekennzeichnet sind. Auf diese Liste bezieht sich der Wert des Parameters. Der Wert *‚1‘* bezieht sich demnach auf den ersten in der Artikelzusammensetzung angegebenen Stoffstrom-Bestandteil, der Wert *‚2‘* auf den zweiten und so weiter, unabhängig davon, ob zwischen oder vor diesen Angaben in der Artikelstamm-Zusammensetzung noch andere Bestandteile aufgeführt sind, die aber keine Stoffstrom-Bestandteile sind.
