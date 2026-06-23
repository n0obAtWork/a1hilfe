# ADR-Gefahrgutlisten-Import

<!-- source: https://amic.de/hilfe/_adrgefahrgutlistenimport.htm -->

Hauptmenü > Stammdatenpflege > Artikelstamm > ADR-Gefahrgutliste

oder Direktsprung **[ADR]**

Die Anwendung ADR-Gefahrgutliste stellt eine Schnittstelle zum Importieren der von der Bundesanstalt für Materialforschung und –prüfung (BAM) herausgegebenen, lizenzpflichtigen BAM-Liste für das europäische Übereinkommen über die internationale Beförderung gefährlicher Güter auf der Straße (Abkürzung: ADR) zur Verfügung.

Auch die Verwendung dieser Import-Schnittstelle unterliegt einer Lizenz. Zur Verwendung muss der [**Steuerparameter „972 – ADR-Gefahrgutlisten Lizenz“**](../../../../firmenstamm/steuerparameter/lizenzen/adr_gefahrgutlisten_lizenz_spa_972.md) auf **„Ja“** gestellt sein.

Hat man von benannter Bundesanstalt eine BAM-Listen Lizenz erworben, hat man mit dieser Schnittstelle die Möglichkeit, die Daten dieser BAM-Liste nach A.eins zu importieren.

| Feld | Beschreibung |
| --- | --- |
| **UN-Nummer** | Kennnummer, für alle gefährlichen Stoffe, die gleichzeitig als gefährliche Güter gelten. Mit ggf. vorangestellten Nullen. |
| **Lfd.-Nummer** | Laufende Nummer, bezogen auf die UN-Nummer |
| **Name** | Benennung und Beschreibung |
| **Klasse** | Gefahrgutklasse |
| **Klassifizierungscode** | Eigenschaften der einzelnen Stoffe bzw. Gegenstände sind in Klassifizierungscodes unterteilt |
| **Verpackungsgruppe** | Nummern der Verpackungsgruppe(n), die dem gefährlichen Stoff zugeordnet sind |
| **Gefahrzettel** | Nummer des Musters der Gefahrzettel/Großzettel |
| **Sondervorschriften** | Numerische Codes der einzuhaltenden Sondervorschriften |
| **Begrenzte und freigestellte Mengen** | **Begrenzte Menge | freigestellte Menge**<br>Höchstmenge des Stoffes je Innenverpackung oder Gegenstand für die Beförderung gefährlicher Güter in begrenzten Mengen.<br>|<br>Alphanumerischer Code für die Freistellung von den Vorschriften des ADR |
| **Verpackungen** | **Anweisung (Verpackungsanweisung):**<br>Alphanumerischer Code der anwendbaren Verpackungsanweisungen.<br>**Sondervorschrift:**<br>Alphanumerischer Code der anwendbaren Sondervorschriften für die Verpackung.<br>**Zusammenpackung:**<br>Mit den Buchstaben „MP“ beginnender alphanumerischer Code der anwendbaren Sondervorschriften für die Zusammenpackung. |
| **Ortsbewegliche Tanks und Schüttgut-Container** | **Anweisung:**<br>Alphanummerischer Code, der einer Anweisung für ortsbewegliche Tanks zugeordnet ist.<br>**Sondervorschrift:**<br>Alphanumerischer Code für die zusätzlich einzuhaltenden Sondervorschriften für ortsbewegliche Tanks |
| **ADR-Tank** | **Tankcodierung:**<br>Alphanummerischer Code, der einen Tanktyp beschreibt (für Gase der Klasse oder Stoffe der Klassen 3 bis 9).<br>**Sondervorschrift:**<br>Alphanumerischer Code für die zusätzlich einzuhaltenden Sondervorschriften für ADR-Tanks. |
| **Fahrzeug für die Beförderung in Tanks** | Code, der für die Beförderung des Stoffes in Tanks zu verwendende Fahrzeug angibt. |
| **Tunnelbeschränkungscode** | **Erster Teil** ist eine Ziffer, welche die Beförderungskategorie angibt, der der Stoff oder Gegenstand für Zwecke der Freistellung in Zusammenhang mit Mengen, die je Beförderungseinheit befördert werden, zugeordnet ist.<br>**Zweiter Teil** enthält in Klammern den Tunnelbeschränkungscode, der sich auf die anwendbare Beschränkung für die Durchfahrt durch Tunnel bezieht. |
| **Sondervorschriften für die Beförderung** | **Versandstücke:**<br>Enthält den (die) alphanumerischen, mit „V“ beginnenden Code(s) der für die Beförderung in Versandstücken anwendbaren Sondervorschriften.<br>**Lose Schüttung:**<br>Enthält den (die) alphanumerischen, mit „VC“ beginnenden sowie den (die) mit „AP“ beginnenden Code(s) der für die Beförderung in loser Schüttung anwendbaren Vorschriften.<br>**Be- und Entladung, Handhabung:**<br>Enthält den (die) alphanumerischen, mit „CV“ beginnenden Code(s) der für die Be- und Entladung sowie die Handhabung anwendbaren Sondervorschriften.<br>**Betrieb:**<br>Enthält den (die) alphanumerischen, mit „B“ beginnenden Code(s) der für den Betrieb anwendbaren Vorschriften. |
| **Nummer zur Kennzeichnung der Gefahr** | Enthält eine Nummer, die für Stoffe und Gegenstände der Klassen 2 bis 9 aus zwei oder drei Ziffern (in bestimmten Fällen mit vorangestelltem Buchstaben „X“) und für Stoffe und Gegenstände der Klasse 1 aus dem Klassifizierungscode besteht. |
| **UN-Nummer** | Kennnummer, für alle gefährlichen Stoffe, die gleichzeitig als gefährliche Güter gelten. Als Ziffer |

<p class="just-emphasize">Funktionen</p>

| Funktion | Funktionstaste | Beschreibung |
| --- | --- | --- |
| **Filter / Bereichsauswahl** | **F2** | Filtermöglichkeiten für die Auswahlliste.<br>è Suche<br><ul><li>-&nbsp;&nbsp;&nbsp;&nbsp; Sucht in einigen Feldern nach dem angegebenen Stichwort.</li></ul> |
| **Import aus ADR-Datei** | **F9** | Startet den Dialog zum Importieren der ADR-Listen-Datei. |
| **Löschen der ADR-Daten** | **F7** | Löscht alle vorhandenen ADR-Daten aus der A.eins-Tabelle. |

<p class="siehe-auch">Siehe auch:</p>

- [Dialog ADR-Gefahrgutlisten Import](./dialog_adr_gefahrgutlisten_import.md)
