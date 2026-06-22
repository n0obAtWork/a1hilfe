# Auswertung / Buchung

<!-- source: https://amic.de/hilfe/_lvs20_Inv_auswertung.htm -->

Hauptmenü > Inventur > Permanente Inventur > Permanente Inventur Prüfungen

Geplante Inventur

Unabhängig davon, ob eine Bestandskorrektur notwendig war, wird im Bewegungsprotokoll des Ladeträgers festgehalten, dass eine Inventur stattgefunden hat.

So kann im Fall den (permanenten) Inventur zu einem Zeitpunkt festgestellt werden:

• Alle Ladeträger des Artikels

• Sind im Zeitraum von x Tagen ([SPA 1045 – Permanente Inventur](../../../firmenstamm/steuerparameter/optionen_warenwirtschaft/permanente_inventur_mit_lvs_spa_1045.md)) gezählt worden

Diese Tatsache bei jeder Erfassung geprüft und bei Erfüllung aller Bedingungen wird dies in der Tabelle „LVS_PermInventurProtokoll“ festgehalten. Die Bestände in LVS und Warenwirtschaft müssen jetzt noch nicht zwingend übereinstimmen.

Jede Bestandsänderung ab diesem Zeitpunkt gilt als Bestandsfortschreibung. Die Differenz bleibt aber die Gleiche.

**Nullmengen-Erfassung**

Hauptmenü > Inventur > Permanente Inventur > Permanente Inventur Prüfungen

Es liegt in der Natur der Sache, dass man Bestände mit einem Bestand von 0 nicht aufzufinden vermag. Sollte nach intensiver Prüfung der Bestand tatsächlich nicht vorhanden sein, so kann dieser Null-Bestand mit der Funktion „Nullzählung LVS erzeugen“ bestätigt werden. Es wird dann die Null-Zählung ins Protokoll eingetragen und in den nächsten Beleg übernommen.

Funktionen

**Report erzeugen**

Hier wird ein Report aufgerufen, der alle in der Auswahlliste angezeigten Artikel und Lokalitäten enthält. Zusätzlich ist der Barcode „INVENTUR“ aufgebracht.

**Inventurbelege LVS erzeugen**

Hier wird ein Inventurdifferenzbelege (Vorgangsklasse 5055) erstellt. Dieser beseitigt alle Differenzen der Artikel zwischen LVS und Warenwirtschaft. Es wird wie folgt gerechnet:

Bestandsdifferenz zum Zeitpunkt der Zählung = D

Warenbestand zum Zeitpunkt der Belegerstellung = W

Zählung = Z

Z = W - D

Bewertung

Hauptmenü > Inventur > Permanente Inventur > Laufende Inventur

Dieser Belegmuss dann im Nachgang in der Variante Vorgänge bewertet werden.

Ungeplante Inventur

Im Bewegungsprotokoll des Ladeträgers wird festgehalten, dass eine Bestandskorrektur stattgefunden hat.

Zudem wird ein Lieferschein mit dieser Position ohne Wert erstellt, der diese Menge aus der Warenwirtschaft ausbucht. Der Kunde dieses Beleges ist steuerfrei einzurichten und in der Vorgangsklasse 5055 als Kontokorrentkunde in [FRZ] zu hinterlegen.

Dieser Beleg muss dann im Nachgang nicht bewertet werden.

Inventurlieferscheine wie diese können und müssen nicht von Lieferschein zu Rechnung umgewandelt werden.
