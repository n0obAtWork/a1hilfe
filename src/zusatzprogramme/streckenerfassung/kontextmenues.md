# Kontextmenüs

<!-- source: https://amic.de/hilfe/_vorgangsmappe_kontextmenu.htm -->

Über das Kontextmenü der Streckenerfassung sind diverse Funktionen ausführbar. Sie ermöglichen aus der Streckenerfassung heraus eine schnelle und komfortable Bedienung diverser Funktionen. Im Zusammenspiel mit den [Events der Grids](./datentabellen.md) ergibt sich eine sehr hohe Bedienerperformance durch die sehr flexiblen Funktionsstrukturen aus der Streckenerfassung heraus.

[Speichern](./kontextmenues.md#Speichern)

[Vorgang korrigieren](./kontextmenues.md#VorgangKorrigieren)

[Vorgang drucken](./kontextmenues.md#VorgangDrucken)

[Sammeldrucken](./kontextmenues.md#kontext_Sammeldrucken)

[Position stornieren](./kontextmenues.md#PositionStornieren)

[aus der Strecke nehmen](./kontextmenues.md#AusDerStreckeNehmen)

[Avis markierte Zeilen](./kontextmenues.md#AvisMarkierteZeilen)

[Neuer Kontrakt](./kontextmenues.md#NeuerKontrakt)

[Archiv anzeigen](./kontextmenues.md#ArchivAnzeigen)

[Lademittel zuordnen](./kontextmenues.md#LademittelZuordnen)

[Route zurücksetzen](./kontextmenues.md#RoutenPlanung)

[Route anzeigen](./kontextmenues.md#RoutenPlanung)

[Position als Ladeeinheit](./kontextmenues.md#PositionAufLadeträger)

[Kontraktartikelausweichliste bearbeiten](./kontextmenues.md#KontraktArtikelAusweichListe)

[Strecke vervielfältigen](./kontextmenues.md#strecke_vervielfaeltigen)

[Rohware](./kontextmenues.md#kontext_rohware)

Einige der Kontextmenüpunkte sind sehr speziell, teilweise Vorgangs- und/oder Feldgebunden.

So wird z.B. der Menüpunkt [Kontraktartikelausweichliste bearbeiten](./kontextmenues.md#KontraktArtikelAusweichListe) nur angezeigt wenn es sich um einen Kontrakt handelt, für diesen eine Kontraktartikelausweichliste hinterlegt ist (Feld Artikelnummer wird farblich angezeigt) und der Cursor über dem markierten Feld im *GFV* positioniert wird während man mit der rechten Maustaste das Kontextmenü öffnet.

<p class="just-emphasize">Speichern ![](../../ImagesExt/image8_1357.png)</p>

Alle Änderungen in den Grids werden gespeichert

<p class="just-emphasize">Vorgang korrigieren ![](../../ImagesExt/image8_1357.png)</p>

Über diesen Kontextmenüpunkt wird die Vorgangsbearbeitungsmaske (je nach Vorgangstyps) des gerade aktiven Vorgangs geladen. Je nach [Gültigkeit](./profile/index.md#registerAllgemein2) wird der Vorgang dann zum Bearbeiten oder Ansehen geöffnet ohne die Streckenerfassungsmaske zu verlassen.

Im Korrekturmodus lässt sich der betreffende Vorgang dann korrigieren. Die gemachten Änderungen werden dann automatisch übernommen.

<p class="just-emphasize">Vorgang drucken ![](../../ImagesExt/image8_1357.png)</p>

Aus der Maske Streckengeschäft kann der Druck des jeweils gerade aktiven Vorgangs gestartet werden.

<p class="just-emphasize">Sammeldrucken ![](../../ImagesExt/image8_1357.png)</p>

Das Sammeldrucken dient dem Drucken mehrerer Auswertungen hintereinander. Bevor das Sammeldrucken gestartet werden kann, müssen alle Daten gespeichert werden.

Danach wird eine zusätzliche Registerkarte geöffnet. Auf dieser finden sich alle Auswertungen des aktuellen [Profils](./profile/index.md) und der Vorgangsdruck. Um wieder auf die Normalansicht zu wechseln muss die „ESC“ Taste oder zum Starten des Drucks die „F9“-Taste gedrückt werden.

Gedruckt werden alle Formulare, welche im Feld „Drucken“ den Wert „Ja“ enthalten. Nachdem der Druck fertiggestellt wurde, wird automatisch in die Normalansicht gewechselt und die Daten neu geladen.

<p class="just-emphasize">Position stornieren ![](../../ImagesExt/image8_1357.png)</p>

Der aktive Vorgang wird aus dem System gelöscht.

<p class="just-emphasize">aus der Strecke nehmen ![](../../ImagesExt/image8_1357.png)</p>

Die Zuordnung des aktiven Vorgangs zu diesem Streckengeschäft wird gelöscht. Der Vorgang bleibt aber im System erhalten. Er kann so z.B. einem anderen Streckengeschäft zugeordnet werden.

<p class="just-emphasize">Avis markierte Zeilen ![](../../ImagesExt/image8_1357.png)</p>

<p class="just-emphasize">Routenplanung (Google Maps) ![](../../ImagesExt/image8_1357.png)</p>

In der Streckenerfassung ist eine Schnittstelle zur „Google.de“ Routenberechnung implementiert. Diese ermöglicht auf einfachste Weise Routen- und Tourenplanungen zu erstellen oder auch einfach nur einen Ort zu finden.

Um eine Routenplanung durchzuführen, müssen die entsprechenden Orte (Spalte Ort) in der gewünschten Reihenfolge per Doppelklick ausgewählt werden. Danach wird die Routenplanung über den Menüeintrag „Sonderfunktionen -> Route anzeigen“ gestartet. Dafür öffnet sich der eingerichtete Standardbrowser und ruft die „Google.de“ Routenberechnung mit den notwendigen Informationen auf.

Die markierten Einträge können danach mit dem Menüeintrag „Sonderfunktionen -> Route zurücksetzen“

<p class="just-emphasize">Kontraktartikelausweichliste bearbeiten ![](../../ImagesExt/image8_1357.png)</p>

Handelt es sich bei einem Vorgang um einen Kontrakt, so wird das Feld Artikelnummer farbig hinterlegt wenn zu diesem Artikel eine Kontraktausweichliste hinterlegt ist. Wird die Maus auf dieses Feld positioniert und die rechte Maustaste betätigt so ist im Kontextmenü der Punkt &lt;Kontraktartikelausweichliste bearbeiten> sichtbar.

Nach Auswahl des Menüpunktes kann die den Artikel zugehörige Kontraktartikelausweichliste bearbeitet werden.

<p class="just-emphasize">Neuer Kontrakt ![](../../ImagesExt/image8_1357.png)</p>

Diese Funktion des Kontextmenüs ermöglicht es während der Abwicklung der Streckenerfassung neue Kontrakte zu erzeugen und diese sofort der Strecke zuzuordnen.

<p class="just-emphasize">Lademittel zuordnen ![](../../ImagesExt/image8_1357.png)</p>

Dieser Kontextmenüpunkt steht nur zur Verfügung wenn sich der Mauszeiger über einer markierten Zelle der Spalte Menge im *GFV* befindet. Es können eine oder mehrere Zellen mit Doppelklick der linken Maustaste markiert bzw. demarkiert werden. Markierte Zellen werden farbig hinterlegt. Nach Auswahl des Menüpunktes öffnet sich die Maske „[Lademittelzuordnung](./positionsstammsatz.md)“ in der für unterschiedliche Lademittel verschiedenen Mengen (Gewichte) der markierten Artikel definiert werden können.

<p class="just-emphasize">Position auf Ladeträger ![](../../ImagesExt/image8_1357.png)</p>

Dieser Menüpunkt steht nur für Zeilen im *GMV* (wenn das Feld Artikelnummer gefüllt ist) zur Verfügung. Hier wird die Maske „[Position auf Ladeträger](./maske_position_auf_ladetraeger.md)“ geöffnet.

<p class="just-emphasize">Archiv anzeigen ![](../../ImagesExt/image8_1357.png)</p>

Über diesen Kontextmenüpunkt wird das Formulararchiv gestartet.

<p class="just-emphasize">Strecke vervielfältigen ![](../../ImagesExt/image8_1357.png)</p>

Über diesen Menüpunkt lässt sich die aktuelle Strecke vervielfältigen. Beim Aufrufen der Funktion wird zuerst geprüft ob die Daten geändert wurden, entweder werden die Daten dann gespeichert oder nochmal geladen, damit sich keine ungespeicherten Daten auf der Maske befinden.

Nach dem Speichern wird die neue Registerkarte „Kopie“ geöffnet und die ursprüngliche Registerkarte versteckt. Auf der neuen Registerkarte befindet sich dann ein Feld zum eingeben der Anzahl Kopien.

Nun kann entweder mit „ESC“ abgebrochen werden oder man übernimmt die Daten. Dann beginnt das kopieren der Daten. Beim kopieren der Daten werden nur die Daten, die auf der Maske zusehen sind übernommen. Die Dauer des Kopierens ist stark abhängig von der Anzahl der Kopien und der Anzahl der Belege die sich in der Strecke befinden.

Beim Kopieren des Streckenstamms werden alle Daten übernommen, bis auf die Streckennummer und Streckenbezeichnung. Diese werde aus dem hinterlegten Nummernkreis des Streckenprofils ermittelt.

<p class="just-emphasize">Rohware ![](../../ImagesExt/image8_1357.png)</p>

Unter diesem Menüpunkt befinden sich die Funktionen für die Rohwareumwandlung. Beim Aufruf dieser werden die markierten oder der aktuelle Beleg entsprechend umgewandelt.

Für die Funktionen die mit „(Alle)“ gekennzeichnet sind, werden alle passenden Belege der aktuellen Datentabelle für die Umwandlung vorgesehen.

Sollen mehrere Belege umgewandelt werden und handelt es sich um Einkaufs- und Verkaufsbelege, wird die Umwandlungsmaske zweimal aufgerufen. (Umwandlung erfolgt separat für Einkauf und Verkauf)
