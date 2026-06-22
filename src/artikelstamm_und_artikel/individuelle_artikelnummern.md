# Individuelle Artikelnummern

<!-- source: https://amic.de/hilfe/_artiindivnummer.htm -->

Hauptmenü > Stammdatenpflege > Artikelstamm > Artikelstamm

oder Direktsprung [ARSI]

In der Anwendung „Individuelle Artikelnummern“ kann man zu einem Kunden/Lieferanten einen oder mehreren Hersteller eines Artikels auswählen.

Dem ausgewählten Artikel können einem Kunden/Lieferanten einige abweichende Daten zugewiesen werden, neben einer individuellen Artikelnummer zum Beispiel auch die für den Artikel im Standard festgelegten Stoffstromanteile und einer von der Default-Mengeneinheit des Artikels abweichenden Mengeneinheit zur Vorbelegung bei der Warenpositionserfassung in der Vorgangserfassung und Vorgangskorrektur inklusive individueller Gebindefaktoren bei Gebindemengeneinheiten.

<details>
<summary>Tabreiterübergreifende Felder</summary>

Es gibt folgende Felder:

| Feld | Beschreibung |
| --- | --- |
| Kunde/Lieferant | Kunden/Lieferantennummer des Datensatzes. Mittels F3 ist eine Auswahlhilfe verfügbar. |
| Artikel-Stamm | Artikel-Stammnummer des Artikels. Mittels F3 ist eine Auswahlhilfe verfügbar. |

Sofern ein Artikel Stoffstromwerte hinterlegt hat, ist der Tabreiter Lieferanten-Stoffstromdaten aktiv.

</details>

<details>
<summary>Artikel</summary>

Es gibt folgende Felder:

| Feld | Beschreibung |
| --- | --- |
| Artikelfremdnummer | Nummer des Artikels beim Kunden bzw. Lieferanten |
| Artikel EAN | Spezielle EAN-Nummer des Artikels beim Kunden bzw. Lieferanten |
| Edi gültig ab | Gültigkeitsbeginn des Artikels im EDI-Bereich |
| Edi gültig bis | Gültigkeitsende des Artikels im EDI-Bereich |
| Hersteller | Ja/Nein Feld. Zeigt an, ob für den Artikel ein Hersteller existiert. |
| Hauptlieferant | Ja/Nein Feld. Zeigt an, ob der ausgewählte Kunde/Lieferant ein Hauptlieferant ist. |
| Bestellsperre | Ja/Nein Feld. Zeigt an, ob für den Artikel eine Bestellsperre existiert. |
| Bestellgröße | Menge einer Liefereinheit (lediglich informativ) |
| Mengeneinheit | Optionale Mengeneinheit zur Vorbelegung bei der Erfassung einer Warenpositionen mit diesem Artikel bei der Vorgangserfassung und Vorgangskorrektur. Diese Option ist gilt nur für Vorgangsklassen und Vorgangsunterklassen, für die diese in der Anwendung ‚Formularzuordnung/Vorgangsunterklasse‘ auf der Registerkarte im Register ‚Eingabe‘ erlaubt sind.  
HINWEIS: Bei Folgeartikeln, Komponenten von Handelsstücklisten sowie Produktions- und Rohwarevorgängen ist diese Option nicht wirksam. |
| Individuelle ME-Bez | Wird eine optionale Mengeneinheit in eine Warenposition übernommen (siehe oben), so wird die Mengeneinheits-Bezeichnung beziehungsweise die Gebindebezeichnung durch einen hier optional eingetragenen Begriff ersetzt. |
| Gebindefaktor 1 | Ist hier ein Wert eingetragen (nur möglich, wenn die o.a. Mengeneinheit eine Gebindemengeneinheit ist), so wird bei der Mengeneinheitsvorbelegung (siehe oben) zusätzlich auf diesen Gebindefaktor anstelle des zum Artikel für die Mengeneinheit definierten Gebindefaktors zurückgegriffen  
Das Änderbarkeits-Kennzeichen für den Faktor kann hier ebenfalls mit dem Wert ‚Ja‘ oder ‚Nein‘ überschrieben werden. Bei leeren Feldern wird der Originalwert aus Gebindemengeneinheit, Artikelstamm-Gebindefaktoren oder Artikel-Gebindefaktoren beibehalten. |
| Gebindefaktor 2 | Ist hier ein Wert eingetragen (nur möglich, wenn die o.a. Mengeneinheit eine Gebindemengeneinheit ist), so wird bei der Mengeneinheitsvorbelegung (siehe oben) zusätzlich auf diesen Gebindefaktor anstelle des zum Artikel für die Mengeneinheit definierten Gebindefaktors zurückgegriffen  
Das Änderbarkeits-Kennzeichen für den Faktor kann hier ebenfalls mit dem Wert ‚Ja‘ oder ‚Nein‘ überschrieben werden. Bei leeren Feldern wird der Originalwert aus Gebindemengeneinheit, Artikelstamm-Gebindefaktoren oder Artikel-Gebindefaktoren beibehalten. |
| Gebindefaktor 3 | Ist hier ein Wert eingetragen (nur möglich, wenn die o.a. Mengeneinheit eine Gebindemengeneinheit ist), so wird bei der Mengeneinheitsvorbelegung (siehe oben) zusätzlich auf diesen Gebindefaktor anstelle des zum Artikel für die Mengeneinheit definierten Gebindefaktors zurückgegriffen  
Das Änderbarkeits-Kennzeichen für den Faktor kann hier ebenfalls mit dem Wert ‚Ja‘ oder ‚Nein‘ überschrieben werden. Bei leeren Feldern wird der Originalwert aus Gebindemengeneinheit, Artikelstamm-Gebindefaktoren oder Artikel-Gebindefaktoren beibehalten. |
| Gebindefaktor 4 | Ist hier ein Wert eingetragen (nur möglich, wenn die o.a. Mengeneinheit eine Gebindemengeneinheit ist), so wird bei der Mengeneinheitsvorbelegung (siehe oben) zusätzlich auf diesen Gebindefaktor anstelle des zum Artikel für die Mengeneinheit definierten Gebindefaktors zurückgegriffen  
Das Änderbarkeits-Kennzeichen für den Faktor kann hier ebenfalls mit dem Wert ‚Ja‘ oder ‚Nein‘ überschrieben werden. Bei leeren Feldern wird der Originalwert aus Gebindemengeneinheit, Artikelstamm-Gebindefaktoren oder Artikel-Gebindefaktoren beibehalten. |
| Text 1 | Möglichkeit Notizen oder ähnliches in Text 1 zu speichern. |
| Text 2 | Möglichkeit Notizen oder ähnliches in Text 2 zu speichern. |
| Text 3 | Möglichkeit Notizen oder ähnliches in Text 3 zu speichern. |

</details>

<details>
<summary>Hersteller</summary>

Es gib folgende Felder:

| Feld | Beschreibung |
| --- | --- |
| Nummer | Nummer des Herstellers. Mittels F3 ist eine Auswahlhilfe verfügbar. |
| Bezeichnung | Bezeichnung des Herstellers. Wird durch F3 in Nummer gefüllt. |
| Kürzel | Kürzel des Herstellers. Mittels F3 ist eine Auswahlhilfe verfügbar. |
| Sperrkennzeichen | Sperrkennzeichen des Herstellers. Mittels F3 ist eine Auswahlhilfe verfügbar. Beschreibt, ob ein Hersteller derzeitig für den Artikel gesperrt ist. |

Um Hersteller eintragen zu können, muss man vorher mittels Direktsprung [HST] die Anwendung Hersteller aufrufen und mit F8 einen Hersteller anlegen.

</details>

<details>
<summary>Lieferanten-Stoffstrom-Daten</summary>

Es gib folgende Felder:

| Feld | Beschreibung |
| --- | --- |
| Nr. | Nummer des Bestandteils. Mittels F3 ist eine Auswahlhilfe verfügbar. |
| Bezeichnung | Bezeichnung des Bestandteiles. Wird durch F3 in Nr. gefüllt. |
| Stoffstromart | Stoffstromart des Bestandteiles. Wird durch F3 in Nr. gefüllt. |
| Anteil | Anteil des Bestandteiles. Ist vorbelegt mit dem Wert aus dem Zusammensetzungsgrid. |
| in | Mengeneinheit des Bestandteiles. Ist vorbelegt mit dem Wert aus dem Zusammensetzungsgrid. Mittels F3 ist eine Auswahlhilfe verfügbar. |
| Mengeneinheit | Mengeneinheitsbezeichnung der Bezeichnung. |

Um das Grid auf dem Lieferanten-Stoffstrom-Daten Tabreiter ausfüllen zu können, müssen für den Artikel Bestandteile in der Zusammensetzung eingerichtet werden.

Dafür öffnet man mittels Direktsprung [ARS] die Variante Artikelstamm. Dort filtert man mit F2 nach dem Artikel und öffnet diesen mit F5.

In der Optionbox des Artikels öffnet man mit F2 die Zusammensetzungsmaske. Dort trägt man nach Bedarf in das Zusammensetzungsgrid Bestandteile ein.

Im Feld Nummer werden mit F3 Bestandteile ausgewählt, welche eine Stoffstrom-Art haben [Siehe Bestandteile](./konstanten_der_artikelverwaltung/bestandteile.md).

Außerdem trägt man in das Feld Anteil den Anteil des Bestandteiles in der Zusammensetzung und in das Feld in die dazugehöriger Mengeneinheit ein.

</details>
