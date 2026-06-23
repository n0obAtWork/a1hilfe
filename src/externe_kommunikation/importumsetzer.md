# Importumsetzer

<!-- source: https://amic.de/hilfe/_importumsetzer.htm -->

Hauptmenü > Externe Kommunikation > Stammdatenimport > Importumsetzer [**IMPUM**]  
    

Mit dem Importumsetzer können Kennzeichen von einem Fremdsystem wie z.B. Terres bequem auf A.eins Kennzeichen umgeschlüsselt werden. Dies gilt natürlch auch in die andere Richtung.

Es wird zu jedem Fremdkennzeichen (Eingangsschlüssel) ein A.eins Kennzeichen (Umsetzung) innerhalb einer Schlüsselklasse zugeordnet.

<p class="just-emphasize">Variante Import-Umsetzer</p>

In dieser Variante können neue Umschlüsselungen angelegt werden.

<p class="just-emphasize">Funktionen: Neu [F8] - Ändern [F5] - Löschen [F7]</p>

Mit der Funktion Neu, Ändern oder Löschen wir die Maske Import Umsetzer geöffnet.

<p class="just-emphasize">Maske</p>

| ![\*](../ImagesExt/image8_1559.jpg "*") Feld | ![\*](../ImagesExt/image8_1559.jpg "*") Bedeutung |
| --- | --- |
| ![\*](../ImagesExt/image8_1556.jpg "*") Schlüsselklasse | ![\*](../ImagesExt/image8_1556.jpg "*") In diesem Feld wird die Klasse angegeben in dem sich das Umschlüsselungpaar befindet. |
| ![\*](../ImagesExt/image8_1556.jpg "*") Eingangsschlüssel | ![\*](../ImagesExt/image8_1556.jpg "*") Der Wert welcher Umgeschlüsselt werden soll. |
| ![\*](../ImagesExt/image8_1556.jpg "*") Umsetzung | ![\*](../ImagesExt/image8_1556.jpg "*") Zugewiesenner Umschlüsselungswert |
| ![\*](../ImagesExt/image8_1556.jpg "*") Info-Text | ![\*](../ImagesExt/image8_1556.jpg "*") Informationstext |

<p class="just-emphasize">Besondere Funktionen</p>

Im Änderfall steht die Funktion ***Alle Ändern*** [**F5**] zur Verfügung, wenn in der Auswahlliste mehr als ein Datensatz markiert worden ist. Dies bedeutet, falls die Änderung gemacht wird, wird dies für alle Datenstätze mitübernommen werden. Des Weiteren kann mit ***Speichern unter…*** [**SF9**] eine neue Umsetzung angelegt werden.

Im Löschenfall steht die Löschfunktion ***Alle Lösche [SF7]*** zur Verfügung, wenn in der Auswahlliste mehr als ein Datensatz markiert worden ist. Damit werden alle ausgewählten Datensätze gelöscht.

<p class="just-emphasize">Funktion Ändern(Tabellarisch) [SF5]</p>

Diese Funktion steht nur zur Verfügung, wenn in der Variante „Import-Umsetzer Itemboxzuordnung“ eine Zuordnung zu der Schlüsselklasse existiert. Die umzuschlüsselnden Werte werden in einer Prozedur bestimmt. Diese werden in die Maske geladen. Diesen Werten können dann die A.eins Kennzeichen zugeordnet werden. Wurde eine Itembox eingerichtet, so kann der Wert darüber ausgewählt werden. Beim Verlassen der Maske werden die Daten, die ein Umschlüsselungspaar darstellen abgespeichert.

<p class="just-emphasize">Variante Import Schlüssel</p>

<p class="just-emphasize">Varinate Importschlüssel Tabelle</p>

<p class="just-emphasize">Variante Import-Umsetzer Itemboxzuordnung</p>

In dieser Variante wird die Steuerung für die Tabellarische(-änderung) eingestellt<strong>.</strong> Mit dieser Funktion können Massendaten einfach umgeschlüsselt werden. Für die Datenaufbereitung wird eine Private Prozedur benötigt.

Mit ***Neu*** [**F8**] wird ein neuer Eintrag angelegt.

| ![\*](../ImagesExt/image8_1557.jpg "*") Maskenfeld | ![\*](../ImagesExt/image8_1557.jpg "*") Bedeutung | ![\*](../ImagesExt/image8_1557.jpg "*") Beispiel |
| --- | --- | --- |
| ![\*](../ImagesExt/image8_1556.jpg "*") Schlüsselklasse | ![\*](../ImagesExt/image8_1556.jpg "*") Hier wird die Schlüsselklasse eingetragen | ![\*](../ImagesExt/image8_1556.jpg "*") 1006 |
| ![\*](../ImagesExt/image8_1556.jpg "*") IB-Eingangsschlüssel | ![\*](../ImagesExt/image8_1556.jpg "*") Falls dem Eingangschlüssel eine Itembox zugeordnet werden soll, wird diese hier eingetragen. Diese würde dann die Bezeichnung des Wertes im Feld Ursprungsfeldbezeichnung anzeigen. | ![\*](../ImagesExt/image8_1556.jpg "*") |
| ![\*](../ImagesExt/image8_1556.jpg "*") IB-Umsetzung | ![\*](../ImagesExt/image8_1556.jpg "*") Hier wird die Itembox hinterlegt, die auf dem Feld Zielfeld wirken soll. | ![\*](../ImagesExt/image8_1556.jpg "*") IB_KU |
| ![\*](../ImagesExt/image8_1556.jpg "*") IB-Bezeichnungsfeld | ![\*](../ImagesExt/image8_1556.jpg "*") Hier wird der return Wert aus der Itembox angegeben, welcher in der Zielfeldbezeichung angezeigt wird. | ![\*](../ImagesExt/image8_1556.jpg "*") AdrFeld1 |
| ![\*](../ImagesExt/image8_1556.jpg "*") Kurzbeschreibung | ![\*](../ImagesExt/image8_1556.jpg "*") Freier Text | ![\*](../ImagesExt/image8_1556.jpg "*") Terres Lieferanten |
| ![\*](../ImagesExt/image8_1556.jpg "*") Datenbeschaffungsprozedur | ![\*](../ImagesExt/image8_1556.jpg "*") Hier wird die Prozedur hinterlegt, welche mir die umzuschlüsselnden Werte liefert. | ![\*](../ImagesExt/image8_1556.jpg "*") ImpumDatendrehscheibeLieferanten |

<p class="just-emphasize">Beispielprozedur</p>

Die Prozedur „ImpumDatendrehscheibeLieferanten“ liefert alle Lieferanten, die aus den Daten der Drehscheibe ermittelt werden konnte und die schon Umgeschlüsselten Werte.
