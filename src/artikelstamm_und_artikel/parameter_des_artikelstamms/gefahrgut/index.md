# Gefahrgut

<!-- source: https://amic.de/hilfe/_gefahrgut.htm -->

Für die Gefahrgutabwicklung können im Artikelstamm-Pflege-Modul diverse Angaben mit der Funktion ***Gefahrgut*** hinterlegt werden. Diese Funktion ist sowohl von der Artikelstamm-Pflege-Maske als auch direkt von der Artikelstamm-Auswahlliste aus aufrufbar und ermöglicht die Pflege von Gefahrgutdaten, die in Formulareinrichtungen gemäß der Vorschriften des ADR ( ***A****ccord européen relatif au transport international des marchandises **D**angereuses par **R**oute,* Europäische Übereinkommen über die internationale Beförderung gefährlicher Güter auf der Straße ) berücksichtigt werden können.

| Feld | Bedeutung |
| --- | --- |
| UN-Nummer | UN-Nummer nach ADR Teil 3 Tabelle A |
| Verpackungsgruppe | Verpackungsgruppe nach ADR Teil 3 Tabelle A |
| Gefahrgut-Klasse | Nummer der [Gefahrgut-Klassen-Definition](../../konstanten_der_artikelverwaltung/parameter_der_gefahrgutabwicklung/index.md#Gefahrgutklasse) |
| Brand-Klasse | Nummer der [Brand-Klassen-Definition](../../konstanten_der_artikelverwaltung/parameter_der_gefahrgutabwicklung/index.md#Brandklasse)<br>Die Brand-Klasse ist entgegen früheren Verordnungen nach ADR nicht mehr erforderlich. Aus historischen Gründen wird diese in A.eins jedoch zur individuellen Nutzung weiterhin angeboten. |
| Toxizitäts-Klasse | Nummer der [Toxizitäts-Klassen-Definition](../../konstanten_der_artikelverwaltung/parameter_der_gefahrgutabwicklung/index.md#Toxiklasse)<br>Die Toxizitäts-Klasse ist entgegen früheren Verordnungen nach ADR nicht mehr erforderlich. Aus historischen Gründen wird diese in A.eins jedoch zur individuellen Nutzung weiterhin angeboten. |
| Postversand zulässig | Kennzeichen (Ja/Nein): Hinweis für die Zulässigkeit des Gefahrgut-Versands per Post. |
| Merkblatt | Der UN-Merkblatt-Verweis ist entgegen früheren Verordnungen nach ADR nicht mehr erforderlich. Aus historischen Gründen wird diese in A.eins jedoch zur individuellen Nutzung weiterhin angeboten. |
| Gefahrgutmenge pro einer Anzahl von Grundmengeneinheiten<br> | Zur Berechnung von Volumen beziehungsweise der Bruttomasse oder Nettomasse nach ADR ist es erforderlich, diesen Bezug in Grundmengeneinheiten der Mengeneinheitsgruppe des Artikelstamms zu hinterlegen. Die Interpretation in Litern oder Kilogramm ergibt sich aus der Art des Gefahrguts. Beispiel: 10 (Liter bzw. Kilogramm) pro 5 Stück. |
| Beförderungskategorie | Nummer der Beförderungskategorie nach ADR Teil 3 Tabelle A |
| Kategoriefaktor | Faktor zur Beförderungssummen-Berechnung nach ADR 1.1.3.6.4 |
| Kategoriegrenzmenge | Informatorisch zur Aufnahme der maximalen Beförderungssumme nach ADR 1.1.3.6.4 |
| Bezugsgröße | Die Bezugsgröße ist entgegen früheren Verordnungen nach ADR nicht mehr erforderlich. Aus historischen Gründen wird diese in A.eins jedoch zur individuellen Nutzung weiterhin angeboten. |
| Bezugstyp | Der Bezugstyp ist entgegen früheren Verordnungen nach ADR nicht mehr erforderlich. Aus historischen Gründen wird diese in A.eins jedoch zur individuellen Nutzung weiterhin angeboten. |
| Verpackungscode | Verpackungscode nach ADR 6.1.2.7 |
| Ziffer | Die Ziffer zum Verpackungscode ist entgegen früheren Verordnungen nach ADR nicht mehr erforderlich. Aus historischen Gründen wird diese in A.eins jedoch zur individuellen Nutzung weiterhin angeboten. |
| Verpackungsanzahl | Die Verpackungsanzahl wird in A.eins zur individuellen Nutzung angeboten. |
| VerpackungsRN | Die Verpackungs-Randnummer ist entgegen früheren Verordnungen nach ADR nicht mehr erforderlich. Aus historischen Gründen wird diese in A.eins jedoch zur individuellen Nutzung weiterhin angeboten. |
| Verpackungstyp | Der Verpackungstyp wird in A.eins zur individuellen Nutzung angeboten und kann zum Beispiel die Art der Verpackung nach ADR 6.1.2.7 enthalten. |
| Verpackungswertstoff | Der Verpackungswertstoff nach ADR 6.1.2.7 kann optional in dieses Feld eingetragen werden. |
| Verpackungscode2 | Feld zur freien Verfügung, zum Beispiel für Verpackungscode von Großpackmitteln nach ADR 6.5.1.4.3 |
| Verpackungsbezeichnung | Bezeichnungs-Feld für Packmittel |
| Tunnelcode | Tunnelbeschränkungscode nach ADR Teil 3 Tabelle A |
| Gefahrzettel | Gefahrzettel nach ADR Teil 3 Tabelle A. Bei mehreren vorgeschriebenen Gefahrzetteln wird empfohlen, diese entsprechend ADR 5.4.1.1.1 c) anzugeben. Beispiel: 6.1 (5.1) |
| Bemerkungen | Das Bemerkungsfeld ist ein Bereich, in dem die notwendigen Angaben nach ADR 5.4.1.1.1 formuliert werden können. |
| Löschbar mit Wasser | Gibt an ob das Gefahrgut mit Wasser löschbar ist. Dieses kann in einen Paket nicht geändert werden, wird aber automatisch angepasst. Wenn das Feld automatisch verändert wurde, wird beim Verlassen der Maske ein entsprechender Hinweis ausgegeben. Wenn der Artikel in einem Paket vorkommt, wird bei Differenzen angezeigt, welche Gefahrgut-Pakete nochmal zu überprüfen sind. |
| Löschbar mit Schaum | Gibt an ob das Gefahrgut mit Schaum löschbar ist. Dieses kann in einen Paket nicht geändert werden, wird aber automatisch angepasst. Wenn das Feld automatisch verändert wurde, wird beim Verlassen der Maske ein entsprechender Hinweis ausgegeben. Wenn der Artikel in einem Paket vorkommt, wird bei Differenzen angezeigt, welche Gefahrgut-Pakete nochmal zu überprüfen sind. |
| Löschbar mit CO2 | Gibt an ob das Gefahrgut mit CO2 löschbar ist. Dieses kann in einen Paket nicht geändert werden, wird aber automatisch angepasst. Wenn das Feld automatisch verändert wurde, wird beim Verlassen der Maske ein entsprechender Hinweis ausgegeben. Wenn der Artikel in einem Paket vorkommt, wird bei Differenzen angezeigt, welche Gefahrgut-Pakete nochmal zu überprüfen sind. |
| Löschbar mit Pulver | Gibt an ob das Gefahrgut mit Pulver löschbar ist. Dieses kann in einen Paket nicht geändert werden, wird aber automatisch angepasst. Wenn das Feld automatisch verändert wurde, wird beim Verlassen der Maske ein entsprechender Hinweis ausgegeben. Wenn der Artikel in einem Paket vorkommt, wird bei Differenzen angezeigt, welche Gefahrgut-Pakete nochmal zu überprüfen sind. |

Sollen negative Positionsmenge für die Ermittlung von Gefahrgutwerten in Vorgängen positiv berücksichtig werden, kann man unter dem Direktsprung **[FRZ]** für ausgewählte Unterklassen dieses auf der Registerkarte **Abwicklung** im Bereich *Gefahrgut* entsprechend einstellen.

<p class="just-emphasize">Zusammensetzung</p>

Auf der Registerkarte „Zusammensetzung“, werden die einzelnen Komponenten des Gefahrgutartikels eingetragen. Die Komponenten werden in Einheiten angegeben. Es dürfen keine Gefahrgut-Pakete zu Gefahrgut-Paketen hinzugefügt werden. Es darf auch kein Artikel zweimal vorkommen.

<p class="just-emphasize">Sätze</p>

Auf der Registerkarte „Zuordnung“, werden die Gefahrgut Informationen dem Artikelstamm zugeordnet. Die Liste aller gängigen Gefahrgutinfos wird mit ausgeliefert.

<p class="just-emphasize">Zulassungsdaten</p>

Auf der Registerkarte „Pflanzenschutz-Mittel“ können die Zulassungsdaten eines Pflanzenschutzmittels gepflegt werden.

**Format der Zulassungsnummer**

GG - *Generationsnummer* – beginnt bei Erstzulassung mit **00**, steigt bei Folgeanträgen (**02**, **04**...)

XXXX - *Individuelle vierstellige Kombination* aus Ziffern/Buchstaben für jedes Mittel.

\-YY - *Hauptzulassung oder Vertriebserweiterung*: „**00**“ = Hauptzulassung, „**60**“ ff. = Vertriebserweiterungen.

**GGXXXX-YY**

**Beispiel:**

**02AB12-00**

- „02“ = zweite Generation
- „AB12“ = individuelle Mittelkennung
- „00“ = Hauptzulassung

<p class="siehe-auch">Siehe auch:</p>

- [ADR-Gefahrgutlisten-Import](./adr_gefahrgutlisten_import/index.md)
