# Kontrakt „Washout and Circle“

<!-- source: https://amic.de/hilfe/_kontraktwashout.htm -->

Hauptmenü > Kontraktverwaltung > Kontrakt Stammdaten

oder den Direktsprung **[KTR]**

<p class="just-emphasize">Allgemein</p>

Ein Washout oder Circle Geschäft wird in der Kontraktverwaltung durchgeführt. Washout und Circle Geschäfte funktionieren nur mit Kontrakten, die den Typ Einkauf und Verkauf besitzen. Um ein Washout oder ein Circle durchzuführen müssen diverse Voraussetzungen erfüllt sein. Bei dieser Geschäftsart wird keine Ware zwischen den Geschäftspartnern bewegt sondern nur der Rest Warenwert. Der Rest Warenwert ermittelt sich aus der Differenz der Kontraktwerte über die fiktiv bewegte Menge. Diese Geschäfte sind in Deutschland steuerfrei.

<p class="just-emphasize">Washout</p>

Was ist ein Washout:

Existieren für einen Kunden ein Einkaufskontrakt und ein Verkaufskontrakt über den gleichen Artikel, so kann ein Washout durchgeführt werden. Die Washout Menge ist entweder die gesamte Kontraktrestmenge, oder die Kontraktrestmenge meines aktuellen Zeitraumes je nach Einstellung meines [Einrichterparameters](../../firmenstamm/einrichterparameter/washout_und_circle_epa_kontraktstamm_washout_circle.md) oder eine manuell Eingegebene Menge.

Wenn keine Kontokorrenten Kunden verwendet werden sondern nur Kreditoren und Debitoren so muss der [Einrichterparameter](../../firmenstamm/einrichterparameter/washout_und_circle_epa_kontraktstamm_washout_circle.md) „Unterschiedliche Kunden bei Washout“ auf „Ja“ gesetzt werden.

<p class="just-emphasize">Circle</p>

Was ist ein Circle:

Ein Circle Geschäft kann dann durchgeführt werden, wenn ein Lieferant und ein Kunde mit einer dritten Firma ein Kontrakt über den gleichen Artikel abgeschlossen haben.

| | Dritt Firma | |
| --- | --- | --- |
| Verkaufskontrakt | | Einkaufskontrakt |
| | | |
| | | |
| Lieferant | | Kunde |
| | | |
| | | |
| Einkaufskontrakt | | Verkaufskontrakt |
| | Meine Firma | |

Hierbei ist festzuhalten, dass nicht mehr der eigentliche Kontraktpreis gültig ist, sondern der kleinste bezahlte Kontraktpreis in der Kette. Dieser ist beim Circle Geschäft einzutragen.

Die Circle Menge ist entweder die gesamte Kontraktrestmenge, oder die Kontraktrestmenge meines aktuellen Zeitraumes je nach Einstellung meines [Einrichterparameters](../../firmenstamm/einrichterparameter/washout_und_circle_epa_kontraktstamm_washout_circle.md) oder eine manuell Eingegebene Menge.

Bei dem Circle Geschäft werden zwei Finale Belege erzeugt. Einmal für den Einkauf und einmal für den Verkauf.

Die Besonderheit beim Einkauf ist:

Wenn der Lieferant nach dem Circle Geschäft Geld von mir bekommt, dann muss ich mir eine Eingangsrechnung schreiben.

Wenn der Lieferant nach dem Circle Geschäft mir noch Geld zu überweisen hat, so schreibe ich mir eine Eingangsgutschrift.

<p class="just-emphasize">Speichern des Circlepreises im Warenbewegungsaddon</p>

Der Circlepreis wird im Warenbewegungsaddon für alle vier Beleg gespeichert, wenn die dazu gehörigen Felder im Warenbewegungsaddon angelegt worden sind.

Die Feldnamen für das Warenbewegungsaddon sind festvorgegeben und können nicht geändert werden.

| Warenbewegungsfeldname | Bedeutung |
| --- | --- |
| CircleMinPreis | In diesem Feld wird der Preis des Circlegeschäfts hinterlegt |
| CircleMinPreisEinh | In diesem Feld wird die Preiseinheit hinterlegt.<br>z.B. Preis \* 1,5 pro Kilogramm |
| CircleMinPreisME | In diesem Feld wird die Preismengeneinheit hinterlegt. |

<p class="just-emphasize">Einrichtung in Aeins</p>

Um ein Washout oder Circle Geschäft durchzuführen müssen noch Einstellungen vorgenommen werden.

Es muss eine [Steuerklasse](../../finanzbuchhaltung/umsatzsteuer/steuersaetze_einrichten/stammdaten_steuerklassen.md) angelegt werden, die den Steuersatz 0 enthält.

Für Washout und Circle wurden dreizehn Steuerparameter eingestellt.

| Steuerparameter | Bedeutung |
| --- | --- |
| 150 | Mit diesem Steuerparameter kann eingestellt werden, ob ein Kontrakt vor der Gültigkeit des Kontraktes bebucht werden kann. |
| 643 | Hier muss eine Steuerklasse hinterlegt werden die den Steuersatz 0 enthält |
| 644 | Eine spezielle Vorgangsunterklasse für die Eingangsrechnung, wird keine Vorgangsunterklasse hinterlegt wird die Vorgansgunterklasse 0 gewählt. |
| 645 | Eine spezielle Vorgangsunterklasse für die Ausgangrechnung, wird keine Vorgangsunterklasse hinterlegt wird die Vorgansgunterklasse 0 gewählt. |
| 646 | Eine spezielle Vorgangsunterklasse für die Endgutschrift, wird keine Vorgangsunterklasse hinterlegt wird die Vorgansgunterklasse 0 gewählt. |
| 647 | Eine spezielle Vorgangsunterklasse für die Endrechnung, wird keine Vorgangsunterklasse hinterlegt wird die Vorgansgunterklasse 0 gewählt. |
| 648 | Vorbelegung der Mengeneinheit für den Preis beim Circle Geschäft |
| 816 | Mit diesem Steuerparameter kann eingestellt werden, welche Zahlungsbedingung für den Kunden beim Washout gilt. |
| 817 | Mit diesem Steuerparameter kann eingestellt werden, welche Zahlungsbedingung für den Lieferanten beim Washout gilt. |
| 818 | Mit diesem Steuerparameter kann eingestellt werden, welche Zahlungsbedingung für den Kunden beim Circle gilt. |
| 819 | Mit diesem Steuerparameter kann eingestellt werden, welche Zahlungsbedingung für den Lieferanten beim Circle gilt. |
| 821 | Mit diesem Steuerparameter kann eingestellt werden, ob mit Preis \* Preiseinheit in der Abschluss Rechnung/Gutschrift gerechnet werden soll, oder ob der Differenz Betrag gebucht werden soll.<br> |
| 837 | Anhand dieses Steuerparameters kann eingestellt werden, ob alle Washout oder Circle Vorgänge über ein bestimmtes Lager laufen sollen.<br>Wenn Parameter „Bestimmtes Lager verwenden“ auf „Ja“ gestellt wird, so muss in das Feld Lagernummer das Lager eingetragen werden, auf den die Washout und Circle Vorgänge gebucht werden sollen.<br>Wird der Parameter auf „Nein“ gestellt, so wird das Lager des jeweiligen Kontraktes gezogen. |

<p class="just-emphasize">Voraussetzungen</p>

1. Es dürfen nur insgesamt zwei Kontrakte ausgewählt werden, einen Einkaufskontrakt sowie einen Verkaufskontrakt.

2. Die Kontrakte müssen jeweils über den gleichen Artikel abgeschlossen worden sein.

3. Die Kontraktrestmenge darf nicht null sein.

4. Ist der [Einrichterparameter](../../firmenstamm/einrichterparameter/washout_und_circle_epa_kontraktstamm_washout_circle.md) „Es darf nur der aktuelle Mengenzeitraum betrachtet werden“ auf „Ja“ eingestellt, so darf die Zeitraumrestmenge nicht null sein. Beide Kontrakte müssen über einen aktuellen Mengenzeitraum verfügen.

5. Das ab Datum des Kontraktes darf nicht unterschritten werden.

6. Das maximale Datum des Kontraktes darf nicht überschritten werden.

7. Für Washout dürfen nur gleiche Kontraktkunden verwendet werden, es sei denn der [Einrichterparamter](../../firmenstamm/einrichterparameter/washout_und_circle_epa_kontraktstamm_washout_circle.md) „Die Kundennummer darf bei einem Washout unterschiedlich sein“ ist gesetzt worden.

8. Für Circle dürfen nur ungleiche Kontraktkunden verwendet werden

9. Wenn der Steuerparameter 837 auf „Ja“ steht, so müssen die Kontrakte als Lagerübergreifend angelegt worden sein. Des Weiteren muss der Kontraktartikel auch auf diesem Lager vorhanden sein.

10. Beide Kontrakte müssen die gleiche Währung haben, ansonsten kann kein Washout oder Circle durchgeführt werden.

11. Es werden keine Bruttorechnungen bei Fremdwährungsbelegen erstellt, auch wenn bei den beteiligten Kunden / Lieferanten das Kennzeichen [Bruttorechnung](../../kunden_und_lieferanten/kunden_und_lieferantenstamm/kennzeichen/index.md) im [Kunden-/Lieferantenstamm](../../kunden_und_lieferanten/uebersicht_kunden_und_lieferanten.md) auf „Ja“ steht.

12. Die Belege werden in der Kontraktwährung erstellt.

<p class="just-emphasize">Ablauf</p>

In der Kontraktverwaltung werden zwei Kontrakte je ein Einkaufskontrakt und Verkaufskontrakt über den gleichen Artikel ausgewählt. Jetzt kann entschieden werden ob ein ***Washout*** **Strg+F11** oder ein ***Circle*** **Shift+F11** durchgeführt werden soll.

Als nächstes öffnet sich die Maske die alle wichtigen Informationen zu den beiden Kontrakten darstellt.

Hier besteht die Möglichkeit, die Menge manuell zu erfassen mit ***„Menge überschreiben“*** **F5** oder es wird je nach Einstellung die Kontraktrestmenge der gesamten Laufzeit oder des aktuellen Zeitraums gewählt. Bei dem Circle Geschäft muss noch der Preis eingetragen werden.

Wird dann ein Washout mit **F9** oder Circle **SF9** gestartet passiert folgendes: 

Es werden automatisch zwei technische Belege für den Wareneinkauf sowie für den Warenverkauf mit einem Wertartikel über den Wert 0 erstellt. Diese Belege verringern die Menge der Kontrakte.

Abhängig von dem Restbetrag wird automatisch eine Gutschrift oder eine Rechnung an den Kunden oder an den Lieferanten beim Washout geschrieben.

Beim Circle wird in Abhängigkeit des Restbetrages automatisch eine Gutschrift oder eine Rechnung an den Kunden und an den Lieferanten geschrieben. 

Damit der Bewertungspreis des Artikels nicht kaputt gemacht wird, werden diese Belege mit dem Artikel als Wertartikel erzeugt.

<p class="just-emphasize">Kontraktmenge</p>

Bei einem Washout Circle Geschäft bestimmt der Kontrakt mit der kleinsten Restmenge, die Menge die für Washout oder Circle benutzt wird.

Mit der Funktion „***Menge überschreiben***“ **F5** kann eine diese Menge verändert werden, diese manuell eingegeben Menge wird dann für das Washout/Circle Geschäft benutzt.

Achtung mit der Funktion kann auch ein Kontrakt überbucht werden.

<p class="just-emphasize">Kontraktpreis</p>

Der Kontraktpreis berechnet sich wie folgt für Washout. Ist ein Einkaufskontrakt abgeschlossen worden über 1500 kg mit einem Preis von 1,50 € und ein Verkaufskontrakt über 1300 kg mit einem Preis von 1,30 € so errechnet sich ein Netto Warenwert von:

 1300 kg \* 1,5 € = 1950 €

 1300 kg \* 1,3 € = 1690 €

 260 €

Der Restwarenwert wird ohne Steuersätze berechnet.

Bei dem Circle Geschäft wird der kleinste Preis für die Ware manuell eingetragen.

<p class="siehe-auch">Siehe auch:</p>

- [Neues Kennzeichen in der Warenbewegung](./neues_kennzeichen_in_der_warenbewegung.md)
- [Variante Kontraktbewegung mit Washout und Circle](./variante_kontraktbewegung_mit_washout_und_circle.md)
