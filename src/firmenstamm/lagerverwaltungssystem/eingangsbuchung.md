# Eingangsbuchung

<!-- source: https://amic.de/hilfe/_lvseingang.htm -->

Hauptmenü > Wareneinkauf > Eingangslieferschein > Eingangslieferscheine bearbeiten

oder Direktsprung **[ELB]**

**I**n der ersten Variante befindet sich die Funktion „***Ladeträgerzuordnen***“. Alternativ kann die Funktion mit **SHIFT+F9** aufgerufen werden.

Es öffnet sich jetzt eine Maske, die alle Positionen des Lieferscheins enthält.

<p class="just-emphasize">Allgemeine Info</p>

Mit dieser Funktion können die Warenpositionen eines Eingangslieferscheins in das Lagerverwaltungssystem eingebucht werden. Es ist möglich eine Warenposition auf einen oder mehrere Ladeträger zu verteilen. Des Weiteren ist möglich eine Position von einem Ladeträger auf einen anderen Ladeträger umzubuchen oder zu löschen.

<p class="just-emphasize">Zuordnen von Positionen zum Ladeträger</p>

Um einer Position des Lieferscheins ein Ladeträger zuzuordnen wird in das Feld Ladeträger die Ladeträgernummer eingetragen. Die Lokalität wird mit der Lokalität des Ladeträgers vorbelegt.

Soll der Ladeträger auf eine andere Lokalität transportiert werden, so ist in das Feld Lokalität eine andere Lokalitätsnummer einzutragen. Das System bucht dann den Ladeträger automatisch von A nach B.

Soll eine Position nicht in auf einen Ladeträger gebucht werden, so wird das Feld Ladeträger einfach leergelassen.

Mit Starte ***Ladeträgerzuordnen***“ **SHIFT+F9** wird die Position auf den Ladeträger gebucht.

<p class="just-emphasize">Mengenverteilung auf unterschiedliche Ladeträger</p>

Soll eine Position auf mehrere Ladeträger verteilt werden, so wird die Positionsmenge in das Mengenfeld eingetragen. Es wird automatisch eine neue Position mit der Restmenge erzeugt. Der neuen Position kann dann ein Ladeträger zugeordnet werden. Wenn eine Position gelöscht wird, die aus einer anderen Position entstanden ist, so wird die Menge wieder auf die Ursprungsposition addiert.

<p class="just-emphasize">Löschen von Positionen</p>

Mit der Tastenkombination **STRG+ALT+ENTF** kann eine Zeile aus dem Grid entfernt werden. Wurde die zu löschende Position schon auf einen Ladeträger gebucht, so wird diese wieder vom Ladeträger entfernt. Durch das Löschen der Position im Grid wird die Position nicht im Lieferschein gelöscht. Beim erneuten Aufrufen der Maske wird die nicht zugeordnete Position wieder angezeigt und dieser kann dann ein Ladeträger zugeordnet werden.

<p class="just-emphasize">Umbuchen</p>

Wurde eine Position auf einen falschen Ladeträger zugeordnet oder eine Position soll auf ein anderen Ladeträger umgebucht werden, dann muss die entsprechende Ladeträgernummer durch die neue Nummer des Ladeträgers ersetzt werden.

Dieselbe Vorgehensweise gilt auch für die Lokalität.

Mit ausführen der Funktion „***Ladeträgerzuordnen***“ **SHIFT+F9** wird die Position umgebucht.
