# Quellbelegreaktivierung bei Stornieren/Löschen von Warebelegen (BA,AG,BS,AU,LI,RE) (SPA 987)

<!-- source: https://amic.de/hilfe/_SPA_987.htm -->

Mit den Einstellungen **Nein**, **im Verkauf**, **im Einkauf** bzw. **im Einkauf und Verkauf** regelt dieser Steuerparameter die Reaktivierung von Quellbelegen des zu löschenden Vorgangs bei Durchführung der Funktion **Stornieren** für Vorgänge des Warenwirtschaftssystems, die keine Rohwarevorgänge sind. Bei entsprechender Einstellung werden die Vorgänge, aus denen der stornierte Vorgang per Umwandlung hervorgegangen ist, wieder in den bearbeitbaren Zustand zurückgesetzt und gegebenenfalls durch den Mandantenserver wieder in das Warenwirtschaftssystem gebucht. Somit erscheint zum Beispiel ein Lieferschein nach Löschen der zugehörigen Rechnung durch die Funktion ***Stornieren*** wieder im Warenbuch und kann korrigiert und erneut zur Rechnung umgewandelt werden.  
Auch das Löschen von Gutschriften, die aus Rechnungen per Umwandlung entstanden sind und mittels des Steuerparameters [SPA] [Gutschrift aus Rechnung wie Stornorechnung (SPA348)](./gutschrift_aus_rechnung_wie_stornorechnung_spa_348.md) die Rechnung gegen Weiterverarbeitung sperren, löst dann die Reaktivierung der Rechnung aus. 

Grundsätzlich können nur Bestellanfragen, Angebote, Bestellungen, Aufträge, Lieferscheine und Rechnungen reaktiviert werden.
