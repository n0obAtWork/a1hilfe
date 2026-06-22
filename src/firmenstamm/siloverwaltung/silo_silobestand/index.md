# Silo / Silobestand

<!-- source: https://amic.de/hilfe/_AW_silo_silobestand.htm -->

In den Varianten „Silo“ und „Silobestand“ werden die vorhandenen Silos angezeigt. Die beiden Varianten unterscheiden sich unter anderem dadurch, dass unter „Silo“ die einzelnen Positionen der Silos angezeigt werden, während unter „Silobestand“ die summierte Gesamtmenge pro Silo seit der letzten Leermeldung angezeigt wird. Des Weiteren werden hier die aktuellen durchschnittlichen Qualitätswerte und Netto- und Sekundärmengen zur jeweiligen Gesamtmenge dargestellt.

Um Silos neu zu erfassen, muss – wie bei Ladeträgern – erst ein Silotyp bzw. [Ladeträgertyp](../../lagerverwaltungssystem/ladetraegertyp.md) erzeugt werden. Im Artikelstamm muss dafür ein Silo-Eintrag erzeugt werden. Das Bruttogewicht dieses Artikelstamms entspricht dann der Kapazität des Silos.

**Durchschnittliche Qualitätswerte und Netto-/Sekundärmengen zum Silobestand**

Die in der Variante „Silobestand“ zu jedem Silo dargestellten Qualitätswerte werden aus den einzelnen Zugängen, mit der jeweiligen Bewegungsmenge gewichtet, in der Reihenfolge des Bewegungszeitpunkts berechnet. Abgänge reduzieren dabei jeweils die für folgende Berechnungen nötigen Mengen, ändern aber die zu ihrem Bewegungszeitpunkt berechneten Qualitäten nicht. Gibt es für eine Zugangsbewegung zu einer Qualität keinen erfassten oder bestimmten Wert, so geht die Menge dieses Zugangs in die Berechnung nicht ein. Diesem Verfahren liegt die Annahme zu Grunde, dass für einen ‚unbekannten‘ Qualitätswert einer Bewegung der bisher berechnete Durchschnitt als Näherung angenommen wird. Dadurch wird ein statistisch möglichst geringer Fehler impliziert. Für Zugänge aus einem anderen Silo wird die Berechnung der Qualitäten der Bewegung per Rekursiv-Aufruf der Berechnungsfunktion zum Bewegungszeitpunkt mit dem Quellsilo durchgeführt.

Als Start-Zeitpunkt zur Berechnung der durchschnittlichen Siloqualitäten (zu einem Ziel-Zeitpunkt) wird grundsätzlich die neueste vor dem Ziel-Zeitpunkt liegende Leermeldung zum Silo herangezogen, es sei denn, es gibt mindestens eine zwischen diesen Zeitpunkten liegende festgeschriebene [Qualitätsübersicht](../qualitaetsuebersicht/index.md). Falls vorhanden, so bestimmt die neueste dieser [Qualitätsübersicht](../qualitaetsuebersicht/index.md)en den Start-Zeitpunkt mit der zu diesem Zeitpunkt vorhandenen Gesamtmenge als Bewegungsmenge.

Die Herkunft der einzelnen Qualitätswerte einer Bewegung wird durch den [Steuerparameter 940 – Quelle der Qualitätswerte zur Siloqualitätsberechnung](../../steuerparameter/waagensteuerung/silokontraktanlage_in_der_waage_spa_935.md#ueb_SPA940) bestimmt. Es wird festgelegt, ob die einzelnen Qualitätswerte nur aus den bereits an der Waage bestimmten Werten entnommen werden oder aber, bei bereits weiterverarbeiteten Belegen, aus den jeweils aktuellsten Instanzen der zugehörigen Rohwarebelegen.

Die Berechnung der Siloqualitäten wie auch der Netto-/Sekundärmengen erfolgt im Standard nicht bei Aufruf der Auswahlliste, da hier zum Teil je nach Datenvorkommen lange Lauf- und Antwortzeiten auftreten können. Vielmehr erledigt dieses die Datenbankfunktion ‚AMIC_SILOQUALTAETMENGEN_UPDATE‘, die z.B. per Event regelmäßig die Daten aktualisieren kann.

Entsprechend der Einstellung des [Steuerparameter 932 – Qualitätsverarbeitung in der Waage](../../steuerparameter/waagensteuerung/qualitaetsverarbeitung_in_der_waage_spa_932.md) wird die Zuordnung der Waagen-Qualitätswerte geregelt, entweder per EPA-Einstellung der Online-Waage-Maske für Bedienerklasse 0 oder entsprechend der in der Anwendung ‚Artikel-Bestandteile‘ mit dem Feld ‚Qualitätsnummer der Waage‘

<p class="siehe-auch">Siehe auch:</p>

- [Bestandsmeldung](./bestandsmeldung.md)
- [Leermeldung](./leermeldung.md)
- [Silobehandlung](./silobehandlung.md)
- [Vorgang-Details](./vorgang_details.md)
- [Hofliste-Details](./hofliste_details.md)
- [Reorganisation](./reorganisation.md)
