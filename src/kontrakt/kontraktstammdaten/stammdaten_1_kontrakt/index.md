# Stammdaten 1 (Kontrakt)

<!-- source: https://amic.de/hilfe/_stammdatenKontrakt.htm -->

Der Programmteil Kontraktverwaltung ist zuständig für die Eingabe und Bearbeitung aller Kontraktstamminformationen (Kunde, Termine, etc.), der Warenpositionen und Warenbewegungen. In die Erfassungsmaske gelangt man über den bekannten Auswahlbildschirm. In dieser Erfassungsmaske sind alle Stammdateninformationen auf einer Bildschirmseite zusammengefasst. Bearbeitet werden in dieser Erfassungsmaske alle Kontraktklassen; es stehen also prinzipiell alle Funktionen sowohl dem Ein- als dem Verkauf zur Verfügung.

Rechts oben am Bildschirm wird mit der Anwahl ein Funktionsauswahlfenster geöffnet, dessen Inhalte jedoch vom Bediener, von der Kontraktklasse und der Kontraktgruppe (siehe später) abhängen. Deshalb werden diese Funktionen erst ab der Position „Einzel-/Gesamtmengen“ zur Verfügung gestellt.

| Stammdaten 1 |
| --- |
| Hinweisfeld:  
Aktiv  
 | Hier werden alle Kontrakte insofern unterschieden, dass aktive und archivierte separat geführt werden, so dass man bei sehr vielen archivierten Kontrakten nicht immer alle diese überlesen muss, um die aktiven zu finden. Bei der Neuanlage ist hier keine Eingabe erforderlich. |
| Hinweisfeld:  
Kein Artikel zugeordnet | Sollten dem Kontrakt keine Artikel zugeordnet sein, erscheint ein roter Text oben rechts auf der Registerkarte. |
| Mengen-/Wertkontrakt | Ein Kontrakt kann als Mengen oder als Wertkontrakt gehandelt werden. |
| Matchcode | Eingabe des gewünschten Matchcodes. |
| [Standard – Kontrakt - Variante](./standard_kontrakt_variante.md) | Kennzeichen, ob es sich um einen „Standardkontrakt“ handelt, und ggf. um welchen Typ: |
| Dispositionskennzeichen  
 | Identifikation des Dispositionsmerkmals, das die Gegenüberstellung („Auszifferung“) von Ein- und Verkaufskontrakten oder deren Teilpartien ermöglicht.  
Mittels eines Steuerungsparameters kann aktiviert werden, dass beim Erfassen von Einkaufskontrakten automatisch Dispo-Kennzeichen mit identischer Nummerierung erzeugt werden, wenn das dann entsprechend vorbelegte Kennzeichen nicht abweichend überschrieben wird. Die Kontraktbezeichnung wird mit übernommen.  
Somit ist es möglich, sich die Arbeit zu erleichtern, wenn generell jeder Einkaufskontrakt einer Anzahl von Verkaufskontrakten zugeordnet werden soll, denn die Zuordnung erfolgt direkt mit der Kontraktnummer des Einkaufskontrakts. |
| Referenz-Nummer  
 | Kontrakt-Identifikation auf der jeweils anderen Seite (Kontraktnummer beim Lieferanten etc.). Die Referenz-Nummer kann entsprechenden Analysen zugrunde gelegt werden oder im Schriftverkehr mit Kunden/ Lieferanten (“unsere Nummer”, “Ihre Nummer”) eingesetzt werden. |
| Mengeneinheit | Die Mengeneinheit in der dieser Kontrakt abgewickelt wird.  
Bei der Preiseingabe muss auf diese Bezugsgröße geachtet werden. In der Standardeinrichtung (siehe SPA) wird bei der Artikeleingabe geprüft, ob die Mengeneinheiten übereinstimmen, ansonsten wird die Eintragung abgelehnt. |
| Nachkommastellen Mengen  
 | Hier wird festgehalten, mit wie vielen Nachkommastellen die Mengenverwaltung arbeiten soll. |
| Versandanschrift | Hier kann eine Versandanschrift hinterlegt werden. Wird dieser Kontrakt in einem Beleg gezogen, so wird die Versandanschrift des Beleges durch die des Kontrakts ersetzt.  
Es kann an dieser Stelle per F3-Auswahl eine beliebige Kunden-/Lieferanten-Versandanschrift oder eine Objektanschrift (Baustellenanschrift) gewählt werden. Außerdem kann hier eine neue Versandanschrift angelegt und gewählt werden, die dann als Kunden-/Lieferanten-Versandanschrift dem Hauptkunden der dem Kontrakt zugeordneten Kontraktgruppe zugeordnet wird. Soll eine hier zugeordnete Anschrift geändert werde, so kann dieses nur im jeweiligen Pflegemodul des Objekts (Baustelle) beziehungsweise des Kunden oder Lieferanten erfolgen.   
Bei mehreren gezogenen Kontrakten innerhalb eines Beleges wird immer die Versandanschrift des letzten Kontrakts verwendet. |
| Kontraktdatum | Kontrakterstellungsdatum, Abschlussdatum des Kontraktes, Tag des Abschlusses |
| Laufzeit ab | Beginn der Gültigkeit |
| Laufzeit bis | Dauer der Gültigkeit |
| Laufzeit bis maximal | Maximale terminliche Überschreitung des Kontraktes, die Überschreitung in Tagen wird errechnet. Wenn keine Überschreitungsmöglichkeit besteht, wird mit Ablauf der Laufzeit der Kontrakt für Buchungen automatisch gesperrt. Mit Eingabe der Maximallaufzeit werden die zulässigen Übertage ausgewiesen. Mit CTRL+F1 kann auch direkt in das Übertagefeld gesprungen werden; aus dieser Eingabe wird dann das Datum zurückgerechnet.  
Innerhalb des Überschreitungszeitraumes bleiben die vereinbarten Bedingungen erhalten. |
| Report Zuschlag | Auf den vereinbarten Kontraktpreis wird ein Zuschlag je Zeiteinheit in Tagen beginnend ab einem festzulegenden Datum erhoben. Der Kontraktpreis erhöht sich also rhythmisch um diesen Betrag. |
| Überziehungszuschlag | Bei Überziehung des Kontraktes (s.o.) kann je Lieferung ein Überziehungszuschlag in Anrechnung gebracht werden. Dieser wird hier eingetragen. |
| Maximale Vorauszeiträume  
. | Wenn ein Kontrakt abgeschlossen wird, in dem Teilmengen für einzelne Zeiträume innerhalb der Gesamtlaufzeit werden, dann kann hier bestimmt werden, ob und für wie viele Zeiträume im Voraus aus diesen zukünftigen Perioden abgebucht werden darf.  
Wenn also im Oktober bereits auf die Dezembermenge zugegriffen werden darf, so ist hier 2 einzugeben |
| Bemerkung | Hier kann eine Bemerkung zum Kontrakt eingetragen werden. |

 

<p class="siehe-auch">Siehe auch:</p>

- [Einzel-/Gesamtmengen](./einzel_gesamtmengen.md)
- [Standard-Kontrakt-Variante](./standard_kontrakt_variante.md)
- [Restmenge über Gesamtzeit](./restmenge_ueber_gesamtzeit.md)
