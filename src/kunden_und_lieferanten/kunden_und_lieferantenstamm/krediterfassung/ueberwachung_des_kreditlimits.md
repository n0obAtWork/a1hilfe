# Überwachung des Kreditlimits

<!-- source: https://amic.de/hilfe/_berwachungdeskreditl.htm -->

Innerhalb der Vorgangserfassung erfolgt die Überprüfung der Limitüberschreitung sowohl vor als auch nach der Warenpositionserfassung. Voraussetzung ist, dass in den Steuerparametern 233 [„Kreditlimit-Prüfung“](../../../firmenstamm/steuerparameter/vorgangsbearbeitung_allg/kreditlimit_pruefung_spa_233.md) und 234 [„Kreditlimit-Prüfung mit Auftrag/Bestellg“](../../../firmenstamm/steuerparameter/vorgangsbearbeitung_allg/kreditlimit_pruefung_mit_auftrag_bestellg_spa_234.md) die Überwachung aktiviert wurde.

Gegen das eingetragene Limit wird der OP-Bestand zuzüglich nicht verbuchter Lieferscheine und Rechnungen verprobt. Mittels SPA 234 kann auch der Auftragsbestand mit einbezogen werden. Wird das Kreditlimit überschritten, erfolgt

Eine Warnung

Eine Speicherung aber Sperre des Belegs

Eine Abweisung des Belegs

Die Überwachung erfolgt nicht im Ansehen-Fall (F6).
