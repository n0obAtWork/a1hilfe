# Tabulatorreihenfolge

<!-- source: https://amic.de/hilfe/tabulatorreihenfolge.htm -->

Funktionen bei der Tabulatorreihenfolge.

Wenn man auf ein Eingabefeld klickt, so öffnet sich eine Maske mit folgenden Feldern. In „**Aktives Feld**“ wird das Feld angezeigt, dass man gerade ausgewählt hat. Klick man dann auf eine der Funktionen „**Next Tabstop, PrevTabstop, Alter Next Tabstop, Alter Prev Tabstop**“, so schließt sich diese Maske und man kann dann in das Feld klicken, dass dann das nächste, vorherige,… in der Tabulatorreihenfolge werden soll.  
![Ein Bild, das Text, Screenshot, Display, Software enthält. KI-generierte Inhalte können fehlerhaft sein.](../../ImagesExt/image8_1464.png "Ein Bild, das Text, Screenshot, Display, Software enthält. KI-generierte Inhalte können fehlerhaft sein.")

| Funktion | Bedeutung |
| --- | --- |
| Next Tabstop | <br> |
| Prev Tabstop | <br> |
| Alter Next Tabstop | <br> |
| Alter Prev Tabstop | <br> |
| Aktives Feld | Anzeige des aktuellen Feldnamens<br> |
| Feldeinstellungen löschen | Setzt die Feldeinstellungen zurück<br> |
| Eingabezwang | Kann Ja und Nein annehmen,<br> |
| Tastaturfilter | Die Werte des Feldes werde durch das Anklicken der Zeile gesetzt. Diese Werte werden unterstützt.<br>• Unfiltered<br>• Digits only<br>• Yes-no<br>• Alphabetic<br>• Numeric<br>• Alphanumeric<br>• Regular Expression<br>• Edit Mask<br>Bei Regular Expression und Edit Mask öffnet sich ein weiteres Fenster in dem die entsprechenden Werte eingegeben werden können.<br> |
| Makroname(leer=JPL) | Hier kann ein Makro hinterlegt werden, in dem folgende Funktionen behandelt werden können. Ist kein Makro hinterlegt, so wird davon ausgegangen, dass die folgenden Funktionen JPL-Funktionen sind und im User JPL stehen.<br> |
| FieldEntry | Funktion die beim Betreten des Feldes ausgeführt wird.<br> |
| FieldExit | Funktion die beim Verlassen des Feldes ausgeführt wird.<br> |
| FieldValid | Funktion die vor dem Verlassen des Feldes ausgeführt wird.<br>Liefert diese Funktion einen Wert größer als 0 zurück, so wird das Feld nicht verlassen.<br> |
| ScreenEnty | Funktion, die beim Betreten und Verlassen der Maske ausgeführt werden. Sie hat zwei Parameter, der erste ist der Name der Maske und der zweite ist ein Flag, welches angibt, wie die Maske betreten bzw. verlassen wird.<br> |
| ScreenExit |
| |
| OnSaveValid<br> | Diese Funktionen erscheinen nur dann, wenn es sich bei der Maske technisch um ein Stammdateninterface handelt - zu erkennen am Aufruf der Maske über „jpl sd_interface“ mit 5,7,8 oder 77.<br>Alle diese Funktionen haben einen Parameter, der angibt, was gerade gemacht worden ist:<br>• 5 für Ändern<br>• 7 für Löschen<br>• 8 für Neu<br>• 77 für Undelete<br><br>Die Funktion OnsSaveValid wird aufgerufen, nachdem die programmeigene Funktion ausgeführt wurde und nur dann, wenn die programmeigene Funktion Ok signalisierte. OnSaveValid muss 0 zurückliefern, wenn Speichern erlaubt sein soll ansonsten einen beliebigen Wert größer 0, wenn nicht gespeichert oder gelöscht werden soll:<br> <br><pre><code>proc UserSaveValid(modus)&#10;{&#10; if (modus == 7)&#10; {&#10; msg emsg „löschen nicht erlaubt“&#10; return 1&#10; }&#10; Return 0&#10;}</code></pre><br> <br>Die anderen Funktionen werden aufgerufen, nachdem gespeichert wurde, oder Speichern mit Nein oder mit Abbruch verlassen wurde.<br>Wenn man ein User-JPL – nicht bei Verwendung eines Makros - angebunden hat, dann werden die Funktionen sofort angelegt. Beispiel für OnSaveJa:<br><br><br><pre><code>proc UserSaveJa(modus)&#10;{&#10; if (modus == 7)&#10; {&#10; return&#10; }&#10; return&#10;}</code></pre><br> <br>**Hinweis**: Der Modus 7 (Löschen) wird extra im Template separat behandelt, damit nicht vergessen wird, dass diese Funktion auch bei Löschen aufgerufen wird!<br> |
| OnSaveJa<br> |
| OnSaveNein<br> |
| *OnSaveAbbruch*<br> |

Wenn der Makroname leer ist und es sich somit um JPL handelt, müssen die verwendeten Funktionen mit „User-Jpl editieren“ noch mit Leben gefüllt werden,
