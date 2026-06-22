# Artikelzusatzrelationen bei Kopie (SPA 1018)

<!-- source: https://amic.de/hilfe/_SPA_1018.htm -->

Es gibt zwei Anwendungsfälle für Trigger, die Artikelzusatzrelationen wie ArtikelAddon vorbelegen, wenn ein Artikel angelegt wird:

1. Vorbelegung zum Zweck der Performance-Verbesserung – In diesem Fall existiert stets ein Dummy-Datensatz zum Artikel, was die Suche von Artikeln mit bestimmten Addon-Feldern beschleunigt, da kein LEFT OUTER JOIN notwendig ist.

2. Intelligente Vorbelegung aufgrund kalkulierter Werte

Im Fall1 kann es sinnvoll sein, die Daten aus dem Quell-Artikel zu überschreiben. In diesem Fall wird der Steuerparameter auf „überschreiben“ gestellt.

Im Fall 2 sollen vermutlich die Daten des Quell-Artikels nicht übernommen werden. In diesem Fall wird der Steuerparameter auf „beibehalten“ gestellt.
