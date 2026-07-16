# Gruppenzuordnung

<!-- source: https://amic.de/hilfe/_gruppenzuordnung.htm -->

Hier erfolgt die Zuordnung von [Artikelgruppe](../../konstanten_der_artikelverwaltung/artikelgruppen.md), [Artikel-Info-Gruppe](../../konstanten_der_artikelverwaltung/artikel_info_gruppen.md), [Verpackungsgruppe](../../konstanten_der_artikelverwaltung/verpackungsgruppen.md) und [Pool](../../konstanten_der_artikelverwaltung/artikelpool.md). Auf die Parameter wurde weiter oben eingegangen.

Ein weiteres sehr wichtiges Element ist die Artikelklasse. Folgende Eintragungen sind möglich:

**Normalartikel:** Der fakturierfähige Artikel; dies ist der Standardwert

**Transportkosten:** für Erweiterungen des Frachtwesens vorgesehen; z.Z. nicht aktiv

**Gefährliche Güter**: für Erweiterungen der Gefahrgutabwicklung vorgesehen; z.Z. nicht aktiv

**Dienstleistungen**: Zusammen mit dem Steuerungsparameter (21,25) „Dienst­leistungen nur als Wertartikel“ kann erreicht werden, dass Dienstleistungen aus­schließlich wertmäßig gebucht werden können; für die normale Warenerfassung bleibt der Artikel gesperrt.

**Leergut:** Hiermit wird der Artikel als Leergut gekennzeichnet. Dies ist Voraus­setzung für den Druck von Nachweisen beim Vorgangsdruck und der Führung des Leergutkontos. Näheres dazu im Bereich „Leergutverwaltung“.

**Verpackung**:

**Bezug auf Hauptartikel**:

Dieser Parameter ist in Verbindung mit der Option Folgeartikel und bei eingerichteten Gruppenzu-/abschlägen und Gruppenrabatten aktiv:

So beziehen sich die Gruppenzuabschläge/Gruppenrabatte nicht auf den Artikel selbst sondern auf den Hauptartikel der Folgeliste. Hierüber kann z.B. das Problem von Verlustver­packungen gelöst werden, deren Erlösschmälerungen direkt auf das Artikelkonto fließen sollen.

Anmerkung für den Formulardruck: Eine Verlustverpackung innerhalb einer Folgeartikelliste bekommt die Artikelvariante 308.

**Muster – diverse**: Z.Z. nicht aktiv

**Saatgutartikel:**

In Zusammenhang mit dem Modul Saatgut wird hier festgelegt, ob es sich um einen Saatgutartikel handelt. Danach sind dann Fruchtart und Sorte festzule­gen. Näheres hierzu im Abschnitt Saatgut.
