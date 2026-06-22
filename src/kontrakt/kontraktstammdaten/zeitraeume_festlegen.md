# Zeiträume festlegen

<!-- source: https://amic.de/hilfe/_zeitrumefestlegen.htm -->

Zuerst wird bestimmt, für welchen Zeitraum die Preis-/Mengenvereinbarungen gelten sollen. Dies erfolgt auf der ersten Erfassungsseite des Kontraktstamms über die Funktionen Mengenzeitraum F10 und Preiszeitraum F11. Nach Betätigen von F8 können die Zeiträume, ggf. unterschiedlich für Mengen und Preise, neu erfasst werden.

Für die Bearbeitung der Mengenzeiträume stehen grundsätzlich zwei Datentabellen zwei Datentabellen zur Verfügung. In der ersten Tabelle werden die Mengen- beziehungsweise Wert-Zeiträume des Kontrakts dargestellt, abhängig davon, ob es sich um einen Mengen- oder Wertkontrakt handelt. Neben Anfangs- und Enddatum des jeweiligen Zeitraums wird die Gesamt-Sollmenge oder der Gesamt-Sollwert des Zeitraums dargestellt. Handelt es sich bei dem Kontrakt um einen Einzelmengen- oder Einzelwert-Kontrakt, so handelt es sich hierbei um die Summe der Sollmengen oder Sollwerte der Kontraktartikel. Bei Gesamtmengen- /Gesamtwert-Kontrakten und bei Einzelmengen-/Einzelwert-Kontrakten mit nur einer Kontraktartikelposition kann der Wert direkt in dieser Spalte geändert werden. Die aktuelle Restmenge beziehungsweise Restwert wird ebenfalls dargestellt.  
Ist die Option [Steuerungsparameter](../../firmenstamm/steuerparameter/kontraktwesen/ratierliche_einstellungen_spa_846.md) 846 „Ratierliche Einstellungen“ „Ktr-Anzeige Minusrest in Folgezeitraum“ mit dem Wert **Ja** eingestellt, so wird eine zusätzliche Spalte **Rest>**0 dargestellt, die negative Restmengen oder Restwerte eines Zeitraums mit der des Folgezeitraums verrechnet und selbst mit dem Wert 0 darstellt. Die Einstellung der Option [Steuerungsparameter](../../firmenstamm/steuerparameter/kontraktwesen/ratierliche_einstellungen_spa_846.md) 846 „Ratierliche Einstellungen“ „Ktr-Anzeige Kumulierte Zeitraum-Reste“ mit dem Wert **Ja** stellt in einer weiteren Spalte den kumulierten Rest dar.

Änderungen von Soll-Mengen und Soll-Werten sowie Zeitraum-Änderungen werden in einem Änderungsprotokoll dokumentiert.

| Feld | Beschreibung |
| --- | --- |
| Angangsdatum | Beginn des Kontraktmengen-Zeitraums |
| Enddatum | Ende des Kontrakt-Zeitraums. |
| Gesamtmenge | Sollmenge des Kontrakt-Zeitraums  
(nur bei Mengen-Kontrakt). |
| Restmenge | Aktuelle Restmenge des Kontrakt-Zeitraums  
(Nur bei Mengen-Kontrakt). |
| Gesamtwert | Sollwert des Kontrakt-Zeitraums  
(nur bei Wert-Kontrakt). |
| Restwert | Aktueller Restwert des Kontrakt-Zeitraums  
(Nur bei Wert-Kontrakt). |
| Rest>0 | Negativer Rest wird mit 0 dargestellt, aktueller Rest ist um negativen Rest des vorhergehenden Zeitraums reduziert.  
(Nur bei eingestellter Option [Steuerungsparameter](../../firmenstamm/steuerparameter/kontraktwesen/ratierliche_einstellungen_spa_846.md) 846 „Ratierliche Einstellungen“ „Ktr-Anzeige Minusrest in Folgezeitraum“ mit dem Wert **Ja**). |
| KumRest | Summe der Werte aus vorhergehender Restspalte (Restmenge, Restwert, Rest>0) bis einschließlich dem aktuellen Zeitraum  
(Nur bei eingestellter Option [Steuerungsparameter](../../firmenstamm/steuerparameter/kontraktwesen/ratierliche_einstellungen_spa_846.md) 846 „Ratierliche Einstellungen“ „Ktr-Anzeige Kumulierte Zeitraum-Reste“ mit dem Wert **Ja**). |

Unterhalb der Datentabelle wird die aktuelle Gesamtsumme sowie die aktuelle Restsumme ausgewiesen.  
Die bei Aufruf des Kontrakts im Änderungsmodus sowie im Neu-Anlagefall beim ersten Speichern vorhandene Gesamt- und Restsumme wird ebenfalls ausgewiesen wie auch die Differenz zu den ursprünglichen Werten. So ist bei Umverteilungen von Mengen-/Werten eine Kontrolle über noch nicht oder zu viel verteilte Werte möglich.

Mit der Funktion Ändern F5 und Neu F8 kann ein Zeitraum geändert beziehungsweise ein neuer Zeitraum angelegt werden. Entsprechend wird der markierte Zeitraum mit der Funktion Löschen F7 nach einer Kontrollabfrage gelöscht, dabei werden dann das End-Datum des vorhergehenden beziehungsweise das Beginn-Datum des folgenden Zeitraums angepasst.

Die Funktion Löschen Kontraktdaten der Maske dient zum Löschen von Zeiträumen, deren Grenzen außerhalb der Gültigkeit des Kontrakts liegen.

Die Funktion Löschen bis Ende der Maske dient zum Löschen der Zeiträume vom markierten bis zum letzten Zeitraum. Dabei werden die Zeitraumgrenzen des vorhergehenden Zeitraums angepasst.

Die Funktion Löschen bis Markierung der Maske dient zum Löschen der Zeiträume vom ersten bis zum markierten Zeitraum. Dabei werden die Zeitraumgrenzen des nachfolgenden Zeitraums angepasst.

Die zweite Datentabelle dient der Darstellung der Kontrakt-Artikel. Bei Einzelmengen- oder Einzelwert-Kontrakten können hier die Sollmengen beziehungsweise Sollwerte der Artikel des in der ersten Datentabelle gewählten Zeitraums geändert werden.  
Bei Gesamtmengen-Kontrakten beziehungsweise Gesamtwert-Kontrakten wird hier bei jedem Kontraktartikel die Zeitraumsollmenge beziehungsweise der Zeitraumsollwert des Kontraktzeitraums dargestellt. Die Übersicht dient dann eher zur Orientierung darüber, welche Kontraktartikel dem Kontrakt zugewiesen sind  
Bei Freimengen-Kontrakten werden Sollmengen in beiden Datentabellen immer mit 0 dargestellt.

| Feld | Beschreibung |
| --- | --- |
| Zeitraum | Beginn des Kontraktmengen-Zeitraums |
| Artikelnummer | Artikelnummer des Artikels der Kontraktartikelposition. |
| Lager | Artikelnummer des Artikels der Kontraktartikelposition  
(Nur bei eingestelltem Wert des [Steuerungsparameter](../../firmenstamm/steuerparameter/kontraktwesen/ratierliche_einstellungen_spa_846.md)s 44 „Variante Kontrakt-Auswahl“ mit dem Wert **mit Lager**). |
| Gesamtmenge | Nur bei Mengen-Kontrakt:  
\- Sollmenge des Kontrakt-Zeitraums des Artikels bei Einzelmengen-Kontrakt,  
\- Sollmenge des Kontrakt-Zeitraums bei Gesamtmengen-Kontrakt  
\- immer 0 bei Freimengen-Kontrakt. |
| Tender | Nur bei Mengen-Kontrakt der Kontraktklasse *Verkaufskontrakt Hedge (4)* oder *Einkaufskontrakt Hedge (14)*:  
Tendermenge des Kontrakt-Zeitraums des Artikels im Kontrakt-Zeitraum. |
| Restmenge | Nur bei Mengen-Kontrakt:  
\- Aktuelle Restmenge des Kontrakt-Zeitraums des Artikels bei Einzelmengen-Kontrakt,  
\- Aktuelle Restmenge des Kontrakt-Zeitraums bei Gesamtmengen-Kontrakt  
\- immer 0 bei Freimengen-Kontrakt. |
| Gesamtwert | Nur bei Wert-Kontrakt:  
\- Sollwert des Kontrakt-Zeitraums des Artikels bei Einzelwert-Kontrakt  
\- Sollwert des Kontrakt-Zeitraums bei Gesamtwert-Kontrakt  
\- immer 0 bei Freiwert-Kontrakt. |
| Restwert | Nur bei Wert-Kontrakt:  
\- Aktueller Restwert des Kontrakt-Zeitraums des Artikels bei Einzelwert-Kontrakt,  
\- Aktueller Restwert des Kontrakt-Zeitraums bei Gesamtwert-Kontrakt  
\- immer 0 bei Freiwert-Kontrakt. |
| Bezeichnung | Bezeichnung des Kontrakt-Artikels. |

Unterhalb der Datentabelle wird die aktuelle Gesamtsumme sowie die aktuelle Restsumme der Artikel im ausgewiesenen Zeitraum dargestellt.  
Die bei Aufruf des Kontrakts im Änderungsmodus sowie im Neu-Anlagefall beim ersten Speichern ermittelte Gesamt- und Restsumme wird ebenfalls ausgewiesen wie auch die Differenz zu den ursprünglichen Werten. So ist bei Umverteilungen von Mengen-/Werten eine Kontrolle über noch nicht oder zu viel verteilte Werte möglich.
