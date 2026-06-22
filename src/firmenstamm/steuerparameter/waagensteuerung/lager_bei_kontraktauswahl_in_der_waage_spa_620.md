# Lager bei Kontraktauswahl in der Waage(SPA 620)

<!-- source: https://amic.de/hilfe/_SPA_620.htm -->

An diesem Steuerparameter wird das Verhalten für das Lager nach der Kontraktauswahl festgelegt.

Folgende Optionen sind vorhanden:

| Option | Beschreibung |
| --- | --- |
| Lager immer aus Kontraktartikel | Es wird immer das Lager aus dem Kontraktartikel verwendet (nicht das Kontrakt Ziellager!). |
| Standardlager des Bedieners<br>(Direktsprung [VKONS]) | Es wird immer das Standardlager des Bedieners verwendet. Wenn der Artikel dort nicht vorhanden ist, wird das Lager aus dem Kontraktartikel benutzt. |
| Kontrakt bestimmt Lager | Es wird das Lager aus dem Kontrakt Ziellager verwendet. Wenn der Artikel dort nicht vorhanden ist, wird das Lager aus dem Kontraktartikel benutzt. |
| Lager aus Kontrakt wenn Ziellager ungleich 0, sonst Standardlager | Es wird das Lager aus dem Kontrakt Ziellager verwendet, sofern dieses nicht 0 ist. Ansonsten wird das Standardlager des Bedieners benutzt. Wenn der Artikel dort nicht vorhanden ist, wird das Lager aus dem Kontraktartikel benutzt. |
