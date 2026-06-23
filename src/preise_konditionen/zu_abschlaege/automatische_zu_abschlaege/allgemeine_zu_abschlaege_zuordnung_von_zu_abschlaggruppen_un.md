# Allgemeine Zu-/Abschläge (Zuordnung von Zu-/Abschlaggruppen und Zu-/Abschlagklassen)

<!-- source: https://amic.de/hilfe/_ZuAbschlaege_ZAVA.htm -->

Preise / Konditionen > Zu-/Abschläge > allgemeine Zu-/Abschläge

Oder Direktsprung **[ZAVA]**

In der Kombination von Kunden und Artikeln entstehen automatische Zu-/Abschlagsberechnungen. Diese Zuordnung erfolgt in dieser Anwendung. Für Einkauf und Verkauf getrennt können hier Zu-/Abschläge für die Kombination von [Zu-/Abschlagklasse](./zu_abschlaggruppe.md) und [Zu-/Abschlagklasse](./zu_abschlagklasse.md) eingerichtet werden.

Der Pfleger ermöglicht die Erfassung eines oder mehrerer Zu-/Abschläge, die in einer definierten Rangfolge eingetragen werden können.

So könnte z.B. grundsätzlich ein Zu-/Abschlag von 2% gegeben werden, jedoch vorrangig ein Zu-/Abschlag ab einem Einkaufswert von 100€ (für die Artikelgruppe) ein Zu-/Abschlag von 5% gelten.

<p class="just-emphasize">Rang</p>

Rangfolge in der dieser Zu-/Abschlag zu berücksichtigen ist. Ein Zu-/Abschlag, der bereits gegebene Zu-/Abschlag berücksichtigt, sollte nicht an oberster Stelle stehen, da andere Zu-/Abschläge bei seiner Berechnung noch nicht existieren.

<p class="just-emphasize">Text-Nr.</p>

Hier kann ein Text aus den [Zu-/Abschlagtexten](./zu_abschlag_texte.md) gewählt werden

<p class="just-emphasize">Prfkt.</p>

Preisfaktor (Anzahl der Mengeneinheiten) für Zu-Abschläge, die nicht prozentual berechnet werden. So kann z.B. ein Zu-Abschlag pro 2 oder 10 Stück(ME) gegeben werden.

<p class="just-emphasize">EKZ-Nr.</p>

Erlöskennziffer des Zu-Abschlags. (0 = wie Artikel) siehe auch „kalk“

<p class="just-emphasize">ZuAb-Art</p>

Wählen Sie hier aus, aus welchem Bereich Ihr Zu-/Abschlag kommen soll.

Zur Auswahl stehen [Generelle Zu-/Abschläge](./generelle_zu_abschlaege.md), [Bezugsgrößenabhängige Zu-/Abschläge](./bezugsgroessenabhaengige_zu_abschlaege.md), [Versandartabhängige Zu-/Abschläge](./versandartabhaengige_zu_abschlaege.md), [Zahlungsartabhängige Zu-/Abschläge](./zahlungsartabhaengige_zu_abschlaege.md).

<p class="just-emphasize">ZuAb-Tab.</p>

Hier wählen Sie einen der Sätze aus, der gelten soll.

<p class="just-emphasize">InZl</p>

Ja/Nein-Entscheidung, ob dieser Zu-Abschlag auf dem Ausdruck unterdrückt werden soll (JA), weil er lediglich zur internen Preisermittlung dient oder dem Belegempfänger sichtbar ausgedruckt werden soll (Nein).

<p class="just-emphasize">GrpR</p>

| Einstellung | Bedeutung |
| --- | --- |
| Zeile | Zeilenrabatt – wirkt auf eine Warenposition |
| Gruppe | Gruppenrabatt – wirkt auf alle Artikel dieser Warengruppe |
| Preis | Dieser Rabatt wirkt zunächst auf den Einzelpreis, bevor dieser mit der Menge multipliziert wird. |

<p class="just-emphasize">kalk</p>

Ja/Nein-Entscheidung, ob dieser Zu-Abschlag ein kalkulatorischer Zu-Abschlag sein soll, der als Teil des Preises berechnet und nicht gesondert ausgewiesen werden soll.

Ein kalkulatorischer Zu-Abschlag wird wie ein InZeile- Zu-Abschlag nicht auf dem Beleg ausgegeben. Ein kalkulatorischer Zu-Abschlag wird jedoch nicht zur Berechnung des Betrages berechnet. Der kalkulatorische Zu-Abschlag sorgt dafür, dass dieser „Zu-Abschlag -Anteil“ abweichend von der Erlöskennziffer des Artikels auf eine andere Erlöskennziffer gebucht wird.

Ein Beispiel: Es wird Kaffee für 10€ verkauft, der vor dem Verkauf gemahlen wird. 0,50€ sind Kosten, die bei jedem Kilo anfallen. Der Erlös von 9,50€ soll auf das Erlöskonto Kaffee und 0,50€ auf das Erlöskonto Kaffeemühle gebucht werden.

<p class="just-emphasize">Sp.</p>

Ja/Nein-Entscheidung, ob dieser Zu-Abschlag vorübergehend gesperrt sein soll.
