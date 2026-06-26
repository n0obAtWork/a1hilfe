# Allgemeine Rabatte (Zuordnung von Rabattgruppen und Rabattklassen)

<!-- source: https://amic.de/hilfe/_Rabatte_RAV.htm -->

Preise / Konditionen > Rabatte > allgemeine Rabatte

Oder Direktsprung **[RAV]**

In der Kombination von Kunden und Artikeln entstehen automatische Rabattberechnungen. Diese Zuordnung erfolgt in dieser Anwendung. Für Einkauf und Verkauf getrennt können hier Rabatte für die Kombination von [Rabattklasse](./rabattklasse.md) und [Rabattgruppe](./rabattgruppe.md) eingerichtet werden.

Der Pfleger ermöglicht die Erfassung eines oder mehrerer Rabatte, die in einer definierten Rangfolge eingetragen werden können.

So könnte z.B. grundsätzlich ein Rabatt von 2% gegeben werden, jedoch vorrangig ein Rabatt ab einem Einkaufswert von 100€ (für die Artikelgruppe) ein Rabatt von 5% gelten.

#### Rang

Rangfolge in der dieser Rabatt zu berücksichtigen ist. Ein Rabatt, der bereits gegebene Rabatte berücksichtigt, sollte nicht an oberster Stelle stehen, da andere Rabatte bei seiner Berechnung noch nicht existieren.

#### Text-Nr.

Hier kann ein Text aus den [Rabatt-Texten](./rabatt_texte.md) gewählt werden

#### Prfkt.

Preisfaktor (Anzahl der Mengeneinheiten) für Rabatte, die nicht prozentual berechnet werden. So kann z.B. ein Rabatt pro 2 oder 10 Stück(ME) gegeben werden.

#### EKZ-Nr.

Erlöskennziffer des Rabatts. (0 = wie Artikel) siehe auch „kalk“

#### Rab-Tab.

Hier wählen Sie einen der [Rabattsätze](./rabattsaetze.md) aus, der gelten soll.

#### InZl

Ja/Nein-Entscheidung, ob dieser Rabatt auf dem Ausdruck unterdrückt werden soll (JA), weil er lediglich zur internen Preisermittlung dient oder dem Belegempfänger sichtbar ausgedruckt werden sill (Nein).

#### GrpR

| Einstellung | Bedeutung |
| --- | --- |
| Zeile | Zeilenrabatt – wirkt auf eine Warenposition |
| Gruppe | Gruppenrabatt – wirkt auf alle Artikel dieser Warengruppe |
| Preis | Dieser Rabatt wirkt zunächst auf den Einzelpreis, bevor dieser mit der Menge multipliziert wird. |

#### kalk

Ja/Nein-Entscheidung, ob dieser Rabatt ein kalkulatorischer Rabatt sein soll, der als Teil des Preises berechnet und nicht gesondert ausgewiesen werden soll.

Kalkulatorische Rabatte werden im Gegensatz zu kalkulatorischen Zu- und Abschlägen in der Praxis kaum verwendet.

Administration > Steuerung > Steuerungsparameter zeigen **[SPA]**

Damit diese Einstellung wirken kann, muss der Steuerparameter [509 - Kalkulatorische Rabatte zulässig](../../../firmenstamm/steuerparameter/preisfindung/kalkulatorische_rabatte_zulaessig_spa_509.md) eingeschaltet sein.

Ein kalkulatorischer Rabatt wird wie ein InZeile-Rabatt nicht auf dem Beleg ausgegeben. Ein kalkulatorischer Rabatt wird jedoch nicht zur Berechnung des Betrages berechnet. Der kalkulatorische Rabatt sorgt dafür, dass dieser „Rabatt-Anteil“ abweichend von der Erlöskennziffer des Artikels auf eine andere Erlöskennziffer gebucht wird.

Ein Beispiel: Es wird Kaffee für 10€ verkauft, der vor dem Verkauf gemahlen wird. 0,50€ sind Kosten, die bei jedem Kilo anfallen. Der Erlös von 9,50€ soll auf das Erlöskonto Kaffee und 0,50€ auf das Erlöskonto Kaffeemühle gebucht werden.

#### Sp.

Ja/Nein-Entscheidung, ob dieser Rabatt vorübergehend gesperrt sein soll.
