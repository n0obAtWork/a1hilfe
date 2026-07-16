# Kontraktnachhaltigkeit überschreibt Warenpositionsnachhaltigkeit (SPA 1155)

<!-- source: https://amic.de/hilfe/_SPA_1155.htm -->

Standard ist „Nein“

| SPA-Einstellung | Kontraktartikel nachhaltig | Warenposition nachhaltig | Nachhaltigkeitswerte | Resultierender Nachhaltigkeitsstatus der Warenposition | Nachhaltigkeitsherkunft |
| --- | --- | --- | --- | --- | --- |
| Nein | Nein | Nein | Nachhaltigkeitsstatus der Warenposition | Nicht Nachhaltig | Kunde |
| Nein | Ja | Nein | Nachhaltigkeitsstatus der Warenposition | Nicht Nachhaltig | Kunde |
| Nein | Nein | Ja | Nachhaltigkeitsstatus der Warenposition | Nachhaltig | Kunde |
| Nein | Ja | Ja | Nachhaltigkeitsstatus des Kontraktartikels überschreibt den Nachhaltigkeitsstatus der Warenposition | Nachhaltig | Kontrakt |
| Ja | Nein | Nein | Nachhaltigkeitsstatus des Kontraktartikels überschreibt den Nachhaltigkeitsstatus der Warenposition | Nicht Nachhaltig | Kontrakt |
| Ja | Ja | Nein | Nachhaltigkeitsstatus des Kontraktartikels überschreibt den Nachhaltigkeitsstatus der Warenposition | Nachhaltig | Kontrakt |
| Ja | Nein | Ja | Nachhaltigkeitsstatus des Kontraktartikels überschreibt den Nachhaltigkeitsstatus der Warenposition | Nicht Nachhaltig | Kontrakt |
| Ja | Ja | Ja | Nachhaltigkeitsstatus des Kontraktartikels überschreibt den Nachhaltigkeitsstatus der Warenposition | Nachhaltig | Kontrakt |

Wird dieser SPA auf „Nein“ gestellt, dann wird bei der Nachhaltigkeitsvorbelegung und der Nachhaltigkeitskorrektur durch den Mandantenserver nur die Warenpositionen mit Nachhaltigkeitswerten des Kontraktartikels aus einem Kontrakt überschrieben, wenn der Kontraktartikel nachhaltig ist und der betroffene Kunde für den Artikel ein gültiges Nachhaltigkeitszertifikat besitzt.  
    

Wird dieser SPA auf „Ja“ gestellt, dann werden immer bei einem zugeordneten Kontrakt, die Nachhaltigkeitswerte des Kontraktartikels benutzt, um die Nachhaltigkeitswerte der Warenposition zu überschreiben.

Wenn der Kontraktartikel in diesem Fall nicht nachhaltig ist, dann wird die Warenposition auch nicht nachhaltig und alle anderen Nachhaltigkeitswerte verschwinden dementsprechend aus der Warenposition.  
    

Betroffene Nachhaltigkeitswerte sind u.a. der Nachhaltigkeitsstatus der Warenposition, die Herkunft dieses Status, das Anbauland des Artikels, die Herkunft dieses Anbaulandes, die THG-Werte und die Herkunft der THG-Werte. Nachhaltigkeitswerte, die als Herkunft „manuell“ haben, bleiben bei der Korrektur und Vorbelegung unberührt.

Achtung: Es wird davon abgeraten, den SPA regelmäßig umzustellen, weil dann vielleicht ungewollt durch eine Nachhaltigkeitskorrektur durch den Mandantenserver, die Nachhaltigkeit unabhängig vom Kontrakt korrigiert wird.

Wenn der SPA 1155 auf Ja steht, dann hat der SPA 1158 keine Auswirkung, da Die Warenposition bezüglich ihrer Nachhaltigkeitswerte mit den Nachhaltigkeitswerten aus dem Kontraktartikel überschrieben wird.
