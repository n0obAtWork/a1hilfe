# Individuelle Zu-/Abschläge bei Kontrakten berücksichtigen (SPA 1160)

<!-- source: https://amic.de/hilfe/_SPA_1160.htm -->

Dieser Steuerparameter beeinflusst die Berücksichtigung individueller Zu-/Abschlagsgruppen bei der Berechnung allgemeiner Zu-/Abschläge innerhalb von Kontrakten.

Durch die Auswahl einer allgemeinen Zu-/Abschlagsgruppe am Kontrakt selbst, wird die Berechnung von Zu-/Abschlägen für den Kontrakt grundsätzlich aktiviert. **Ohne Bestimmung einer solchen Gruppe am Kontrakt werden keinerlei Zu-/Abschläge berechnet**.

Neben der Zu-/Abschlagsgruppe am Kontrakt können individuelle Zu-/Abschläge existieren, die sich vorrangig aus der Kombination von Kontrakttyp, Kunde und Artikel ergeben. Solche individuellen Zu-/Abschlagsgruppen hatten in der Vergangenheit immer schon Vorrang vor der am Kontrakt hinterlegten allgemeinen Zu-/Abschlagsgruppe: Wird eine individuelle Zu-/Abschlagsgruppe gefunden, so wird die Zu-/Abschlagsgruppe am Kontrakt ignoriert – dieses Verhalten entspricht der Steuer-parameter-Einstellung „Ja“ (Default). Wird der Steuerparameter auf „Nein“ gestellt, so werden individuelle Zu-/Abschlagsgruppen ignoriert und nur die am Kontrakt hinterlegte Zu-/Abschlagsgruppe bei der Kontraktberechnung berücksichtigt.

Dieses Verhalten kann zusätzlich über eine direkt am Kontrakt verfügbare Auswahlmöglichkeit übersteuert werden, so dass sich eine kontraktspezifische Berücksichtigung individueller Zu-/Abschlagsgruppen ergibt.
