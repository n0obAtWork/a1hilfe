# Vorzeichenabhängige Mengendarstellung in der Kontraktauswahlliste

<!-- source: https://amic.de/hilfe/_83_33201.htm -->

In der Auswahllistenvariante 'Kontrakte' der Anwendung Kontrakte [KTR] wurde bei der Darstellung der ratierlichen Werte von Mengen, Restmengen und kumulierten Restmengen die Einstellung des Kriteriums 'Menge mit Vorzeichen' = 'Ja' der Bereichsauswahl nicht korrekt berücksichtigt. Dieser Umstand wurde nun behoben. Zudem kam es bei Kontrakten mit einer anderen Standardkontraktvariante als "Monatl. lin. Abnahme" zu Unstimmigkeiten bei der Zuordnung von Liefermengen zu den einzelnen ratierlichen Monaten, da in allen Fällen der Beginn des Kontraktzeitraums zur Monatszuordnung verwendet wurde. Dieses Verfahren wird jetzt nur noch bei vorliegen der Standardkontraktvariante "Monatl. lin. Abnahme" genutzt. In allen anderen Fällen wird das Bewegungsdatum der Lieferung verwendet. 

Releasenote Kategorie:

Ticket: 716987[33201]

Version: 8.3.2312.22

Datum: 22.12.2023

Anwendung: Kontrakte [KTR]

Variante: Kontrakte

Funktion/Report: Auswahlliste

[Weitere Informationen](http://www.amic.de/hilfe/_kontrakt_auswahllist_felder.htm)

Tags:

Releasenote, 8.3.2312.22, 33201, 716987
