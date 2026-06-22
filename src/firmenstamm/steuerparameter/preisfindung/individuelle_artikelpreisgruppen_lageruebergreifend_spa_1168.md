# Individuelle Artikelpreisgruppen lagerübergreifend (SPA 1168)

<!-- source: https://amic.de/hilfe/_SPA_1168.htm -->

Parameter betrifft individuelle Preisgruppen, individuelle Rabattgruppen und individuelle Zu-/Abschlagsgruppen am Artikel, jeweils getrennt nach den Seiten Einkauf oder Verkauf.

Wird die individuelle Preispflege für einen Artikel geöffnet, dem eine oder mehrere dieser Gruppen fehlen, so werden fehlende Gruppen nunmehr vom System automatisch erzeugt. Das Verhalten wird dabei durch den neuen Steuerparameter gesteuert:

Bei „Ja“: Alle Artikel mit identischer Artikelstamm-ID, unabhängig von ihrem konkreten Lagerort, werden durchsucht und die maximal zugewiesene individuelle Preisgruppe wird auch dem neuen Artikel zugewiesen. Annahmegemäß ist dies auch die zuletzt verwendete Preisgruppe, was zu einer lagerübergreifenden, einheitlichen Gruppenzuordnung bei allen Artikeln führen wird. Wird diese maximale Gruppe nicht gefunden, wird automatisch eine neue Gruppe erzeugt und dem Artikel zugewiesen.

Bei „Nein“ (Default-Wert): Insofern eine individuelle Gruppenzuordnung fehlt, wird automatisch eine neue erzeugt und dem Artikel zugewiesen.

Bei „Fragen“: pro Sachverhalt wird nachgefragt, ob eine maximale individuelle Gruppe gesucht und zugewiesen werden soll (Ja-Fall) oder ob eine neue Gruppe generiert wird (Nein-Fall):

Hinweis: **die automatische Zuweisung erfolgt zukünftig immer**, nur noch die hierfür zu verwendende Preisgruppe kann über diesen SPA gesteuert werden.
