# Behebung Locking-Fehler Maske Preiskonditionen

<!-- source: https://amic.de/hilfe/_90_36656.htm -->

Bei Änderungen innerhalb der Maske [PRI] wird eine Datenbanksperre für die Kombination aus Individueller Preisklasse und Artikelpreisgruppe gesetzt, um so ungewollte Änderungen im Mehrbenutzerbetrieb zu verhindern. Bei bestimmten Konstellationen blieb die Datenbanksperre auch nach Schließen der Maske erhalten, so dass nach erneutem Aufruf der Maske für die gleiche Daten-Kombination der Datensatz fälschlicherweise als gesperrt angezeigt wurde. Der Fehler wurde behoben - beim Schließen der Maske werden alle Sperren freigegeben.

<p class="just-emphasize">Releasenote Kategorie:</p>

Ticket: 741454[36656]

Version: 9.0.2502.5

Datum: 15.10.2025

Anwendung: Individualpreispfleger [PRI] [PRIE]

Variante: -

Funktion/Report: -

[Weitere Informationen](http://www.amic.de/hilfe/_PFLMODUL_IPRRAB.htm)

<p class="just-emphasize">Tags:</p>

Releasenote, 9.0.2502.5, 36656, 741454
