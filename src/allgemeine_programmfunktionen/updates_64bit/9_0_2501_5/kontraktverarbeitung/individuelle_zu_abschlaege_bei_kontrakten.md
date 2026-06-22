# Individuelle Zu-/Abschläge bei Kontrakten

<!-- source: https://amic.de/hilfe/_90_36294.htm -->

Die Behandlung individueller Zu-/Abschlagsgruppen bei Kontrakten wurde durch die Einführung eines neuen Steuerparameter [SPA] "Individuelle Zu-/Abschläge bei Kontrakten berücksichtigen" (1160) wie folgt überarbeitet: In A.eins besteht die Möglichkeit, eine Zu- und Abschlagsgruppe in einem Kontrakt zu hinterlegen. Damit ist es bei Verwendung dieses Kontraktes in einem Vorgang möglich, zusätzlich definitere Zu- und Abschläge zu ziehen. War im Kontrakt eine Zu- und Abschlagsgruppe hinterlegt und in Kunden / Artikelkombination eine individuelle Zu- und Abschlagseinrichtung vorhanden, übersteuerte die Individualeinrichtung die im Kontrakt definierte Einrichtung. Dieses Verhalten kann nun global über den "Steuerparameter" [SPA] 1060 gesteuert werden. Wird dieser mit "Ja" eingestellt, ändert sich am bisherigen Verfahren nichts. Wird dieser auf "Nein" gestellt, werden individuelle Zu- und Abschläge nicht mehr gezogen. Um mehr Flexibilität zu haben, kann diese Einstellung auch im Kontrakt auf Tab-Reiter Konstanten pro Kontrakt eingestellt werden. Diese Einstellung übersteuert dann die SPA-Einstellung. Wichtig ist, dass individuelle Zu- und Abschläge nur ziehen, wenn in Zu- und Abschlagsgruppe wirklich ein Wert &lt;>"0" gepflegt ist.

<p class="just-emphasize">Releasenote Kategorie:</p>

Ticket: 743049[36294]

Version: 9.0.2501.5

Datum:

Anwendung: Kontrakte, Warenvorgangsbehandlung

Variante: -

Funktion/Report: -

[Weitere Informationen](http://www.amic.de/hilfe/_SPA_1160.htm)

Tags:

Releasenote, 9.0.2501.5, 36294, 743049
