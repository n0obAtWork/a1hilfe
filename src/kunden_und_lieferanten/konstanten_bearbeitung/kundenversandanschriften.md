# Kundenversandanschriften

<!-- source: https://amic.de/hilfe/_kundenversandanschri.htm -->

Hauptmenü > Stammdatenpflege > Konstanten Kundenstamm > Kundenversandanschriften

oder Direktsprung **[KUVS]**

Gibt es zu einer Rechnungsanschrift mehrere Lieferempfänger, so können verschiedene Versandanschriften hinterlegt werden, die dann innerhalb der Belegerfassung zum Einsatz kommen.

Für die Nachhaltigkeitsvorbelegung kann hier ein Anbauland hinterlegt und gespeichert werden. Bei der Belegerfassung, kann dann auf einem UFLD-Feld(108) die Kundenversandanschrift angegeben werden. Dadurch kann bei richtiger Einrichtung das in der Versandanschrift hinterlegte Anbauland gezogen werden.

Im Kunden muss also ein Nachhaltigkeitszertifikat existieren und der gewünschte Artikel muss in Verbindung mit dem Anbauland aus der Versandanschrift übereinstimmen. Ansonsten wird das Anbauland aus der Versandanschrift ignoriert.

An Versandanschriften sind keinerlei Informationen geknüpft, es handelt sich um reine Anschrifteninformation. Wenn gewünscht wird, Statistiken an Lieferanschriften zu binden, so ist es sinnvoll, im Kundenstamm mit Oberkundenbeziehungen zu arbeiten.

Im linken Bereich des Eingabebildschirms wird die Kundenadresse angezeigt und im rechten die Versandanschrift. Zusätzlich wird die Anzahl der Versandanschriften pro Kunde angezeigt. Wird dieser Pfleger aus dem Kundenstamm heraus aufgerufen, so kann man mit den Pfeiltasten zwischen den Versandanschriften des aktuellen Kunden blättern.

| Felder |
| --- |
| Gruppe (Intra) | Intrastatgruppe des zugehörigen Kunden. Dieses Feld ist nur private Zwecke nutzbar, die offizielle Intrastat - Meldung benutzt dieses Feld nicht. |
| ILN-Nr | ILN Nummer dieser Adresse |

Hat man zu einem Kunden sehr viele Versandanschriften, so kann über die Funktion ***Suchen*** **F6** nach Name, Straße und/oder Ort innerhalb der Versandanschriften dieses Kunden gesucht werden. Auf der sich öffnenden Suchmaske können entweder bis zu drei Kritieren direkt angegeben werden oder man sucht mit **F3** über die Itembox.

Beim Löschen von Versandanschriften wird die Adressnummer des Datensatzes mit einem negativen Vorzeichen versehen (analog zu Anschriften [ansch]), wenn der Steuerparameter Anschriften archivieren [SPA 574](../../firmenstamm/steuerparameter/kundenstammdaten/anschriften_archivieren_spa_574.md) auf Ja steht.
