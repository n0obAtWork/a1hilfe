# Beispiel Darstellung eines Bildes

<!-- source: https://amic.de/hilfe/beispieldarstellungeinesbildes.htm -->

Hauptmenü > Administration > Werkzeuge > Informationssystem > Variante Informationssystem

Direktsprung **[AIS]**

Man kann Bilder aus der Tabelle Bitimages einfach auf einer Maske darstellen. Hier wird kurz gezeigt, wie man ein Feld einrichten muss, damit auf dem Pfleger für Artikel das zugeordnete Bild auf einem der Register zu sehen ist.

Im A.eins Informationssystem in der Variante Informationssystem legt man sich einen neuen Eintrag (**F8**) an. Zuerst muss die Gruppe angegeben werden. Hat man bereits ein oder mehrere Felder zu einer Gruppe erfasst, kann man diese hier mit **F3** auswählen. Die Felder „**Makro**“, „**Ändern Vorlauf**“ und „**Einfügen Vorlauf**“ werden dann vorbelegt.  
    

Um ein Bild des Artikels darzustellen, sind einige Einträge Notwendig. Der Name der Gruppe soll **Artikelbild** lauten.

<p class="just-emphasize">Register Feldbeschreibung:</p>

| | Beschreibung |
| --- | --- |
| Feldname<br> | Auch für Label, die nicht aus der Datenbank gefüllt werden, müssen Feldnamen vergeben werden. Hier muss der Label den Namen des Feldes aus der Tabelle Artikel erhalten, der die Imageid enthält: **Artikelimage**<br> |
| Feldtyp<br> | Der Feldtyp für die Imageid muss natürlich **Label** sein.<br> |
| Datenformat<br> | **Image**<br> |
| Zeile und Spalte<br> | Die Position kann entweder über ein Raster oder pixelgenau angegeben werden. Sollen es Pixel sein, so ist ein kleines p an die Zahl anzuhängen (z.B.: 125p). In unserem Beispiel sollen die Felder sich am Raster orientieren, also Spalte 1 und Zeile 1.<br> |
| Länge<br> | Wie viel Zeichen darf der Label lang sein. Die Länge ist relativ unwichtig, da das Bild immer so groß dargestellt wird, wie es ist. |
| Tipptext<br> | Ist ein Hinweistext, der erscheint, wenn der Mauszeiger über diesem Feld steht. Wenn er leer gelassen wird, so wird der Text<br>„Mit Doppelklick zum Bild bearbeiten…“<br>eingeblendet. |

<p class="just-emphasize">Register Datenbeschreibung:</p>

| | Beschreibung |
| --- | --- |
| Herkunftstyp | **Relation** |
| Relation/Prozedur | **Artikel** |
| Ident Feld | **Artikelid** |

Zum lesen des Daten wird aus diesen Informationen folgendes Statement gebildet:

```sql
Select Artikelimage from Artikel where Artikelid=???
```

Um jetzt noch die Fragezeichen mit einem Wert zu füllen, muss man in der Maskenzuordnung unter „**Ident Masken-Feldname/Wert**“ das Feld angeben, dass den Wert liefert.

<p class="just-emphasize">Maskenzuordnung:</p>

Nun fehlt noch die Zuweisung der Gruppe zur Maske. Dazu wechselt man in die Variante Maskenzuordnung und legt eine neue Maskenzuordnung an:

| | Beschreibung |
| --- | --- |
| Maske | **DHARTNEU** |
| Gruppe | **Artikelbild** |
| Haupttabelle | Bleibt in diesem Fall leer. |
| Ident Masken-Feldname/Wert | **h.ArtikelId$**<br>**ACHTUNG:** *Auf Groß- und Kleinschreibung achten!* |
| Darstellung | **auf dem Register** |
| Bezeichnung/Register | **Grafik** |
